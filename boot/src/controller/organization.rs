use crate::error;
use app_interface::app_service;
use app_interface::auth::Claims;
use app_interface::user::dto::command::{CreateTeamCommand, TeamMemberCommand};
use app_interface::user::dto::OrganizationDto;
use app_interface::user::IUserAppService;
use axum::{Extension, Json};
use axum_valid::Valid;

#[tracing::instrument()]
pub async fn current_organization(
    Extension(_claims): Extension<Claims>,
) -> error::Result<Json<Option<OrganizationDto>>> {
    let user_app_service = app_service!(IUserAppService);
    let ret = user_app_service.current_organization().await?;
    Ok(Json(ret))
}

#[tracing::instrument()]
pub async fn create_team(
    Extension(claims): Extension<Claims>,
    Valid(Json(command)): Valid<Json<CreateTeamCommand>>,
) -> error::Result<()> {
    let user_app_service = app_service!(IUserAppService);
    user_app_service
        .create_team(claims.current_user(), command)
        .await?;
    Ok(())
}

#[tracing::instrument()]
pub async fn team_add_member(
    Extension(_claims): Extension<Claims>,
    Valid(Json(command)): Valid<Json<TeamMemberCommand>>,
) -> error::Result<()> {
    let user_app_service = app_service!(IUserAppService);
    user_app_service.team_add_member(command).await?;
    Ok(())
}

#[tracing::instrument()]
pub async fn team_remove_member(
    Extension(_claims): Extension<Claims>,
    Valid(Json(command)): Valid<Json<TeamMemberCommand>>,
) -> error::Result<()> {
    let user_app_service = app_service!(IUserAppService);
    user_app_service.team_remove_member(command).await?;
    Ok(())
}
