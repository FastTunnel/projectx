use serde_json::Value;

pub struct CreateTemplateParam {
    pub name: String,
    pub custom_code: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub organization: String,
}

#[derive(Debug, Clone, Default)]
pub struct SpaceWorkItem {
    pub id: String,
    pub identifier: String,
    pub assigned_to: String,
    pub category: String,
    pub creator: String,
    pub document: String,
    pub finish_time: Option<i64>,
    pub gmt_create: i64,
    pub gmt_modified: Option<i64>,
    pub modifier: Option<String>,
    pub parent: Option<String>,
    pub serial_number: i32,
    pub space: Option<String>,
    pub space_type: String,
    pub status: String,
    pub status_stage: String,
    pub subject: String,
    pub update_status_at: i64,
    pub work_item_type: String,
}

pub struct FieldValue {
    pub id: u64,
    pub field_identifier: String,
    pub field_name: String,
    pub field_type: String,
    pub value: Value,
}

