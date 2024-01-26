use crate::model::Config;
use crate::repository::IConfigRepository;
use async_trait::async_trait;
use domain_common::error;
use serde_json::Value;
use std::sync::Arc;

#[async_trait]
pub trait IConfigService<T>: Send + Sync {
    async fn save(&self, tx: &mut T, key: &str, value: &str) -> error::Result<()>;
    async fn get_config(&self, tx: &mut T, key: &str) -> error::Result<Option<Config>>;
    async fn get_config_list(&self, tx: &mut T, key_prefix: &str) -> error::Result<Vec<Config>>;
}

pub struct ConfigService<T> {
    config_repo: Arc<dyn IConfigRepository<Transaction = T>>,
}

impl<T> ConfigService<T> {
    pub fn new(config_repo: Arc<dyn IConfigRepository<Transaction = T>>) -> Self {
        Self { config_repo }
    }
}

#[async_trait]
impl<T> IConfigService<T> for ConfigService<T>
where
    T: Send + Sync,
{
    async fn save(&self, tx: &mut T, key: &str, value: &str) -> error::Result<()> {
        let mut config = Config {
            key: key.to_string(),
            value: Value::String(value.to_string()),
            version: 0,
        };
        self.config_repo.save(tx, &mut config).await?;
        Ok(())
    }

    async fn get_config(&self, tx: &mut T, key: &str) -> error::Result<Option<Config>> {
        todo!()
    }

    async fn get_config_list(&self, tx: &mut T, key_prefix: &str) -> error::Result<Vec<Config>> {
        todo!()
    }
}
