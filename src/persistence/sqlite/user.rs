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

    async fn get_user_by_id(&self, id: &Uuid) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, UserDbSqlite>("SELECT id, username, email, password_hash, created_at FROM users WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(user.map(|u| u.into()))
    }

    async fn get_user_by_username(&self, username: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, UserDbSqlite>("SELECT id, username, email, password_hash, created_at FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(user.map(|u| u.into()))
    }

    async fn update_user(&self, id: &Uuid, username: &str, email: &str) -> AppResult<()> {
        let result = sqlx::query(
            "UPDATE users SET username = ?, email = ? WHERE id = ?",
        )
        .bind(username)
        .bind(email)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }

    async fn delete_user(&self, id: &Uuid) -> AppResult<bool> {
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:")
            .await
            .expect("Failed to create in-memory database");

        // Create users table
        sqlx::query(
            r#"
            CREATE TABLE users (
                id TEXT PRIMARY KEY,
                username TEXT UNIQUE NOT NULL,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&pool)
        .await
        .expect("Failed to create users table");

        pool
    }

    #[tokio::test]
    async fn test_create_and_get_user() {
        let pool = setup_test_db().await;
        let repo = SqliteUserRepository::new(pool);

        // Create a user
        let username = format!("test_user_{}", Uuid::new_v4());
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
    async fn test_get_user_by_id() {
        let pool = setup_test_db().await;
        let repo = SqliteUserRepository::new(pool);

        // Create a user
        let username = format!("test_user_{}", Uuid::new_v4());
        let email = format!("{}@example.com", username);
        let password_hash = "hashed_password";

        repo.create_user(&username, &email, password_hash)
            .await
            .expect("Failed to create user");

        // Get user by username to get the ID
        let user = repo
            .get_user_by_username(&username)
            .await
            .expect("Failed to get user")
            .expect("User not found");

        // Get user by ID
        let user_by_id = repo
            .get_user_by_id(&user.id)
            .await
            .expect("Failed to get user by ID");

        assert!(user_by_id.is_some());
        let user_by_id = user_by_id.unwrap();
        assert_eq!(user_by_id.id, user.id);
        assert_eq!(user_by_id.username, username);
    }

    #[tokio::test]
    async fn test_update_user() {
        let pool = setup_test_db().await;
        let repo = SqliteUserRepository::new(pool);

        // Create a user
        let username = format!("test_user_{}", Uuid::new_v4());
        let email = format!("{}@example.com", username);
        let password_hash = "hashed_password";

        repo.create_user(&username, &email, password_hash)
            .await
            .expect("Failed to create user");

        // Get user
        let user = repo
            .get_user_by_username(&username)
            .await
            .expect("Failed to get user")
            .expect("User not found");

        // Update user
        let new_username = format!("updated_{}", username);
        let new_email = format!("{}@example.com", new_username);

        repo.update_user(&user.id, &new_username, &new_email)
            .await
            .expect("Failed to update user");

        // Verify update
        let updated_user = repo
            .get_user_by_id(&user.id)
            .await
            .expect("Failed to get updated user")
            .expect("User not found");

        assert_eq!(updated_user.username, new_username);
        assert_eq!(updated_user.email, new_email);
        assert_eq!(updated_user.password_hash, password_hash); // Password should remain unchanged
    }

    #[tokio::test]
    async fn test_delete_user() {
        let pool = setup_test_db().await;
        let repo = SqliteUserRepository::new(pool);

        // Create a user
        let username = format!("test_user_{}", Uuid::new_v4());
        let email = format!("{}@example.com", username);
        let password_hash = "hashed_password";

        repo.create_user(&username, &email, password_hash)
            .await
            .expect("Failed to create user");

        // Get user
        let user = repo
            .get_user_by_username(&username)
            .await
            .expect("Failed to get user")
            .expect("User not found");

        // Delete user
        let deleted = repo
            .delete_user(&user.id)
            .await
            .expect("Failed to delete user");

        assert!(deleted);

        // Verify deletion
        let user_after_delete = repo
            .get_user_by_id(&user.id)
            .await
            .expect("Failed to query for deleted user");

        assert!(user_after_delete.is_none());
    }

    #[tokio::test]
    async fn test_delete_nonexistent_user() {
        let pool = setup_test_db().await;
        let repo = SqliteUserRepository::new(pool);

        let fake_id = Uuid::new_v4();
        let deleted = repo
            .delete_user(&fake_id)
            .await
            .expect("Failed to attempt delete");

        assert!(!deleted);
    }

    #[tokio::test]
    async fn test_update_nonexistent_user() {
        let pool = setup_test_db().await;
        let repo = SqliteUserRepository::new(pool);

        let fake_id = Uuid::new_v4();
        let result = repo
            .update_user(&fake_id, "new_username", "new@example.com")
            .await;

        assert!(result.is_err());
        match result {
            Err(AppError::NotFound(_)) => (),
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_get_nonexistent_user_by_username() {
        let pool = setup_test_db().await;
        let repo = SqliteUserRepository::new(pool);

        let user = repo
            .get_user_by_username("nonexistent_user")
            .await
            .expect("Failed to query for nonexistent user");

        assert!(user.is_none());
    }

    #[tokio::test]
    async fn test_get_nonexistent_user_by_id() {
        let pool = setup_test_db().await;
        let repo = SqliteUserRepository::new(pool);

        let fake_id = Uuid::new_v4();
        let user = repo
            .get_user_by_id(&fake_id)
            .await
            .expect("Failed to query for nonexistent user");

        assert!(user.is_none());
    }
}
