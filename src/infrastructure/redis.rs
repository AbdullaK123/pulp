use anyhow::Result;
use bb8_redis::{bb8, RedisConnectionManager};
use bb8::{Pool};
use crate::infrastructure::config::APP_CONFIG;
use std::env::var;
use tracing::{debug, info};


pub async fn create_redis_pool() -> Result<Pool<RedisConnectionManager>> {
    
    let redis_url = var("REDIS_URL").expect("Redis URL must be set in .env file");
    debug!("Initializing Redis pool with configured settings");

    let manager = RedisConnectionManager::new(redis_url)?;
    
    let pool =
        Pool::builder()
            .max_size(APP_CONFIG.redis.max_size)
            .build(manager)
            .await?;
    info!("Connected to Redis and established pool");
    Ok(pool)
}