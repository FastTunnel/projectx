use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkTimeType {
    pub id: u64,
    pub identifier: String,
    pub name: String,
    pub gmt_create: DateTime<Utc>,
    pub creator: String,
    pub organization: String,
    pub order: i32,
}

impl WorkTimeType {
    pub fn init_work_time_type(organization: String) -> Vec<WorkTimeType> {
        vec![
            WorkTimeType {
                id: 0,
                identifier: uuid::Uuid::new_v4().to_string(),
                name: "设计".to_string(),
                gmt_create: Utc::now(),
                creator: "system".to_string(),
                organization: organization.clone(),
                order: 1,
            },
            WorkTimeType {
                id: 0,
                identifier: uuid::Uuid::new_v4().to_string(),
                name: "研发".to_string(),
                gmt_create: Utc::now(),
                creator: "system".to_string(),
                organization: organization.clone(),
                order: 2,
            },
            WorkTimeType {
                id: 0,
                identifier: uuid::Uuid::new_v4().to_string(),
                name: "联调".to_string(),
                gmt_create: Utc::now(),
                creator: "system".to_string(),
                organization: organization.clone(),
                order: 3,
            },
            WorkTimeType {
                id: 0,
                identifier: uuid::Uuid::new_v4().to_string(),
                name: "测试".to_string(),
                gmt_create: Utc::now(),
                creator: uuid::Uuid::new_v4().to_string(),
                organization: organization.clone(),
                order: 4,
            },
            WorkTimeType {
                id: 0,
                identifier: uuid::Uuid::new_v4().to_string(),
                name: "文档".to_string(),
                gmt_create: Utc::now(),
                creator: "system".to_string(),
                organization: organization.clone(),
                order: 5,
            },
        ]
    }
}
