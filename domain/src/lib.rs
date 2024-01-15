use async_trait::async_trait;

pub mod error;
pub mod sys;
pub mod user;
pub mod workspace;

#[async_trait]
pub trait Repository: Send + Sync {
    type Transaction;
}

#[async_trait]
pub trait Service: Send + Sync {}
