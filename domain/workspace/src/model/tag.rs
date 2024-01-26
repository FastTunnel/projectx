use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Default)]
pub struct Tag {
    pub id: u64,
    pub identifier: String,
    pub name: String,
    pub gmt_create: DateTime<Utc>,
    pub creator: String,
    pub color: String,
    pub space: String,
}
