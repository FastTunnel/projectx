use crate::model::value::Space;
use async_trait::async_trait;
use domain_common::{error, Repository};

#[async_trait]
pub trait ISpaceRepository: Repository {
    async fn find_space_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &String,
    ) -> error::Result<Option<Space>>;
    async fn save(&self, tx: &mut Self::Transaction, space: &mut Space) -> error::Result<()>;
}
