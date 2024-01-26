use std::sync::Arc;

use app_interface::system::IConfigAppService;
use app_interface::user::IUserAppService;
use app_interface::APP_STATE;
use domain_workspace::service::WorkspaceService;

use crate::adapter::client::global::GlobalConfigFacade;
use crate::adapter::client::user::UserFacade;
use crate::adapter::repository::space::SpaceRepository;
use crate::application::service::WorkspaceAppService;

pub mod adapter;
pub mod application;

pub fn init_workspace_context_component() {
    let user_app_service = APP_STATE.get_service::<dyn IUserAppService>();
    let config_app_service = APP_STATE.get_service::<dyn IConfigAppService>();
    let user_facade = Arc::new(UserFacade::new(user_app_service));
    let global_config_facade = Arc::new(GlobalConfigFacade::new(config_app_service));
    let space_repository = Arc::new(SpaceRepository::new());
    let workspace_service = WorkspaceService::new(
        user_facade.clone(),
        global_config_facade.clone(),
        space_repository.clone(),
    );
    let workspace_app_service = WorkspaceAppService::new(
        Arc::new(workspace_service),
        global_config_facade,
        user_facade,
        space_repository,
    );

    let workspace_app_service_arc: Arc<dyn app_interface::workspace::IWorkspaceAppService> =
        Arc::new(workspace_app_service);
    APP_STATE.add_service(workspace_app_service_arc);
}
