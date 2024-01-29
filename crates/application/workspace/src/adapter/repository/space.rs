use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryTrait};

use crate::adapter::repository::po;
use app_interface::define_repo;
use domain_common::error;
use domain_workspace::enums::ResourceType;
use domain_workspace::model::project::{Project, ProjectSet};
use domain_workspace::model::setting::space_work_item_set::SpaceWorkItemSet;
use domain_workspace::model::setting::status::Status;
use domain_workspace::model::tag::Tag;
use domain_workspace::repository::ISpaceRepository;

use crate::adapter::repository::po::{space, space_member, tag};

define_repo!(SpaceRepository);

#[async_trait]
impl ISpaceRepository for SpaceRepository {
    async fn find_project_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &String,
    ) -> error::Result<Option<Project>> {
        let project = space::Entity::find()
            .filter(
                space::Column::Identifier
                    .eq(id)
                    .and(space::Column::ResourceType.eq(ResourceType::Project.to_string())),
            )
            .one(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;

        let project = match project {
            None => {
                return Ok(None);
            }
            Some(v) => v,
        };
        let project: error::Result<Project> = project.into();
        Ok(Some(project?))
    }

    async fn find_project_set_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &String,
    ) -> error::Result<Option<ProjectSet>> {
        let project_set = space::Entity::find()
            .filter(
                space::Column::Identifier
                    .eq(id)
                    .and(space::Column::ResourceType.eq(ResourceType::ProjectSet.to_string())),
            )
            .one(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        let project_set = match project_set {
            None => {
                return Ok(None);
            }
            Some(v) => v,
        };
        let project_set: error::Result<ProjectSet> = project_set.into();
        Ok(Some(project_set?))
    }

    async fn find_all_project_set(
        &self,
        tx: &mut Self::Transaction,
        organization: &String,
    ) -> error::Result<Vec<ProjectSet>> {
        let project_sets = space::Entity::find()
            .filter(
                space::Column::Organization
                    .eq(organization)
                    .and(space::Column::ResourceType.eq(ResourceType::ProjectSet.to_string())),
            )
            .all(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        let project_sets: error::Result<Vec<ProjectSet>> = project_sets
            .into_iter()
            .map(|v| v.into())
            .collect::<Result<Vec<ProjectSet>, error::DomainError>>();
        Ok(project_sets?)
    }

    async fn find_space_work_item_sets(
        &self,
        tx: &mut Self::Transaction,
        space_id: &String,
        category: &String,
    ) -> error::Result<Vec<SpaceWorkItemSet>> {
        let work_item_sets = po::work_item_set::Entity::find()
            .filter(
                po::work_item_set::Column::Space
                    .eq(space_id)
                    .and(po::work_item_set::Column::Category.eq(category)),
            )
            .all(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        let work_item_sets: error::Result<Vec<SpaceWorkItemSet>> = work_item_sets
            .into_iter()
            .map(|v| v.into())
            .collect::<Result<Vec<SpaceWorkItemSet>, error::DomainError>>();
        Ok(work_item_sets?)
    }

    async fn find_all_project(
        &self,
        tx: &mut Self::Transaction,
        organization: &String,
        parent: Option<&String>,
    ) -> error::Result<Vec<Project>> {
        let projects = space::Entity::find()
            .filter(
                space::Column::Organization
                    .eq(organization)
                    .and(space::Column::ResourceType.eq(ResourceType::Project.to_string())),
            )
            .apply_if(parent, |q, v| q.filter(space::Column::Parent.eq(v)))
            .all(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        let projects: error::Result<Vec<Project>> = projects
            .into_iter()
            .map(|v| v.into())
            .collect::<Result<Vec<Project>, error::DomainError>>();
        Ok(projects?)
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
        let status = po::status::Entity::find()
            .filter(
                po::status::Column::Identifier
                    .is_in(ids)
                    .and(po::status::Column::ResourceType.eq(ResourceType::Project.to_string())),
            )
            .all(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        let status: error::Result<Vec<Status>> = status.into_iter().map(|v| v.into()).collect();
        Ok(status?)
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
        let po: space::ActiveModel = space.clone().into();
        po.save(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        Ok(())
    }
}
