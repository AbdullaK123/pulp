use sqlx::PgPool;
use uuid::Uuid;
use crate::data::models::api_key::ApiKey;
use crate::infrastructure::errors::InfrastructureError;

#[derive(Clone)]
pub struct ApiKeyRepository {
    pool: PgPool
}

impl ApiKeyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    pub async fn create(&self, user_id: Uuid, key_hash: String, name: Option<String>) -> Result<ApiKey, InfrastructureError> {
        let api_key = sqlx::query_as!(
            ApiKey,
            "
            INSERT INTO api_keys  (user_id, key_hash, name)
            VALUES ($1, $2, $3)
            RETURNING *
            ",
            user_id,
            key_hash,
            if name.is_some() { name.unwrap() } else {"my-key".to_string()}
        ).fetch_one(&self.pool).await?;
        Ok(api_key)
    }

    pub async fn revoke(&self, id: Uuid, user_id: Uuid) -> Result<bool, InfrastructureError> {
        let result = sqlx::query!(
            "
            UPDATE api_keys
            SET revoked_at = NOW()
            WHERE id=$1 AND user_id=$2
            ",
            id,
            user_id
        ).execute(&self.pool).await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn use_key(&self, id: Uuid) -> Result<bool, InfrastructureError> {
        let result = sqlx::query!(
            "
            UPDATE api_keys
            SET last_used_at = NOW()
            WHERE id=$1
            ",
            id
        ).execute(&self.pool).await?;
        Ok(result.rows_affected() > 0)
    }
    
    pub async fn search_api_key(&self, key_hash: String) -> Result<Option<ApiKey>, InfrastructureError> {
        let result = sqlx::query_as!(
            ApiKey,
            "
            SELECT * 
            FROM api_keys
            WHERE key_hash = $1
            ",
            key_hash
        ).fetch_optional(&self.pool).await?;
        Ok(result)
    }
}