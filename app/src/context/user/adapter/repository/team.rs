use async_trait::async_trait;
use chrono::{TimeZone, Utc};
use sea_orm::prelude::*;
use sea_orm::{JoinType, NotSet, QuerySelect, QueryTrait, Set};

use domain::error;
use domain::user::model::{Organization, Team};
use domain::user::repository::{ITeamRepository, TeamQuery};

use crate::context::user::adapter::repository::po::{organization, team, team_member};
use crate::define_repo;

define_repo!(TeamRepository);

#[async_trait]
impl ITeamRepository for TeamRepository {
    async fn find_by_identifier(
        &self,
        tx: &mut Self::Transaction,
        id: &String,
    ) -> error::Result<Option<Team>> {
        match self
            .query(
                tx,
                TeamQuery {
                    identifier: Some(id.clone()),
                    ..Default::default()
                },
            )
            .await?
            .pop()
        {
            None => Ok(None),
            Some(t) => Ok(Some(t)),
        }
    }

    async fn find_by_name(
        &self,
        tx: &mut Self::Transaction,
        name: &String,
    ) -> error::Result<Option<Team>> {
        match self
            .query(
                tx,
                TeamQuery {
                    name: Some(name.clone()),
                    ..Default::default()
                },
            )
            .await?
            .pop()
        {
            None => Ok(None),
            Some(t) => Ok(Some(t)),
        }
    }

    async fn query(&self, tx: &mut Self::Transaction, q: TeamQuery) -> error::Result<Vec<Team>> {
        let team = team::Entity::find()
            .join_rev(
                JoinType::InnerJoin,
                organization::Entity::belongs_to(team::Entity)
                    .from(organization::Column::Identifier)
                    .to(team::Column::Organization)
                    .into(),
            )
            .apply_if(q.id, |q, id| q.filter(team::Column::Id.eq(id)))
            .apply_if(q.identifier, |q, identifier| {
                q.filter(team::Column::Identifier.eq(identifier))
            })
            .apply_if(q.name, |q, name| q.filter(team::Column::Name.eq(name)))
            .apply_if(q.organization_id, |q, organization_id| {
                q.filter(team::Column::Organization.eq(organization_id))
            })
            .select_also(organization::Entity)
            .all(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?
            .into_iter()
            .filter(|(_, organization)| organization.is_some())
            .map(|(team_po, organization)| Team {
                id: team_po.id,
                identifier: team_po.identifier.clone(),
                name: team_po.name.clone(),
                description: team_po.description.clone(),
                public: team_po.public,
                icon: team_po.icon.clone(),
                parent: team_po.parent.clone(),
                leader: team_po.leader.clone(),
                gmt_create: Utc.timestamp_opt(team_po.gmt_create, 0).unwrap(),
                creator: team_po.creator.clone(),
                gmt_modified: team_po
                    .gmt_modified
                    .map(|v| Utc.timestamp_opt(v, 0).unwrap()),
                modifier: team_po.modifier.clone(),
                organization: Some(Organization {
                    id: organization.as_ref().unwrap().id,
                    identifier: organization.as_ref().unwrap().identifier.clone(),
                    name: organization.as_ref().unwrap().name.clone(),
                    description: organization.as_ref().unwrap().description.clone(),
                    pinyin: organization.as_ref().unwrap().pinyin.clone(),
                    public: organization.as_ref().unwrap().public,
                    gmt_create: Utc
                        .timestamp_opt(organization.as_ref().unwrap().gmt_create, 0)
                        .unwrap(),

                    gmt_modified: organization
                        .as_ref()
                        .unwrap()
                        .gmt_modified
                        .map(|v| Utc.timestamp_opt(v, 0).unwrap()),

                    icon: organization.as_ref().unwrap().icon.clone(),
                }),
                members: vec![],
            })
            .collect::<Vec<_>>();
        Ok(team)
    }

    async fn save(&self, tx: &mut Self::Transaction, team: &mut Team) -> error::Result<u64> {
        let team_model = team::ActiveModel {
            id: if team.id == 0 { NotSet } else { Set(team.id) },
            identifier: Set(team.identifier.clone()),
            name: Set(team.name.clone()),
            description: Set(team.description.clone()),
            organization: Set(team
                .organization
                .as_ref()
                .ok_or_else(|| {
                    error::DomainError::IllegalArgument("organization is required".to_string())
                })?
                .identifier
                .clone()),
            public: Set(team.public),
            icon: Set(team.icon.clone()),
            parent: Set(team.parent.clone()),
            leader: Set(team.leader.clone()),
            gmt_create: Set(team.gmt_create.timestamp()),
            creator: Set(team.creator.clone()),
            gmt_modified: Set(team.gmt_modified.map(|v| v.timestamp())),
            modifier: Set(team.modifier.clone()),
        }
        .save(tx)
        .await
        .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        team.id = team_model.id.unwrap();

        team_member::Entity::delete_many()
            .filter(team_member::Column::Team.eq(team.id))
            .exec(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;

        if team.members.is_empty() {
            return Ok(team.id);
        }

        let team_members = team
            .members
            .iter()
            .map(|m| team_member::ActiveModel {
                id: NotSet,
                team: Set(team.identifier.clone()),
                user: Set(m.identifier.clone()),
            })
            .collect::<Vec<_>>();

        team_member::Entity::insert_many(team_members)
            .exec(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;

        Ok(team.id)
    }
}
