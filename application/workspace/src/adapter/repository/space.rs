use crate::adapter::repository::po::space;
use app_interface::define_repo;
use async_trait::async_trait;
use domain_common::error;
use domain_workspace::model::value::Space;
use domain_workspace::repository::ISpaceRepository;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

define_repo!(SpaceRepository);

#[async_trait]
impl ISpaceRepository for SpaceRepository {
    async fn find_space_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &String,
    ) -> error::Result<Option<Space>> {
        let space_po = super::po::space::Entity::find()
            .filter(super::po::space::Column::Identifier.eq(id))
            .one(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        let space_po = match space_po {
            None => {
                return Ok(None);
            }
            Some(v) => v,
        };
        let space: error::Result<Space> = space_po.into();
        Ok(Some(space?))
    }

    async fn save(&self, tx: &mut Self::Transaction, space: &mut Space) -> error::Result<()> {
        match space {
            Space::ProjectSet { .. } => {}
            Space::Project { .. } => {}
        }
        let po: space::ActiveModel = space.clone().into();
        po.save(tx)
            .await
            .map_err(|e| error::DomainError::DatabaseError(e.into()))?;
        Ok(())
    }
}
