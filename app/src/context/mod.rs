use std::any::Any;
use std::sync::Arc;

use axum::response::IntoResponse;
use tracing::instrument;

pub mod sys;
pub mod user;

pub struct Container {
    pub inner: Arc<dyn Any + Send + Sync + 'static>,
}

impl Container {
    pub fn downcast<T: 'static + Sync + Send>(&self) -> Option<Arc<T>> {
        self.inner.clone().downcast::<T>().ok()
    }
}

#[instrument]
pub async fn index() -> impl IntoResponse {}
