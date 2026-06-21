use validator::{
    Validate
};
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Deserialize, Validate)]
pub struct RegistrationRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min=8))]
    pub password: String
}

pub type LoginRequest = RegistrationRequest;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all="lowercase")]
pub enum AuthProvider {
    Google,
    Backend
}

impl From<String> for AuthProvider {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "google" => AuthProvider::Google,
            _ => AuthProvider::Backend,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all="lowercase")]
pub enum Tier {
    Free,
    Paid
}

impl From<String> for Tier {
    fn from(tier: String) -> Self {
        match tier.to_lowercase().as_str() {
            "free" => Tier::Free,
            "paid" => Tier::Paid,
            _ => panic!("Deserialization failed. Must be 'free' or 'paid'.")
        }
    }
}

impl From<Option<String>> for AuthProvider {
    fn from(provider: Option<String>) -> Self {
        // Return Some(AuthProvider) instead of a bare AuthProvider
        if provider.is_some() {
            AuthProvider::Google
        } else {
            AuthProvider::Backend
        }
    }
}

#[derive(FromRow, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub auth_provider: AuthProvider,
    pub tier: Tier,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LogoutResponse {
    pub message: String
} 