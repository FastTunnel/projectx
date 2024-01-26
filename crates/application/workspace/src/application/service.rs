use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use app_interface::workspace::dto::command::{
    ProjectCreateCommand, ProjectSetCreateCommand, TemplateCreateCommand,
};
use app_interface::workspace::dto::{ProjectDTO, ProjectSetDTO, TagDTO, TemplateDTO, UserDTO};
use app_interface::workspace::IWorkspaceAppService;
use app_interface::{error, DbTx, APP_STATE};
use domain_workspace::facade::{IGlobalConfigFacade, IUserFacade};
use domain_workspace::model::project::{CreateProjectParam, CreateProjectSetParam};
use domain_workspace::model::setting::status::Status;
use domain_workspace::model::value::CreateTemplateParam;
use domain_workspace::repository::ISpaceRepository;
use domain_workspace::service::IWorkspaceService;
use sea_orm::TransactionTrait;

pub struct WorkspaceAppService {
    workspace_service: Arc<dyn IWorkspaceService<DbTx>>,
    global_config_facade: Arc<dyn IGlobalConfigFacade>,
    user_facade: Arc<dyn IUserFacade>,
    workspace_repo: Arc<dyn ISpaceRepository<Transaction = DbTx>>,
}

impl WorkspaceAppService {
    pub fn new(
        workspace_service: Arc<dyn IWorkspaceService<DbTx>>,
        global_config_facade: Arc<dyn IGlobalConfigFacade>,
        user_facade: Arc<dyn IUserFacade>,
        workspace_repo: Arc<dyn ISpaceRepository<Transaction = DbTx>>,
    ) -> Self {
        Self {
            workspace_service,
            global_config_facade,
            user_facade,
            workspace_repo,
        }
    }
}

#[async_trait]
impl IWorkspaceAppService for WorkspaceAppService {
    async fn init_system(&self, org_id: &str) -> error::Result<()> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        self.workspace_service
            .init_global_config(&mut transaction, org_id)
            .await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn find_all_template(&self, organization: &String) -> error::Result<Vec<TemplateDTO>> {
        let templates = self
            .global_config_facade
            .find_all_template(organization)
            .await?;
        let result = templates
            .into_iter()
            .map(|template| template.into())
            .collect();
        Ok(result)
    }

    async fn query_all_project_set(
        &self,
        organization: &String,
    ) -> error::Result<Vec<ProjectSetDTO>> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        let mut project_sets = self
            .workspace_repo
            .find_all_project_set(&mut transaction, organization)
            .await?;
        let status_ids = project_sets
            .iter()
            .map(|v| v.status_identifier.clone())
            .collect::<Vec<String>>();
        let status = self
            .workspace_repo
            .find_status_by_ids(&mut transaction, &status_ids)
            .await?;
        let status_map = status
            .into_iter()
            .map(|x| (x.identifier.clone(), x))
            .collect::<HashMap<String, Status>>();
        project_sets.iter_mut().for_each(|v| {
            v.status = status_map.get(&v.status_identifier).cloned();
        });
        transaction.commit().await?;
        let result = project_sets
            .into_iter()
            .map(|project_set| project_set.into())
            .collect();
        Ok(result)
    }

    async fn query_all_project(
        &self,
        organization: &String,
        project_set: Option<&String>,
    ) -> error::Result<Vec<ProjectDTO>> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        let mut projects = self
            .workspace_repo
            .find_all_project(&mut transaction, organization, project_set)
            .await?;
        transaction.commit().await?;
        let result = projects.into_iter().map(|project| project.into()).collect();
        Ok(result)
    }

    async fn template_detail(
        &self,
        organization: &String,
        template_id: &String,
    ) -> error::Result<Option<TemplateDTO>> {
        let template = self
            .global_config_facade
            .find_template_by_identifier(organization, template_id)
            .await?;
        match template {
            None => {
                return Ok(None);
            }
            Some(t) => {
                let role = self
                    .user_facade
                    .query_roles_by_own(&organization, &t.identifier)
                    .await?;
                let mut template: TemplateDTO = t.into();
                template.roles = role.into_iter().map(|r| r.into()).collect();
                Ok(Some(template))
            }
        }
    }

    async fn create_template(
        &self,
        cmd: &TemplateCreateCommand,
        creator: &str,
    ) -> error::Result<String> {
        let param = CreateTemplateParam {
            name: cmd.name.clone(),
            custom_code: cmd.custom_code.clone(),
            description: cmd.description.clone(),
            icon: cmd.icon.clone(),
            organization: cmd.organization.clone(),
        };
        let mut transaction = APP_STATE.db_tx().begin().await?;
        let template_id = self
            .workspace_service
            .create_template(&mut transaction, param, creator)
            .await?;
        transaction.commit().await?;
        Ok(template_id)
    }

    async fn create_project(
        &self,
        space: &ProjectCreateCommand,
        creator: &str,
    ) -> error::Result<String> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        let param = CreateProjectParam {
            name: space.name.clone(),
            custom_code: space.custom_code.clone(),
            description: space.description.clone(),
            icon: space.icon.clone(),
            organization: space.organization.clone(),
            project_set: space.project_set.clone(),
            project_template: space.template.clone(),
        };
        let project = self
            .workspace_service
            .create_project(&mut transaction, param, creator)
            .await?;
        transaction.commit().await?;
        Ok(project.identifier)
    }

    async fn create_project_set(
        &self,
        space: &ProjectSetCreateCommand,
        creator: &str,
    ) -> error::Result<String> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        let param = CreateProjectSetParam {
            name: space.name.clone(),
            custom_code: space.custom_code.clone(),
            description: space.description.clone(),
            icon: space.icon.clone(),
            organization: space.organization.clone(),
        };
        let project_set = self
            .workspace_service
            .create_project_set(&mut transaction, param, creator)
            .await?;
        transaction.commit().await?;
        Ok(project_set.identifier)
    }

    async fn query_space_member(&self, space_id: &String) -> error::Result<Vec<UserDTO>> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        let users = self
            .workspace_service
            .find_space_members(&mut transaction, space_id)
            .await?;
        transaction.commit().await?;
        let result = users.into_iter().map(|user| user.into()).collect();
        Ok(result)
    }

    async fn query_space_tag(&self, space_id: &String) -> error::Result<Vec<TagDTO>> {
        let mut transaction = APP_STATE.db_tx().begin().await?;
        let tags = self
            .workspace_service
            .find_space_tags(&mut transaction, space_id)
            .await?;
        transaction.commit().await?;
        let result = tags.into_iter().map(|tag| tag.into()).collect();
        Ok(result)
    }
}
