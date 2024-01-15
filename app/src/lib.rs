extern crate core;

use axum::http::Method;
use axum::middleware::from_fn;
use std::any::TypeId;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;
use std::time::Duration;

use axum::routing::{get, post};
use axum::Router;
use ctor::ctor;
use dashmap::DashMap;
use sea_orm::{
    ConnectOptions, Database, DatabaseConnection, DatabaseTransaction, EntityTrait, FromQueryResult,
};

use tantivy::Index;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::log;

use crate::auth::auth;

use mem_event_bus::EventBus;

use crate::config::AppConf;
use crate::context::{user, Container};

pub mod args;
pub mod auth;
pub mod config;
pub mod context;
mod error;
pub mod index;

/// 宏函数，从容器中获取用户应用服务
#[macro_export]
macro_rules! app_service {
    (&$state:ident,$app_service:ty) => {{
        let service = $state
            .get_service::<$app_service>()
            .ok_or(crate::error::AppError::ServiceNotFound)
            .unwrap();
        service
            .downcast::<$app_service>()
            .ok_or(crate::error::AppError::ServiceNotFound)
            .unwrap()
    }};
    () => {};
}

#[macro_export]
macro_rules! define_repo {
    ($repo:ident) => {
        pub struct $repo {}

        impl $repo {
            pub fn new() -> Self {
                Self {}
            }
        }

        impl domain::Repository for $repo {
            type Transaction = crate::DbTx;
        }
    };
}

pub type DbPool = DatabaseConnection;
pub type DbTx = DatabaseTransaction;

pub struct AppState {
    services: DashMap<TypeId, Arc<Container>>,
    pub event_bus: EventBus,
    pub index: AtomicPtr<Index>,
    pub db_pool: AtomicPtr<DbPool>,
    pub keys: AtomicPtr<auth::Keys>,
}

impl Drop for AppState {
    fn drop(&mut self) {
        let index = self.index.swap(std::ptr::null_mut(), Ordering::Relaxed);
        if !index.is_null() {
            let _ = unsafe { Box::from_raw(index) };
        }
        let db_pool = self.db_pool.swap(std::ptr::null_mut(), Ordering::Relaxed);
        if !db_pool.is_null() {
            let _ = unsafe { Box::from_raw(db_pool) };
        }
        let key = self.keys.swap(std::ptr::null_mut(), Ordering::Relaxed);
        if !key.is_null() {
            let _ = unsafe { Box::from_raw(key) };
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            services: DashMap::new(),
            event_bus: Default::default(),
            index: AtomicPtr::new(std::ptr::null_mut()),
            db_pool: AtomicPtr::new(std::ptr::null_mut()),
            keys: AtomicPtr::new(std::ptr::null_mut()),
        }
    }

    pub fn add_service<T: ?Sized + 'static>(&self, service: Arc<Container>) {
        self.services.insert(TypeId::of::<T>(), service);
    }

    pub fn get_service<T: ?Sized + 'static>(&self) -> Option<Arc<Container>> {
        self.services
            .get(&TypeId::of::<T>())
            .map(|v| v.value().clone())
    }

    pub fn db_tx(&self) -> &'static DbPool {
        unsafe { self.db_pool.load(Ordering::Relaxed).as_ref().unwrap() }
    }
}

#[ctor]
static APP_STATE: Arc<AppState> = Arc::new(AppState::new());

pub fn init_jwt_secret_keys(config: &AppConf) {
    let keys = auth::Keys::new(config.jwt_secret.as_bytes());
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
                .nest("/user", user::adapter::controller::init_user_router())
                .nest("/org", user::adapter::controller::init_org_router()),
        )
        .layer(from_fn(auth))
        .route("/authorize", post(user::adapter::controller::user::login))
        .route(
            "/init",
            post(user::adapter::controller::organization::init_system),
        )
        .route("/", get(context::index))
        .layer(
            CorsLayer::new()
                .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_origin(Any)
                .allow_credentials(false),
        )
        .layer(TraceLayer::new_for_http())
}

pub fn parse_query_to_model<M: FromQueryResult, E: EntityTrait>(
    res: &sea_orm::QueryResult,
) -> Result<M, sea_orm::DbErr> {
    M::from_query_result(res, E::default().table_name())
}
