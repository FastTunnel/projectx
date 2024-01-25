use crate::model::role::Role;
use crate::model::setting::base::FlowItem;
use crate::model::setting::field::Field;
use crate::model::setting::global::GlobalConfig;
use crate::model::setting::space_work_item_set::SpaceWorkItemSet;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub static TEMPLATE_KEY: &'static str = "/template/v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub gmt_create: DateTime<Utc>,
    pub gmt_modified: Option<DateTime<Utc>>,
    pub creator: String,
    pub modifier: Option<String>,
    pub identifier: String,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub enable: bool,

    pub organization: String,
    pub project_fields: Vec<Field>,
    pub project_roles: Vec<Role>,
    pub(crate) project_status_flow: Vec<FlowItem>,
    pub(crate) project_work_item_set: Vec<SpaceWorkItemSet>,
}

impl Template {
    pub fn init_template(org_id: &str, global_config: &GlobalConfig) -> Vec<Self> {
        vec![
            Template {
                gmt_create: Utc::now(),
                gmt_modified: None,
                creator: "system".to_string(),
                modifier: None,
                identifier: uuid::Uuid::new_v4().to_string(),
                name: "默认模板".to_string(),

                display_name: "默认模板".to_string(),
                description: None,
                icon: None,
                enable: true,
                organization: org_id.to_string(),
                project_fields: global_config.project_fields.clone(),
                project_roles: Vec::new(),
                project_status_flow: global_config.project_status_flow.clone(),
                project_work_item_set: global_config.project_work_item_set.clone(),
            },
            Template {
                gmt_create: Utc::now(),
                gmt_modified: None,
                creator: "system".to_string(),
                modifier: None,
                identifier: uuid::Uuid::new_v4().to_string(),
                name: "Scrum".to_string(),

                display_name: "Scrum".to_string(),
                description: None,
                icon: None,
                enable: true,
                organization: org_id.to_string(),
                project_fields: global_config.project_fields.clone(),
                project_roles: Vec::new(),
                project_status_flow: global_config.project_status_flow.clone(),
                project_work_item_set: global_config.project_work_item_set.clone(),
            },
        ]
    }
}
