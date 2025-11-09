use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{PgPool, SqlitePool};
use uuid::Uuid;

use crate::{
    application::{
        app_error::{AppError, AppResult},
        user_service::UserRepository,
    },
    domain::user::User,
};

// ============================================================================
// Database Pool Enum
// ============================================================================

#[derive(Clone)]
pub enum DbPool {
    Postgres(PgPool),
    Sqlite(SqlitePool),
}

// ============================================================================
// Universal User Repository (supports both PostgreSQL and SQLite)
// ============================================================================

#[derive(Clone)]
pub struct SqlUserRepository {
    pool: DbPool,
}

impl SqlUserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

// Database model for User - PostgreSQL
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct UserDbPg {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

// Database model for User - SQLite
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct UserDbSqlite {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: String,
}

impl From<UserDbPg> for User {
    fn from(user_db: UserDbPg) -> Self {
        User {
            id: user_db.id,
            username: user_db.username,
            password_hash: user_db.password_hash,
            created_at: user_db.created_at,
        }
    }
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
            password_hash: user_db.password_hash,
            created_at,
        }
    }
}

// Implement the UserRepository trait
#[async_trait]
impl UserRepository for SqlUserRepository {
    async fn create_user(&self, username: &str, email: &str, password_hash: &str) -> AppResult<()> {
        match &self.pool {
            DbPool::Postgres(pool) => {
                let uuid = Uuid::new_v4();
                sqlx::query(
                    "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)"
                )
                .bind(uuid)
                .bind(username)
                .bind(email)
                .bind(password_hash)
                .execute(pool)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
            }
            DbPool::Sqlite(pool) => {
                let uuid = Uuid::new_v4().to_string();
                sqlx::query(
                    "INSERT INTO users (id, username, email, password_hash) VALUES (?, ?, ?, ?)",
                )
                .bind(uuid)
                .bind(username)
                .bind(email)
                .bind(password_hash)
                .execute(pool)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
            }
        }

        Ok(())
    }
}
