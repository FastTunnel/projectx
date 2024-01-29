use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub identifier: String,
    pub name: String,
    pub email: Option<String>,
}
