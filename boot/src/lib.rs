extern crate core;

use std::sync::atomic::Ordering;
use std::time::Duration;

use axum::http::Method;
use axum::middleware::from_fn;
use axum::routing::post;
use axum::{Json, Router};
use axum_valid::Valid;
use sea_orm::{ConnectOptions, Database, EntityTrait, FromQueryResult};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::log;

use app_interface::user::dto::command::InitSystemCommand;
use app_interface::user::IUserAppService;
use app_interface::workspace::IWorkspaceAppService;
use app_interface::{app_service, APP_STATE};

use crate::auth::{auth_middleware, handler_404, login};
use crate::config::AppConf;
use crate::controller::{init_org_router, init_user_router, init_workspace_router};

pub mod args;
pub mod auth;
pub mod config;

mod controller;
pub mod error;
pub mod index;

pub fn init_jwt_secret_keys(config: &AppConf) {
    let keys = app_interface::auth::Keys::new(config.jwt_secret.as_bytes());
    APP_STATE
        .keys
        .store(Box::into_raw(Box::new(keys)) as *mut _, Ordering::Relaxed);
}

pub async fn init_db_pool(config: &AppConf) {
    let mut opt = ConnectOptions::new(config.database.url.as_str());
    opt.min_connections(config.database.min_connections)
        .max_connections(config.database.max_connections)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    let pool = Database::connect(opt).await.expect("数据库连接失败");
    tracing::debug!("数据库连接成功");

    APP_STATE
        .db_pool
        .store(Box::into_raw(Box::new(pool)) as *mut _, Ordering::Relaxed);
}

pub fn init_router() -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/user", init_user_router())
                .nest("/org", init_org_router())
                .nest("/workspace", init_workspace_router()),
        )
        .layer(from_fn(auth_middleware))
        .route("/login", post(login))
        .route("/init", post(init_system))
        .layer(
            CorsLayer::new()
                .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_origin(Any)
                .allow_credentials(false),
        )
        .fallback(handler_404)
        .layer(TraceLayer::new_for_http())
}

pub fn parse_query_to_model<M: FromQueryResult, E: EntityTrait>(
    res: &sea_orm::QueryResult,
) -> Result<M, sea_orm::DbErr> {
    M::from_query_result(res, E::default().table_name())
}

#[tracing::instrument()]
pub async fn init_system(
    Valid(Json(command)): Valid<Json<InitSystemCommand>>,
) -> error::Result<()> {
    let user_app_service = app_service!(IUserAppService);
    let workspace_app_service = app_service!(IWorkspaceAppService);
    let (org, _user) = user_app_service.init_system(&command).await?;
    workspace_app_service.init_system(&org.identifier).await?;
    tracing::debug!("初始化系统成功");
    Ok(())
}
