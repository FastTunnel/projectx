use app_interface::APP_STATE;
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

pub async fn init_table() {
    let tx = APP_STATE.db_tx();
    tx.execute_unprepared(
        r#"
        CREATE TABLE IF NOT EXISTS `permission` (
            `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
            `key` VARCHAR(36) NOT NULL COMMENT '权限key',
            `name` VARCHAR(36) NOT NULL COMMENT '权限名称',
            `parent_key` VARCHAR(36) COMMENT '父权限key',
            `is_group` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否是权限组',
            PRIMARY KEY (`id`),
            UNIQUE KEY `key` (`key`)
        ) comment '权限表' charset = utf8mb4 collate = utf8mb4_general_ci;
        "#,
    )
    .await
    .unwrap();
}
