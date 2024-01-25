use axum::{Extension, Json};
use axum_valid::Valid;

use crate::error;
use app_interface::app_service;
use app_interface::auth::Claims;
use app_interface::workspace::dto::command::TemplateCreateCommand;
use app_interface::workspace::dto::query::TemplateQuery;
use app_interface::workspace::dto::TemplateDTO;
use app_interface::workspace::IWorkspaceAppService;

#[tracing::instrument()]
pub async fn find_all_template(
    Json(param): Json<TemplateQuery>,
) -> error::Result<Json<Vec<TemplateDTO>>> {
    let workspace_app_service = app_service!(IWorkspaceAppService);
    let ret = workspace_app_service
        .find_all_template(&param.organization)
        .await?;
    Ok(Json(ret))
}

#[tracing::instrument()]
pub async fn template_detail(
    Json(param): Json<TemplateQuery>,
) -> error::Result<Json<Option<TemplateDTO>>> {
    let workspace_app_service = app_service!(IWorkspaceAppService);
    if param.template.is_none() {
        return Ok(Json(None));
    }
    let template_id = param.template.unwrap();
    let ret = workspace_app_service
        .template_detail(&param.organization, &template_id)
        .await?;
    Ok(Json(ret))
}

#[tracing::instrument()]
pub async fn create_template(
    Extension(claims): Extension<Claims>,
    Valid(Json(cmd)): Valid<Json<TemplateCreateCommand>>,
) -> error::Result<Json<String>> {
    let workspace_app_service = app_service!(IWorkspaceAppService);
    let template_id = workspace_app_service
        .create_template(&cmd, claims.current_user())
        .await?;
    Ok(Json(template_id))
}
