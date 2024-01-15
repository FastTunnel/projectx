use std::sync::Arc;

use domain::sys::service::ConfigService;

use crate::context::sys::adapter::repository::config::ConfigRepository;
use crate::context::sys::application::service::ConfigAppService;
use crate::context::Container;
use crate::APP_STATE;

pub(super) mod repository;

pub fn init_sys_context_component() {
    let config_repository = ConfigRepository::new();
    let config_service = ConfigService::new(Arc::new(config_repository));
    let config_app_service = ConfigAppService::new(Arc::new(config_service));
    let container = Container {
        inner: Arc::new(config_app_service),
    };
    APP_STATE.add_service::<ConfigAppService>(Arc::new(container));
}
