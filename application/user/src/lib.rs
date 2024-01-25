use std::any::TypeId;
use std::sync::Arc;

use app_interface::system::IConfigAppService;
use app_interface::user::IUserAppService;
use app_interface::{app_service, APP_STATE};
use domain_user::service::UserService;

use crate::adapter::client::config::ConfigFacade;
use crate::adapter::client::jwt::JwtFacade;
use crate::adapter::publisher::user::UserEventPublisher;
use crate::adapter::repository::organization::OrganizationRepository;
use crate::adapter::repository::role::RoleRepository;
use crate::adapter::repository::team::TeamRepository;
use crate::adapter::repository::user::UserRepository;
use crate::application::service::UserAppService;

pub mod adapter;
pub mod application;

pub fn init_user_context_component() {
    let user_repository = Arc::new(UserRepository::new());
    let role_repository = Arc::new(RoleRepository::new());
    let organization_repository = OrganizationRepository::new();
    let team_repository = TeamRepository::new();
    let user_event_publisher = UserEventPublisher::new();
    let jwt_facade = JwtFacade::new();
    let config_app_service = APP_STATE.get_service::<dyn IConfigAppService>();
    let config_facade = ConfigFacade::new(config_app_service);
    let user_service = UserService::new(
        user_repository.clone(),
        role_repository.clone(),
        Arc::new(organization_repository),
        Arc::new(team_repository),
        Arc::new(user_event_publisher),
        Arc::new(jwt_facade),
        Arc::new(config_facade),
    );
    let user_app_service = UserAppService::new(Arc::new(user_service), role_repository.clone());
    let user_app_service_arc: Arc<dyn IUserAppService> = Arc::new(user_app_service);
    APP_STATE.add_service(user_app_service_arc)
}
