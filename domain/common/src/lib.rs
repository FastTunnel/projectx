pub mod error;

use async_trait::async_trait;

#[async_trait]
pub trait Repository: Send + Sync {
    type Transaction;
}

#[async_trait]
pub trait Service: Send + Sync {}
