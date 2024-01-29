use std::sync::atomic::Ordering;

use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{Json, RequestPartsExt};
use axum_extra::headers::authorization::{Bearer, Credentials};
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use axum_valid::Valid;
use jsonwebtoken::{decode, Validation};
use serde::Serialize;

use app_interface::auth::Claims;
use app_interface::user::dto::command::UserLoginCommand;
use app_interface::user::IUserAppService;
use app_interface::{app_service, APP_STATE};

use crate::error;
use crate::error::AuthError;

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

#[tracing::instrument()]
pub async fn login(
    Valid(Json(payload)): Valid<Json<UserLoginCommand>>,
) -> error::Result<Json<AuthBody>> {
    let user_app_service = app_service!(IUserAppService);
    let token = user_app_service.user_login(payload).await?;
    Ok(Json(AuthBody::new(token, "bearer".to_string())))
}

pub async fn auth_middleware(req: Request, next: Next) -> impl IntoResponse {
    auth_process(req, next)
        .await
        .unwrap_or_else(|err| err.into_response())
}

async fn auth_process(mut req: Request, next: Next) -> Result<Response, AuthError> {
    let (mut parts, body) = req.into_parts();
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| {
            tracing::error!("Missing authorization header");
            AuthError::MissingCredentials
        })?;
    let token_data = decode::<Claims>(
        bearer.token(),
        &unsafe { &APP_STATE.keys.load(Ordering::Relaxed).as_ref().unwrap() }.decoding,
        &Validation::default(),
    )
    .map_err(|_| {
        tracing::error!("Invalid token");
        AuthError::InvalidToken
    })?;

    if !token_data.claims.is_expired() {
        req = Request::from_parts(parts, body);
        req.extensions_mut().insert(token_data.claims);
        Ok(next.run(req).await)
    } else if token_data.claims.can_refresh() {
        let new_token = app_service!(IUserAppService)
            .refresh_token(
                token_data.claims.current_user(),
                &bearer.token().to_string(),
            )
            .await
            .map_err(|_| {
                tracing::error!("Token creation error");
                AuthError::TokenCreation
            })?;
        req = Request::from_parts(parts, body);
        let token_data = decode::<Claims>(
            new_token.as_str(),
            &unsafe { &APP_STATE.keys.load(Ordering::Relaxed).as_ref().unwrap() }.decoding,
            &Validation::default(),
        )
        .map_err(|_| {
            tracing::error!("Invalid token");
            AuthError::InvalidToken
        })?;
        req.extensions_mut().insert(token_data.claims);
        let mut resp = next.run(req).await;
        resp.headers_mut().insert(
            axum_extra::headers::HeaderName::from_static("Authorization"),
            Authorization::bearer(new_token.as_str())
                .map_err(|_| {
                    tracing::error!("Token creation error");
                    AuthError::TokenCreation
                })?
                .0
                .encode(),
        );
        tracing::debug!("Token refreshed");
        Ok(resp)
    } else {
        tracing::error!("Token expired");
        Err(AuthError::ExpiredToken)
    }
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    pub fn new(access_token: String, token_type: String) -> Self {
        Self {
            access_token,
            token_type,
        }
    }
}
