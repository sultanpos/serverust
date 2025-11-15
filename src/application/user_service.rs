use secrecy::{ExposeSecret, SecretString};
use std::sync::Arc;
use tracing::{info, instrument};

#[cfg(test)]
use async_trait::async_trait;

use crate::{application::app_error::AppResult, persistence::user_repo::UserRepository};

// ============================================================================
// Port Traits (Interfaces for dependencies)
// ============================================================================

pub trait PasswordHasher: Send + Sync {
    fn hash_password(&self, password: &str) -> AppResult<String>;
}

// ============================================================================
// User Service (Business Logic)
// ============================================================================

#[derive(Clone)]
pub struct UserService {
    hasher: Arc<dyn PasswordHasher>,
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(hasher: Arc<dyn PasswordHasher>, repository: Arc<dyn UserRepository>) -> Self {
        Self { hasher, repository }
    }

    #[instrument(skip(self, password))]
    pub async fn register_user(
        &self,
        username: &str,
        email: &str,
        password: &SecretString,
    ) -> AppResult<()> {
        info!("Registering user: {}", username);

        let hash = self.hasher.hash_password(password.expose_secret())?;
        self.repository.create_user(username, email, &hash).await?;

        info!("User registered successfully: {}", username);

        Ok(())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct MockUserRepository;

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn create_user(
            &self,
            username: &str,
            email: &str,
            _password_hash: &str,
        ) -> AppResult<()> {
            assert_eq!(username, "testuser");
            assert_eq!(email, "testuser@gmail.com");
            Ok(())
        }
        async fn get_user_by_username(
            &self,
            _username: &str,
        ) -> AppResult<Option<crate::domain::user::User>> {
            Ok(None)
        }
    }

    struct MockPasswordHasher;

    impl PasswordHasher for MockPasswordHasher {
        fn hash_password(&self, password: &str) -> AppResult<String> {
            Ok(format!("{}_hashed", password))
        }
    }

    #[tokio::test]
    async fn test_register_user() {
        let service = UserService::new(Arc::new(MockPasswordHasher), Arc::new(MockUserRepository));

        let result = service
            .register_user("testuser", "testuser@gmail.com", &"password123".into())
            .await;

        assert!(result.is_ok());
    }
}
