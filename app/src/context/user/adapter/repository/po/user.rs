use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub identifier: String,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub disabled: bool,
    pub gmt_create: i64,
    pub creator: String,
    pub gmt_modified: Option<i64>,
    pub modifier: Option<String>,
    pub last_login_at: Option<i64>,
    pub token: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
