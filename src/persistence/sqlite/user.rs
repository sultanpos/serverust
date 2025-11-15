use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    application::app_error::{AppError, AppResult},
    domain::user::User,
    persistence::user_repo::UserRepository,
};

// ============================================================================
// SQLite User Repository
// ============================================================================

#[derive(Clone)]
pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

// Database model for User - SQLite
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct UserDbSqlite {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: String,
}

impl From<UserDbSqlite> for User {
    fn from(user_db: UserDbSqlite) -> Self {
        let id = Uuid::parse_str(&user_db.id).unwrap_or_else(|_| Uuid::new_v4());

        let created_at = NaiveDateTime::parse_from_str(&user_db.created_at, "%Y-%m-%d %H:%M:%S%.f")
            .or_else(|_| NaiveDateTime::parse_from_str(&user_db.created_at, "%Y-%m-%d %H:%M:%S"))
            .unwrap_or_else(|_| chrono::Utc::now().naive_utc());

        User {
            id,
            username: user_db.username,
            email: user_db.email,
            password_hash: user_db.password_hash,
            created_at,
        }
    }
}

// Implement the UserRepository trait for SQLite
#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn create_user(&self, username: &str, email: &str, password_hash: &str) -> AppResult<()> {
        let uuid = Uuid::new_v4().to_string();

        sqlx::query("INSERT INTO users (id, username, email, password_hash) VALUES (?, ?, ?, ?)")
            .bind(uuid)
            .bind(username)
            .bind(email)
            .bind(password_hash)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    async fn get_user_by_username(&self, username: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, UserDbSqlite>(
            "SELECT id, username, email, password_hash, created_at FROM users WHERE username = ?",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(user.map(|u| u.into()))
    }
}
