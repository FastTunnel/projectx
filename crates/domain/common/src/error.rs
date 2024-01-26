use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("illegal argument: {0}")]
    IllegalArgument(String),
    #[error("password not match")]
    PasswordNotMatch,
    #[error("client call error")]
    ClientCallError,
    #[error("data not found")]
    DataNotFound,
    #[error("database error {0}")]
    DatabaseError(Box<dyn std::error::Error + Send + Sync>),
    #[error("data already exists")]
    DataAlreadyExists,
    #[error("jwt error {0}")]
    JwtError(String),
    #[error("application is initialized")]
    AppInitialized,
    #[error("application init failed {0}")]
    AppInitFailed(String),
    #[error("client call error {0}")]
    CallClientError(Box<dyn std::error::Error + Send + Sync>),
    #[error("application not initialized")]
    AppNotInitialized,
    #[error("inner error {0}")]
    InnerError(String),
}

pub type Result<T> = std::result::Result<T, DomainError>;
