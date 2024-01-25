use crate::error;
use crate::workspace::dto::command::TemplateCreateCommand;
use crate::workspace::dto::TemplateDTO;
use async_trait::async_trait;

pub mod dto {
    use chrono::{DateTime, Utc};
    use domain_workspace::model::role::Role;
    use domain_workspace::model::setting::template::Template;
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    pub mod command {
        use serde::Deserialize;
        use validator::Validate;

        #[derive(Debug, Deserialize, Validate)]
        pub struct TemplateCreateCommand {
            #[validate(length(min = 5, max = 100, message = "Can not be empty"))]
            pub organization: String,
            #[validate(length(min = 5, max = 100, message = "Can not be empty"))]
            pub name: String,
            #[validate(length(min = 5, max = 100, message = "Can not be empty"))]
            pub display_name: String,
            pub custom_code: String,
            pub description: Option<String>,
            pub icon: Option<String>,
        }
    }

    pub mod query {
        use serde::Deserialize;
        use validator::Validate;

        #[derive(Debug, Deserialize, Validate)]
        pub struct TemplateQuery {
            #[validate(length(min = 5, max = 100, message = "Can not be empty"))]
            pub organization: String,
            pub template: Option<String>,
        }
    }

    #[derive(Debug, Serialize, Deserialize, Validate)]
    pub struct RoleDTO {
        pub id: u64,
        pub identifier: String,
        pub own: Option<String>,
        pub name: String,
        pub description: Option<String>,
        pub organization: String,
        pub parent: Option<String>,
        pub default_role: bool,
        pub gmt_create: DateTime<Utc>,
        pub creator: String,
        pub is_project_set_role: bool,
        pub gmt_modified: Option<DateTime<Utc>>,
        pub modifier: Option<String>,
    }

    impl Into<RoleDTO> for Role {
        fn into(self) -> RoleDTO {
            RoleDTO {
                id: self.id,
                identifier: self.identifier,
                own: self.own,
                name: self.name,
                description: self.description,
                organization: self.organization,
                parent: self.parent,
                default_role: self.default_role,
                gmt_create: self.gmt_create,
                creator: self.creator,
                is_project_set_role: self.is_project_set_role,
                gmt_modified: self.gmt_modified,
                modifier: self.modifier,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Validate)]
    pub struct TemplateDTO {
        pub gmt_create: DateTime<Utc>,
        pub gmt_modified: Option<DateTime<Utc>>,
        pub creator: String,
        pub modifier: Option<String>,
        pub identifier: String,
        pub name: String,
        pub display_name: String,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub enable: bool,
        pub organization: String,
        pub roles: Vec<RoleDTO>,
    }

    impl Into<TemplateDTO> for Template {
        fn into(self) -> TemplateDTO {
            TemplateDTO {
                gmt_create: self.gmt_create,
                gmt_modified: self.gmt_modified,
                creator: self.creator,
                modifier: self.modifier,
                identifier: self.identifier,
                name: self.name,
                display_name: self.display_name,
                description: self.description,
                icon: self.icon,
                enable: self.enable,
                organization: self.organization,
                roles: vec![],
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Validate)]
    pub struct GlobalConfigDTO {}
}

#[async_trait]
pub trait IWorkspaceAppService: Send + Sync {
    async fn init_system(&self, org_id: &str) -> error::Result<()>;

    async fn find_all_template(
        &self,
        organization: &String,
    ) -> crate::error::Result<Vec<dto::TemplateDTO>>;
    async fn template_detail(
        &self,
        organization: &String,
        template_id: &String,
    ) -> crate::error::Result<Option<TemplateDTO>>;

    async fn create_template(
        &self,
        template: &TemplateCreateCommand,
        creator: &str,
    ) -> crate::error::Result<String>;
}
