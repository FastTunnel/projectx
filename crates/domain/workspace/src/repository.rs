use crate::model::project::{Project, ProjectSet};
use crate::model::setting::space_work_item_set::SpaceWorkItemSet;
use crate::model::setting::status::Status;
use crate::model::tag::Tag;
use async_trait::async_trait;
use domain_common::{error, Repository};

#[async_trait]
pub trait ISpaceRepository: Repository {
    async fn find_project_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &String,
    ) -> error::Result<Option<Project>>;
    async fn find_project_set_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &String,
    ) -> error::Result<Option<ProjectSet>>;
    async fn find_all_project_set(
        &self,
        tx: &mut Self::Transaction,
        organization: &String,
    ) -> error::Result<Vec<ProjectSet>>;

    async fn find_space_work_item_sets(
        &self,
        tx: &mut Self::Transaction,
        space_id: &String,
        category: &String,
    ) -> error::Result<Vec<SpaceWorkItemSet>>;

    async fn find_all_project(
        &self,
        tx: &mut Self::Transaction,
        organization: &String,
        parent: Option<&String>,
    ) -> error::Result<Vec<Project>>;

    async fn find_space_member_ids(
        &self,
        tx: &mut Self::Transaction,
        id: &String,
    ) -> error::Result<Vec<String>>;

    async fn find_space_tags(
        &self,
        tx: &mut Self::Transaction,
        id: &String,
    ) -> error::Result<Vec<Tag>>;

    async fn find_status_by_ids(
        &self,
        tx: &mut Self::Transaction,
        ids: &[String],
    ) -> error::Result<Vec<Status>>;

    async fn save_project(
        &self,
        tx: &mut Self::Transaction,
        space: &mut Project,
    ) -> error::Result<()>;

    async fn save_project_set(
        &self,
        tx: &mut Self::Transaction,
        space: &mut ProjectSet,
    ) -> error::Result<()>;
}
