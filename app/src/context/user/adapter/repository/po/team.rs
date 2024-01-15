use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "team")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub identifier: String,
    pub name: String,
    pub description: Option<String>,
    pub organization: String,
    pub public: bool,
    pub parent: Option<String>,
    pub gmt_create: i64,
    pub creator: String,
    pub gmt_modified: Option<i64>,
    pub modifier: Option<String>,
    pub icon: Option<String>,
    pub leader: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
