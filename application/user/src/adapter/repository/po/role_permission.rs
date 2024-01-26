use app_interface::APP_STATE;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "role_permission")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub role: String,
    pub permission: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn init_table() {
    let tx = APP_STATE.db_tx();
    tx.execute_unprepared(
        r#"
        CREATE TABLE IF NOT EXISTS `role_permission` (
            `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
            `role` VARCHAR(36) NOT NULL COMMENT '角色',
            `permission` VARCHAR(36) NOT NULL COMMENT '权限',
            PRIMARY KEY (`id`),
            UNIQUE KEY `role_permission` (`role`, `permission`)
        ) comment '角色权限表' charset = utf8mb4 collate = utf8mb4_general_ci;
        "#,
    )
    .await
    .unwrap();
}
