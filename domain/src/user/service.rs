use std::sync::Arc;

use async_trait::async_trait;
use passwords::PasswordGenerator;

use crate::error;
use crate::user::facade::{IConfigFacade, IJwtFacade};
use crate::user::model::{
    CreateOrganizationParam, CreateRoleParam, CreateTeamParam, Organization, Role, Team, User,
};
use crate::user::publisher::{IUserEventPublisher, UserEvent};
use crate::user::repository::{
    IOrganizationRepository, IRoleRepository, ITeamRepository, IUserRepository,
};

use super::model::CreateUserParam;

#[async_trait]
pub trait IUserService<T>: Send + Sync {
    async fn create_user(
        &self,
        tx: &mut T,
        creator: &String,
        param: CreateUserParam,
    ) -> error::Result<()>;
    async fn login(
        &self,
        tx: &mut T,
        username: &String,
        password: &String,
    ) -> error::Result<String>;

    async fn init(&self, tx: &mut T, param: CreateOrganizationParam) -> error::Result<()>;

    async fn create_role(
        &self,
        tx: &mut T,
        creator: &String,
        param: CreateRoleParam,
    ) -> error::Result<()>;

    async fn bind_role(&self, tx: &mut T, user: &String, roles: Vec<String>) -> error::Result<()>;

    async fn unbind_role(&self, tx: &mut T, user: &String, roles: Vec<String>)
        -> error::Result<()>;

    async fn user_detail(&self, tx: &mut T, identifier: &String) -> error::Result<Option<User>>;

    async fn refresh_token(
        &self,
        tx: &mut T,
        user_identifier: &String,
        token: &String,
    ) -> error::Result<String>;

    async fn create_team(
        &self,
        tx: &mut T,
        creator: &String,
        organization: &String,
        param: CreateTeamParam,
    ) -> error::Result<()>;

    async fn team_add_member(
        &self,
        tx: &mut T,
        team: &String,
        members: Vec<String>,
    ) -> error::Result<()>;

    async fn team_remove_member(
        &self,
        tx: &mut T,
        team: &String,
        members: Vec<String>,
    ) -> error::Result<()>;

    async fn sys_init_organization(&self, tx: &mut T) -> error::Result<Option<Organization>>;
}

pub struct UserService<T> {
    user_repo: Arc<dyn IUserRepository<Transaction = T>>,
    role_repo: Arc<dyn IRoleRepository<Transaction = T>>,
    organization_repo: Arc<dyn IOrganizationRepository<Transaction = T>>,
    team_repo: Arc<dyn ITeamRepository<Transaction = T>>,
    user_publisher: Arc<dyn IUserEventPublisher>,
    jwt_facade: Arc<dyn IJwtFacade>,
    config_facade: Arc<dyn IConfigFacade>,
}

impl<T> UserService<T> {
    pub fn new(
        user_repo: Arc<dyn IUserRepository<Transaction = T>>,
        role_repo: Arc<dyn IRoleRepository<Transaction = T>>,
        organization_repo: Arc<dyn IOrganizationRepository<Transaction = T>>,
        team_repo: Arc<dyn ITeamRepository<Transaction = T>>,
        user_publisher: Arc<dyn IUserEventPublisher>,
        jwt_facade: Arc<dyn IJwtFacade>,
        config_facade: Arc<dyn IConfigFacade>,
    ) -> Self {
        Self {
            user_repo,
            role_repo,
            organization_repo,
            team_repo,
            user_publisher,
            jwt_facade,
            config_facade,
        }
    }
}

#[async_trait]
impl<T> IUserService<T> for UserService<T>
where
    T: Send + Sync,
{
    async fn create_user(
        &self,
        tx: &mut T,
        creator: &String,
        param: CreateUserParam,
    ) -> error::Result<()> {
        if self
            .user_repo
            .find_by_username(
                tx,
                &param
                    .username
                    .clone()
                    .or(param.phone.clone())
                    .or(param.email.clone())
                    .ok_or(error::DomainError::IllegalArgument("username".into()))?,
            )
            .await?
            .is_some()
        {
            return Err(error::DomainError::DataAlreadyExists);
        }
        let mut user = User::create_user(param, creator)?;
        user.verify_user_info()?;
        self.user_repo.save(tx, &mut user).await?;
        self.user_publisher
            .publish(UserEvent::Created(creator.clone(), user))
            .await?;
        Ok(())
    }

    async fn login(
        &self,
        tx: &mut T,
        username: &String,
        password: &String,
    ) -> error::Result<String> {
        let mut user = self
            .user_repo
            .find_by_username(tx, &username)
            .await?
            .ok_or(error::DomainError::DataNotFound)?;
        let token = self
            .jwt_facade
            .generate_token(&user.identifier, &user.username)
            .await?;
        user.login(&password, &token)?;
        self.user_repo.save(tx, &mut user).await?;
        Ok(token)
    }

    async fn init(&self, tx: &mut T, param: CreateOrganizationParam) -> error::Result<()> {
        let is_initialized = self.config_facade.sys_is_init().await?;
        if is_initialized {
            return Err(error::DomainError::AppInitialized);
        }

        // init organization
        let mut organization = Organization::create_organization(param)?;
        self.organization_repo.save(tx, &mut organization).await?;

        // init admin user
        let password = PasswordGenerator::new()
            .length(8)
            .numbers(true)
            .lowercase_letters(true)
            .uppercase_letters(true)
            .symbols(true)
            .spaces(false)
            .exclude_similar_characters(true)
            .strict(true)
            .generate_one()
            .map_err(|_| error::DomainError::AppInitFailed("password gen error".into()))?;
        let param = CreateUserParam {
            username: Some("admin".to_string()),
            phone: None,
            email: None,
            password,
            display_name: Some("管理员".to_string()),
        };
        let mut user = User::create_user(param, &"system".into())?;
        self.user_repo.save(tx, &mut user).await?;

        self.config_facade.sys_init().await?;
        Ok(())
    }

    async fn create_role(
        &self,
        tx: &mut T,
        creator: &String,
        param: CreateRoleParam,
    ) -> error::Result<()> {
        // check if role name exists
        if self
            .role_repo
            .find_by_name(tx, &param.name)
            .await?
            .is_some()
        {
            return Err(error::DomainError::DataAlreadyExists);
        }
        // check if organization exists
        if self
            .organization_repo
            .find_by_identifier(tx, &param.organization)
            .await?
            .is_none()
        {
            return Err(error::DomainError::DataNotFound);
        }
        let mut role = Role::create_role(param, creator)?;
        self.role_repo.save(tx, &mut role).await?;
        Ok(())
    }

    async fn bind_role(&self, tx: &mut T, user: &String, roles: Vec<String>) -> error::Result<()> {
        let mut user = self
            .user_repo
            .find_by_identifier(tx, user)
            .await?
            .ok_or(error::DomainError::DataNotFound)?;
        let roles = self
            .role_repo
            .find_simple_by_identifiers(tx, &roles)
            .await?;
        user.bind_roles(roles);
        self.user_repo.save(tx, &mut user).await?;
        Ok(())
    }

    async fn unbind_role(
        &self,
        tx: &mut T,
        user: &String,
        roles: Vec<String>,
    ) -> error::Result<()> {
        let mut user = self
            .user_repo
            .find_by_identifier(tx, user)
            .await?
            .ok_or(error::DomainError::DataNotFound)?;
        let roles = self
            .role_repo
            .find_simple_by_identifiers(tx, &roles)
            .await?;
        user.unbind_roles(roles);
        self.user_repo.save(tx, &mut user).await?;
        Ok(())
    }

    async fn user_detail(&self, tx: &mut T, identifier: &String) -> error::Result<Option<User>> {
        let mut user = self.user_repo.find_by_identifier(tx, identifier).await?;
        if let Some(user) = &mut user {
            let id = &user.identifier;
            let roles = self.role_repo.find_by_user_identifier(tx, id).await?;
            user.roles = roles;
        }
        Ok(user)
    }

    async fn refresh_token(
        &self,
        tx: &mut T,
        user_identifier: &String,
        token: &String,
    ) -> error::Result<String> {
        let mut user = self
            .user_repo
            .find_by_identifier(tx, &user_identifier)
            .await?
            .ok_or(error::DomainError::DataNotFound)?;

        if user.token.is_none() {
            return Err(error::DomainError::JwtError("token not found".to_string()));
        }
        if user.token.as_ref().unwrap() != token {
            return Err(error::DomainError::JwtError("token not match".to_string()));
        }
        let new_token = self
            .jwt_facade
            .generate_token(&user.identifier, &user.username)
            .await?;
        user.refresh_token(&new_token);
        self.user_repo.save(tx, &mut user).await?;
        Ok(new_token)
    }

    async fn create_team(
        &self,
        tx: &mut T,
        creator: &String,
        organization: &String,
        param: CreateTeamParam,
    ) -> error::Result<()> {
        // check if team name exists
        if self
            .team_repo
            .find_by_name(tx, &param.name)
            .await?
            .is_some()
        {
            return Err(error::DomainError::DataAlreadyExists);
        }
        // check if organization exists
        if let Some(organization) = self
            .organization_repo
            .find_by_identifier(tx, organization)
            .await?
        {
            let mut team = Team::create_team(organization, param, creator)?;
            self.team_repo.save(tx, &mut team).await?;
        } else {
            return Err(error::DomainError::DataNotFound);
        }
        Ok(())
    }

    async fn team_add_member(
        &self,
        tx: &mut T,
        team: &String,
        members: Vec<String>,
    ) -> error::Result<()> {
        let mut team = self
            .team_repo
            .find_by_identifier(tx, team)
            .await?
            .ok_or(error::DomainError::DataNotFound)?;
        let users = self
            .user_repo
            .find_simple_by_identifiers(tx, &members)
            .await?;
        team.add_member(users);
        self.team_repo.save(tx, &mut team).await?;
        Ok(())
    }
    async fn team_remove_member(
        &self,
        tx: &mut T,
        team: &String,
        members: Vec<String>,
    ) -> error::Result<()> {
        let mut team = self
            .team_repo
            .find_by_identifier(tx, team)
            .await?
            .ok_or(error::DomainError::DataNotFound)?;
        let users = self
            .user_repo
            .find_simple_by_identifiers(tx, &members)
            .await?;
        team.remove_member(users);
        self.team_repo.save(tx, &mut team).await?;
        Ok(())
    }

    async fn sys_init_organization(&self, tx: &mut T) -> error::Result<Option<Organization>> {
        if !self.config_facade.sys_is_init().await? {
            return Err(error::DomainError::AppNotInitialized);
        }
        self.organization_repo.find_first(tx).await
    }
}
