use crate::model::role::Role;
use crate::model::setting::base::FlowItem;
use crate::model::setting::field::Field;
use crate::model::setting::global::GlobalConfig;
use crate::model::setting::space_work_item_set::SpaceWorkItemSet;
use crate::model::setting::status::Status;
use crate::model::setting::template::Template;
use crate::model::tag::Tag;
use crate::model::user::User;
use chrono::{DateTime, Utc};
use domain_common::error;

#[derive(Debug, Clone)]
pub struct ProjectSet {
    pub id: u64,
    pub identifier: String,
    pub organization: String,
    pub custom_code: String,
    pub description: Option<String>,
    pub gmt_create: DateTime<Utc>,
    pub gmt_modified: Option<DateTime<Utc>>,
    pub icon: Option<String>,
    pub creator: String,
    pub modifier: Option<String>,
    pub name: String,
    pub status_identifier: String,

    pub status: Option<Status>,
    pub members: Vec<User>,
    pub roles: Vec<Role>,
    pub status_flow: Vec<FlowItem>,
}

impl ProjectSet {
    pub fn new_project_set(
        param: CreateProjectSetParam,
        global: &GlobalConfig,
        creator: impl Into<String>,
    ) -> error::Result<Self> {
        let identifier = uuid::Uuid::new_v4().to_string();
        let status = global
            .project_set_status_flow
            .first()
            .map(|v| v.current_status_identifier.clone())
            .ok_or(error::DomainError::AppNotInitialized)?;
        Ok(ProjectSet {
            id: 0,
            identifier,
            organization: param.organization,
            custom_code: param.custom_code,
            description: param.description,
            gmt_create: Utc::now(),
            gmt_modified: None,
            icon: param.icon,
            creator: creator.into(),
            modifier: None,
            name: param.name,
            status_identifier: status.clone(),
            status: None,
            members: vec![],
            roles: vec![],
            status_flow: global.project_set_status_flow.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Project {
    pub id: u64,
    pub identifier: String,
    pub organization: String,
    pub custom_code: String,
    pub description: Option<String>,
    pub gmt_create: DateTime<Utc>,
    pub gmt_modified: Option<DateTime<Utc>>,
    pub has_superior_space: bool,
    pub icon: Option<String>,
    pub creator: String,
    pub modifier: Option<String>,
    pub name: String,
    pub parent: Option<String>,
    pub template: String,
    pub status_identifier: String,

    pub status: Option<Status>,
    /// 项目标签配置
    pub tags: Vec<Tag>,
    /// 项目成员配置
    pub members: Vec<User>,
    /// 项目字段配置
    pub fields: Vec<Field>,
    /// 项目角色配置
    pub roles: Vec<Role>,
    /// 项目状态配置
    pub status_flow: Vec<FlowItem>,
    /// 项目工作项配置
    pub project_work_item_set: Vec<SpaceWorkItemSet>,
}

impl Project {
    pub fn new_project(
        param: CreateProjectParam,
        template: &Template,
        creator: impl Into<String>,
    ) -> error::Result<Self> {
        let identifier = uuid::Uuid::new_v4().to_string();
        let status = template
            .project_status_flow
            .first()
            .map(|v| v.current_status_identifier.clone())
            .ok_or(error::DomainError::AppNotInitialized)?;
        Ok(Project {
            id: 0,
            identifier,
            organization: param.organization,
            custom_code: param.custom_code,
            description: param.description,
            gmt_create: Utc::now(),
            gmt_modified: None,
            has_superior_space: param.project_set.is_some(),
            icon: param.icon,
            creator: creator.into(),
            modifier: None,
            name: param.name,
            parent: param.project_set,
            template: param.project_template,
            status_identifier: status.clone(),
            status: None,
            tags: vec![],
            members: vec![],
            fields: template.project_fields.clone(),
            roles: vec![],
            status_flow: template.project_status_flow.clone(),
            project_work_item_set: template
                .project_work_item_set
                .iter()
                .map(|v| {
                    let mut role = v.clone();
                    role.id = 0;
                    role
                })
                .collect(),
        })
    }
}

pub struct CreateProjectSetParam {
    pub name: String,
    pub custom_code: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub organization: String,
}
pub struct CreateProjectParam {
    pub name: String,
    pub custom_code: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub organization: String,
    pub project_set: Option<String>,
    pub project_template: String,
}
