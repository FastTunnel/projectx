use crate::model::role::Role;
use crate::model::setting::base::FlowItem;
use crate::model::setting::field::Field;
use chrono::{DateTime, Utc};
use domain_common::error;
use serde_json::Value;

use crate::model::setting::global::GlobalConfig;
use crate::model::setting::space_work_item_set::SpaceWorkItemSet;
use crate::model::setting::template::Template;
use crate::model::user::User;

#[derive(Debug, Clone)]
pub enum Space {
    ProjectSet {
        id: u64,
        identifier: String,
        organization: String,
        custom_code: String,
        description: Option<String>,
        gmt_create: DateTime<Utc>,
        gmt_modified: Option<DateTime<Utc>>,
        icon: Option<String>,
        creator: String,
        modifier: Option<String>,
        name: String,
        status: String,
        project_set_members: Vec<User>,
        project_set_roles: Vec<Role>,
        project_set_status_flow: Vec<FlowItem>,
    },
    Project {
        id: u64,
        identifier: String,
        organization: String,
        custom_code: String,
        description: Option<String>,
        gmt_create: DateTime<Utc>,
        gmt_modified: Option<DateTime<Utc>>,
        has_superior_space: bool,
        icon: Option<String>,
        creator: String,
        modifier: Option<String>,
        name: String,
        parent: Option<String>,
        template: String,
        status: String,
        /// 项目标签配置
        project_tags: Vec<Tag>,
        /// 项目成员配置
        project_members: Vec<User>,
        /// 项目字段配置
        project_fields: Vec<Field>,
        /// 项目角色配置
        project_roles: Vec<Role>,
        /// 项目状态配置
        project_status_flow: Vec<FlowItem>,
        /// 项目工作项配置
        project_work_item_set: Vec<SpaceWorkItemSet>,
        /// 工作项 (仅项目有此字段)
        project_work_item: Vec<SpaceWorkItem>,
    },
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

pub struct CreateTemplateParam {
    pub name: String,
    pub custom_code: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub organization: String,
}

impl Space {
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
        Ok(Space::ProjectSet {
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
            status,
            project_set_members: vec![],
            project_set_roles: vec![],
            project_set_status_flow: global.project_set_status_flow.clone(),
        })
    }

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
        Ok(Space::Project {
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
            status,
            project_tags: vec![],
            project_members: vec![],
            project_fields: template.project_fields.clone(),
            project_roles: vec![],
            project_status_flow: template.project_status_flow.clone(),
            project_work_item_set: template
                .project_work_item_set
                .iter()
                .map(|v| {
                    let mut role = v.clone();
                    role.id = 0;
                    role
                })
                .collect(),
            project_work_item: vec![],
        })
    }
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

#[derive(Debug, Clone, Default)]
pub struct Tag {
    pub id: u64,
    pub identifier: String,
    pub name: String,
    pub gmt_create: i64,
    pub creator: String,
    pub color: String,
    pub space: String,
}
