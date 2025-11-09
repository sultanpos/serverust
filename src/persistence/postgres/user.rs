use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    application::app_error::{AppError, AppResult},
    domain::user::User,
    persistence::user_repo::UserRepository,
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

// Database model for User - PostgreSQL
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct UserDbPg {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

impl From<UserDbPg> for User {
    fn from(user_db: UserDbPg) -> Self {
        User {
            id: user_db.id,
            username: user_db.username,
            email: user_db.email,
            password_hash: user_db.password_hash,
            created_at: user_db.created_at,
        }
    }
}

// Implement the UserRepository trait for PostgreSQL
#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create_user(&self, username: &str, email: &str, password_hash: &str) -> AppResult<()> {
        let uuid = Uuid::new_v4();

        sqlx::query(
            "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)",
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

    async fn get_user_by_id(&self, id: &Uuid) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, UserDbPg>(
            "SELECT id, username, email, password_hash, created_at FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(user.map(|u| u.into()))
    }

    async fn get_user_by_username(&self, username: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, UserDbPg>(
            "SELECT id, username, email, password_hash, created_at FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(user.map(|u| u.into()))
    }

    async fn update_user(&self, id: &Uuid, username: &str, email: &str) -> AppResult<()> {
        let result = sqlx::query("UPDATE users SET username = $1, email = $2 WHERE id = $3")
            .bind(username)
            .bind(email)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }

    async fn delete_user(&self, id: &Uuid) -> AppResult<bool> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    async fn setup_test_db() -> PgPool {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@localhost:5432/sultan_test".to_string()
        });

        PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to test database")
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_create_and_get_user() {
        let pool = setup_test_db().await;
        let repo = PostgresUserRepository::new(pool);

        // Create a user (use short UUID to stay under varchar(50) limit)
        let uuid_short = Uuid::new_v4()
            .to_string()
            .split('-')
            .next()
            .unwrap()
            .to_string();
        let username = format!("user_{}", uuid_short);
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

        // Clean up
        repo.delete_user(&user.id)
            .await
            .expect("Failed to delete user");
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_user_by_id() {
        let pool = setup_test_db().await;
        let repo = PostgresUserRepository::new(pool);

        // Create a user
        let uuid_short = Uuid::new_v4()
            .to_string()
            .split('-')
            .next()
            .unwrap()
            .to_string();
        let username = format!("user_{}", uuid_short);
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

        // Clean up
        repo.delete_user(&user.id)
            .await
            .expect("Failed to delete user");
    }

    #[tokio::test]
    #[ignore]
    async fn test_update_user() {
        let pool = setup_test_db().await;
        let repo = PostgresUserRepository::new(pool);

        // Create a user with short UUID suffix to stay under 50 char limit
        let uuid_short = Uuid::new_v4()
            .to_string()
            .split('-')
            .next()
            .unwrap()
            .to_string();
        let username = format!("user_{}", uuid_short);
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

        // Update user (new_username will be under 50 chars: "upd_user_" + 8 chars = ~17 chars)
        let new_username = format!("upd_{}", username);
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

        // Clean up
        repo.delete_user(&user.id)
            .await
            .expect("Failed to delete user");
    }

    #[tokio::test]
    #[ignore]
    async fn test_delete_user() {
        let pool = setup_test_db().await;
        let repo = PostgresUserRepository::new(pool);

        // Create a user
        let uuid_short = Uuid::new_v4()
            .to_string()
            .split('-')
            .next()
            .unwrap()
            .to_string();
        let username = format!("user_{}", uuid_short);
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
    #[ignore]
    async fn test_delete_nonexistent_user() {
        let pool = setup_test_db().await;
        let repo = PostgresUserRepository::new(pool);

        let fake_id = Uuid::new_v4();
        let deleted = repo
            .delete_user(&fake_id)
            .await
            .expect("Failed to attempt delete");

        assert!(!deleted);
    }

    #[tokio::test]
    #[ignore]
    async fn test_update_nonexistent_user() {
        let pool = setup_test_db().await;
        let repo = PostgresUserRepository::new(pool);

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
}
