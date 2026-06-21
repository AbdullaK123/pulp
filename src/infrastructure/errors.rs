use thiserror::Error;
use bb8_redis::redis;
use bb8::RunError;
use bb8_redis::redis::RedisError;
use crate::shared::errors::SharedError;

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Database failure: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Redis failure: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("Redis Pool timeout!")]
    RedisConnectionPoolError,
    
    #[error(transparent)]
    Shared(#[from] SharedError),

    #[error("Internal error: {0}")]
    Internal(String)
}

impl From<RunError<RedisError>> for InfrastructureError {
    fn from(error: RunError<RedisError>) -> Self {
        match error {
            RunError::TimedOut =>InfrastructureError::RedisConnectionPoolError,
            RunError::User(redis_err) => InfrastructureError::RedisError(redis_err)
        }
    }
}