use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    application::{
        app_error::{AppError, AppResult},
        user_service::UserRepository,
    },
    domain::user::User,
};

// ============================================================================
// PostgreSQL User Repository
// ============================================================================

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// Database model for User
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct UserDb {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

impl From<UserDb> for User {
    fn from(user_db: UserDb) -> Self {
        User {
            id: user_db.id,
            username: user_db.username,
            password_hash: user_db.password_hash,
            created_at: user_db.created_at,
        }
    }
}

// Implement the UserRepository trait
#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create_user(&self, username: &str, email: &str, password_hash: &str) -> AppResult<()> {
        let uuid = Uuid::new_v4();

        sqlx::query(
            "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)"
        )
        .bind(uuid)
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}
