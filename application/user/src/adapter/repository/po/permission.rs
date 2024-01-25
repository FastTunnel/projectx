use sea_orm::entity::prelude::*;
use sea_orm::{NotSet, Set};

use domain_user::model::Permissions;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "permission")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub key: String,
    pub name: String,
    pub parent_key: Option<String>,
    pub is_group: bool,
}

impl From<Model> for Permissions {
    fn from(value: Model) -> Self {
        Permissions {
            id: value.id,
            key: value.key,
            name: value.name,
            group_permission: value.is_group,
        }
    }
}

impl Into<ActiveModel> for &mut Permissions {
    fn into(self) -> ActiveModel {
        let permission = self;
        ActiveModel {
            id: if permission.id == 0 {
                NotSet
            } else {
                Set(permission.id)
            },
            key: Set(permission.key.clone()),
            name: Set(permission.name.clone()),
            parent_key: Set(None),
            is_group: Set(permission.group_permission),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
