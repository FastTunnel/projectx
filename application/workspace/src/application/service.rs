use std::sync::Arc;

use async_trait::async_trait;

use app_interface::workspace::dto::command::TemplateCreateCommand;
use app_interface::workspace::dto::TemplateDTO;
use app_interface::workspace::IWorkspaceAppService;
use app_interface::{error, DbTx, APP_STATE};
use domain_workspace::facade::{IGlobalConfigFacade, IUserFacade};
use domain_workspace::model::value::CreateTemplateParam;
use domain_workspace::service::IWorkspaceService;
use sea_orm::TransactionTrait;

pub struct WorkspaceAppService {
    workspace_service: Arc<dyn IWorkspaceService<DbTx>>,
    global_config_facade: Arc<dyn IGlobalConfigFacade>,
    user_facade: Arc<dyn IUserFacade>,
}

impl WorkspaceAppService {
    pub fn new(
        workspace_service: Arc<dyn IWorkspaceService<DbTx>>,
        global_config_facade: Arc<dyn IGlobalConfigFacade>,
        user_facade: Arc<dyn IUserFacade>,
    ) -> Self {
        Self {
            workspace_service,
            global_config_facade,
            user_facade,
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
}
