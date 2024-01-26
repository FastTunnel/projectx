use app_interface::utils::ToDateTime;
use app_interface::APP_STATE;
use domain_user::model::User;
use sea_orm::entity::prelude::*;
use sea_orm::{NotSet, Set};

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

impl From<Model> for User {
    fn from(value: Model) -> Self {
        User {
            id: value.id,
            identifier: value.identifier,
            username: value.username,
            password: value.password,
            salt: value.salt,
            disabled: value.disabled,
            gmt_create: value.gmt_create.to_date_time(),
            creator: value.creator,
            gmt_modified: value.gmt_modified.map(|v| v.to_date_time()),
            modifier: value.modifier,
            last_login_at: value.last_login_at.map(|v| v.to_date_time()),
            token: value.token,
            user_profile: None,
            roles: vec![],
        }
    }
}

impl Into<ActiveModel> for &mut User {
    fn into(self) -> ActiveModel {
        let user = self;
        ActiveModel {
            id: if user.id == 0 { NotSet } else { Set(user.id) },
            identifier: Set(user.identifier.clone()),
            username: Set(user.username.clone()),
            password: Set(user.password.clone()),
            salt: Set(user.salt.clone()),
            disabled: Set(user.disabled),
            gmt_create: Set(user.gmt_create.timestamp()),
            creator: Set(user.creator.clone()),
            gmt_modified: Set(user.gmt_modified.map(|v| v.timestamp())),
            modifier: Set(user.modifier.clone()),
            last_login_at: Set(user.last_login_at.map(|v| v.timestamp())),
            token: Set(user.token.clone()),
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
        CREATE TABLE IF NOT EXISTS `user` (
          `id` bigint unsigned NOT NULL AUTO_INCREMENT,
          `identifier` varchar(36) COLLATE utf8mb4_unicode_ci NOT NULL,
          `username` varchar(100) COLLATE utf8mb4_unicode_ci NOT NULL,
          `password` varchar(100) COLLATE utf8mb4_unicode_ci NOT NULL,
          `salt` char(100) COLLATE utf8mb4_unicode_ci NOT NULL,
          `disabled` tinyint(1) NOT NULL DEFAULT '0',
          `gmt_create` int NOT NULL,
          `creator` varchar(36) COLLATE utf8mb4_unicode_ci NOT NULL,
          `gmt_modified` int DEFAULT NULL,
          `modifier` varchar(36) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
          `last_login_at` int DEFAULT NULL,
          `token` varchar(200) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
          PRIMARY KEY (`id`),
          UNIQUE KEY `identifier` (`identifier`),
          UNIQUE KEY `username` (`username`)
        ) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户表'
        "#,
    )
    .await
    .unwrap();
}
