use app_interface::system::IConfigAppService;
use domain_common::error;
use domain_common::error::DomainError;
use std::sync::Arc;

use domain_user::facade::IConfigFacade;

pub struct ConfigFacade {
    config_app_service: Arc<dyn IConfigAppService>,
}

impl ConfigFacade {
    pub fn new(config_app_service: Arc<dyn IConfigAppService>) -> Self {
        ConfigFacade { config_app_service }
    }
}

#[async_trait::async_trait]
impl IConfigFacade for ConfigFacade {
    async fn sys_is_init(&self) -> error::Result<bool> {
        self.config_app_service
            .sys_is_init()
            .await
            .map_err(|e| DomainError::CallClientError(e.into()))
    }

    async fn sys_init(&self) -> error::Result<()> {
        self.config_app_service
            .init_system()
            .await
            .map_err(|e| DomainError::CallClientError(e.into()))
    }
}
