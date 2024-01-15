use std::sync::Arc;

use crate::context::sys::application::IConfigAppService;
use domain::user::facade::IConfigFacade;

use crate::context::sys::application::service::ConfigAppService;

pub struct ConfigFacade {
    config_app_service: Arc<ConfigAppService>,
}

impl ConfigFacade {
    pub fn new(config_app_service: Arc<ConfigAppService>) -> Self {
        ConfigFacade { config_app_service }
    }
}

#[async_trait::async_trait]
impl IConfigFacade for ConfigFacade {
    async fn sys_is_init(&self) -> domain::error::Result<bool> {
        self.config_app_service
            .ays_is_init()
            .await
            .map_err(|e| domain::error::DomainError::CallClientError(e.into()))
    }

    async fn sys_init(&self) -> domain::error::Result<()> {
        self.config_app_service
            .init_system()
            .await
            .map_err(|e| domain::error::DomainError::CallClientError(e.into()))
    }
}
