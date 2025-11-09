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
