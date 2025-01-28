use crate::error::AppError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
pub fn generate(password: &str, salt: Option<&str>) -> Result<(String, String), AppError> {
    let salt = if let Some(salt) = salt {
        SaltString::from_b64(salt).unwrap()
    } else {
        SaltString::generate(&mut OsRng)
    };
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Auth(e.to_string()))?
        .to_string();
    Ok((password_hash, salt.to_string()))
}

pub fn verify(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();
    let parsed_password = PasswordHash::new(&password_hash).unwrap();
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_password)
        .is_ok())
}
