use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowItem {
    pub current_status_identifier: String,
    pub next_status_identifiers: Vec<String>,
}

impl FlowItem {
    pub(crate) fn init_project_flow_item() -> Vec<FlowItem> {
        vec![
            FlowItem {
                current_status_identifier: "NotStarted".to_string(),
                next_status_identifiers: vec!["InProgress".to_string(), "Completed".to_string()],
            },
            FlowItem {
                current_status_identifier: "InProgress".to_string(),
                next_status_identifiers: vec!["NotStarted".to_string(), "Completed".to_string()],
            },
            FlowItem {
                current_status_identifier: "Completed".to_string(),
                next_status_identifiers: vec!["NotStarted".to_string(), "InProgress".to_string()],
            },
        ]
    }
}
