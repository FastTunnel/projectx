use crate::model::Config;
use async_trait::async_trait;
use domain_common::{error, Repository};

#[async_trait]
pub trait IConfigRepository: Repository {
    async fn save(&self, tx: &mut Self::Transaction, config: &mut Config) -> error::Result<()>;
    async fn find_config(
        &self,
        tx: &mut Self::Transaction,
        config: &String,
    ) -> error::Result<Option<Config>>;
    async fn find_config_list(
        &self,
        tx: &mut Self::Transaction,
        key_prefix: &String,
    ) -> error::Result<Vec<Config>>;
}
