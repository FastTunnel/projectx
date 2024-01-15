use async_trait::async_trait;
use chrono::{TimeZone, Utc};
use sea_orm::prelude::*;
use sea_orm::{NotSet, QueryTrait, Set};

use domain::error;
use domain::user::model::Organization;
use domain::user::repository::{IOrganizationRepository, OrganizationQuery};

use crate::context::user::adapter::repository::po::organization;
use crate::define_repo;

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
            .map(|organization_po| Organization {
                id: organization_po.id,
                identifier: organization_po.identifier.clone(),
                name: organization_po.name.clone(),
                description: organization_po.description.clone(),
                pinyin: organization_po.pinyin.clone(),
                public: organization_po.public,
                gmt_create: Utc.timestamp_opt(organization_po.gmt_create, 0).unwrap(),
                gmt_modified: organization_po
                    .gmt_modified
                    .map(|v| Utc.timestamp_opt(v, 0).unwrap()),
                icon: None,
            })
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
        let organization_model = organization::ActiveModel {
            id: if organization.id == 0 {
                NotSet
            } else {
                Set(organization.id)
            },
            identifier: Set(organization.identifier.clone()),
            name: Set(organization.name.clone()),
            description: Set(organization.description.clone()),
            pinyin: Set(organization.pinyin.clone()),
            public: Set(organization.public),
            gmt_create: Set(organization.gmt_create.timestamp()),
            gmt_modified: Set(organization.gmt_modified.map(|v| v.timestamp())),
            icon: Set(organization.icon.clone()),
        }
        .save(tx)
        .await
        .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        organization.id = organization_model.id.unwrap();
        Ok(organization.id)
    }
}
