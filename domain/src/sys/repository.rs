use async_trait::async_trait;

use crate::sys::model::Config;
use crate::{error, Repository};

#[async_trait]
pub trait IConfigRepository: Repository {
    async fn save(&self, tx: &mut Self::Transaction, config: &mut Config) -> error::Result<()>;
    async fn get_config(
        &self,
        tx: &mut Self::Transaction,
        config: &String,
    ) -> error::Result<Option<Config>>;
}
