use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde_json::{json, Value};
use tower_sessions::Session;
use crate::data::models::user::{LoginRequest, LogoutResponse, RegistrationRequest, UserResponse};
use crate::data::repositories::api_key::ApiKeyRepository;
use crate::data::repositories::user::UserRepository;
use crate::service::errors::ServiceError;
use crate::shared::password::verify_password;
use sha2::{Digest, Sha256};
use sha2::digest::Update;
use uuid::Uuid;
use crate::data::models::api_key::{ApiKey, ApiKeyResponse};
use crate::shared::errors::SharedError;

#[derive(Clone)]
pub struct AuthService {
    user_repo: UserRepository,
    api_key_repo: ApiKeyRepository
}

impl AuthService {
    pub fn new(user_repo: UserRepository, api_key_repo: ApiKeyRepository) -> Self {
        Self {
            user_repo,
            api_key_repo
        }
    }

    fn generate_api_key() -> String {
        let mut rng = rand::thread_rng();
        let secret_token: String = (0..32).map(|_| rng.sample(Alphanumeric) as char).collect();
        let key = format!("pulp_{secret_token}");
        key
    }

    fn hash_api_key(api_key: String) -> String {
        let mut hasher = Sha256::new();
        Digest::update(&mut hasher, api_key.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub async fn register(&self, payload: RegistrationRequest) -> Result<UserResponse, ServiceError> {
        let user = self.user_repo.fetch_user_by_email(payload.email.clone()).await?;
        if user.is_some() {
            return Err(ServiceError::ConflictError("User with that email already exists".to_string()))
        }
        let created_user = self.user_repo.register(payload).await?;
        Ok(created_user.into())
    }

    pub async fn login(&self, session: Session,  payload: LoginRequest) -> Result<UserResponse, ServiceError> {

        let user = self.user_repo.fetch_user_by_email(payload.email.clone()).await?;

        if user.is_none() {
            return Err(ServiceError::NotFoundError)
        }

        let user = user.clone().unwrap();

        let password_hash = &user.password_hash;

        let password_correct = verify_password(&payload.password, password_hash.clone())?;

        if !password_correct {
            return Err(ServiceError::AuthError)
        }

        session.cycle_id().await.unwrap();

        let user_id = &user.id;

        session.insert("user_id", *user_id).await.unwrap();

        Ok(user.into())
    }

    pub async fn logout(&self, session: Session) -> Result<LogoutResponse, ServiceError> {
        session.flush().await.unwrap();
        Ok(LogoutResponse {
            message: "Successfully logged out".to_string()
        })
    }

    pub async fn create_api_key(&self, user_id: Uuid, name: Option<String>) -> Result<ApiKeyResponse, ServiceError> {
        let key = Self::generate_api_key();
        let key_hash = Self::hash_api_key(key.clone());
        let result = self.api_key_repo.create(user_id, key_hash, name).await?;
        Ok(ApiKeyResponse {
            id: result.id,
            name: result.name.unwrap(),
            key,
            warning: "Make sure to copy your personal API key now. You won't be able to see it again!".to_string()
        })
    }

    pub async fn is_valid_api_key(&self, key: String) -> Result<Option<ApiKey>, ServiceError> {
        let key_hash = Self::hash_api_key(key);
        let result = self.api_key_repo.search_api_key(key_hash).await?;
        Ok(result)
    }

    pub async fn revoke_api_key(&self, id: Uuid, user_id: Uuid) -> Result<Json<Value>, ServiceError> {
        let result = self.api_key_repo.revoke(id, user_id).await?;
        if result {
            Ok(Json(json!({
                "message": "Successfully revoked api key"
            })))
        } else {
            Err(ServiceError::NotFoundError)
        }
    } 
}