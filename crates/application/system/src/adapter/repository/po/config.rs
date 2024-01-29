use domain_system::model::Config;
use sea_orm::entity::prelude::*;
use sea_orm::Set;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sys_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub key: String,
    pub value: Json,
    pub version: i64,
}

impl From<Model> for Config {
    fn from(value: Model) -> Self {
        Config {
            key: value.key,
            value: value.value,
            version: value.version,
        }
    }
}

impl From<Config> for ActiveModel {
    fn from(value: Config) -> Self {
        ActiveModel {
            key: Set(value.key),
            value: Set(value.value),
            version: Set(value.version),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
