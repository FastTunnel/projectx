use crate::error;
use async_trait::async_trait;

#[async_trait]
pub trait IJwtFacade: Sync + Send {
    async fn generate_token(&self, user_id: &String, username: &String) -> error::Result<String>;
}

#[async_trait]
pub trait IConfigFacade: Sync + Send {
    async fn sys_is_init(&self) -> error::Result<bool>;
    async fn sys_init(&self) -> error::Result<()>;
}
