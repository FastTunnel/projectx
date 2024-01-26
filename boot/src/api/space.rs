use axum::extract::Path;
use axum::{Extension, Json};
use axum_valid::Valid;

use app_interface::app_service;
use app_interface::auth::Claims;
use app_interface::workspace::dto::command::{ProjectCreateCommand, ProjectSetCreateCommand};
use app_interface::workspace::dto::query::{ProjectQuery, ProjectSetQuery};
use app_interface::workspace::dto::{ProjectDTO, ProjectSetDTO, TagDTO, UserDTO};
use app_interface::workspace::IWorkspaceAppService;

use crate::error;

#[tracing::instrument()]
pub async fn query_space_member(Path(space_id): Path<String>) -> error::Result<Json<Vec<UserDTO>>> {
    let work_space_service = app_service!(IWorkspaceAppService);
    Ok(Json(
        work_space_service.query_space_member(&space_id).await?,
    ))
}

#[tracing::instrument()]
pub async fn query_space_tag(Path(space_id): Path<String>) -> error::Result<Json<Vec<TagDTO>>> {
    let work_space_service = app_service!(IWorkspaceAppService);
    Ok(Json(work_space_service.query_space_tag(&space_id).await?))
}

#[tracing::instrument()]
pub async fn create_project(
    Extension(claims): Extension<Claims>,
    Valid(Json(project_create_command)): Valid<Json<ProjectCreateCommand>>,
) -> error::Result<Json<String>> {
    let work_space_service = app_service!(IWorkspaceAppService);
    Ok(Json(
        work_space_service
            .create_project(&project_create_command, claims.current_user())
            .await?,
    ))
}

#[tracing::instrument()]
pub async fn create_project_set(
    Extension(claims): Extension<Claims>,
    Valid(Json(project_set_create_command)): Valid<Json<ProjectSetCreateCommand>>,
) -> error::Result<Json<String>> {
    let work_space_service = app_service!(IWorkspaceAppService);
    Ok(Json(
        work_space_service
            .create_project_set(&project_set_create_command, claims.current_user())
            .await?,
    ))
}
#[tracing::instrument()]
pub async fn query_all_project_set(
    Json(project_set_query): Json<ProjectSetQuery>,
) -> error::Result<Json<Vec<ProjectSetDTO>>> {
    let work_space_service = app_service!(IWorkspaceAppService);
    work_space_service
        .query_all_project_set(&project_set_query.organization)
        .await?;
    todo!()
}
#[tracing::instrument()]
pub async fn query_all_project(
    Json(project_query): Json<ProjectQuery>,
) -> error::Result<Json<Vec<ProjectDTO>>> {
    let work_space_service = app_service!(IWorkspaceAppService);
    work_space_service
        .query_all_project(
            &project_query.organization,
            project_query.project_set.as_ref(),
        )
        .await?;
    todo!()
}
