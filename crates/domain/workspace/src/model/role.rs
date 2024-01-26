use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: u64,
    pub identifier: String,
    pub own: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub organization: String,
    pub parent: Option<String>,
    pub default_role: bool,
    pub gmt_create: DateTime<Utc>,
    pub creator: String,
    pub is_project_set_role: bool,
    pub gmt_modified: Option<DateTime<Utc>>,
    pub modifier: Option<String>,
}
