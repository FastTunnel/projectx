use app_interface::utils::ToDateTime;
use sea_orm::entity::prelude::*;
use sea_orm::{NotSet, Set};

use domain_user::model::Organization;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "organization")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub identifier: String,
    pub name: String,
    pub pinyin: String,
    pub public: bool,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub gmt_create: i64,
    pub gmt_modified: Option<i64>,
}

impl From<Model> for Organization {
    fn from(value: Model) -> Self {
        Organization {
            id: value.id,
            identifier: value.identifier,
            name: value.name,
            pinyin: value.pinyin,
            public: value.public,
            icon: value.icon,
            description: value.description,
            gmt_create: value.gmt_create.to_date_time(),
            gmt_modified: value.gmt_modified.map(|v| v.to_date_time()),
        }
    }
}

impl Into<ActiveModel> for &mut Organization {
    fn into(self) -> ActiveModel {
        let organization = self;
        ActiveModel {
            id: if organization.id == 0 {
                NotSet
            } else {
                Set(organization.id)
            },
            identifier: Set(organization.identifier.clone()),
            name: Set(organization.name.clone()),
            pinyin: Set(organization.pinyin.clone()),
            public: Set(organization.public),
            icon: Set(organization.icon.clone()),
            description: Set(organization.description.clone()),
            gmt_create: Set(organization.gmt_create.timestamp()),
            gmt_modified: Set(organization.gmt_modified.map(|v| v.timestamp())),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
