use app_interface::app_service;
use axum::{Extension, Json};
use axum_valid::Valid;

use app_interface::auth::Claims;
use app_interface::user::dto::command::{RoleBindCommand, RoleCreateCommand, UserCreateCommand};
use app_interface::user::dto::query::RoleQuery;
use app_interface::user::dto::{RoleDTO, UserDto};
use app_interface::user::IUserAppService;

use crate::error;

#[tracing::instrument()]
pub async fn register_user(
    Extension(claims): Extension<Claims>,
    Valid(Json(create_user_command)): Valid<Json<UserCreateCommand>>,
) -> error::Result<()> {
    let user_app_service = app_service!(IUserAppService);
    user_app_service
        .user_register(claims.current_user(), create_user_command)
        .await?;
    Ok(())
}

#[tracing::instrument()]
pub async fn user_detail(
    Extension(claims): Extension<Claims>,
) -> error::Result<Json<Option<UserDto>>> {
    let user_app_service = app_service!(IUserAppService);
    let ret = user_app_service.user_detail(&claims.current_user()).await?;
    Ok(Json(ret))
}

#[tracing::instrument()]
pub async fn create_role(
    Extension(claims): Extension<Claims>,
    Valid(Json(role)): Valid<Json<RoleCreateCommand>>,
) -> error::Result<()> {
    let user_app_service = app_service!(IUserAppService);
    user_app_service
        .create_role(claims.current_user(), &role)
        .await?;
    Ok(())
}

#[tracing::instrument()]
pub async fn user_bind_roles(
    Extension(claims): Extension<Claims>,
    Valid(Json(command)): Valid<Json<RoleBindCommand>>,
) -> error::Result<()> {
    let user_app_service = app_service!(IUserAppService);
    user_app_service.bind_role(command).await?;
    Ok(())
}

#[tracing::instrument()]
pub async fn user_unbind_roles(
    Extension(claims): Extension<Claims>,
    Valid(Json(command)): Valid<Json<RoleBindCommand>>,
) -> error::Result<()> {
    let user_app_service = app_service!(IUserAppService);
    user_app_service.unbind_role(command).await?;
    Ok(())
}

#[tracing::instrument()]
pub async fn query_roles_by_org_and_own(
    Valid(Json(param)): Valid<Json<RoleQuery>>,
) -> error::Result<Json<Vec<RoleDTO>>> {
    let user_app_service = app_service!(IUserAppService);
    let ret = user_app_service
        .query_roles_by_own(&param.organization, param.own)
        .await?;
    Ok(Json(ret))
}
