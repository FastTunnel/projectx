use crate::context::sys::adapter::repository::po::config;
use crate::define_repo;
use async_trait::async_trait;
use domain::error;
use domain::sys::model::Config;
use domain::sys::repository::IConfigRepository;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde_json::json;

define_repo!(ConfigRepository);

#[async_trait]
impl IConfigRepository for ConfigRepository {
    async fn save(&self, tx: &mut Self::Transaction, config: &mut Config) -> error::Result<()> {
        let sys_conf = config::Entity::find()
            .filter(config::Column::Key.eq(config.name()))
            .order_by_desc(config::Column::Version)
            .limit(1)
            .one(tx)
            .await
            .map_err(|err| error::DomainError::DatabaseError(err.into()))?;

        let value = match config {
            Config::SysInfo { is_init } => {
                json!({
                    "is_init": is_init,
                })
            }
        };
        let model = config::ActiveModel {
            key: Set(config.name()),
            value: Set(value),
            version: Set(sys_conf.as_ref().map(|conf| conf.version).unwrap_or(0) + 1),
        };
        if sys_conf.is_some() {
            model
                .update(tx)
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

    async fn get_config(
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
            Some(model) => match config_name.as_str() {
                "sys_info" => {
                    let is_init = model.value["is_init"].as_bool().unwrap_or({
                        tracing::warn!("sys_info.is_init is not bool");
                        true
                    });
                    Ok(Some(Config::SysInfo { is_init }))
                }
                _ => Ok(None),
            },
            None => Ok(None),
        }
    }
}
