use crate::adapter::repository::po::{user, user_profile, user_role};

use app_interface::define_repo;
use async_trait::async_trait;
use domain_common::error;
use domain_user::model::{User, UserProfile};
use domain_user::repository::{IUserRepository, UserQuery};
use sea_orm::prelude::*;
use sea_orm::{EntityTrait, JoinType, NotSet, QuerySelect, QueryTrait, Set};

define_repo!(UserRepository);

#[async_trait]
impl IUserRepository for UserRepository {
    async fn find_by_email(
        &self,
        tx: &mut Self::Transaction,
        email: &String,
    ) -> error::Result<Option<User>> {
        self.query(
            tx,
            UserQuery {
                username: Some(email.clone()),
                ..Default::default()
            },
        )
        .await?
        .pop()
        .map(|u| Ok(Some(u)))
        .unwrap_or(Ok(None))
    }

    async fn find_by_phone(
        &self,
        tx: &mut Self::Transaction,
        phone: &String,
    ) -> error::Result<Option<User>> {
        self.query(
            tx,
            UserQuery {
                username: Some(phone.clone()),
                ..Default::default()
            },
        )
        .await?
        .pop()
        .map(|u| Ok(Some(u)))
        .unwrap_or(Ok(None))
    }

    async fn find_by_username(
        &self,
        tx: &mut Self::Transaction,
        username: &String,
    ) -> error::Result<Option<User>> {
        self.query(
            tx,
            UserQuery {
                username: Some(username.clone()),
                ..Default::default()
            },
        )
        .await?
        .pop()
        .map(|u| Ok(Some(u)))
        .unwrap_or(Ok(None))
    }

    async fn find_by_identifier(
        &self,
        tx: &mut Self::Transaction,
        identifier: &String,
    ) -> error::Result<Option<User>> {
        self.query(
            tx,
            UserQuery {
                identifier: Some(identifier.clone()),
                ..Default::default()
            },
        )
        .await?
        .pop()
        .map(|u| Ok(Some(u)))
        .unwrap_or(Ok(None))
    }

    async fn find_simple_by_identifiers(
        &self,
        tx: &mut Self::Transaction,
        identifiers: &[String],
    ) -> error::Result<Vec<User>> {
        let ret = user::Entity::find()
            .filter(user::Column::Identifier.is_in(identifiers))
            .all(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?
            .into_iter()
            .map(|user_po| user_po.into())
            .collect::<Vec<_>>();
        Ok(ret)
    }

    async fn query(&self, tx: &mut Self::Transaction, q: UserQuery) -> error::Result<Vec<User>> {
        let ret = user::Entity::find()
            .join_rev(
                JoinType::InnerJoin,
                user_profile::Entity::belongs_to(user::Entity)
                    .from(user_profile::Column::Identifier)
                    .to(user::Column::Identifier)
                    .into(),
            )
            .select_also(user_profile::Entity)
            .apply_if(q.id, |q, id| q.filter(user::Column::Id.eq(id)))
            .apply_if(q.identifier, |q, identifier| {
                q.filter(user::Column::Identifier.eq(identifier))
            })
            .apply_if(q.username, |q, username| {
                q.filter(user::Column::Username.eq(username))
            })
            .apply_if(q.phone, |q, phone| {
                q.filter(user_profile::Column::Phone.eq(phone))
            })
            .all(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?
            .into_iter()
            .filter(|(_, user_profile)| user_profile.is_some())
            .map(|(user_po, user_profile)| {
                let mut user: User = user_po.into();
                let user_profile: UserProfile = user_profile.unwrap().into();
                user.user_profile = Some(user_profile);
                user
            })
            .collect::<Vec<_>>();
        Ok(ret)
    }

    async fn save(&self, tx: &mut Self::Transaction, user: &mut User) -> error::Result<u64> {
        let user_model_po: user::ActiveModel = user.into();
        let user_model_po = user_model_po
            .save(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        user.id = user_model_po.id.unwrap();
        if user.user_profile.is_none() {
            return Err(error::DomainError::IllegalArgument(
                "user_profile is none".into(),
            ));
        }
        let user_profile = user.user_profile.as_mut().unwrap();
        let user_profile_po: user_profile::ActiveModel = user_profile.into();
        let user_profile_po = user_profile_po
            .save(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        user_profile.id = user_profile_po.id.unwrap();

        user_role::Entity::delete_many()
            .filter(user_role::Column::User.eq(user.identifier.clone()))
            .exec(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;

        if !user.roles.is_empty() {
            let roles = user
                .roles
                .iter()
                .map(|r| user_role::ActiveModel {
                    id: NotSet,
                    user: Set(user.identifier.clone()),
                    role: Set(r.identifier.clone()),
                })
                .collect::<Vec<_>>();
            user_role::Entity::insert_many(roles)
                .exec(tx)
                .await
                .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
            user.roles.iter_mut().for_each(|r| {
                r.id = r.id;
            });
        }
        Ok(user.id)
    }
}
