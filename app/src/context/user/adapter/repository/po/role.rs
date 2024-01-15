use sea_orm::entity::prelude::*;

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
    pub creator: String,
    pub gmt_modified: Option<i64>,
    pub modifier: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
