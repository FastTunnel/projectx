use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_profile")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub identifier: String,
    pub display_name: Option<String>,
    pub gmt_entry: Option<i64>,
    pub gmt_leave: Option<i64>,
    pub leave: bool,
    pub email: Option<String>,
    pub email_verified: bool,
    pub phone: Option<String>,
    pub birthday: Option<i64>,
    pub country: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
    pub pinyin: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
