use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use crate::application::{
    app_error::{AppError, AppResult},
    user_service::PasswordHasher as PasswordHasherTrait,
};

#[derive(Default)]
pub struct Argon2PasswordHasher {
    hasher: Argon2<'static>,
}

impl PasswordHasherTrait for Argon2PasswordHasher {
    fn hash_password(&self, password: &str) -> AppResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = self
            .hasher
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::Internal("Password hashing failed".into()))?
            .to_string();

        Ok(hash)
    }
}
