use tokio::signal;
use tracing::debug;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

use app::args::parse_config_from_shell;
use app::config::AppConf;
use app::context::sys::adapter::init_sys_context_component;
use app::context::user::init_user_context_component;
use app::index::init_index;
use app::{init_db_pool, init_jwt_secret_keys, init_router};

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    init_tracing_log();

    init_sys_context_component();
    init_user_context_component();

    runtime.block_on(async {
        let config: AppConf = parse_config_from_shell().await;
        debug!("配置文件加载成功");

        init_jwt_secret_keys(&config);
        debug!("JWT密钥初始化成功");

        init_db_pool(&config).await;
        debug!("数据库连接池初始化成功");

        init_index(&config).await;
        debug!("全文搜索引擎初始化成功");

        let listener =
            tokio::net::TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))
                .await
                .expect("监听失败");
        debug!("监听成功");

        axum::serve(listener, init_router())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap();
    });
}

fn init_tracing_log() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .with(
            fmt::layer()
                .with_target(true)
                .with_line_number(true)
                .with_file(true)
                .with_ansi(true)
                .with_level(true)
                .pretty(),
        )
        .init();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
