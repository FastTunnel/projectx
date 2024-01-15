use axum::{Extension, Json};
use axum_valid::Valid;

use crate::{APP_STATE, error};
use crate::auth::{AuthBody, Claims};
use crate::context::user::application::dto::command::{
    RoleBindCommand, RoleCreateCommand, UserCreateCommand,
    UserLoginCommand,
};
use crate::context::user::application::dto::UserDto;
use crate::context::user::application::IUserAppService;
use crate::context::user::application::service::UserAppService;

#[tracing::instrument()]
pub async fn register_user(
    Extension(claims): Extension<Claims>,
    Valid(Json(create_user_command)): Valid<Json<UserCreateCommand>>,
) -> error::Result<()> {
    let user_app_service = crate::app_service!(&APP_STATE, UserAppService);
    user_app_service
        .user_register(claims.current_user(), create_user_command)
        .await?;
    Ok(())
}

#[tracing::instrument()]
pub async fn login(
    Valid(Json(payload)): Valid<Json<UserLoginCommand>>,
) -> error::Result<Json<AuthBody>> {
    let user_app_service = crate::app_service!(&APP_STATE, UserAppService);
    let token = user_app_service.user_login(payload).await?;
    Ok(Json(AuthBody::new(token, "bearer".to_string())))
}

#[tracing::instrument()]
pub async fn user_detail(
    Extension(claims): Extension<Claims>,
) -> error::Result<Json<Option<UserDto>>> {
    let user_app_service = crate::app_service!(&APP_STATE, UserAppService);
    let ret = user_app_service.user_detail(&claims.current_user()).await?;
    Ok(Json(ret))
}

#[tracing::instrument()]
pub async fn create_role(
    Extension(claims): Extension<Claims>,
    Valid(Json(role)): Valid<Json<RoleCreateCommand>>,
) -> error::Result<()> {
    let user_app_service = crate::app_service!(&APP_STATE, UserAppService);
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
    let user_app_service = crate::app_service!(&APP_STATE, UserAppService);
    user_app_service.bind_role(command).await?;
    Ok(())
}

#[tracing::instrument()]
pub async fn user_unbind_roles(
    Extension(claims): Extension<Claims>,
    Valid(Json(command)): Valid<Json<RoleBindCommand>>,
) -> error::Result<()> {
    let user_app_service = crate::app_service!(&APP_STATE, UserAppService);
    user_app_service.unbind_role(command).await?;
    Ok(())
}
