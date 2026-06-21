use thiserror::Error;
use crate::infrastructure::errors::InfrastructureError;
use crate::shared::errors::SharedError;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Internal Service Error: {0}")]
    InternalServiceError(#[from] InfrastructureError),
    #[error("Internal Error: {0}")]
    InternalError(#[from] SharedError),
    #[error("Conflict Error: {0}")]
    ConflictError(String),
    #[error("Resource not found")]
    NotFoundError,
    #[error("Invalid credentials")]
    AuthError,
    #[error("Authorization error: {0}")]
    ForbiddenError(String)
}

