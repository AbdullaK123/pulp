use axum::extract::{Query, State};
use axum::{Json, Router};
use axum::routing::post;
use serde_json::Value;
use tower_sessions::Session;
use uuid::Uuid;
use crate::app::app::AppState;
use crate::app::errors::AppError;
use crate::app::extractors::user_id::UserId;
use crate::data::models::api_key::ApiKeyResponse;
use crate::data::models::user::{LoginRequest, LogoutResponse, RegistrationRequest, User, UserResponse};
use tracing::{info, warn, error, debug};
use serde::Deserialize;


pub async fn register(
    State(app_state): State<AppState>,
    Json(payload): Json<RegistrationRequest>
) -> Result<Json<UserResponse>, AppError> {
    info!(email = %payload.email, "Register attempt");
    match app_state.auth_service.register(payload).await {
        Ok(resp) => {
            info!(user_id = %resp.id, "Register success");
            Ok(Json(resp))
        },
        Err(e) => {
            warn!(error = %e, "Register failed");
            Err(AppError::Service(e))
        }
    }
}

pub async fn login(
    State(app_state): State<AppState>,
    session: Session,
    Json(payload): Json<LoginRequest>
) -> Result<Json<UserResponse>, AppError> {
    info!(email = %payload.email, "Login attempt");
    match app_state.auth_service.login(session, payload).await {
        Ok(resp) => {
            info!(user_id = %resp.id, "Login success");
            Ok(Json(resp))
        },
        Err(e) => {
            warn!(error = %e, "Login failed");
            Err(AppError::Service(e))
        }
    }
}

pub async fn logout(
    State(app_state): State<AppState>,
    session: Session
) -> Result<Json<LogoutResponse>, AppError> {
    info!("Logout attempt");
    match app_state.auth_service.logout(session).await {
        Ok(resp) => {
            info!("Logout success");
            Ok(Json(resp))
        },
        Err(e) => {
            warn!(error = %e, "Logout failed");
            Err(AppError::Service(e))
        }
    }
}

#[derive(Deserialize)]
struct CreateApiKeyParams {
    name: Option<String>,
}

pub async fn create_api_key(
    State(app_state): State<AppState>,
    UserId(current_user_id): UserId,
    Query(params): Query<CreateApiKeyParams>
) -> Result<Json<ApiKeyResponse>, AppError> {
    let name_ref = params.name.as_deref().unwrap_or("(none)");
    info!(%current_user_id, name = name_ref, "Create API key attempt");
    match app_state.auth_service.create_api_key(current_user_id, params.name).await {
        Ok(resp) => {
            info!(%current_user_id, key_id = %resp.id, "Create API key success");
            Ok(Json(resp))
        },
        Err(e) => {
            warn!(%current_user_id, error = %e, "Create API key failed");
            Err(AppError::Service(e))
        }
    }
}

pub async fn revoke_api_key(
    State(app_state): State<AppState>,
    UserId(current_user_id): UserId,
    Query(params): Query<RevokeParams>
) -> Result<Json<Value>, AppError> {
    info!(%current_user_id, %params.key_id, "Revoke API key attempt");
    match app_state.auth_service.revoke_api_key(params.key_id, current_user_id).await {
        Ok(resp) => {
            info!(%current_user_id, %params.key_id, "Revoke API key success");
            Ok(resp)
        },
        Err(e) => {
            warn!(%current_user_id, %params.key_id, error = %e, "Revoke API key failed");
            Err(AppError::Service(e))
        }
    }
}

#[derive(Deserialize)]
struct RevokeParams { key_id: Uuid }

pub fn create_auth_controller() -> Router<AppState> {

    let base_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/api-keys/create", post(create_api_key))
        .route("/api-keys/revoke", post(revoke_api_key));

    Router::new().nest("/auth", base_routes)
}