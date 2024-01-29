use sea_orm::TransactionTrait;
use std::sync::Arc;

use app_interface::system::IConfigAppService;
use app_interface::{error, DbTx, APP_STATE};
use domain_common::error::DomainError;
use domain_system::model::Config;

use domain_system::service::IConfigService;

pub struct ConfigAppService {
    config_service: Arc<dyn IConfigService<DbTx>>,
    sys_init_key: String,
}

impl ConfigAppService {
    pub fn new(config_service: Arc<dyn IConfigService<DbTx>>) -> Self {
        Self {
            config_service,
            sys_init_key: "/sys_init".to_string(),
        }
    }
}

#[async_trait::async_trait]
impl IConfigAppService for ConfigAppService {
    async fn init_system(&self) -> error::Result<()> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        self.config_service
            .save(&mut transaction, &self.sys_init_key, "true")
            .await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn sys_is_init(&self) -> error::Result<bool> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        let result = self
            .config_service
            .get_config(&mut transaction, &self.sys_init_key)
            .await
            .map_err(|e| DomainError::CallClientError(e.into()))?;
        transaction.commit().await?;
        match result {
            None => Ok(false),
            Some(_) => Ok(true),
        }
    }

    async fn save(&self, key: &str, value: &str) -> error::Result<()> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        self.config_service
            .save(&mut transaction, key, value)
            .await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn get_config(&self, key: &str) -> error::Result<Option<Config>> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        let result = self
            .config_service
            .get_config(&mut transaction, key)
            .await?;
        transaction.commit().await?;
        Ok(result)
    }

    async fn get_config_list(&self, key_prefix: &str) -> error::Result<Vec<Config>> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        let result = self
            .config_service
            .get_config_list(&mut transaction, key_prefix)
            .await?;
        transaction.commit().await?;
        Ok(result)
    }
}
