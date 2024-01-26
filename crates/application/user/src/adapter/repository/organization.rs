use crate::adapter::repository::po::organization;
use app_interface::define_repo;
use async_trait::async_trait;
use domain_common::error;
use domain_user::model::Organization;
use domain_user::repository::{IOrganizationRepository, OrganizationQuery};
use sea_orm::prelude::*;
use sea_orm::QueryTrait;

define_repo!(OrganizationRepository);

#[async_trait]
impl IOrganizationRepository for OrganizationRepository {
    async fn find_by_identifier(
        &self,
        tx: &mut Self::Transaction,
        identifier: &String,
    ) -> error::Result<Option<Organization>> {
        match self
            .query(
                tx,
                OrganizationQuery {
                    identifier: Some(identifier.clone()),
                    ..Default::default()
                },
            )
            .await?
            .pop()
        {
            None => Ok(None),
            Some(o) => Ok(Some(o)),
        }
    }

    async fn find_by_name(
        &self,
        tx: &mut Self::Transaction,
        name: &String,
    ) -> error::Result<Option<Organization>> {
        match self
            .query(
                tx,
                OrganizationQuery {
                    name: Some(name.clone()),
                    ..Default::default()
                },
            )
            .await?
            .pop()
        {
            None => Ok(None),
            Some(o) => Ok(Some(o)),
        }
    }

    async fn query(
        &self,
        tx: &mut Self::Transaction,
        q: OrganizationQuery,
    ) -> error::Result<Vec<Organization>> {
        let organization = organization::Entity::find()
            .apply_if(q.id, |q, id| q.filter(organization::Column::Id.eq(id)))
            .apply_if(q.identifier, |q, identifier| {
                q.filter(organization::Column::Identifier.eq(identifier))
            })
            .apply_if(q.name, |q, name| {
                q.filter(organization::Column::Name.eq(name))
            })
            .all(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?
            .into_iter()
            .map(|organization_po| organization_po.into())
            .collect::<Vec<_>>();
        Ok(organization)
    }

    async fn is_initialized(&self, tx: &mut Self::Transaction) -> error::Result<bool> {
        let count = organization::Entity::find()
            .count(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        Ok(count > 0)
    }

    async fn find_first(&self, tx: &mut Self::Transaction) -> error::Result<Option<Organization>> {
        self.query(tx, OrganizationQuery::default())
            .await
            .map(|mut v| v.pop())
    }

    async fn save(
        &self,
        tx: &mut Self::Transaction,
        organization: &mut Organization,
    ) -> error::Result<u64> {
        let organization_po: organization::ActiveModel = organization.into();
        let organization_po = organization_po
            .save(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        organization.id = organization_po.id.unwrap();
        Ok(organization.id)
    }
}
