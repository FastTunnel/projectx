use std::any::TypeId;
use std::sync::Arc;

use app_interface::system::IConfigAppService;
use app_interface::APP_STATE;
use domain_system::service::ConfigService;

use crate::adapter::repository::config::ConfigRepository;
use crate::application::service::ConfigAppService;

pub mod adapter;
pub mod application;

pub fn init_sys_context_component() {
    let config_repository = ConfigRepository::new();
    let config_service = ConfigService::new(Arc::new(config_repository));
    let config_app_service = ConfigAppService::new(Arc::new(config_service));
    let config_app_service_arc: Arc<dyn IConfigAppService> = Arc::new(config_app_service);
    APP_STATE.add_service(config_app_service_arc);
}
