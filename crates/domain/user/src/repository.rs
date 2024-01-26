use async_trait::async_trait;

use domain_common::{error, Repository};

use crate::model::{Organization, Role, Team, User};

#[derive(Debug, Default)]
pub struct UserQuery {
    pub id: Option<u64>,
    pub identifier: Option<String>,
    pub username: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[async_trait]
pub trait IUserRepository: Repository {
    async fn find_by_email(
        &self,
        tx: &mut Self::Transaction,
        email: &String,
    ) -> error::Result<Option<User>>;
    async fn find_by_phone(
        &self,
        tx: &mut Self::Transaction,
        phone: &String,
    ) -> error::Result<Option<User>>;
    async fn find_by_username(
        &self,
        tx: &mut Self::Transaction,
        username: &String,
    ) -> error::Result<Option<User>>;
    async fn find_by_identifier(
        &self,
        tx: &mut Self::Transaction,
        identifier: &String,
    ) -> error::Result<Option<User>>;

    async fn find_simple_by_identifiers(
        &self,
        tx: &mut Self::Transaction,
        identifiers: &[String],
    ) -> error::Result<Vec<User>>;

    async fn query(&self, tx: &mut Self::Transaction, q: UserQuery) -> error::Result<Vec<User>>;

    async fn save(&self, tx: &mut Self::Transaction, user: &mut User) -> error::Result<u64>;
}

#[derive(Debug, Default)]
pub struct OrganizationQuery {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub name: Option<String>,
}

#[async_trait]
pub trait IOrganizationRepository: Repository {
    async fn find_by_identifier(
        &self,
        tx: &mut Self::Transaction,
        identifier: &String,
    ) -> error::Result<Option<Organization>>;
    async fn find_by_name(
        &self,
        tx: &mut Self::Transaction,
        name: &String,
    ) -> error::Result<Option<Organization>>;
    async fn query(
        &self,
        tx: &mut Self::Transaction,
        q: OrganizationQuery,
    ) -> error::Result<Vec<Organization>>;
    async fn is_initialized(&self, tx: &mut Self::Transaction) -> error::Result<bool>;

    async fn find_first(&self, tx: &mut Self::Transaction) -> error::Result<Option<Organization>>;

    async fn save(
        &self,
        tx: &mut Self::Transaction,
        organization: &mut Organization,
    ) -> error::Result<u64>;
}

#[derive(Debug, Default)]
pub struct RoleQuery {
    pub id: Option<u64>,
    pub identifier: Option<String>,
    pub name: Option<String>,
}

#[async_trait]
pub trait IRoleRepository: Repository {
    async fn query(&self, tx: &mut Self::Transaction, q: RoleQuery) -> error::Result<Vec<Role>>;
    async fn find_by_identifier(
        &self,
        tx: &mut Self::Transaction,
        id: &String,
    ) -> error::Result<Option<Role>>;

    async fn find_simple_by_identifiers(
        &self,
        tx: &mut Self::Transaction,
        ids: &[String],
    ) -> error::Result<Vec<Role>>;

    async fn user_bind_roles(
        &self,
        tx: &mut Self::Transaction,
        user: &String,
        roles: Vec<String>,
    ) -> error::Result<()>;
    async fn user_unbind_roles(
        &self,
        tx: &mut Self::Transaction,
        user: &String,
        roles: Vec<String>,
    ) -> error::Result<()>;

    async fn find_by_name(
        &self,
        tx: &mut Self::Transaction,
        name: &String,
    ) -> error::Result<Option<Role>>;

    async fn find_by_user_identifier(
        &self,
        tx: &mut Self::Transaction,
        user_identifier: &String,
    ) -> error::Result<Vec<Role>>;

    async fn save(&self, tx: &mut Self::Transaction, role: &mut Role) -> error::Result<u64>;
    async fn save_all(&self, tx: &mut Self::Transaction, role: &mut Vec<Role>)
        -> error::Result<()>;

    async fn query_roles_by_org_and_own(
        &self,
        tx: &mut Self::Transaction,
        organization: &String,
        own: Option<String>,
    ) -> error::Result<Vec<Role>>;
}

#[derive(Debug, Default)]
pub struct TeamQuery {
    pub id: Option<u64>,
    pub identifier: Option<String>,
    pub name: Option<String>,
    pub organization_id: Option<u64>,
}

#[async_trait]
pub trait ITeamRepository: Repository {
    async fn find_by_identifier(
        &self,
        tx: &mut Self::Transaction,
        id: &String,
    ) -> error::Result<Option<Team>>;
    async fn find_by_name(
        &self,
        tx: &mut Self::Transaction,
        name: &String,
    ) -> error::Result<Option<Team>>;

    async fn query(&self, tx: &mut Self::Transaction, q: TeamQuery) -> error::Result<Vec<Team>>;

    async fn save(&self, tx: &mut Self::Transaction, team: &mut Team) -> error::Result<u64>;
}
