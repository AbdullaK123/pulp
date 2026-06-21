use axum::response::{IntoResponse, Response};
use axum::http::{StatusCode};
use axum::Json;
use thiserror::Error;
use crate::data::errors::DataError;
use crate::infrastructure::errors::InfrastructureError;
use crate::service::errors::ServiceError;
use serde_json::json;
use crate::shared::errors::SharedError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Infrastructure( #[from] InfrastructureError ),
    #[error(transparent)]
    Data(#[from] DataError),
    #[error(transparent)]
    Service(#[from] ServiceError),
    #[error(transparent)]
    Shared(#[from] SharedError)
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        
        let (status, client_message) = match &self {
            AppError::Infrastructure(e) => (StatusCode::SERVICE_UNAVAILABLE, e.to_string()),
            AppError::Data(e) => {
                match e {
                    DataError::ValidationError(err) => (StatusCode::NOT_ACCEPTABLE, err.to_string())
                }
            },
            AppError::Service(e) => {
                match e {
                    ServiceError::InternalError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
                    ServiceError::AuthError => (StatusCode::UNAUTHORIZED, "Authentication failed".to_string()),
                    ServiceError::NotFoundError => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
                    ServiceError::ConflictError(err) => (StatusCode::CONFLICT, err.to_string()),
                    ServiceError::InternalServiceError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
                    ServiceError::ForbiddenError(err) => (StatusCode::FORBIDDEN, err.to_string())
                }
            },
            AppError::Shared(e) => {
                match e {
                    SharedError::InternalError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                }
            }
        };
        
        let body = json!({
            "error": {
                "message": client_message,
                "status": status.as_u16()
            }
        });

        (status, Json(body)).into_response()
    }
}