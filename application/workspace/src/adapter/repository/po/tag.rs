use app_interface::utils::ToDateTime;
use app_interface::APP_STATE;
use domain_workspace::model::tag::Tag;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tag", comment = "标签")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: u64,
    pub identifier: String,
    pub name: String,
    pub gmt_create: i64,
    pub creator: String,
    pub color: String,
    pub space: String,
}

impl Into<Tag> for Model {
    fn into(self) -> Tag {
        Tag {
            id: self.id,
            identifier: self.identifier,
            name: self.name,
            gmt_create: self.gmt_create.to_date_time(),
            creator: self.creator,
            color: self.color,
            space: self.space,
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
        CREATE TABLE IF NOT EXISTS `tag` (
            `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
            `identifier` VARCHAR(36) NOT NULL COMMENT '标识符',
            `name` VARCHAR(255) NOT NULL DEFAULT '' COMMENT '名称',
            `gmt_create` INT NOT NULL DEFAULT (unix_timestamp()) COMMENT '创建时间',
            `creator` VARCHAR(36) NOT NULL COMMENT '创建者',
            `color` VARCHAR(10) NOT NULL DEFAULT '#000000' COMMENT '颜色',
            `space` VARCHAR(36) NOT NULL COMMENT '所属项目集/项目',
            PRIMARY KEY (`id`),
            UNIQUE KEY `identifier` (`identifier`)
        ) comment '状态表' charset = utf8mb4 collate = utf8mb4_general_ci;
        "#,
    )
    .await
    .unwrap();
}
