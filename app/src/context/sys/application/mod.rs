use crate::error;
use async_trait::async_trait;

mod dto;
pub mod service;

#[async_trait]
pub trait IConfigAppService: Send + Sync {
    async fn init_system(&self) -> error::Result<()>;
    async fn ays_is_init(&self) -> error::Result<bool>;
}
