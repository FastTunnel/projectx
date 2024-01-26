use sea_orm::DbErr;
use thiserror::Error;

use domain_common::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("domain error: {0}")]
    DomainError(#[from] error::DomainError),
    #[error("service not found")]
    ServiceNotFound,
    #[error("transaction error: {0}")]
    TransactionError(#[from] DbErr),
}

pub type Result<T> = std::result::Result<T, AppError>;
