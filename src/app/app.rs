use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::get;
use serde_json::json;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use crate::app::controllers::auth::create_auth_controller;
use crate::app::middleware::session::create_session_layer;
use crate::data::repositories::api_key::ApiKeyRepository;
use crate::data::repositories::user::UserRepository;
use crate::infrastructure::database::create_pool;
use crate::service::auth::service::AuthService;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub auth_service: AuthService
}

pub async fn create_app() -> Router {

    async fn health() -> impl IntoResponse {
        Json(json!({
            "status": "healthy",
            "service": "pulp-api"
        }))
    }

    let pool = create_pool().await.expect("Failed to create connection pool");
    let user_repo = UserRepository::new(pool.clone());
    let api_key_repo = ApiKeyRepository::new(pool.clone());
    let auth_service = AuthService::new(user_repo, api_key_repo);

    sqlx::migrate!("./migrations").run(&pool).await.expect("Migrations failed.");

    let app_state = AppState {
        pool: pool.clone(),
        auth_service
    };

    let session_layer = create_session_layer(pool.clone(), false).await;

    let all_controllers = Router::new()
        .merge(create_auth_controller())
        .layer(session_layer);

    let app =
        Router::new()
            .route("/health", get(health))
            .nest("/api/v1/", all_controllers.with_state(app_state))
            .layer(TraceLayer::new_for_http());
    app
}