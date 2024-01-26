use app_interface::APP_STATE;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "team_member")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub team: String,
    pub user: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn init_table() {
    let tx = APP_STATE.db_tx();
    tx.execute_unprepared(
        r#"
        CREATE TABLE IF NOT EXISTS `team_member` (
            `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
            `team` VARCHAR(36) NOT NULL COMMENT '团队',
            `user` VARCHAR(36) NOT NULL COMMENT '用户',
            PRIMARY KEY (`id`),
            UNIQUE KEY `team_member` (`team`, `user`)
        ) comment '团队成员表' charset = utf8mb4 collate = utf8mb4_general_ci;
        "#,
    )
    .await
    .unwrap();
}
