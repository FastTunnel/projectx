use app_interface::utils::ToDateTime;
use app_interface::APP_STATE;
use sea_orm::entity::prelude::*;
use sea_orm::{NotSet, Set};

use domain_user::model::UserProfile;

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

impl From<Model> for UserProfile {
    fn from(value: Model) -> Self {
        UserProfile {
            id: value.id,
            identifier: value.identifier,
            display_name: value.display_name,
            gmt_entry: value.gmt_entry.map(|v| v.to_date_time()),
            gmt_leave: value.gmt_leave.map(|v| v.to_date_time()),
            leave: value.leave,
            email: value.email,
            email_verified: value.email_verified,
            phone: value.phone,
            birthday: value.birthday.map(|v| v.to_date_time()),
            country: value.country,
            province: value.province,
            city: value.city,
            address: value.address,
            pinyin: value.pinyin,
            avatar: value.avatar,
        }
    }
}

impl Into<ActiveModel> for &mut UserProfile {
    fn into(self) -> ActiveModel {
        let user_profile = self;
        ActiveModel {
            id: if user_profile.id == 0 {
                NotSet
            } else {
                Set(user_profile.id)
            },
            identifier: Set(user_profile.identifier.clone()),
            display_name: Set(user_profile.display_name.clone()),
            gmt_entry: Set(user_profile.gmt_entry.map(|v| v.timestamp())),
            gmt_leave: Set(user_profile.gmt_leave.map(|v| v.timestamp())),
            leave: Set(user_profile.leave),
            email: Set(user_profile.email.clone()),
            email_verified: Set(user_profile.email_verified),
            phone: Set(user_profile.phone.clone()),
            birthday: Set(user_profile.birthday.map(|v| v.timestamp())),
            country: Set(user_profile.country.clone()),
            province: Set(user_profile.province.clone()),
            city: Set(user_profile.city.clone()),
            address: Set(user_profile.address.clone()),
            pinyin: Set(user_profile.pinyin.clone()),
            avatar: Set(user_profile.avatar.clone()),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn init_table() {
    let tx = APP_STATE.db_tx();
    tx.execute_unprepared(
        r#"
           CREATE TABLE IF NOT EXISTS `user_profile` (
              `id` bigint unsigned NOT NULL AUTO_INCREMENT,
              `identifier` varchar(36) COLLATE utf8mb4_unicode_ci NOT NULL,
              `display_name` varchar(100) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
              `gmt_entry` int DEFAULT NULL,
              `gmt_leave` int DEFAULT NULL,
              `leave` tinyint(1) NOT NULL DEFAULT '0',
              `email` varchar(100) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
              `email_verified` tinyint(1) NOT NULL DEFAULT '0',
              `phone` varchar(20) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
              `birthday` int DEFAULT NULL,
              `country` varchar(20) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
              `province` varchar(20) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
              `city` varchar(20) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
              `address` varchar(200) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
              `pinyin` varchar(200) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
              `avatar` varchar(200) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
              PRIMARY KEY (`id`),
              UNIQUE KEY `identifier` (`identifier`)
            ) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户资料表'
        "#,
    )
    .await
    .unwrap();
}
