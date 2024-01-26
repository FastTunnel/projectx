use crate::adapter::repository::po::{space, space_member, tag};
use app_interface::define_repo;
use async_trait::async_trait;
use domain_common::error;
use domain_workspace::model::project::{Project, ProjectSet};
use domain_workspace::model::setting::status::Status;
use domain_workspace::model::tag::Tag;
use domain_workspace::repository::ISpaceRepository;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Schema};

define_repo!(SpaceRepository);

#[async_trait]
impl ISpaceRepository for SpaceRepository {
    async fn find_project_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &String,
    ) -> error::Result<Option<Project>> {
        let space_po = space::Entity::find()
            .filter(space::Column::Identifier.eq(id))
            .one(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        let space_po = match space_po {
            None => {
                return Ok(None);
            }
            Some(v) => v,
        };
        todo!()
    }

    async fn find_project_set_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &String,
    ) -> error::Result<Option<ProjectSet>> {
        let space_po = space::Entity::find()
            .filter(space::Column::Identifier.eq(id))
            .one(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        let space_po = match space_po {
            None => {
                return Ok(None);
            }
            Some(v) => v,
        };
        todo!()
    }

    async fn find_all_project_set(
        &self,
        tx: &mut Self::Transaction,
        organization: &String,
    ) -> error::Result<Vec<ProjectSet>> {
        todo!()
    }

    async fn find_all_project(
        &self,
        tx: &mut Self::Transaction,
        organization: &String,
        parent: Option<&String>,
    ) -> error::Result<Vec<Project>> {
        todo!()
    }

    async fn find_space_member_ids(
        &self,
        tx: &mut Self::Transaction,
        space_id: &String,
    ) -> error::Result<Vec<String>> {
        let members = space_member::Entity::find()
            .filter(space_member::Column::Space.eq(space_id))
            .all(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?
            .into_iter()
            .map(|v| v.member)
            .collect();
        Ok(members)
    }

    async fn find_space_tags(
        &self,
        tx: &mut Self::Transaction,
        id: &String,
    ) -> error::Result<Vec<Tag>> {
        let tags = tag::Entity::find()
            .filter(tag::Column::Space.eq(id))
            .all(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?
            .into_iter()
            .map(|v| v.into())
            .collect();
        Ok(tags)
    }

    async fn find_status_by_ids(
        &self,
        tx: &mut Self::Transaction,
        ids: &[String],
    ) -> error::Result<Vec<Status>> {
        todo!()
    }

    async fn save_project(
        &self,
        tx: &mut Self::Transaction,
        space: &mut Project,
    ) -> error::Result<()> {
        let po: space::ActiveModel = space.clone().into();
        po.save(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        Ok(())
    }

    async fn save_project_set(
        &self,
        tx: &mut Self::Transaction,
        space: &mut ProjectSet,
    ) -> error::Result<()> {
        todo!()
    }
}
