use app_interface::utils::ToDateTime;
use sea_orm::entity::prelude::*;
use sea_orm::{NotSet, Set};

use domain_user::model::Role;

#[derive(Clone, Debug, Hash, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "role")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub identifier: String,
    pub name: String,
    pub description: Option<String>,
    pub organization: String,
    pub parent: Option<String>,
    pub default_role: bool,
    pub gmt_create: i64,
    pub is_project_set_role: bool,
    pub creator: String,
    pub own: Option<String>,
    pub gmt_modified: Option<i64>,
    pub modifier: Option<String>,
}

impl From<Model> for Role {
    fn from(value: Model) -> Self {
        Role {
            id: value.id,
            identifier: value.identifier,
            own: None,
            name: value.name,
            description: value.description,
            organization: value.organization,
            parent: value.parent,
            default_role: value.default_role,
            gmt_create: value.gmt_create.to_date_time(),
            is_project_set_role: value.is_project_set_role,
            creator: value.creator,
            gmt_modified: value.gmt_modified.map(|v| v.to_date_time()),
            modifier: value.modifier,
            permissions: vec![],
        }
    }
}

impl Into<ActiveModel> for &mut Role {
    fn into(self) -> ActiveModel {
        let role = self;
        ActiveModel {
            id: if role.id == 0 { NotSet } else { Set(role.id) },
            identifier: Set(role.identifier.clone()),
            name: Set(role.name.clone()),
            description: Set(role.description.clone()),
            organization: Set(role.organization.clone()),
            parent: Set(role.parent.clone()),
            default_role: Set(role.default_role),
            gmt_create: Set(role.gmt_create.timestamp()),
            is_project_set_role: Set(role.is_project_set_role),
            creator: Set(role.creator.clone()),
            own: Set(role.own.clone()),
            gmt_modified: Set(role.gmt_modified.map(|v| v.timestamp())),
            modifier: Set(role.modifier.clone()),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
