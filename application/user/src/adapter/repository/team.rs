use crate::adapter::repository::po::{organization, team, team_member};
use app_interface::define_repo;
use async_trait::async_trait;
use domain_common::error;
use domain_user::model::Team;
use domain_user::repository::{ITeamRepository, TeamQuery};
use sea_orm::prelude::*;
use sea_orm::{JoinType, NotSet, QuerySelect, QueryTrait, Set};

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
            .map(|(team_po, organization)| {
                let mut team: Team = team_po.into();
                team.organization = Some(organization.unwrap().into());
                team
            })
            .collect::<Vec<_>>();
        Ok(team)
    }

    async fn save(&self, tx: &mut Self::Transaction, team: &mut Team) -> error::Result<u64> {
        if team.organization.is_none() {
            return Err(error::DomainError::IllegalArgument(
                "organization is required".to_string(),
            ));
        }
        let team_po: team::ActiveModel = team.into();
        let team_po = team_po
            .save(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        team.id = team_po.id.unwrap();

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
