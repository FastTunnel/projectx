use async_trait::async_trait;

use crate::error;
use crate::user::dto::command::{
    CreateTeamCommand, InitSystemCommand, RoleBindCommand, RoleCreateCommand, TeamMemberCommand,
    UserCreateCommand, UserLoginCommand,
};
use crate::user::dto::{OrganizationDto, RoleDTO, UserDto};
use domain_user::model::{Organization, User};

pub mod dto {
    use chrono::{DateTime, Utc};
    use domain_user::model::Role;
    use serde::{Deserialize, Serialize};

    pub mod command {
        use serde::Deserialize;
        use validator::{Validate, ValidationError};

        #[derive(Debug, Deserialize, Validate)]
        #[validate(schema(function = "valid_user_create_fn", skip_on_field_errors = false))]
        pub struct UserCreateCommand {
            pub username: Option<String>,
            pub phone: Option<String>,
            pub email: Option<String>,
            pub password: String,
            pub display_name: Option<String>,
        }

        fn valid_user_create_fn(param: &UserCreateCommand) -> Result<(), ValidationError> {
            if param.username.is_some() || param.phone.is_some() || param.email.is_some() {
                Ok(())
            } else {
                Err(ValidationError::new(
                    "username, phone, email Can not be empty",
                ))
            }
        }

        #[derive(Debug, Deserialize, Validate)]
        pub struct UserLoginCommand {
            #[validate(length(min = 5, max = 30, message = "Can not be empty"))]
            pub username: String,
            #[validate(length(min = 8, max = 30, message = "Can not be empty"))]
            pub password: String,
        }

        #[derive(Debug, Deserialize, Validate)]
        pub struct CreateTeamCommand {
            #[validate(length(min = 5, max = 100, message = "Can not be empty"))]
            pub name: String,
            pub description: Option<String>,
            pub organization: String,
            pub public: bool,
            pub icon: Option<String>,
            pub parent: Option<String>,
            pub leader: Option<String>,
        }

        #[derive(Debug, Deserialize, Validate)]
        pub struct TeamMemberCommand {
            pub team: String,
            pub members: Vec<String>,
        }

        #[derive(Debug, Deserialize, Validate)]
        pub struct RoleCreateCommand {
            #[validate(length(min = 5, max = 100, message = "Can not be empty"))]
            pub name: String,
            pub description: Option<String>,
            pub organization: String,
            pub parent: Option<String>,
            pub is_project_role: bool,
            pub own: Option<String>,
        }

        #[derive(Debug, Deserialize, Validate)]
        pub struct RoleBindCommand {
            #[validate(length(min = 5, max = 100, message = "Can not be empty"))]
            pub user: String,
            #[validate(length(min = 1, message = "Can not be empty"))]
            pub roles: Vec<String>,
        }

        #[derive(Debug, Deserialize, Validate)]
        pub struct InitSystemCommand {
            pub name: String,
            pub description: Option<String>,
            pub icon: Option<String>,
        }
    }

    pub mod query {
        use serde::Deserialize;
        use validator::Validate;

        #[derive(Debug, Deserialize, Validate)]
        pub struct RoleQuery {
            #[validate(length(min = 5, max = 100, message = "Can not be empty"))]
            pub organization: String,
            pub own: Option<String>,
        }
    }

    #[derive(Debug, Serialize)]
    pub struct UserDto {
        pub id: u64,
        pub identifier: String,
        pub display_name: Option<String>,
        pub gmt_entry: Option<DateTime<Utc>>,
        pub gmt_leave: Option<DateTime<Utc>>,
        pub leave: bool,
        pub email: Option<String>,
        pub email_verified: bool,
        pub phone: Option<String>,
        pub birthday: Option<DateTime<Utc>>,
        pub country: Option<String>,
        pub province: Option<String>,
        pub city: Option<String>,
        pub address: Option<String>,
        pub pinyin: Option<String>,
        pub avatar: Option<String>,
    }

    #[derive(Debug, Serialize)]
    pub struct OrganizationDto {
        pub id: u64,
        pub identifier: String,
        pub name: String,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub public: bool,
        pub pinyin: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RoleDTO {
        pub id: u64,
        pub identifier: String,
        pub name: String,
        pub description: Option<String>,
        pub organization: String,
        pub parent: Option<String>,
        pub is_project_role: bool,
        pub own: Option<String>,
    }

    impl Into<RoleDTO> for Role {
        fn into(self) -> RoleDTO {
            RoleDTO {
                id: self.id,
                identifier: self.identifier,
                name: self.name,
                description: self.description,
                organization: self.organization,
                parent: self.parent,
                is_project_role: self.is_project_set_role,
                own: self.own,
            }
        }
    }
}

#[async_trait]
pub trait IUserAppService: Send + Sync {
    async fn user_register(&self, user: &String, command: UserCreateCommand) -> error::Result<()>;

    async fn refresh_token(
        &self,
        user_identifier: &String,
        token: &String,
    ) -> error::Result<String>;

    async fn user_login(&self, command: UserLoginCommand) -> error::Result<String>;

    async fn user_detail(&self, identifier: &String) -> error::Result<Option<UserDto>>;

    async fn init_system(&self, command: &InitSystemCommand)
        -> error::Result<(Organization, User)>;

    async fn current_organization(&self) -> error::Result<Option<OrganizationDto>>;

    async fn create_role(&self, creator: &String, command: &RoleCreateCommand)
        -> error::Result<()>;

    async fn bind_role(&self, command: RoleBindCommand) -> error::Result<()>;

    async fn unbind_role(&self, command: RoleBindCommand) -> error::Result<()>;

    async fn create_team(&self, creator: &String, command: CreateTeamCommand) -> error::Result<()>;

    async fn team_add_member(&self, command: TeamMemberCommand) -> error::Result<()>;
    async fn team_remove_member(&self, command: TeamMemberCommand) -> error::Result<()>;

    async fn query_roles_by_own(
        &self,
        organization: &String,
        own: Option<String>,
    ) -> error::Result<Vec<RoleDTO>>;
}
