use crate::model::role::Role;
use crate::model::setting::global::GlobalConfig;
use crate::model::setting::template::Template;
use async_trait::async_trait;
use domain_common::error;

#[async_trait::async_trait]
pub trait IUserFacade: Send + Sync {
    async fn query_global_roles(&self, org_id: &str) -> error::Result<Vec<Role>>;
    async fn query_roles_by_own(&self, org_id: &str, own: &str) -> error::Result<Vec<Role>>;
    async fn create_role(&self, role: &mut Vec<Role>) -> error::Result<()>;
}

#[async_trait]
pub trait IGlobalConfigFacade: Send + Sync {
    async fn find_global_config_by_org(
        &self,
        organization: &String,
    ) -> error::Result<Option<GlobalConfig>>;
    async fn find_template_by_identifier(
        &self,
        organization: &String,
        identifier: &String,
    ) -> error::Result<Option<Template>>;
    async fn find_all_template(&self, organization: &String) -> error::Result<Vec<Template>>;
    async fn save_global_config(&self, global_config: &mut GlobalConfig) -> error::Result<()>;

    async fn save_templates(&self, templates: &mut Vec<Template>) -> error::Result<()>;
    async fn save_template(&self, templates: &mut Template) -> error::Result<()>;
}
