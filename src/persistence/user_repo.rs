use async_trait::async_trait;
use sqlx::{PgPool, SqlitePool};
use uuid::Uuid;

use crate::application::app_error::AppResult;
use crate::domain::user::User;

// ============================================================================
// Database Pool Enum
// ============================================================================

#[derive(Clone)]
pub enum DbPool {
    Postgres(PgPool),
    Sqlite(SqlitePool),
}

// ============================================================================
// User Repository Trait
// ============================================================================

/// Trait for user repository operations
/// This trait is implemented by both PostgreSQL and SQLite repositories
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Create a new user in the database
    async fn create_user(&self, username: &str, email: &str, password_hash: &str) -> AppResult<()>;

    /// Get a user by their ID
    async fn get_user_by_id(&self, id: &Uuid) -> AppResult<Option<User>>;

    /// Get a user by their username
    async fn get_user_by_username(&self, username: &str) -> AppResult<Option<User>>;

    /// Update an existing user
    async fn update_user(&self, id: &Uuid, username: &str, email: &str) -> AppResult<()>;

    /// Delete a user by their ID
    async fn delete_user(&self, id: &Uuid) -> AppResult<bool>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::postgres::user::PostgresUserRepository;
    use crate::persistence::sqlite::user::SqliteUserRepository;
    use std::sync::Arc;

    // Helper to create SQLite test repository
    async fn setup_sqlite_repo() -> Arc<dyn UserRepository> {
        let pool = sqlx::SqlitePool::connect(":memory:")
            .await
            .expect("Failed to create in-memory SQLite database");

        // Run migrations
        sqlx::migrate!("./migrations-sqlite")
            .run(&pool)
            .await
            .expect("Failed to run SQLite migrations");

        Arc::new(SqliteUserRepository::new(pool))
    }

    async fn setup_postgres_repo() -> Arc<dyn UserRepository> {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL environment variable must be set for PostgreSQL tests");

        let pool = sqlx::PgPool::connect(&database_url)
            .await
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to connect to PostgreSQL database.\n\
                     DATABASE_URL: {}\n\
                     Error: {}\n\
                     Make sure PostgreSQL is running and the database exists.",
                    database_url, e
                )
            });

        // Clean up existing test data
        sqlx::query("TRUNCATE TABLE users CASCADE")
            .execute(&pool)
            .await
            .expect("Failed to truncate users table");

        Arc::new(PostgresUserRepository::new(pool))
    }

    fn generate_test_username() -> String {
        let uuid_short = Uuid::new_v4()
            .to_string()
            .split('-')
            .next()
            .unwrap()
            .to_string();
        format!("user_{}", uuid_short)
    }

    async fn test_create_and_get_user_impl(repo: Arc<dyn UserRepository>) {
        // Create a user
        let username = generate_test_username();
        let email = format!("{}@example.com", username);
        let password_hash = "hashed_password";

        repo.create_user(&username, &email, password_hash)
            .await
            .expect("Failed to create user");

        // Get user by username
        let user = repo
            .get_user_by_username(&username)
            .await
            .expect("Failed to get user");

        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(user.username, username);
        assert_eq!(user.email, email);
        assert_eq!(user.password_hash, password_hash);
    }

    #[tokio::test]
    async fn test_sqlite_create_and_get_user() {
        let repo = setup_sqlite_repo().await;
        test_create_and_get_user_impl(repo).await;
    }

    #[tokio::test]
    async fn test_postgres_create_and_get_user() {
        let repo = setup_postgres_repo().await;
        test_create_and_get_user_impl(repo).await;
    }
}
