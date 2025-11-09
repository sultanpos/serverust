use async_trait::async_trait;
use sqlx::{PgPool, SqlitePool};

use crate::application::app_error::AppResult;

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
    
    // TODO: Add more methods as needed:
    // async fn get_user_by_id(&self, id: &Uuid) -> AppResult<Option<User>>;
    // async fn get_user_by_username(&self, username: &str) -> AppResult<Option<User>>;
    // async fn update_user(&self, user: &User) -> AppResult<()>;
    // async fn delete_user(&self, id: &Uuid) -> AppResult<()>;
}