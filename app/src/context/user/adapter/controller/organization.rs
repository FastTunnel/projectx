use axum::{Extension, Json};
use axum_valid::Valid;

use crate::auth::Claims;
use crate::context::user::application::dto::command::{
    CreateTeamCommand, InitOrganizationCommand, TeamMemberCommand,
};
use crate::context::user::application::dto::OrganizationDto;
use crate::context::user::application::service::UserAppService;
use crate::context::user::application::IUserAppService;
use crate::{error, APP_STATE};

#[tracing::instrument()]
pub async fn init_system(
    Valid(Json(command)): Valid<Json<InitOrganizationCommand>>,
) -> error::Result<()> {
    let user_app_service = crate::app_service!(&APP_STATE, UserAppService);
    user_app_service.init_system(&command).await?;
    Ok(())
}

#[tracing::instrument()]
pub async fn current_organization(
    Extension(_claims): Extension<Claims>,
) -> error::Result<Json<Option<OrganizationDto>>> {
    let user_app_service = crate::app_service!(&APP_STATE, UserAppService);
    let ret = user_app_service.current_organization().await?;
    Ok(Json(ret))
}

#[tracing::instrument()]
pub async fn create_team(
    Extension(claims): Extension<Claims>,
    Valid(Json(command)): Valid<Json<CreateTeamCommand>>,
) -> error::Result<()> {
    let user_app_service = crate::app_service!(&APP_STATE, UserAppService);
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
    let user_app_service = crate::app_service!(&APP_STATE, UserAppService);
    user_app_service.team_add_member(command).await?;
    Ok(())
}

#[tracing::instrument()]
pub async fn team_remove_member(
    Extension(_claims): Extension<Claims>,
    Valid(Json(command)): Valid<Json<TeamMemberCommand>>,
) -> error::Result<()> {
    let user_app_service = crate::app_service!(&APP_STATE, UserAppService);
    user_app_service.team_remove_member(command).await?;
    Ok(())
}
