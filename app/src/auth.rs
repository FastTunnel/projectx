use std::fmt::Display;
use std::sync::atomic::Ordering;

use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::RequestPartsExt;
use axum_extra::headers::authorization::{Bearer, Credentials};
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::context::user::application::service::UserAppService;
use crate::context::user::application::IUserAppService;
use crate::error::AuthError;
use crate::{app_service, APP_STATE};

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

pub async fn auth(req: Request, next: Next) -> impl IntoResponse {
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
        let new_token = app_service!(&APP_STATE, UserAppService)
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
        .unwrap();
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

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    pub fn new(sub: String, exp: usize) -> Self {
        Self { sub, exp }
    }
    pub fn is_expired(&self) -> bool {
        self.exp < chrono::Utc::now().timestamp() as usize
    }
    pub fn can_refresh(&self) -> bool {
        self.exp < chrono::Utc::now().timestamp() as usize + 60 * 60
    }
    pub fn current_user(&self) -> &String {
        &self.sub
    }
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "subject:{}", self.sub)
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
