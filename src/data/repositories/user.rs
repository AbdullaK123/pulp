use sqlx::PgPool;
use uuid::Uuid;
use crate::data::models::user::{User};
use crate::infrastructure::errors::InfrastructureError;
use super::super::models::user::RegistrationRequest;
use super::super::super::shared::password::{hash_password};

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    pub async fn fetch_user_by_id(&self, id: Uuid) -> Result<Option<User>, InfrastructureError> {
        let result: Option<User> = sqlx::query_as!(
            User,
            "
            SELECT *
            FROM users 
            WHERE id=$1
            ",
            id
        ).fetch_optional(&self.pool).await?;
        Ok(result)
    }
    
    pub async fn fetch_user_by_email(&self, email: String) -> Result<Option<User>, InfrastructureError> {
        let result: Option<User> = sqlx::query_as!(
            User,
            "
            SELECT *
            FROM users
            WHERE email=$1
            ",
            email
        ).fetch_optional(&self.pool).await?;
        Ok(result)
    }

    pub async fn register(&self, payload: RegistrationRequest) -> Result<User, InfrastructureError> {

        let password_hash = tokio::task::spawn_blocking(move || {
            hash_password(&payload.password)
        }).await.expect("Failed to hash password.")?;

        let result = sqlx::query_as!(
            User,
            "
                INSERT INTO users (email, password_hash)
                VALUES ($1, $2)
                RETURNING *
            ",
            payload.email,
            password_hash
        ).fetch_one(&self.pool).await?;

        Ok(result)
    }
}