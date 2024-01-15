use crate::error;
use crate::user::model::User;
use async_trait::async_trait;

pub enum UserEvent {
    Created(String, User),
}

#[async_trait]
pub trait IUserEventPublisher: Send + Sync {
    async fn publish(&self, event: UserEvent) -> error::Result<()>;
}
