use app_interface::APP_STATE;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_role")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub user: String,
    pub role: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn init_table() {
    let tx = APP_STATE.db_tx();
    tx.execute_unprepared(
        r#"
        CREATE TABLE IF NOT EXISTS `user_role` (
            `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
            `user` VARCHAR(36) NOT NULL COMMENT '用户',
            `role` VARCHAR(36) NOT NULL COMMENT '角色',
            PRIMARY KEY (`id`),
            UNIQUE KEY `user_role` (`user`, `role`)
        ) comment '用户角色表' charset = utf8mb4 collate = utf8mb4_general_ci;
        "#,
    )
    .await
    .unwrap();
}
