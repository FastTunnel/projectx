use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::{Arc, RwLock};

use once_cell::sync::Lazy;
use sea_orm::{DatabaseConnection, DatabaseTransaction};
use tantivy::Index;

use mem_event_bus::EventBus;

pub mod auth;
pub mod error;
pub mod system;
pub mod user;
pub mod utils;
pub mod workspace;

#[macro_export]
macro_rules! define_repo {
    ($repo:ident) => {
        pub struct $repo {}

        impl $repo {
            pub fn new() -> Self {
                Self {}
            }
        }

        impl domain_common::Repository for $repo {
            type Transaction = app_interface::DbTx;
        }
    };
}

pub type DbPool = DatabaseConnection;
pub type DbTx = DatabaseTransaction;

pub struct AppState {
    services: RwLock<anymap2::SendSyncAnyMap>,
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
            services: RwLock::new(anymap2::SendSyncAnyMap::new()),
            event_bus: Default::default(),
            index: AtomicPtr::new(std::ptr::null_mut()),
            db_pool: AtomicPtr::new(std::ptr::null_mut()),
            keys: AtomicPtr::new(std::ptr::null_mut()),
        }
    }

    pub fn add_service<T: ?Sized + Send + Sync + 'static>(&self, v: Arc<T>) {
        let _old = self.services.write().unwrap().insert(v);
    }
    pub fn get_service<T: ?Sized + Send + Sync + 'static>(&self) -> Arc<T> {
        let guard = self.services.read().unwrap();
        let option: Option<&Arc<T>> = guard.get();
        option.cloned().expect("can not find service")
    }

    pub fn db_tx(&self) -> &'static DbPool {
        unsafe { self.db_pool.load(Ordering::Relaxed).as_ref().unwrap() }
    }
}

#[macro_export]
macro_rules! app_service {
    ($app_service:path) => {{
        app_interface::APP_STATE.get_service::<dyn $app_service>()
    }};
    () => {};
}

pub static APP_STATE: Lazy<Arc<AppState>> = Lazy::new(|| Arc::new(AppState::new()));
