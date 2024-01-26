use app_interface::utils::ToDateTime;
use app_interface::APP_STATE;
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

pub async fn init_table() {
    let tx = APP_STATE.db_tx();
    tx.execute_unprepared(
        r#"
         CREATE TABLE IF NOT EXISTS `team` (
              `id` bigint unsigned NOT NULL AUTO_INCREMENT,
              `identifier` varchar(36) COLLATE utf8mb4_unicode_ci NOT NULL,
              `name` varchar(100) COLLATE utf8mb4_unicode_ci NOT NULL,
              `description` varchar(200) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
              `organization` varchar(36) COLLATE utf8mb4_unicode_ci NOT NULL,
              `public` tinyint(1) NOT NULL DEFAULT '0',
              `parent` varchar(36) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
              `gmt_create` int NOT NULL,
              `creator` varchar(36) COLLATE utf8mb4_unicode_ci NOT NULL,
              `gmt_modified` int DEFAULT NULL,
              `modifier` varchar(36) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
              `icon` varchar(200) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
              `leader` varchar(36) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
              PRIMARY KEY (`id`),
              UNIQUE KEY `identifier` (`identifier`)
         ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='团队表'

"#,
    )
    .await
    .unwrap();
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
