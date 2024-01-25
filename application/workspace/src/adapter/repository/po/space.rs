use app_interface::utils::ToDateTime;
use domain_common::error;
use domain_workspace::enums::{OwnType, ResourceType};
use domain_workspace::model::value::Space;
use sea_orm::entity::prelude::*;
use sea_orm::{NotSet, Set};

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
    /// see [domain::workspace::enums::OwnType] for more details
    pub own_type: String,
    pub resource_type: Option<String>,
}

impl From<Model> for error::Result<Space> {
    fn from(value: Model) -> Self {
        let space_po = value;

        let resource_type = space_po
            .resource_type
            .and_then(|v| ResourceType::from_string(&v))
            .ok_or(error::DomainError::InnerError(
                "未查到当前resource_type".into(),
            ))?;
        match resource_type {
            ResourceType::ProjectSet => {
                let gmt_create = space_po.gmt_create.map(|v| v.to_date_time()).ok_or(
                    error::DomainError::InnerError("project_set's gmt_create is null".into()),
                )?;
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
                let space = Space::ProjectSet {
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
                    status,
                    project_set_members: vec![],
                    project_set_roles: vec![],

                    project_set_status_flow: vec![],
                };
                Ok(space)
            }
            ResourceType::Project => {
                let gmt_create = space_po.gmt_create.map(|v| v.to_date_time()).ok_or(
                    error::DomainError::InnerError("project's gmt_create is null".into()),
                )?;
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
                let space = Space::Project {
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
                    status,
                    project_tags: vec![],
                    project_members: vec![],
                    project_fields: vec![],
                    project_roles: vec![],

                    project_status_flow: vec![],
                    project_work_item_set: vec![],
                    project_work_item: vec![],
                };
                Ok(space)
            }
            _ => {
                return Err(error::DomainError::InnerError(
                    "未查到当前resource_type".into(),
                ))
            }
        }
    }
}

impl Into<ActiveModel> for Space {
    fn into(self) -> ActiveModel {
        match self {
            Space::ProjectSet {
                id,
                identifier,
                organization,
                custom_code,
                description,
                gmt_create,
                gmt_modified,
                icon,
                creator,
                modifier,
                name,
                status,
                ..
            } => {
                let space = ActiveModel {
                    id: if id == 0 { NotSet } else { Set(id) },
                    identifier: Set(identifier),
                    organization: Set(organization),
                    custom_code: Set(Some(custom_code)),
                    description: Set(description),
                    gmt_create: Set(Some(gmt_create.timestamp())),
                    gmt_modified: Set(gmt_modified.map(|v| v.timestamp())),
                    has_superior_space: Set(None),
                    icon: Set(icon),
                    creator: Set(Some(creator)),
                    modifier: Set(modifier),
                    name: Set(Some(name)),
                    parent: Set(None),
                    template: Set(None),
                    status: Set(Some(status)),
                    own_type: Set(OwnType::Project.to_string()),
                    resource_type: Set(Some(ResourceType::ProjectSet.to_string())),
                };
                space
            }
            Space::Project {
                id,
                identifier,
                organization,
                custom_code,
                description,
                gmt_create,
                gmt_modified,
                has_superior_space,
                icon,
                creator,
                modifier,
                name,
                parent,
                template,
                status,
                ..
            } => {
                let space = ActiveModel {
                    id: if id == 0 { NotSet } else { Set(id) },
                    identifier: Set(identifier),
                    organization: Set(organization),
                    custom_code: Set(Some(custom_code)),
                    description: Set(description),
                    gmt_create: Set(Some(gmt_create.timestamp())),
                    gmt_modified: Set(gmt_modified.map(|v| v.timestamp())),
                    has_superior_space: Set(Some(has_superior_space)),
                    icon: Set(icon),
                    creator: Set(Some(creator)),
                    modifier: Set(modifier),
                    name: Set(Some(name)),
                    parent: Set(parent),
                    template: Set(Some(template)),
                    status: Set(Some(status)),
                    own_type: Set(OwnType::Project.to_string()),
                    resource_type: Set(Some(ResourceType::ProjectSet.to_string())),
                };
                space
            }
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
