use async_trait::async_trait;
use domain_workspace::enums::ResourceType;
use domain_workspace::model::setting::base::FlowItem;

use crate::error;
use crate::workspace::dto::command::{
    ProjectCreateCommand, ProjectSetCreateCommand, SpaceMemberAddCommand, SpaceMemberRemoveCommand,
    TemplateCreateCommand,
};
use crate::workspace::dto::{
    ProjectDTO, ProjectSetDTO, SpaceWorkItemSetDTO, TagDTO, TemplateDTO, UserDTO,
};

pub mod dto {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    use domain_workspace::enums::{Category, ResourceType};
    use domain_workspace::model::project::{Project, ProjectSet};
    use domain_workspace::model::role::Role;
    use domain_workspace::model::setting::base::FlowItem;
    use domain_workspace::model::setting::field::Field;
    use domain_workspace::model::setting::space_work_item_set::SpaceWorkItemSet;
    use domain_workspace::model::setting::status::Status;
    use domain_workspace::model::setting::template::Template;
    use domain_workspace::model::tag::Tag;
    use domain_workspace::model::user::User;

    pub mod command {
        use domain_workspace::enums::ResourceType;
        use serde::Deserialize;
        use validator::{Validate, ValidationError, ValidationErrors};

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

        #[derive(Debug, Deserialize, Validate)]
        pub struct ProjectCreateCommand {
            pub name: String,
            pub custom_code: String,
            pub description: Option<String>,
            pub icon: Option<String>,
            pub organization: String,
            pub project_set: Option<String>,
            pub template: String,
        }
        #[derive(Debug, Deserialize, Validate)]
        pub struct ProjectSetCreateCommand {
            pub name: String,
            pub custom_code: String,
            pub description: Option<String>,
            pub icon: Option<String>,
            pub organization: String,
        }
        #[derive(Debug, Deserialize, Validate)]
        pub struct SpaceMemberAddCommand {
            pub space_type: String,
            pub user_ids: Vec<String>,
        }

        impl Validate for SpaceMemberAddCommand {
            fn validate(&self) -> Result<(), ValidationErrors> {
                let mut errors = ValidationErrors::new();
                if self.space_type != ResourceType::ProjectSet.to_string()
                    && self.space_type != ResourceType::Project
                {
                    errors.add(
                        "space_type",
                        ValidationError::new("space_type must be project or project_set"),
                    );
                }
                if self.user_ids.is_empty() {
                    errors.add(
                        "user_ids",
                        ValidationError::new("user_ids can not be empty"),
                    );
                }
                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }
        }

        #[derive(Debug, Deserialize, Validate)]
        pub struct SpaceMemberRemoveCommand {
            pub space_type: String,
            pub user_ids: Vec<String>,
        }

        impl Validate for SpaceMemberRemoveCommand {
            fn validate(&self) -> Result<(), ValidationErrors> {
                let mut errors = ValidationErrors::new();
                if self.space_type != ResourceType::ProjectSet.to_string()
                    && self.space_type != ResourceType::Project
                {
                    errors.add(
                        "space_type",
                        ValidationError::new("space_type must be project or project_set"),
                    );
                }
                if self.user_ids.is_empty() {
                    errors.add(
                        "user_ids",
                        ValidationError::new("user_ids can not be empty"),
                    );
                }
                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }
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
        #[derive(Debug, Deserialize, Validate)]
        pub struct ProjectQuery {
            pub organization: String,
            pub project_set: Option<String>,
        }
        #[derive(Debug, Deserialize, Validate)]
        pub struct ProjectSetQuery {
            pub organization: String,
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
    pub struct SpaceWorkItemSetDTO {
        pub id: u64,
        pub identifier: String,
        pub category: Category,
        pub creator: String,
        pub gmt_create: DateTime<Utc>,
        pub gmt_modified: Option<DateTime<Utc>>,
        pub modifier: Option<String>,
        pub name: String,
        pub name_en: String,
        pub display_name: String,
        pub description: String,
        pub space: String,
        pub is_deleted: bool,
        pub is_system: bool,
        pub work_item_status_flow: Vec<FlowItem>,
        pub organization: String,
    }

    impl Into<SpaceWorkItemSetDTO> for SpaceWorkItemSet {
        fn into(self) -> SpaceWorkItemSetDTO {
            SpaceWorkItemSetDTO {
                id: self.id,
                identifier: self.identifier,
                category: self.category,
                creator: self.creator,
                gmt_create: self.gmt_create,
                gmt_modified: self.gmt_modified,
                modifier: self.modifier,
                name: self.name,
                name_en: self.name_en,
                display_name: self.display_name,
                description: self.description,
                space: self.space,
                is_deleted: self.is_deleted,
                is_system: self.is_system,
                work_item_status_flow: self.work_item_status_flow,
                organization: self.organization,
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
    pub struct ProjectDTO {
        pub id: u64,
        pub identifier: String,
        pub organization: String,
        pub custom_code: String,
        pub description: Option<String>,
        pub gmt_create: DateTime<Utc>,
        pub gmt_modified: Option<DateTime<Utc>>,
        pub has_superior_space: bool,
        pub icon: Option<String>,
        pub creator: String,
        pub modifier: Option<String>,
        pub name: String,
        pub parent: Option<String>,
        pub template: String,
        pub status: Option<StatusDTO>,
    }

    impl Into<ProjectDTO> for Project {
        fn into(self) -> ProjectDTO {
            ProjectDTO {
                id: self.id,
                identifier: self.identifier,
                organization: self.organization,
                custom_code: self.custom_code,
                description: self.description,
                gmt_create: self.gmt_create,
                gmt_modified: self.gmt_modified,
                has_superior_space: self.has_superior_space,
                icon: self.icon,
                creator: self.creator,
                modifier: self.modifier,
                name: self.name,
                parent: self.parent,
                template: self.template,
                status: self.status.map(|s| s.into()),
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Validate)]
    pub struct StatusDTO {
        pub id: u64,
        pub identifier: String,
        pub description: String,
        pub name: String,
        pub name_en: String,
        pub gmt_create: DateTime<Utc>,
        pub gmt_modified: Option<DateTime<Utc>>,
        pub creator: String,
        pub modifier: Option<String>,
        pub stage_code: String,
        pub organization: String,
        pub resource_type: ResourceType,
    }

    impl Into<StatusDTO> for Status {
        fn into(self) -> StatusDTO {
            StatusDTO {
                id: self.id,
                identifier: self.identifier,
                description: self.description,
                name: self.name,
                name_en: self.name_en,
                gmt_create: self.gmt_create,
                gmt_modified: self.gmt_modified,
                creator: self.creator,
                modifier: self.modifier,
                stage_code: self.stage_code,
                organization: self.organization,
                resource_type: self.resource_type,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Validate)]
    pub struct ProjectSetDTO {
        pub id: u64,
        pub identifier: String,
        pub organization: String,
        pub custom_code: String,
        pub description: Option<String>,
        pub gmt_create: DateTime<Utc>,
        pub gmt_modified: Option<DateTime<Utc>>,
        pub icon: Option<String>,
        pub creator: String,
        pub modifier: Option<String>,
        pub name: String,
        pub status: Option<StatusDTO>,
    }

    impl Into<ProjectSetDTO> for ProjectSet {
        fn into(self) -> ProjectSetDTO {
            ProjectSetDTO {
                id: self.id,
                identifier: self.identifier,
                organization: self.organization,
                custom_code: self.custom_code,
                description: self.description,
                gmt_create: self.gmt_create,
                gmt_modified: self.gmt_modified,
                icon: self.icon,
                creator: self.creator,
                modifier: self.modifier,
                name: self.name,
                status: self.status.map(|s| s.into()),
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Validate)]
    pub struct UserDTO {
        pub id: u64,
        pub identifier: String,
        pub name: String,
        pub email: Option<String>,
    }

    impl Into<UserDTO> for User {
        fn into(self) -> UserDTO {
            UserDTO {
                id: self.id,
                identifier: self.identifier,
                name: self.name,
                email: self.email,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Validate)]
    pub struct TagDTO {
        pub id: u64,
        pub identifier: String,
        pub name: String,
        pub color: String,
        pub space: String,
    }

    impl Into<TagDTO> for Tag {
        fn into(self) -> TagDTO {
            TagDTO {
                id: self.id,
                identifier: self.identifier,
                name: self.name,
                color: self.color,
                space: self.space,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Validate)]
    pub struct GlobalConfigDTO {}
}

#[async_trait]
pub trait IWorkspaceAppService: Send + Sync {
    async fn init_system(&self, org_id: &str) -> error::Result<()>;

    async fn find_all_template(&self, organization: &String) -> error::Result<Vec<TemplateDTO>>;
    async fn query_all_project_set(
        &self,
        organization: &String,
    ) -> error::Result<Vec<ProjectSetDTO>>;
    async fn query_all_project(
        &self,
        organization: &String,
        project_set: Option<&String>,
    ) -> error::Result<Vec<ProjectDTO>>;

    async fn template_detail(
        &self,
        organization: &String,
        template_id: &String,
    ) -> error::Result<Option<TemplateDTO>>;

    async fn create_template(
        &self,
        template: &TemplateCreateCommand,
        creator: &str,
    ) -> error::Result<String>;

    async fn create_project(
        &self,
        space: &ProjectCreateCommand,
        creator: &str,
    ) -> error::Result<String>;
    async fn create_project_set(
        &self,
        space: &ProjectSetCreateCommand,
        creator: &str,
    ) -> error::Result<String>;

    async fn query_space_member(&self, space_id: &String) -> error::Result<Vec<UserDTO>>;

    async fn query_space_tag(&self, space_id: &String) -> error::Result<Vec<TagDTO>>;

    async fn query_space_status_flow(
        &self,
        space_type: &String,
        space_id: &String,
    ) -> error::Result<Vec<FlowItem>>;
    async fn query_space_work_item_set(
        &self,
        space_id: &String,
        category: &String,
    ) -> error::Result<Vec<SpaceWorkItemSetDTO>>;

    async fn space_member_add(
        &self,
        space_id: &String,
        command: &SpaceMemberAddCommand,
        creator: &str,
    ) -> error::Result<()>;

    async fn space_member_remove(
        &self,
        space_id: &String,
        command: &SpaceMemberRemoveCommand,
        creator: &str,
    ) -> error::Result<()>;
}
