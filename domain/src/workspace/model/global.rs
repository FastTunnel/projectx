use chrono::{DateTime, Utc};
use json::JsonValue;

use crate::workspace::enums::{FieldType, ResourceType};

pub struct Field {
    pub id: u64,
    pub identifier: String,
    pub gmt_create: DateTime<Utc>,
    pub gmt_modified: Option<i64>,
    pub creator: String,
    pub modifier: Option<String>,
    pub name: String,
    pub name_en: String,
    pub display_name: String,
    pub description: Option<String>,
    pub field_type: FieldType,
    pub resource_type: ResourceType,
    pub format: Option<JsonValue>,
    pub class_name: String,
    pub default_value: Option<JsonValue>,
    pub options: Option<JsonValue>,
    pub is_required: bool,
    pub unit: Option<String>,
    pub verify_condition: Option<JsonValue>,
    pub position: i32,
    pub default_value_list: Option<JsonValue>,
    pub is_system_required: bool,
    pub hint: Option<String>,
    pub is_show_when_create: bool,
    pub is_hidden: bool,
    pub cascading_option: Option<JsonValue>,
    pub is_org_global: bool,
    pub has_read_only_rule: bool,
    pub is_deleted: bool,
    pub extra: Option<JsonValue>,
}

pub struct CreateFieldParam {
    pub name: String,
    pub name_en: String,
    pub display_name: String,
    pub description: Option<String>,
    pub field_type: FieldType,
    pub resource_type: ResourceType,
    pub format: Option<JsonValue>,
    pub default_value: Option<JsonValue>,
    pub options: Option<JsonValue>,
    pub is_required: bool,
    pub unit: Option<String>,
    pub verify_condition: Option<JsonValue>,
    pub default_value_list: Option<JsonValue>,
    pub hint: Option<String>,
    pub is_show_when_create: bool,
    pub is_hidden: bool,
    pub extra: Option<JsonValue>,
}

impl Field {
    pub fn new_filed(
        CreateFieldParam {
            name,
            name_en,
            display_name,
            description,
            field_type,
            resource_type,
            format,
            default_value,
            options,
            is_required,
            unit,
            verify_condition,
            default_value_list,
            hint,
            is_show_when_create,
            is_hidden,
            extra,
        }: CreateFieldParam,
        creator: impl Into<String>,
    ) -> Field {
        let id = uuid::Uuid::new_v4().to_string();
        Field {
            id: 0,
            identifier: id,
            gmt_create: Utc::now(),
            gmt_modified: None,
            creator: creator.into(),
            modifier: None,
            name,
            name_en,
            display_name,
            description,
            field_type,
            resource_type: resource_type.clone(),
            format,
            class_name: resource_type.to_string(),
            default_value,
            options,
            is_required,
            unit,
            verify_condition,
            position: 0,
            default_value_list,
            is_system_required: false,
            hint,
            is_show_when_create,
            is_hidden,
            cascading_option: None,
            is_org_global: false,
            has_read_only_rule: false,
            is_deleted: false,
            extra,
        }
    }
}

pub struct Status {
    pub id: u64,
    pub identifier: String,
    pub description: String,
    pub name: String,
    pub name_en: String,
    pub gmt_create: i64,
    pub gmt_modified: Option<i64>,
    pub creator: String,
    pub modifier: Option<String>,
    pub resource_type: ResourceType,
    pub stage_code: String,
}

pub struct WorkTimeType {
    pub id: u64,
    pub identifier: String,
    pub name: String,
    pub gmt_create: i64,
    pub creator: String,
    pub order: i32,
}

pub struct WorkItemField {
    pub id: u64,
    pub field_identifier: String,
    pub field_name: String,
    pub field_type: String,
    pub add_user_identifier: String,
    pub add_user_name: String,
    pub add_date_time: String,
    pub is_required: bool,
    pub create_display: bool,
    pub order: i32,
    pub default_value: Option<JsonValue>,
}

pub struct WorkItemFlowItem {
    pub current_status_identifier: String,
    pub next_status_identifiers: Vec<String>,
}

pub struct WorkItem {
    pub id: u64,
    pub identifier: String,
    pub category: String,
    pub description: String,
    pub enable: bool,
    pub gmt_create: i64,
    pub creator: String,
    pub icon: Option<String>,
    pub is_deleted: bool,
    pub name: String,
    pub name_en: String,
    pub system_default: bool,
    pub gmt_modified: Option<i64>,
    pub modified_identifier: Option<String>,
    pub fields: Vec<Field>,
    pub flow_items: Vec<WorkItemFlowItem>,
}
