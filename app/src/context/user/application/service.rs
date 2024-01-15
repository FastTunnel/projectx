use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::TransactionTrait;

use domain::user::model::CreateUserParam;
use domain::user::service::IUserService;

use crate::context::user::application::dto::command::{
    CreateTeamCommand, InitOrganizationCommand, RoleBindCommand, RoleCreateCommand,
    TeamMemberCommand, UserCreateCommand, UserLoginCommand,
};

use crate::context::user::application::dto::{OrganizationDto, UserDto};
use crate::context::user::application::IUserAppService;
use crate::{error, DbTx, APP_STATE};

pub struct UserAppService {
    user_service: Arc<dyn IUserService<DbTx>>,
}

impl UserAppService {
    pub fn new(user_service: Arc<dyn IUserService<DbTx>>) -> Self {
        Self { user_service }
    }
}

#[async_trait]
impl IUserAppService for UserAppService {
    async fn user_register(&self, user: &String, command: UserCreateCommand) -> error::Result<()> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        self.user_service
            .create_user(
                &mut transaction,
                user,
                CreateUserParam {
                    username: command.username,
                    password: command.password,
                    email: command.email,
                    display_name: command.display_name,
                    phone: command.phone,
                },
            )
            .await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn refresh_token(
        &self,
        user_identifier: &String,
        token: &String,
    ) -> error::Result<String> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        let new_token = self
            .user_service
            .refresh_token(&mut transaction, user_identifier, token)
            .await?;
        transaction.commit().await?;
        Ok(new_token)
    }

    async fn user_login(
        &self,
        UserLoginCommand { username, password }: UserLoginCommand,
    ) -> error::Result<String> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        let token = self
            .user_service
            .login(&mut transaction, &username, &password)
            .await?;
        transaction.commit().await?;
        Ok(token)
    }

    async fn user_detail(&self, identifier: &String) -> error::Result<Option<UserDto>> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        let user = self
            .user_service
            .user_detail(&mut transaction, identifier)
            .await?;
        transaction.commit().await?;
        if user.is_none() || user.as_ref().unwrap().user_profile.is_none() {
            return Ok(None);
        }
        Ok(user.map(|user| {
            let user_profile = user.user_profile.unwrap();
            UserDto {
                id: user.id,
                email: user_profile.email,
                email_verified: false,
                phone: user_profile.phone,
                birthday: user_profile.birthday,
                country: user_profile.country,
                province: user_profile.province,
                city: user_profile.city,
                address: user_profile.address,
                pinyin: user_profile.pinyin,
                display_name: user_profile.display_name,
                gmt_entry: user_profile.gmt_entry,
                gmt_leave: user_profile.gmt_leave,
                identifier: user.identifier,
                avatar: user_profile.avatar,
                leave: user_profile.leave,
            }
        }))
    }

    async fn init_system(&self, command: &InitOrganizationCommand) -> error::Result<()> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        self.user_service
            .init(
                &mut transaction,
                domain::user::model::CreateOrganizationParam {
                    name: command.name.clone(),
                    description: command.description.clone(),
                    icon: command.icon.clone(),
                },
            )
            .await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn current_organization(&self) -> error::Result<Option<OrganizationDto>> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        let org = self
            .user_service
            .sys_init_organization(&mut transaction)
            .await
            .map(|v| {
                v.map(|v| OrganizationDto {
                    id: v.id,
                    identifier: v.identifier,
                    name: v.name,
                    description: v.description,
                    icon: v.icon,
                    public: v.public,
                    pinyin: v.pinyin,
                })
            })?;
        transaction.commit().await?;
        Ok(org)
    }

    async fn create_role(
        &self,
        creator: &String,
        command: &RoleCreateCommand,
    ) -> error::Result<()> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        self.user_service
            .create_role(
                &mut transaction,
                creator,
                domain::user::model::CreateRoleParam {
                    name: command.name.clone(),
                    description: command.description.clone(),
                    organization: command.organization.clone(),
                    parent: command.parent.clone(),
                },
            )
            .await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn bind_role(
        &self,
        RoleBindCommand { user, roles }: RoleBindCommand,
    ) -> error::Result<()> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        self.user_service
            .bind_role(&mut transaction, &user, roles)
            .await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn unbind_role(
        &self,
        RoleBindCommand { user, roles }: RoleBindCommand,
    ) -> error::Result<()> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        self.user_service
            .unbind_role(&mut transaction, &user, roles)
            .await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn create_team(&self, creator: &String, command: CreateTeamCommand) -> error::Result<()> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        let param = domain::user::model::CreateTeamParam {
            name: command.name,
            description: command.description,
            public: command.public,
            icon: command.icon,
            parent: command.parent,
            leader: command.leader,
        };
        self.user_service
            .create_team(&mut transaction, &command.organization, creator, param)
            .await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn team_add_member(&self, command: TeamMemberCommand) -> error::Result<()> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        self.user_service
            .team_add_member(&mut transaction, &command.team, command.members)
            .await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn team_remove_member(&self, command: TeamMemberCommand) -> error::Result<()> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        self.user_service
            .team_remove_member(&mut transaction, &command.team, command.members)
            .await?;
        transaction.commit().await?;
        Ok(())
    }
}
