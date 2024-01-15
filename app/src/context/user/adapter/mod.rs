use std::sync::Arc;

use domain::user::service::UserService;

use crate::context::sys::application::service::ConfigAppService;
use crate::context::user::adapter::client::config::ConfigFacade;
use crate::context::user::adapter::client::jwt::JwtFacade;
use crate::context::user::adapter::publisher::user::UserEventPublisher;
use crate::context::user::adapter::repository::organization::OrganizationRepository;
use crate::context::user::adapter::repository::role::RoleRepository;
use crate::context::user::adapter::repository::team::TeamRepository;
use crate::context::user::adapter::repository::user::UserRepository;
use crate::context::Container;
use crate::{app_service, APP_STATE};

use crate::context::user::application::service::UserAppService;

mod client;
pub mod controller;
mod publisher;
mod repository;
mod subscriber;

pub fn init_user_context_component() {
    let user_repository = UserRepository::new();
    let role_repository = RoleRepository::new();
    let organization_repository = OrganizationRepository::new();
    let team_repository = TeamRepository::new();
    let user_event_publisher = UserEventPublisher::new();
    let jwt_facade = JwtFacade::new();
    let config_app_service = app_service!(&APP_STATE, ConfigAppService);
    let config_facade = ConfigFacade::new(config_app_service);
    let user_service = UserService::new(
        Arc::new(user_repository),
        Arc::new(role_repository),
        Arc::new(organization_repository),
        Arc::new(team_repository),
        Arc::new(user_event_publisher),
        Arc::new(jwt_facade),
        Arc::new(config_facade),
    );
    let container = Container {
        inner: Arc::new(UserAppService::new(Arc::new(user_service))),
    };
    APP_STATE.add_service::<UserAppService>(Arc::new(container));
}
