use app_interface::utils::{ToDateTime, ToTimestamp};
use domain_common::error;
use domain_workspace::enums::{FieldType, ResourceType};
use domain_workspace::model::setting::field::Field;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter, NotSet, Set};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "field")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub identifier: String,
    pub gmt_create: i64,
    pub gmt_modified: Option<i64>,
    pub creator: String,
    pub modifier: Option<String>,
    pub name: String,
    pub name_en: String,
    pub display_name: String,
    pub description: Option<String>,
    pub field_type: String,
    pub resource_type: String,
    pub format: Option<serde_json::Value>,
    pub default_value: Option<serde_json::Value>,
    pub options: Option<serde_json::Value>,
    pub is_required: bool,
    pub unit: Option<String>,
    pub verify_condition: Option<serde_json::Value>,
    pub position: i32,
    pub default_value_list: Option<serde_json::Value>,
    pub is_system_required: bool,
    pub hint: Option<String>,
    pub is_show_when_create: bool,
    pub is_hidden: bool,
    pub cascading_option: Option<serde_json::Value>,
    pub has_read_only_rule: bool,
    pub is_deleted: bool,
    pub organization: String,
    pub space: String,
    pub extra: Option<serde_json::Value>,
}

impl From<Model> for error::Result<Field> {
    fn from(value: Model) -> Self {
        Ok(Field {
            id: value.id,
            identifier: value.identifier,
            gmt_create: value.gmt_create.to_date_time(),
            gmt_modified: value.gmt_modified.map(|v| v.to_date_time()),
            creator: value.creator,
            modifier: value.modifier,
            name: value.name,
            name_en: value.name_en,
            display_name: value.display_name,
            description: value.description,
            field_type: FieldType::from_string(&value.field_type)
                .ok_or(error::DomainError::InnerError("field type error".into()))?,
            resource_type: ResourceType::from_string(&value.resource_type)
                .ok_or(error::DomainError::InnerError("resource type error".into()))?,
            format: value.format,
            default_value: value.default_value,
            options: value.options,
            is_required: value.is_required,
            unit: value.unit,
            verify_condition: value.verify_condition,
            position: value.position,
            default_value_list: value.default_value_list,
            is_system_required: value.is_system_required,
            hint: value.hint,
            is_show_when_create: value.is_show_when_create,
            is_hidden: value.is_hidden,
            cascading_option: value.cascading_option,

            has_read_only_rule: value.has_read_only_rule,
            is_deleted: value.is_deleted,
            organization: value.organization,
            space: value.space,
            extra: value.extra,
        })
    }
}

impl Into<ActiveModel> for Field {
    fn into(self) -> ActiveModel {
        ActiveModel {
            id: if self.id == 0 { NotSet } else { Set(self.id) },
            identifier: Set(self.identifier),
            gmt_create: Set(self.gmt_create.to_timestamp()),
            gmt_modified: Set(self.gmt_modified.map(|v| v.to_timestamp())),
            creator: Set(self.creator),
            modifier: Set(self.modifier),
            name: Set(self.name),
            name_en: Set(self.name_en),
            display_name: Set(self.display_name),

            description: Set(self.description),
            field_type: Set(self.field_type.to_string()),
            resource_type: Set(self.resource_type.to_string()),
            format: Set(self.format),
            default_value: Set(self.default_value),
            options: Set(self.options),
            is_required: Set(self.is_required),
            unit: Set(self.unit),

            verify_condition: Set(self.verify_condition),
            position: Set(self.position),
            default_value_list: Set(self.default_value_list),
            is_system_required: Set(self.is_system_required),
            hint: Set(self.hint),
            is_show_when_create: Set(self.is_show_when_create),
            is_hidden: Set(self.is_hidden),
            cascading_option: Set(self.cascading_option),
            has_read_only_rule: Set(self.has_read_only_rule),
            is_deleted: Set(self.is_deleted),
            organization: Set(self.organization),
            space: Set(self.space),
            extra: Set(self.extra),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
