use app_interface::error::AppError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use domain_common::error;
use sea_orm::DbErr;
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum WebError {
    #[error("domain error: {0}")]
    DomainError(#[from] error::DomainError),
    #[error("service not found")]
    ServiceNotFound,
    #[error("transaction error: {0}")]
    TransactionError(#[from] DbErr),
    #[error("param invalid: {0}")]
    ParamInvalid(#[from] ValidationErrors),
    #[error("app error: {0}")]
    AppError(#[from] AppError),
}

pub type Result<T> = std::result::Result<T, WebError>;

impl IntoResponse for WebError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        tracing::error!("{:?}", self);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            match self {
                WebError::DomainError(e) => {
                    format!("DomainError: {}", e)
                }
                WebError::ServiceNotFound => "ServiceNotFound".to_string(),
                WebError::TransactionError(e) => {
                    format!("TransactionError: {}", e)
                }
                WebError::ParamInvalid(e) => {
                    format!("ParamInvalid: {}", e)
                }
                WebError::AppError(e) => {
                    format!("AppError: {}", e)
                }
            },
        )
            .into_response()
    }
}

#[derive(Debug)]
pub enum AuthError {
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    ExpiredToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Expired token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
