use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}, PasswordHash};
use anyhow::{Result, anyhow};
use crate::shared::errors::SharedError;

pub fn hash_password(password: &str) -> Result<String, SharedError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| SharedError::InternalError(anyhow!("Password hashing failed: {}", e.to_string())));
    hash
}

pub fn verify_password(password: &str, hash: String) -> Result<bool, SharedError> {
    let parsed_hash = PasswordHash::new(&hash)
        .map_err(|e| SharedError::InternalError(anyhow!("Failed to parse hash: {}", e.to_string())))?;
    let result = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    Ok(result)
}