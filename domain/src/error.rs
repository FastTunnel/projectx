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
    #[error("app is initialized")]
    AppInitialized,
    #[error("app init failed {0}")]
    AppInitFailed(String),
    #[error("client call error {0}")]
    CallClientError(Box<dyn std::error::Error + Send + Sync>),
    #[error("app not initialized")]
    AppNotInitialized,
}

pub type Result<T> = std::result::Result<T, DomainError>;
