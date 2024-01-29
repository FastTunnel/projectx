use app_interface::define_repo;
use async_trait::async_trait;
use itertools::Itertools;
use sea_orm::prelude::*;
use sea_orm::{JoinType, NotSet, QuerySelect, QueryTrait, Set};

use crate::adapter::repository::po::{permission, role, role_permission, user_role};
use domain_common::error;
use domain_user::model::Role;
use domain_user::repository::{IRoleRepository, RoleQuery};

define_repo!(RoleRepository);

#[async_trait]
impl IRoleRepository for RoleRepository {
    async fn query(&self, tx: &mut Self::Transaction, q: RoleQuery) -> error::Result<Vec<Role>> {
        let roles = role::Entity::find()
            .join_rev(
                JoinType::LeftJoin,
                role_permission::Entity::belongs_to(role::Entity)
                    .from(role_permission::Column::Role)
                    .to(role::Column::Identifier)
                    .into(),
            )
            .join_rev(
                JoinType::LeftJoin,
                permission::Entity::belongs_to(role_permission::Entity)
                    .from(permission::Column::Key)
                    .to(role_permission::Column::Permission)
                    .into(),
            )
            .apply_if(q.id, |q, v| q.filter(role::Column::Id.eq(v)))
            .apply_if(q.identifier, |q, v| {
                q.filter(role::Column::Identifier.eq(v))
            })
            .apply_if(q.name, |q, v| q.filter(role::Column::Name.eq(v)))
            .select_also(permission::Entity)
            .all(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?
            .into_iter()
            .into_group_map()
            .into_iter()
            .map(|(role_po, permissions)| {
                let mut role: Role = role_po.into();
                let permissions = permissions
                    .into_iter()
                    .filter(|permission_po| permission_po.is_some())
                    .map(|permission_po| permission_po.unwrap().into())
                    .collect();
                role.permissions = permissions;
                role
            })
            .collect::<Vec<_>>();
        Ok(roles)
    }

    async fn find_by_identifier(
        &self,
        tx: &mut Self::Transaction,
        id: &String,
    ) -> error::Result<Option<Role>> {
        self.query(
            tx,
            RoleQuery {
                identifier: Some(id.clone()),
                ..Default::default()
            },
        )
        .await?
        .pop()
        .map(|r| Ok(Some(r)))
        .unwrap_or(Ok(None))
    }

    async fn find_simple_by_identifiers(
        &self,
        tx: &mut Self::Transaction,
        ids: &[String],
    ) -> error::Result<Vec<Role>> {
        let roles = role::Entity::find()
            .filter(role::Column::Identifier.is_in(ids))
            .all(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?
            .into_iter()
            .map(|role_po| role_po.into())
            .collect::<Vec<_>>();
        Ok(roles)
    }

    async fn user_bind_roles(
        &self,
        tx: &mut Self::Transaction,
        user: &String,
        roles: Vec<String>,
    ) -> error::Result<()> {
        user_role::Entity::delete_many()
            .filter(user_role::Column::User.eq(user))
            .exec(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        let roles = roles
            .into_iter()
            .map(|role| user_role::ActiveModel {
                id: NotSet,
                user: Set(user.clone()),
                role: Set(role),
            })
            .collect::<Vec<_>>();
        user_role::Entity::insert_many(roles)
            .exec(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        Ok(())
    }

    async fn user_unbind_roles(
        &self,
        tx: &mut Self::Transaction,
        user: &String,
        roles: Vec<String>,
    ) -> error::Result<()> {
        user_role::Entity::delete_many()
            .filter(user_role::Column::User.eq(user))
            .filter(user_role::Column::Role.is_in(roles))
            .exec(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        Ok(())
    }

    async fn find_by_name(
        &self,
        tx: &mut Self::Transaction,
        name: &String,
    ) -> error::Result<Option<Role>> {
        self.query(
            tx,
            RoleQuery {
                name: Some(name.clone()),
                ..Default::default()
            },
        )
        .await?
        .pop()
        .map(|r| Ok(Some(r)))
        .unwrap_or(Ok(None))
    }

    async fn find_by_user_identifier(
        &self,
        tx: &mut Self::Transaction,
        user_identifier: &String,
    ) -> error::Result<Vec<Role>> {
        let vec = role::Entity::find()
            .join_rev(
                JoinType::InnerJoin,
                user_role::Entity::belongs_to(role::Entity)
                    .from(user_role::Column::Role)
                    .to(role::Column::Identifier)
                    .into(),
            )
            .join_rev(
                JoinType::LeftJoin,
                role_permission::Entity::belongs_to(role::Entity)
                    .from(role_permission::Column::Role)
                    .to(role::Column::Identifier)
                    .into(),
            )
            .join_rev(
                JoinType::LeftJoin,
                permission::Entity::belongs_to(role_permission::Entity)
                    .from(permission::Column::Key)
                    .to(role_permission::Column::Permission)
                    .into(),
            )
            .filter(user_role::Column::User.eq(user_identifier))
            .select_also(permission::Entity)
            .all(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?
            .into_iter()
            .into_group_map()
            .into_iter()
            .map(|(role_po, permissions)| {
                let mut role: Role = role_po.into();
                let permissions = permissions
                    .into_iter()
                    .filter(|permission_po| permission_po.is_some())
                    .map(|permission_po| permission_po.unwrap().into())
                    .collect();
                role.permissions = permissions;
                role
            })
            .collect::<Vec<_>>();
        Ok(vec)
    }

    async fn save(&self, tx: &mut Self::Transaction, role: &mut Role) -> error::Result<u64> {
        let role_model: role::ActiveModel = role.into();
        let role_model = role_model
            .save(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        role.id = *(role_model.id.as_ref());
        role_permission::Entity::delete_many()
            .filter(role_permission::Column::Role.eq(role.identifier.clone()))
            .exec(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        if !role.permissions.is_empty() {
            let perms = role
                .permissions
                .iter_mut()
                .map(|p| role_permission::ActiveModel {
                    id: NotSet,
                    role: Set(role.identifier.clone()),
                    permission: Set(p.key.clone()),
                })
                .collect::<Vec<_>>();
            role_permission::Entity::insert_many(perms);
        }

        Ok(role.id)
    }

    async fn save_all(
        &self,
        tx: &mut Self::Transaction,
        role: &mut Vec<Role>,
    ) -> error::Result<()> {
        let role_model: Vec<role::ActiveModel> = role.into_iter().map(|v| v.into()).collect();
        let _ = role::Entity::insert_many(role_model)
            .exec(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        Ok(())
    }

    async fn query_roles_by_org_and_own(
        &self,
        tx: &mut Self::Transaction,
        organization: &String,
        own: Option<String>,
    ) -> error::Result<Vec<Role>> {
        let roles = role::Entity::find()
            .filter(role::Column::Organization.eq(organization))
            .apply_if(own, |q, v| q.filter(role::Column::Own.eq(v)))
            .all(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?
            .into_iter()
            .map(|role_po| role_po.into())
            .collect::<Vec<_>>();
        Ok(roles)
    }
}
