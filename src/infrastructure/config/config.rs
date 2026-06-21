use serde::Deserialize;
use std::fs::File;
use serde_yaml_bw;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64
}

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub max_size: u32
}


#[derive(Debug, Deserialize)]
pub struct Config {
    pub db: DatabaseConfig,
    pub redis: RedisConfig
}

impl Config {
    pub fn load() -> Self {
        match File::open("./config.yaml") {
            Ok(file) => match serde_yaml_bw::from_reader(file) {
                Ok(cfg) => cfg,
                Err(_) => {
                    // Fallback to sensible defaults if parsing fails
                    Self::default()
                }
            },
            Err(_) => {
                // Fallback to sensible defaults if file missing
                Self::default()
            }
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            max_connections: 5,
            min_connections: 1,
            acquire_timeout: 5,
            idle_timeout: 300,
            max_lifetime: 1800,
        }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self { max_size: 16 }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db: DatabaseConfig::default(),
            redis: RedisConfig::default(),
        }
    }
}