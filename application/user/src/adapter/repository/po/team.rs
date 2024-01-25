use app_interface::utils::ToDateTime;
use domain_user::model::Team;
use sea_orm::entity::prelude::*;
use sea_orm::{NotSet, Set};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "team")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub identifier: String,
    pub name: String,
    pub description: Option<String>,
    pub organization: String,
    pub public: bool,
    pub parent: Option<String>,
    pub gmt_create: i64,
    pub creator: String,
    pub gmt_modified: Option<i64>,
    pub modifier: Option<String>,
    pub icon: Option<String>,
    pub leader: Option<String>,
}

impl From<Model> for Team {
    fn from(value: Model) -> Self {
        Team {
            id: value.id,
            identifier: value.identifier,
            name: value.name,
            description: value.description,
            organization: None,
            public: value.public,
            parent: value.parent,
            gmt_create: value.gmt_create.to_date_time(),
            creator: value.creator,
            gmt_modified: value.gmt_modified.map(|v| v.to_date_time()),
            modifier: value.modifier,
            icon: value.icon,
            leader: value.leader,
            members: vec![],
        }
    }
}

impl Into<ActiveModel> for &mut Team {
    fn into(self) -> ActiveModel {
        let team = self;
        ActiveModel {
            id: if team.id == 0 { NotSet } else { Set(team.id) },
            identifier: Set(team.identifier.clone()),
            name: Set(team.name.clone()),
            description: Set(team.description.clone()),
            organization: Set(team.organization.as_ref().unwrap().identifier.clone()),
            public: Set(team.public),
            parent: Set(team.parent.clone()),
            gmt_create: Set(team.gmt_create.timestamp()),
            creator: Set(team.creator.clone()),
            gmt_modified: Set(team.gmt_modified.map(|v| v.timestamp())),
            modifier: Set(team.modifier.clone()),
            icon: Set(team.icon.clone()),
            leader: Set(team.leader.clone()),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
