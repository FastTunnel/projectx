use app_interface::utils::ToDateTime;
use app_interface::APP_STATE;
use domain_common::error;
use domain_common::error::DomainError;
use domain_workspace::enums::ResourceType;
use domain_workspace::model::setting::status::Status;
use sea_orm::entity::prelude::*;
use sea_orm::{DeriveEntityModel, NotSet, Set};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "status")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub identifier: String,
    pub description: String,
    pub name: String,
    pub name_en: String,
    pub gmt_create: i64,
    pub gmt_modified: Option<i64>,
    pub creator: String,
    pub modifier: Option<String>,

    pub stage_code: String,
    pub organization: String,
    pub resource_type: String,
}

impl From<Model> for error::Result<Status> {
    fn from(value: Model) -> Self {
        Ok(Status {
            id: value.id,
            identifier: value.identifier,
            description: value.description,
            name: value.name,
            name_en: value.name_en,
            gmt_create: value.gmt_create.to_date_time(),
            gmt_modified: value.gmt_modified.map(|v| v.to_date_time()),
            creator: value.creator,
            modifier: value.modifier,
            stage_code: value.stage_code,
            organization: value.organization,
            resource_type: ResourceType::from_string(&value.resource_type)
                .ok_or(DomainError::InnerError("resource type error".into()))?,
        })
    }
}

impl Into<ActiveModel> for Status {
    fn into(self) -> ActiveModel {
        ActiveModel {
            id: if self.id == 0 { NotSet } else { Set(self.id) },
            identifier: Set(self.identifier),
            description: Set(self.description),
            name: Set(self.name),
            name_en: Set(self.name_en),
            gmt_create: Set(self.gmt_create.timestamp()),
            gmt_modified: Set(self.gmt_modified.map(|v| v.timestamp())),
            creator: Set(self.creator),
            modifier: Set(self.modifier),
            stage_code: Set(self.stage_code),
            organization: Set(self.organization),
            resource_type: Set(self.resource_type.to_string()),
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
        CREATE TABLE IF NOT EXISTS `status` (
            `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
            `identifier` VARCHAR(36) NOT NULL COMMENT '标识符',
            `description` VARCHAR(255) NOT NULL DEFAULT '' COMMENT '描述',
            `name` VARCHAR(255) NOT NULL DEFAULT '' COMMENT '名称',
            `name_en` VARCHAR(255) NOT NULL DEFAULT '' COMMENT '英文名称',
            `gmt_create` INT NOT NULL DEFAULT (unix_timestamp()) COMMENT '创建时间',
            `gmt_modified` INT NULL DEFAULT NULL COMMENT '修改时间',
            `creator` VARCHAR(36) NOT NULL COMMENT '创建者',
            `modifier` VARCHAR(36) NULL DEFAULT NULL COMMENT '修改者',
            `stage_code` VARCHAR(36) NOT NULL COMMENT '阶段',
            `organization` VARCHAR(36) NOT NULL COMMENT '组织',
            `resource_type` VARCHAR(36) NOT NULL COMMENT '资源类型',
            PRIMARY KEY (`id`),
            UNIQUE KEY `identifier` (`identifier`)
        ) comment '状态表' charset = utf8mb4 collate = utf8mb4_general_ci;
        "#,
    )
    .await
    .unwrap();
}
