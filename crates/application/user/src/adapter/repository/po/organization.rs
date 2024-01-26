use app_interface::utils::ToDateTime;
use app_interface::APP_STATE;
use sea_orm::entity::prelude::*;
use sea_orm::{NotSet, Set};

use domain_user::model::Organization;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "organization")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub identifier: String,
    pub name: String,
    pub pinyin: String,
    pub public: bool,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub gmt_create: i64,
    pub gmt_modified: Option<i64>,
}

impl From<Model> for Organization {
    fn from(value: Model) -> Self {
        Organization {
            id: value.id,
            identifier: value.identifier,
            name: value.name,
            pinyin: value.pinyin,
            public: value.public,
            icon: value.icon,
            description: value.description,
            gmt_create: value.gmt_create.to_date_time(),
            gmt_modified: value.gmt_modified.map(|v| v.to_date_time()),
        }
    }
}

impl Into<ActiveModel> for &mut Organization {
    fn into(self) -> ActiveModel {
        let organization = self;
        ActiveModel {
            id: if organization.id == 0 {
                NotSet
            } else {
                Set(organization.id)
            },
            identifier: Set(organization.identifier.clone()),
            name: Set(organization.name.clone()),
            pinyin: Set(organization.pinyin.clone()),
            public: Set(organization.public),
            icon: Set(organization.icon.clone()),
            description: Set(organization.description.clone()),
            gmt_create: Set(organization.gmt_create.timestamp()),
            gmt_modified: Set(organization.gmt_modified.map(|v| v.timestamp())),
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
        CREATE TABLE IF NOT EXISTS `organization` (
            `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
            `identifier` VARCHAR(36) NOT NULL COMMENT '标识符',
            `name` VARCHAR(255) NOT NULL DEFAULT '' COMMENT '名称',
            `pinyin` VARCHAR(255) NOT NULL DEFAULT '' COMMENT '拼音',
            `public` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否公开',
            `icon` VARCHAR(255) NULL COMMENT '图标',
            `description` VARCHAR(255) NULL COMMENT '描述',
            `gmt_create` INT NOT NULL DEFAULT (unix_timestamp()) COMMENT '创建时间',
            `gmt_modified` INT NULL COMMENT '修改时间',
            PRIMARY KEY (`id`),
            UNIQUE KEY `identifier` (`identifier`)
        ) comment '组织表' charset = utf8mb4 collate = utf8mb4_general_ci;
        "#,
    )
    .await
    .unwrap();
}
