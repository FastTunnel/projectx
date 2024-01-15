use async_trait::async_trait;
use chrono::{TimeZone, Utc};
use sea_orm::{EntityTrait, JoinType, NotSet, QuerySelect, QueryTrait, Set};

use crate::context::user::adapter::repository::po::{user, user_profile, user_role};
use crate::define_repo;
use domain::error;
use domain::user::model::{User, UserProfile};
use domain::user::repository::{IUserRepository, UserQuery};
use sea_orm::prelude::*;

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
            .map(|user_po| User {
                id: user_po.id,
                identifier: user_po.identifier,
                username: user_po.username,
                password: user_po.password,
                salt: user_po.salt,
                disabled: user_po.disabled,
                gmt_create: Utc.timestamp_opt(user_po.gmt_create, 0).unwrap(),
                creator: user_po.creator,
                gmt_modified: user_po
                    .gmt_modified
                    .map(|v| Utc.timestamp_opt(v, 0).unwrap()),
                modifier: user_po.modifier,
                last_login_at: user_po
                    .last_login_at
                    .map(|v| Utc.timestamp_opt(v, 0).unwrap()),
                token: user_po.token,
                user_profile: None,
                roles: vec![],
            })
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
                let user_profile_po = user_profile.unwrap();
                User {
                    id: user_po.id,
                    identifier: user_po.identifier,
                    username: user_po.username,
                    password: user_po.password,
                    salt: user_po.salt,
                    disabled: user_po.disabled,
                    gmt_create: Utc.timestamp_opt(user_po.gmt_create, 0).unwrap(),
                    creator: user_po.creator,
                    gmt_modified: user_po
                        .gmt_modified
                        .map(|v| Utc.timestamp_opt(v, 0).unwrap()),
                    modifier: user_po.modifier,
                    last_login_at: user_po
                        .last_login_at
                        .map(|v| Utc.timestamp_opt(v, 0).unwrap()),
                    token: user_po.token,
                    user_profile: Some(UserProfile {
                        id: user_profile_po.id,
                        identifier: user_profile_po.identifier,
                        display_name: user_profile_po.display_name,
                        gmt_entry: user_profile_po
                            .gmt_entry
                            .map(|v| Utc.timestamp_opt(v, 0).unwrap()),
                        gmt_leave: user_profile_po
                            .gmt_leave
                            .map(|v| Utc.timestamp_opt(v, 0).unwrap()),
                        leave: user_profile_po.leave,
                        email: user_profile_po.email,
                        email_verified: user_profile_po.email_verified,
                        phone: user_profile_po.phone,
                        birthday: user_profile_po
                            .birthday
                            .map(|v| Utc.timestamp_opt(v, 0).unwrap()),
                        country: user_profile_po.country,
                        province: user_profile_po.province,
                        city: user_profile_po.city,
                        address: user_profile_po.address,
                        pinyin: user_profile_po.pinyin,
                        avatar: user_profile_po.avatar,
                    }),
                    roles: vec![],
                }
            })
            .collect::<Vec<_>>();
        Ok(ret)
    }

    async fn save(&self, tx: &mut Self::Transaction, user: &mut User) -> error::Result<u64> {
        let user_model = user::ActiveModel {
            id: if user.id == 0 { NotSet } else { Set(user.id) },
            identifier: Set(user.identifier.clone()),
            username: Set(user.username.clone()),
            password: Set(user.password.clone()),
            salt: Set(user.salt.clone()),
            disabled: Set(user.disabled),
            gmt_create: Set(user.gmt_create.timestamp()),
            creator: Set(user.creator.clone()),
            gmt_modified: Set(user.gmt_modified.map(|v| v.timestamp())),
            modifier: Set(user.modifier.clone()),
            last_login_at: Set(user.last_login_at.map(|v| v.timestamp())),
            token: Set(user.token.clone()),
        }
        .save(tx)
        .await
        .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        user.id = user_model.id.unwrap();
        if user.user_profile.is_none() {
            return Err(error::DomainError::IllegalArgument(
                "user_profile is none".into(),
            ));
        }
        let user_profile = user.user_profile.as_mut().unwrap();
        let _user_profile_model = user_profile::ActiveModel {
            id: if user_profile.id == 0 {
                NotSet
            } else {
                Set(user_profile.id)
            },
            identifier: Set(user_profile.identifier.clone()),
            display_name: Set(user_profile.display_name.clone()),
            gmt_entry: Set(user_profile.gmt_entry.map(|v| v.timestamp())),
            gmt_leave: Set(user_profile.gmt_leave.map(|v| v.timestamp())),
            leave: Set(user_profile.leave),
            email: Set(user_profile.email.clone()),
            email_verified: Set(user_profile.email_verified),
            phone: Set(user_profile.phone.clone()),
            birthday: Set(user_profile.birthday.map(|v| v.timestamp())),
            country: Set(user_profile.country.clone()),
            province: Set(user_profile.province.clone()),
            city: Set(user_profile.city.clone()),
            address: Set(user_profile.address.clone()),
            pinyin: Set(user_profile.pinyin.clone()),
            avatar: Set(user_profile.avatar.clone()),
        }
        .save(tx)
        .await
        .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        user_profile.id = _user_profile_model.id.unwrap();

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
