use app_interface::utils::ToDateTime;
use domain_common::error;
use domain_workspace::enums::Category;
use domain_workspace::model::setting::base::FlowItem;
use domain_workspace::model::setting::space_work_item_set::SpaceWorkItemSet;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "space_work_item_set")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub identifier: String,
    pub category: String,
    pub creator: String,
    pub gmt_create: i64,
    pub gmt_modified: Option<i64>,
    pub modifier: Option<String>,
    pub name: String,
    pub name_en: String,
    pub display_name: String,
    pub description: String,
    pub space: String,
    pub is_deleted: bool,
    pub is_system: bool,
    pub status_flow: String,
    pub organization: String,
}

impl From<Model> for error::Result<SpaceWorkItemSet> {
    fn from(value: Model) -> Self {
        let flow_items: Vec<FlowItem> = serde_json::from_str(&value.status_flow).map_err(|e| {
            error::DomainError::InnerError(format!("status_flow json error: {}", e))
        })?;
        let category = Category::from_string(&value.category).ok_or(
            error::DomainError::InnerError("category is not valid".into()),
        )?;
        Ok(SpaceWorkItemSet {
            id: value.id,
            identifier: value.identifier,
            category,
            creator: value.creator,
            gmt_create: value.gmt_create.to_date_time(),
            gmt_modified: value.gmt_modified.map(|v| v.to_date_time()),
            modifier: value.modifier,
            name: value.name,
            name_en: value.name_en,
            display_name: value.display_name,
            description: value.description,
            space: value.space,
            is_deleted: value.is_deleted,
            is_system: value.is_system,
            work_item_fields: vec![],
            work_item_status_flow: flow_items,
            organization: value.organization,
        })
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
