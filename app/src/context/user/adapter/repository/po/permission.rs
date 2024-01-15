use sea_orm::entity::prelude::*;

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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
