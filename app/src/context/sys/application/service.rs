use std::sync::Arc;

use sea_orm::TransactionTrait;

use domain::sys::service::IConfigService;

use crate::context::sys::application::IConfigAppService;
use crate::{error, DbTx};

pub struct ConfigAppService {
    config_service: Arc<dyn IConfigService<DbTx>>,
}

impl ConfigAppService {
    pub fn new(config_service: Arc<dyn IConfigService<DbTx>>) -> Self {
        Self { config_service }
    }
}

#[async_trait::async_trait]
impl IConfigAppService for ConfigAppService {
    async fn init_system(&self) -> error::Result<()> {
        let mut transaction = crate::APP_STATE.db_tx().begin().await?;
        self.config_service.sys_init(&mut transaction).await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn ays_is_init(&self) -> error::Result<bool> {
        let mut transaction = crate::APP_STATE.db_tx().begin().await?;
        let is_init = self.config_service.sys_is_init(&mut transaction).await?;
        transaction.commit().await?;
        Ok(is_init)
    }
}
