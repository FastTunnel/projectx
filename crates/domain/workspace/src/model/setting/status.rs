use crate::enums::{ResourceType, SpaceStage, WorkItemStage};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub id: u64,
    pub identifier: String,
    pub description: String,
    pub name: String,
    pub name_en: String,
    pub gmt_create: DateTime<Utc>,
    pub gmt_modified: Option<DateTime<Utc>>,
    pub creator: String,
    pub modifier: Option<String>,
    pub stage_code: String,
    pub organization: String,
    pub resource_type: ResourceType,
}
impl Status {
    fn new_global_status(
        identifier: &str,
        name: &str,
        stage_code: String,
        organization: &str,
        resource_type: ResourceType,
    ) -> Self {
        Status {
            id: 0,
            identifier: identifier.to_owned(),
            description: name.to_owned(),
            name: name.to_owned(),
            name_en: identifier.to_owned(),
            gmt_create: Utc::now(),
            gmt_modified: None,
            creator: "system".to_string(),
            modifier: None,
            stage_code,
            organization: organization.to_owned(),
            resource_type,
        }
    }
}

impl Status {
    pub fn init_work_item_status(org_id: &str) -> Vec<Status> {
        vec![
            Self::new_global_status(
                "Pending",
                "待处理",
                WorkItemStage::Ack.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "Selected",
                "已选择",
                WorkItemStage::Ack.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "Processing",
                "处理中",
                WorkItemStage::Handle.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "Analyze",
                "分析中",
                WorkItemStage::Analyze.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "Completed",
                "已完成",
                WorkItemStage::NormalEnd.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "Canceled",
                "已取消",
                WorkItemStage::AbnormalEnd.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "Rejected",
                "已拒绝",
                WorkItemStage::AbnormalEnd.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "Reopened",
                "已重开",
                WorkItemStage::Ack.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "Reopen",
                "再次打开",
                WorkItemStage::Ack.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "Closed",
                "已关闭",
                WorkItemStage::NormalEnd.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "Designing",
                "设计中",
                WorkItemStage::Design.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "DesignComplete",
                "设计完成",
                WorkItemStage::Design.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "Unconfirmed",
                "待确认",
                WorkItemStage::Ack.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "Develop",
                "开发中",
                WorkItemStage::Develop.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "DevelopCompleted",
                "开发完成",
                WorkItemStage::Develop.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "JointDebugging",
                "联调中",
                WorkItemStage::Develop.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "Testing",
                "测试中",
                WorkItemStage::Test.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "TestComplete",
                "测试完成",
                WorkItemStage::Test.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "Fixed",
                "已修复",
                WorkItemStage::Verify.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "NoFixYet",
                "暂不修复",
                WorkItemStage::Verify.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "InvalidBug",
                "无效缺陷",
                WorkItemStage::Verify.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "DuplicateBug",
                "重复缺陷",
                WorkItemStage::Verify.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "NotReproducibleBug",
                "无法重现",
                WorkItemStage::Verify.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "Publishing",
                "发布中",
                WorkItemStage::Release.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "PublishComplete",
                "发布完成",
                WorkItemStage::Release.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "ClosedAndFix",
                "已关闭（已修复）",
                WorkItemStage::NormalEnd.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "ClosedAndNoFix",
                "已关闭（未修复）",
                WorkItemStage::NormalEnd.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "PostponeFix",
                "推迟修复",
                WorkItemStage::Ack.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "Confirmed",
                "已确认",
                WorkItemStage::Ack.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "PrepareDevelop",
                "待开发",
                WorkItemStage::Develop.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "PrepareAccept",
                "待验收",
                WorkItemStage::Verify.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
            Self::new_global_status(
                "Acceptance",
                "验收中",
                WorkItemStage::Verify.to_string(),
                org_id,
                ResourceType::WorkItem,
            ),
        ]
    }
    pub fn init_project_status(org_id: &str) -> Vec<Status> {
        vec![
            Self::new_global_status(
                "NotStarted",
                "未开始",
                SpaceStage::NotStarted.to_string(),
                org_id,
                ResourceType::Project,
            ),
            Self::new_global_status(
                "InProgress",
                "进行中",
                SpaceStage::InProgress.to_string(),
                org_id,
                ResourceType::Project,
            ),
            Self::new_global_status(
                "Completed",
                "已完成",
                SpaceStage::Completed.to_string(),
                org_id,
                ResourceType::Project,
            ),
        ]
    }
    pub fn init_project_set_status(org_id: String) -> Vec<Status> {
        vec![
            Self::new_global_status(
                "not_started",
                "未开始",
                SpaceStage::NotStarted.to_string(),
                &org_id,
                ResourceType::ProjectSet,
            ),
            Self::new_global_status(
                "in_progress",
                "进行中",
                SpaceStage::InProgress.to_string(),
                &org_id,
                ResourceType::ProjectSet,
            ),
            Self::new_global_status(
                "completed",
                "已完成",
                SpaceStage::Completed.to_string(),
                &org_id,
                ResourceType::ProjectSet,
            ),
        ]
    }
}
