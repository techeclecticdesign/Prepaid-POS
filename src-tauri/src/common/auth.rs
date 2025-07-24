use crate::common::error::AppError;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use password_hash::SaltString;
use rand::rngs::OsRng;
use std::time::Instant;

#[derive(Default)]
pub struct AuthState {
    pub logged_in: bool,
    pub last_activity: Option<Instant>,
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Unexpected(e.to_string()))?
        .to_string();
    Ok(hash)
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash);
    if let Ok(parsed) = parsed_hash {
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok()
    } else {
        false
    }
}
