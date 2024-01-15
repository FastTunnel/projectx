use async_trait::async_trait;

use crate::context::user::application::dto::command::{
    CreateTeamCommand, InitOrganizationCommand, RoleBindCommand, RoleCreateCommand,
    TeamMemberCommand, UserCreateCommand, UserLoginCommand,
};
use crate::context::user::application::dto::{OrganizationDto, UserDto};
use crate::error;

pub(crate) mod dto;
pub mod service;

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

    async fn init_system(&self, command: &InitOrganizationCommand) -> error::Result<()>;

    async fn current_organization(&self) -> error::Result<Option<OrganizationDto>>;

    async fn create_role(&self, creator: &String, command: &RoleCreateCommand)
        -> error::Result<()>;

    async fn bind_role(&self, command: RoleBindCommand) -> error::Result<()>;

    async fn unbind_role(&self, command: RoleBindCommand) -> error::Result<()>;

    async fn create_team(&self, creator: &String, command: CreateTeamCommand) -> error::Result<()>;

    async fn team_add_member(&self, command: TeamMemberCommand) -> error::Result<()>;
    async fn team_remove_member(&self, command: TeamMemberCommand) -> error::Result<()>;
}
