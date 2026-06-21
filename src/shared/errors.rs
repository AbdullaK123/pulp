use thiserror::Error;
use anyhow::Error;

#[derive(Error, Debug)]
pub enum SharedError {
    #[error(transparent)]
    InternalError(#[from] Error)
}