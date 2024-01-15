use crate::context::user::adapter::repository::po::{permission, role, role_permission, user_role};
use crate::define_repo;
use async_trait::async_trait;
use chrono::{TimeZone, Utc};
use domain::error;
use domain::user::model::Role;
use domain::user::repository::{IRoleRepository, RoleQuery};
use itertools::Itertools;
use sea_orm::prelude::*;
use sea_orm::{JoinType, NotSet, QuerySelect, QueryTrait, Set};
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
            .map(|(role_po, permissions)| Role {
                id: role_po.id,
                identifier: role_po.identifier.clone(),
                name: role_po.name.clone(),
                description: role_po.description.clone(),
                organization: role_po.organization.clone(),
                parent: role_po.parent.clone(),
                default_role: role_po.default_role,
                gmt_create: Utc.timestamp_opt(role_po.gmt_create, 0).unwrap(),
                creator: role_po.creator.clone(),
                gmt_modified: role_po
                    .gmt_modified
                    .map(|v| Utc.timestamp_opt(v, 0).unwrap()),
                modifier: role_po.modifier.clone(),
                permissions: permissions
                    .iter()
                    .filter(|permission_po| permission_po.is_some())
                    .map(|permission_po| {
                        let permission_po = permission_po.as_ref().unwrap();
                        domain::user::model::Permissions {
                            id: permission_po.id,
                            key: permission_po.key.clone(),
                            name: permission_po.name.clone(),
                            group_permission: permission_po.is_group,
                        }
                    })
                    .collect(),
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
            .map(|role_po| Role {
                id: role_po.id,
                identifier: role_po.identifier.clone(),
                name: role_po.name.clone(),
                description: role_po.description.clone(),
                organization: role_po.organization.clone(),
                parent: role_po.parent.clone(),
                default_role: role_po.default_role,
                gmt_create: Utc.timestamp_opt(role_po.gmt_create, 0).unwrap(),
                creator: role_po.creator.clone(),
                gmt_modified: role_po
                    .gmt_modified
                    .map(|v| Utc.timestamp_opt(v, 0).unwrap()),
                modifier: role_po.modifier.clone(),
                permissions: Vec::new(),
            })
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
            .map(|(role_po, permissions)| Role {
                id: role_po.id,
                identifier: role_po.identifier.clone(),
                name: role_po.name.clone(),
                description: role_po.description.clone(),
                organization: role_po.organization.clone(),
                parent: role_po.parent.clone(),
                default_role: role_po.default_role,
                gmt_create: Utc.timestamp_opt(role_po.gmt_create, 0).unwrap(),
                creator: role_po.creator.clone(),
                gmt_modified: role_po
                    .gmt_modified
                    .map(|v| Utc.timestamp_opt(v, 0).unwrap()),
                modifier: role_po.modifier.clone(),
                permissions: permissions
                    .iter()
                    .filter(|permission_po| permission_po.is_some())
                    .map(|permission_po| {
                        let permission_po = permission_po.as_ref().unwrap();
                        domain::user::model::Permissions {
                            id: permission_po.id,
                            key: permission_po.key.clone(),
                            name: permission_po.name.clone(),
                            group_permission: permission_po.is_group,
                        }
                    })
                    .collect(),
            })
            .collect::<Vec<_>>();
        Ok(vec)
    }

    async fn save(&self, tx: &mut Self::Transaction, role: &mut Role) -> error::Result<u64> {
        let role_model = role::ActiveModel {
            id: if role.id == 0 { NotSet } else { Set(role.id) },
            identifier: Set(role.identifier.clone()),
            name: Set(role.name.clone()),
            description: Set(role.description.clone()),
            organization: Set(role.organization.clone()),
            parent: Set(role.parent.clone()),
            default_role: Set(role.default_role),
            gmt_create: Set(role.gmt_create.timestamp()),
            creator: Set(role.creator.clone()),
            gmt_modified: Set(role.gmt_modified.map(|v| v.timestamp())),
            modifier: Set(role.modifier.clone()),
        }
        .save(tx)
        .await
        .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        role.id = role_model.id.unwrap();
        role_permission::Entity::delete_many()
            .filter(role_permission::Column::Role.eq(role.identifier.clone()))
            .exec(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;

        if !role.permissions.is_empty() {
            let perms = role
                .permissions
                .iter()
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
}
