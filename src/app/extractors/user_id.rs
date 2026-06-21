use axum::{
    extract::FromRequestParts,
    http::{request::Parts}
};
use axum::extract::{FromRef, State};
use tower_sessions::Session;
use uuid::Uuid;
use crate::app::app::AppState;
use crate::app::errors::AppError;
use crate::data::errors::DataError;
use crate::infrastructure::errors::InfrastructureError;
use crate::service::errors::ServiceError;

pub struct UserId(pub Uuid);


impl <S> FromRequestParts<S> for UserId
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {

        let State(app_state) = State::<AppState>::from_request_parts(parts, state)
            .await
            .map_err(|e| AppError::Infrastructure(InfrastructureError::Internal(e.to_string())))?;

        // Try API key auth if header present; otherwise fall back to session
        if let Some(key_header) = parts.headers.get("X-API-Key") {
            if let Ok(key_str) = key_header.to_str() {
                if let Ok(Some(api_key)) = app_state.auth_service.is_valid_api_key(key_str.to_string()).await {
                    if api_key.revoked_at.is_none() {
                        return Ok(UserId(api_key.user_id));
                    }
                }
            }
        }

        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::Service(ServiceError::AuthError))?;

        let user_id: Option<Uuid> = session.get("user_id")
            .await
            .map_err(|e|
                AppError::Infrastructure(InfrastructureError::Internal(e.to_string()))
            )?;

        match user_id {
            Some(id) => Ok(UserId(id)),
            None => Err(AppError::Service(ServiceError::AuthError))
        }

    }

}