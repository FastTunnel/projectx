use std::sync::Arc;

use app_interface::user::IUserAppService;
use domain_common::error;
use domain_workspace::facade::IUserFacade;
use domain_workspace::model::role::Role;

pub struct UserFacade {
    user_app_service: Arc<dyn IUserAppService>,
}

impl UserFacade {
    pub fn new(user_app_service: Arc<dyn IUserAppService>) -> Self {
        UserFacade { user_app_service }
    }
}

#[async_trait::async_trait]
impl IUserFacade for UserFacade {
    async fn query_global_roles(&self, org_id: &str) -> error::Result<Vec<Role>> {
        todo!()
    }

    async fn query_roles_by_own(&self, org_id: &str, own: &str) -> error::Result<Vec<Role>> {
        todo!()
    }

    async fn query_users_by_ids(
        &self,
        ids: &Vec<String>,
    ) -> error::Result<Vec<domain_workspace::model::user::User>> {
        todo!()
    }

    async fn create_role(&self, role: &mut Vec<Role>) -> error::Result<()> {
        todo!()
    }
}
