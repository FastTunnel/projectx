use sea_orm::entity::prelude::*;
use sea_orm::{NotSet, Set};

use app_interface::utils::ToDateTime;
use domain_common::error;
use domain_workspace::enums::ResourceType;
use domain_workspace::model::project::{Project, ProjectSet};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "space")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub identifier: String,
    pub organization: String,
    pub custom_code: Option<String>,
    pub description: Option<String>,
    pub gmt_create: Option<i64>,
    pub gmt_modified: Option<i64>,
    pub has_superior_space: Option<bool>,
    pub icon: Option<String>,
    pub creator: Option<String>,
    pub modifier: Option<String>,
    pub name: Option<String>,
    pub parent: Option<String>,
    pub template: Option<String>,
    pub status: Option<String>,
    pub resource_type: Option<String>,
    pub status_flow: String,
}

impl From<Model> for error::Result<Project> {
    fn from(value: Model) -> Self {
        let space_po = value;
        let resource_type = space_po
            .resource_type
            .and_then(|v| ResourceType::from_string(&v))
            .ok_or(error::DomainError::InnerError(
                "未查到当前resource_type".into(),
            ))?;
        if let ResourceType::Project = resource_type {
            Err(error::DomainError::InnerError(
                "resource_type is not project".into(),
            ))?;
        }
        let gmt_create =
            space_po
                .gmt_create
                .map(|v| v.to_date_time())
                .ok_or(error::DomainError::InnerError(
                    "project's gmt_create is null".into(),
                ))?;
        let custom_code = space_po.custom_code.ok_or(error::DomainError::InnerError(
            "project's custom_code is null".into(),
        ))?;
        let creator = space_po.creator.ok_or(error::DomainError::InnerError(
            "project's creator is null".into(),
        ))?;
        let name = space_po.name.ok_or(error::DomainError::InnerError(
            "project's name is null".into(),
        ))?;
        let has_superior_space =
            space_po
                .has_superior_space
                .ok_or(error::DomainError::InnerError(
                    "project's has_superior_space is null".into(),
                ))?;
        let template = space_po.template.ok_or(error::DomainError::InnerError(
            "project's template is null".into(),
        ))?;
        let status = space_po.status.ok_or(error::DomainError::InnerError(
            "project's status is null".into(),
        ))?;
        let space = Project {
            id: space_po.id,
            identifier: space_po.identifier,
            organization: space_po.organization,
            custom_code,
            description: space_po.description,
            gmt_create,
            gmt_modified: space_po.gmt_modified.map(|v| v.to_date_time()),
            has_superior_space,
            icon: space_po.icon,
            creator,
            modifier: space_po.modifier,
            name,
            parent: space_po.parent,
            template,
            status_identifier: status.clone(),
            status: None,
            tags: vec![],
            members: vec![],
            fields: vec![],
            roles: vec![],
            status_flow: vec![],
            project_work_item_set: vec![],
        };
        Ok(space)
    }
}
impl From<Model> for error::Result<ProjectSet> {
    fn from(value: Model) -> Self {
        let space_po = value;
        let resource_type = space_po
            .resource_type
            .and_then(|v| ResourceType::from_string(&v))
            .ok_or(error::DomainError::InnerError(
                "未查到当前resource_type".into(),
            ))?;
        if let ResourceType::ProjectSet = resource_type {
            Err(error::DomainError::InnerError(
                "resource_type is not project_set".into(),
            ))?;
        }
        let gmt_create =
            space_po
                .gmt_create
                .map(|v| v.to_date_time())
                .ok_or(error::DomainError::InnerError(
                    "project_set's gmt_create is null".into(),
                ))?;
        let custom_code = space_po.custom_code.ok_or(error::DomainError::InnerError(
            "project_set's custom_code is null".into(),
        ))?;
        let creator = space_po.creator.ok_or(error::DomainError::InnerError(
            "project_set's creator is null".into(),
        ))?;
        let name = space_po.name.ok_or(error::DomainError::InnerError(
            "project_set's name is null".into(),
        ))?;
        let status = space_po.status.ok_or(error::DomainError::InnerError(
            "project_set's status is null".into(),
        ))?;
        let space = ProjectSet {
            id: space_po.id,
            identifier: space_po.identifier,
            organization: space_po.organization,
            custom_code,
            description: space_po.description,
            gmt_create,
            gmt_modified: space_po.gmt_modified.map(|v| v.to_date_time()),
            icon: space_po.icon,
            creator,
            modifier: space_po.modifier,
            name,
            status_identifier: status.clone(),
            status: None,
            members: vec![],
            roles: vec![],
            status_flow: vec![],
        };
        Ok(space)
    }
}

impl Into<ActiveModel> for ProjectSet {
    fn into(self) -> ActiveModel {
        ActiveModel {
            id: if self.id == 0 { NotSet } else { Set(self.id) },
            identifier: Set(self.identifier),
            organization: Set(self.organization),
            custom_code: Set(Some(self.custom_code)),
            description: Set(self.description),
            gmt_create: Set(Some(self.gmt_create.timestamp())),
            gmt_modified: Set(self.gmt_modified.map(|v| v.timestamp())),
            has_superior_space: Set(None),
            icon: Set(self.icon),
            creator: Set(Some(self.creator)),
            modifier: Set(self.modifier),
            name: Set(Some(self.name)),
            parent: Set(None),
            template: Set(None),
            status: Set(Some(self.status_identifier)),
            resource_type: Set(Some(ResourceType::ProjectSet.to_string())),
            status_flow: Set(serde_json::to_string(&self.status_flow).unwrap()),
        }
    }
}

impl Into<ActiveModel> for Project {
    fn into(self) -> ActiveModel {
        ActiveModel {
            id: if self.id == 0 { NotSet } else { Set(self.id) },
            identifier: Set(self.identifier),
            organization: Set(self.organization),
            custom_code: Set(Some(self.custom_code)),
            description: Set(self.description),
            gmt_create: Set(Some(self.gmt_create.timestamp())),
            gmt_modified: Set(self.gmt_modified.map(|v| v.timestamp())),
            has_superior_space: Set(Some(self.has_superior_space)),
            icon: Set(self.icon),
            creator: Set(Some(self.creator)),
            modifier: Set(self.modifier),
            name: Set(Some(self.name)),
            parent: Set(self.parent),
            template: Set(Some(self.template)),
            status: Set(Some(self.status_identifier)),
            resource_type: Set(Some(ResourceType::Project.to_string())),
            status_flow: Set(serde_json::to_string(&self.status_flow).unwrap()),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
