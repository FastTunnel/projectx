use app_interface::APP_STATE;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "space_member")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub space: String,
    pub member: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn init_table() {
    let tx = APP_STATE.db_tx();
    tx.execute_unprepared(
        r#"
        CREATE TABLE IF NOT EXISTS `space_member` (
            `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
            `space` VARCHAR(36) NOT NULL COMMENT '项目集',
            `member` VARCHAR(36) NOT NULL COMMENT '成员',
            PRIMARY KEY (`id`),
            UNIQUE KEY `space_member` (`space`, `member`)
        ) comment '项目集成员表' charset = utf8mb4 collate = utf8mb4_general_ci;
        "#,
    )
    .await
    .unwrap();
}
