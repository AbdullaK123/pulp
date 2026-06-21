use anyhow::Result;
use sqlx::PgPool;
use std::env::var;
use std::time::Duration;
use sqlx::postgres::PgPoolOptions;
use crate::infrastructure::config::APP_CONFIG;
use tracing::{info, debug, error};

pub async fn create_pool() -> Result<PgPool> {

    let db_url = var("DATABASE_URL").expect("Database URL must be set in .env file");
    // Avoid logging full URL to prevent secret leakage; log only host/database if possible
    debug!("Initializing Postgres pool with configured settings");

    let pool =
        PgPoolOptions::new()
            .max_connections(APP_CONFIG.db.max_connections)
            .min_connections(APP_CONFIG.db.min_connections)
            .acquire_timeout(Duration::from_secs(APP_CONFIG.db.acquire_timeout))
            .idle_timeout(Duration::from_secs(APP_CONFIG.db.idle_timeout))
            .max_lifetime(Duration::from_secs(APP_CONFIG.db.max_lifetime))
            .connect(&db_url)
            .await?;

    info!("Connected to Postgres and established pool");

    Ok(pool)
}