use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

use app_interface::define_repo;
use domain_common::error;
use domain_system::model::Config;
use domain_system::repository::IConfigRepository;

use crate::adapter::repository::po::config;

define_repo!(ConfigRepository);

#[async_trait]
impl IConfigRepository for ConfigRepository {
    async fn save(&self, tx: &mut Self::Transaction, config: &mut Config) -> error::Result<()> {
        let sys_conf = config::Entity::find()
            .filter(config::Column::Key.eq(&config.key))
            .order_by_desc(config::Column::Version)
            .limit(1)
            .one(tx)
            .await
            .map_err(|err| error::DomainError::DatabaseError(err.into()))?;
        let model = config::ActiveModel {
            key: Set(config.key.clone()),
            value: Set(config.value.clone()),
            version: Set(sys_conf.as_ref().map(|conf| conf.version).unwrap_or(0) + 1),
        };
        if sys_conf.is_some() {
            config::Entity::update(model)
                .filter(
                    config::Column::Key
                        .eq(&config.key)
                        .and(config::Column::Version.eq(sys_conf.as_ref().unwrap().version)),
                )
                .exec(tx)
                .await
                .map_err(|err| error::DomainError::DatabaseError(err.into()))?;
        } else {
            model
                .insert(tx)
                .await
                .map_err(|err| error::DomainError::DatabaseError(err.into()))?;
        }
        Ok(())
    }

    async fn find_config(
        &self,
        tx: &mut Self::Transaction,
        config_name: &String,
    ) -> error::Result<Option<Config>> {
        let model = config::Entity::find()
            .filter(config::Column::Key.eq(config_name))
            .order_by_desc(config::Column::Version)
            .limit(1)
            .one(tx)
            .await
            .map_err(|err| error::DomainError::DatabaseError(err.into()))?;
        match model {
            Some(model) => Ok(Some(model.into())),
            None => Ok(None),
        }
    }

    async fn find_config_list(
        &self,
        tx: &mut Self::Transaction,
        key_prefix: &String,
    ) -> error::Result<Vec<Config>> {
        let models = config::Entity::find()
            .filter(config::Column::Key.like(format!("{}%", key_prefix)))
            .order_by_desc(config::Column::Version)
            .all(tx)
            .await
            .map_err(|err| error::DomainError::DatabaseError(err.into()))?;

        Ok(models
            .into_iter()
            .map(|model| model.into())
            .collect::<Vec<_>>())
    }
}
