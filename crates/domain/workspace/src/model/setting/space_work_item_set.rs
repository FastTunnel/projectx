use crate::enums::Category;
use crate::model::setting::base::FlowItem;
use crate::model::setting::field::Field;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceWorkItemSet {
    pub id: u64,
    pub identifier: String,
    pub category: Category,
    pub creator: String,
    pub gmt_create: DateTime<Utc>,
    pub gmt_modified: Option<DateTime<Utc>>,
    pub modifier: Option<String>,
    pub name: String,
    pub name_en: String,
    pub display_name: String,
    pub description: String,
    pub space: String,
    pub is_deleted: bool,
    pub is_system: bool,
    pub work_item_fields: Vec<Field>,
    pub work_item_status_flow: Vec<FlowItem>,
    pub organization: String,
}

impl SpaceWorkItemSet {
    fn init_system_config(
        identifier: &str,
        name: &str,
        organization: &str,
        category: Category,
        work_item_fields: Vec<Field>,
        work_item_status_flow: Vec<FlowItem>,
    ) -> Self {
        SpaceWorkItemSet {
            id: 0,
            identifier: identifier.to_string(),
            category,
            creator: "system".to_string(),
            gmt_create: Utc::now(),
            gmt_modified: None,
            modifier: None,
            name: name.to_string(),
            name_en: identifier.to_string(),
            display_name: name.to_string(),
            description: name.to_string(),
            space: "system".to_string(),
            is_deleted: false,
            is_system: true,
            work_item_fields,
            work_item_status_flow,
            organization: organization.to_string(),
        }
    }
    pub fn init_work_item_set(org_id: &str, fields: &[Field]) -> Vec<SpaceWorkItemSet> {
        let original_requirement_flow = vec![
            FlowItem {
                current_status_identifier: "Unconfirmed".to_string(),
                next_status_identifiers: vec!["Selected".to_string(), "Canceled".to_string()],
            },
            FlowItem {
                current_status_identifier: "Selected".to_string(),
                next_status_identifiers: vec![
                    "Unconfirmed".to_string(),
                    "Designing".to_string(),
                    "PrepareDevelop".to_string(),
                    "Develop".to_string(),
                    "Canceled".to_string(),
                ],
            },
            FlowItem {
                current_status_identifier: "Designing".to_string(),
                next_status_identifiers: vec![
                    "Unconfirmed".to_string(),
                    "PrepareDevelop".to_string(),
                    "Develop".to_string(),
                    "Canceled".to_string(),
                ],
            },
            FlowItem {
                current_status_identifier: "PrepareDevelop".to_string(),
                next_status_identifiers: vec![
                    "Designing".to_string(),
                    "Develop".to_string(),
                    "Canceled".to_string(),
                ],
            },
            FlowItem {
                current_status_identifier: "Develop".to_string(),
                next_status_identifiers: vec![
                    "Designing".to_string(),
                    "PrepareDevelop".to_string(),
                    "PrepareAccept".to_string(),
                    "Acceptance".to_string(),
                    "Canceled".to_string(),
                ],
            },
            FlowItem {
                current_status_identifier: "PrepareAccept".to_string(),
                next_status_identifiers: vec![
                    "Develop".to_string(),
                    "Acceptance".to_string(),
                    "Completed".to_string(),
                    "Canceled".to_string(),
                ],
            },
            FlowItem {
                current_status_identifier: "Acceptance".to_string(),
                next_status_identifiers: vec![
                    "Develop".to_string(),
                    "PrepareAccept".to_string(),
                    "Completed".to_string(),
                    "Canceled".to_string(),
                ],
            },
            FlowItem {
                current_status_identifier: "Completed".to_string(),
                next_status_identifiers: vec![
                    "PrepareAccept".to_string(),
                    "Acceptance".to_string(),
                ],
            },
            FlowItem {
                current_status_identifier: "Canceled".to_string(),
                next_status_identifiers: vec!["Unconfirmed".to_string()],
            },
        ];

        let topic_flow = vec![
            FlowItem {
                current_status_identifier: "Pending".to_string(),
                next_status_identifiers: vec![
                    "Designing".to_string(),
                    "PrepareDevelop".to_string(),
                    "Develop".to_string(),
                    "DevelopCompleted".to_string(),
                    "Canceled".to_string(),
                ],
            },
            FlowItem {
                current_status_identifier: "Designing".to_string(),
                next_status_identifiers: vec![
                    "Pending".to_string(),
                    "PrepareDevelop".to_string(),
                    "Develop".to_string(),
                    "DevelopCompleted".to_string(),
                    "Acceptance".to_string(),
                    "Completed".to_string(),
                    "Canceled".to_string(),
                ],
            },
            FlowItem {
                current_status_identifier: "PrepareDevelop".to_string(),
                next_status_identifiers: vec![
                    "Pending".to_string(),
                    "Designing".to_string(),
                    "Develop".to_string(),
                    "DevelopCompleted".to_string(),
                    "Acceptance".to_string(),
                    "Completed".to_string(),
                    "Canceled".to_string(),
                ],
            },
            FlowItem {
                current_status_identifier: "Develop".to_string(),
                next_status_identifiers: vec![
                    "Designing".to_string(),
                    "PrepareDevelop".to_string(),
                    "DevelopCompleted".to_string(),
                    "Acceptance".to_string(),
                    "Completed".to_string(),
                    "Canceled".to_string(),
                ],
            },
            FlowItem {
                current_status_identifier: "DevelopCompleted".to_string(),
                next_status_identifiers: vec![
                    "Designing".to_string(),
                    "PrepareDevelop".to_string(),
                    "Develop".to_string(),
                    "Acceptance".to_string(),
                    "Completed".to_string(),
                    "Canceled".to_string(),
                ],
            },
            FlowItem {
                current_status_identifier: "Acceptance".to_string(),
                next_status_identifiers: vec![
                    "Designing".to_string(),
                    "PrepareDevelop".to_string(),
                    "Develop".to_string(),
                    "DevelopCompleted".to_string(),
                    "Completed".to_string(),
                    "Canceled".to_string(),
                ],
            },
            FlowItem {
                current_status_identifier: "Completed".to_string(),
                next_status_identifiers: vec![
                    "Designing".to_string(),
                    "PrepareDevelop".to_string(),
                    "Develop".to_string(),
                    "DevelopCompleted".to_string(),
                    "Acceptance".to_string(),
                    "Canceled".to_string(),
                ],
            },
            FlowItem {
                current_status_identifier: "Canceled".to_string(),
                next_status_identifiers: vec!["Pending".to_string()],
            },
        ];

        let simple_flow = vec![
            FlowItem {
                current_status_identifier: "Pending".to_string(),
                next_status_identifiers: vec![
                    "Processing".to_string(),
                    "Completed".to_string(),
                    "Canceled".to_string(),
                ],
            },
            FlowItem {
                current_status_identifier: "Processing".to_string(),
                next_status_identifiers: vec![
                    "Pending".to_string(),
                    "Completed".to_string(),
                    "Canceled".to_string(),
                ],
            },
            FlowItem {
                current_status_identifier: "Completed".to_string(),
                next_status_identifiers: vec![
                    "Pending".to_string(),
                    "Processing".to_string(),
                    "Canceled".to_string(),
                ],
            },
            FlowItem {
                current_status_identifier: "Canceled".to_string(),
                next_status_identifiers: vec![
                    "Pending".to_string(),
                    "Processing".to_string(),
                    "Completed".to_string(),
                ],
            },
        ];

        vec![
            Self::init_system_config(
                "UserFeedback",
                "用户反馈",
                org_id,
                Category::OriginalRequirement,
                Field::clone_work_item_set_fields(
                    fields,
                    vec![
                        "status",
                        "owner",
                        "priority",
                        "belonging_project",
                        "feedback_type",
                        "participant",
                        "tracker",
                        "tag",
                        "exp_completion_time",
                    ],
                ),
                original_requirement_flow.clone(),
            ),
            Self::init_system_config(
                "BusinessRequest",
                "客户诉求",
                org_id,
                Category::OriginalRequirement,
                Field::clone_work_item_set_fields(
                    fields,
                    vec![
                        "status",
                        "owner",
                        "priority",
                        "belonging_project",
                        "participant",
                        "tracker",
                        "tag",
                        "exp_completion_time",
                    ],
                ),
                original_requirement_flow,
            ),
            Self::init_system_config(
                "Topic",
                "产品主题",
                org_id,
                Category::Topic,
                Field::clone_work_item_set_fields(
                    fields,
                    vec![
                        "status",
                        "owner",
                        "belonging_project",
                        "participant",
                        "tracker",
                        "meaning",
                        "working_effort",
                        "meaning_of_working_effort",
                    ],
                ),
                topic_flow,
            ),
            Self::init_system_config(
                "BusinessRequirement",
                "业务类需求",
                org_id,
                Category::Requirement,
                Field::clone_work_item_set_fields(
                    fields,
                    vec![
                        "status",
                        "owner",
                        "priority",
                        "belonging_project",
                        "sprint",
                        "participant",
                        "tracker",
                        "tag",
                        "planned_start_time",
                        "planned_completion_time",
                    ],
                ),
                vec![
                    FlowItem {
                        current_status_identifier: "Pending".to_string(),
                        next_status_identifiers: vec![
                            "Selected".to_string(),
                            "Canceled".to_string(),
                        ],
                    },
                    FlowItem {
                        current_status_identifier: "Selected".to_string(),
                        next_status_identifiers: vec![
                            "Analyze".to_string(),
                            "Designing".to_string(),
                            "Develop".to_string(),
                            "Completed".to_string(),
                            "Canceled".to_string(),
                        ],
                    },
                    FlowItem {
                        current_status_identifier: "Analyze".to_string(),
                        next_status_identifiers: vec![
                            "Designing".to_string(),
                            "Develop".to_string(),
                            "Completed".to_string(),
                            "Canceled".to_string(),
                        ],
                    },
                    FlowItem {
                        current_status_identifier: "Designing".to_string(),
                        next_status_identifiers: vec![
                            "Analyze".to_string(),
                            "Develop".to_string(),
                            "Completed".to_string(),
                            "Canceled".to_string(),
                        ],
                    },
                    FlowItem {
                        current_status_identifier: "Develop".to_string(),
                        next_status_identifiers: vec![
                            "Analyze".to_string(),
                            "Designing".to_string(),
                            "Completed".to_string(),
                            "Canceled".to_string(),
                        ],
                    },
                    FlowItem {
                        current_status_identifier: "Completed".to_string(),
                        next_status_identifiers: vec![
                            "Analyze".to_string(),
                            "Designing".to_string(),
                            "Develop".to_string(),
                        ],
                    },
                    FlowItem {
                        current_status_identifier: "Canceled".to_string(),
                        next_status_identifiers: vec!["Pending".to_string()],
                    },
                ],
            ),
            Self::init_system_config(
                "TechnicalRequirement",
                "技术类需求",
                org_id,
                Category::Requirement,
                Field::clone_work_item_set_fields(
                    fields,
                    vec![
                        "status",
                        "owner",
                        "priority",
                        "belonging_project",
                        "sprint",
                        "participant",
                        "tracker",
                        "tag",
                        "planned_start_time",
                        "planned_completion_time",
                    ],
                ),
                simple_flow.clone(),
            ),
            Self::init_system_config(
                "Requirement",
                "产品类需求",
                org_id,
                Category::Requirement,
                Field::clone_work_item_set_fields(
                    fields,
                    vec![
                        "status",
                        "owner",
                        "priority",
                        "belonging_project",
                        "sprint",
                        "participant",
                        "tracker",
                        "tag",
                        "planned_start_time",
                        "planned_completion_time",
                    ],
                ),
                vec![
                    FlowItem {
                        current_status_identifier: "Pending".to_string(),
                        next_status_identifiers: vec![
                            "Designing".to_string(),
                            "Develop".to_string(),
                            "Testing".to_string(),
                            "Completed".to_string(),
                            "Canceled".to_string(),
                        ],
                    },
                    FlowItem {
                        current_status_identifier: "Designing".to_string(),
                        next_status_identifiers: vec![
                            "Pending".to_string(),
                            "Develop".to_string(),
                            "Testing".to_string(),
                            "Completed".to_string(),
                            "Canceled".to_string(),
                        ],
                    },
                    FlowItem {
                        current_status_identifier: "Develop".to_string(),
                        next_status_identifiers: vec![
                            "Pending".to_string(),
                            "Designing".to_string(),
                            "Testing".to_string(),
                            "Completed".to_string(),
                            "Canceled".to_string(),
                        ],
                    },
                    FlowItem {
                        current_status_identifier: "Testing".to_string(),
                        next_status_identifiers: vec![
                            "Pending".to_string(),
                            "Designing".to_string(),
                            "Develop".to_string(),
                            "Completed".to_string(),
                            "Canceled".to_string(),
                        ],
                    },
                    FlowItem {
                        current_status_identifier: "Completed".to_string(),
                        next_status_identifiers: vec![
                            "Designing".to_string(),
                            "Develop".to_string(),
                            "Testing".to_string(),
                        ],
                    },
                    FlowItem {
                        current_status_identifier: "Canceled".to_string(),
                        next_status_identifiers: vec!["Pending".to_string()],
                    },
                ],
            ),
            Self::init_system_config(
                "Task",
                "任务",
                org_id,
                Category::Task,
                Field::clone_work_item_set_fields(
                    fields,
                    vec![
                        "status",
                        "owner",
                        "priority",
                        "belonging_project",
                        "sprint",
                        "participant",
                        "tracker",
                        "tag",
                        "planned_start_time",
                        "planned_completion_time",
                    ],
                ),
                simple_flow.clone(),
            ),
            Self::init_system_config(
                "OnlineFault",
                "线上故障",
                org_id,
                Category::Bug,
                Field::clone_work_item_set_fields(
                    fields,
                    vec![
                        "status",
                        "owner",
                        "verifier",
                        "belonging_project",
                        "sprint",
                        "priority",
                        "serious_level",
                        "participant",
                        "tracker",
                        "tag",
                        "planned_start_time",
                        "planned_completion_time",
                        "reason_for_not_fix",
                    ],
                ),
                vec![
                    FlowItem {
                        current_status_identifier: "Reopened".to_string(),
                        next_status_identifiers: vec![
                            "Processing".to_string(),
                            "Fixed".to_string(),
                            "NoFixYet".to_string(),
                        ],
                    },
                    FlowItem {
                        current_status_identifier: "Unconfirmed".to_string(),
                        next_status_identifiers: vec![
                            "Processing".to_string(),
                            "Fixed".to_string(),
                            "NoFixYet".to_string(),
                        ],
                    },
                    FlowItem {
                        current_status_identifier: "Processing".to_string(),
                        next_status_identifiers: vec!["Fixed".to_string(), "NoFixYet".to_string()],
                    },
                    FlowItem {
                        current_status_identifier: "Fixed".to_string(),
                        next_status_identifiers: vec!["Reopened".to_string(), "Closed".to_string()],
                    },
                    FlowItem {
                        current_status_identifier: "NoFixYet".to_string(),
                        next_status_identifiers: vec!["Reopened".to_string(), "Closed".to_string()],
                    },
                    FlowItem {
                        current_status_identifier: "Closed".to_string(),
                        next_status_identifiers: vec!["Reopened".to_string()],
                    },
                ],
            ),
            Self::init_system_config(
                "Bug",
                "缺陷",
                org_id,
                Category::Bug,
                Field::clone_work_item_set_fields(
                    fields,
                    vec![
                        "status",
                        "owner",
                        "verifier",
                        "priority",
                        "belonging_project",
                        "sprint",
                        "serious_level",
                        "participant",
                        "tracker",
                        "tag",
                        "planned_start_time",
                        "planned_completion_time",
                        "reason_for_not_fix",
                    ],
                ),
                vec![
                    FlowItem {
                        current_status_identifier: "Unconfirmed".to_string(),
                        next_status_identifiers: vec![
                            "Processing".to_string(),
                            "Fixed".to_string(),
                            "NoFixYet".to_string(),
                            "Closed".to_string(),
                        ],
                    },
                    FlowItem {
                        current_status_identifier: "Reopened".to_string(),
                        next_status_identifiers: vec![
                            "Processing".to_string(),
                            "Fixed".to_string(),
                            "NoFixYet".to_string(),
                        ],
                    },
                    FlowItem {
                        current_status_identifier: "Processing".to_string(),
                        next_status_identifiers: vec!["Fixed".to_string(), "NoFixYet".to_string()],
                    },
                    FlowItem {
                        current_status_identifier: "Fixed".to_string(),
                        next_status_identifiers: vec!["Reopened".to_string(), "Closed".to_string()],
                    },
                    FlowItem {
                        current_status_identifier: "NoFixYet".to_string(),
                        next_status_identifiers: vec!["Reopened".to_string(), "Closed".to_string()],
                    },
                    FlowItem {
                        current_status_identifier: "Closed".to_string(),
                        next_status_identifiers: vec!["Reopened".to_string()],
                    },
                ],
            ),
            Self::init_system_config(
                "Risk",
                "风险",
                org_id,
                Category::Risk,
                Field::clone_work_item_set_fields(
                    fields,
                    vec![
                        "status",
                        "owner",
                        "priority",
                        "belonging_project",
                        "participant",
                        "tracker",
                        "tag",
                        "planned_start_time",
                        "planned_completion_time",
                    ],
                ),
                simple_flow,
            ),
        ]
    }
}
