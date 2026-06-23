use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Deserialize)]
pub struct ApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub key_hint: String,
    pub key_hash: String,
    pub name: Option<String>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>
}

#[derive(Serialize, Deserialize)]
pub struct ApiKeyResponse {
    pub id: Uuid,
    pub name: String,
    pub key: String,
    pub warning: String
}

#[derive(Serialize, Deserialize)]
pub struct ApiKeyHintItem {
    pub id: Uuid,
    pub name: String,
    pub key_hint: String,
    pub last_used_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>
}

impl From<ApiKey> for ApiKeyHintItem {
    fn from(value: ApiKey) -> Self {
        Self {
            id: value.id,
            name: value.name.unwrap(),
            key_hint: value.key_hint,
            last_used_at: value.last_used_at,
            revoked_at: value.revoked_at,
            created_at: value.created_at
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiKeyHintListResponse {
    keys: Vec<ApiKeyHintItem>
}

impl From<Vec<ApiKey>> for ApiKeyHintListResponse {
    fn from(value: Vec<ApiKey>) -> Self {
        let list_items = value.into_iter().map(|key| key.into()).collect();
        Self {
            keys: list_items
        }
    }
}