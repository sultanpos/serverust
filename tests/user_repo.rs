/// User repository integration tests
///
/// These tests verify that both PostgreSQL and SQLite implementations
/// of the UserRepository trait have identical behavior.
mod common;

use std::sync::Arc;
use uuid::Uuid;

use common::{init_postgres_pool, init_sqlite_pool};
use sultan::persistence::postgres::user::PostgresUserRepository;
use sultan::persistence::sqlite::user::SqliteUserRepository;
use sultan::persistence::user_repo::UserRepository;

// ============================================================================
// Repository Factory Functions
// ============================================================================

/// Create a new SQLite repository instance (uses shared pool)
async fn create_sqlite_repo() -> Arc<dyn UserRepository> {
    let pool = init_sqlite_pool().await;
    Arc::new(SqliteUserRepository::new(pool))
}

/// Create a new PostgreSQL repository instance (uses shared pool)
async fn create_postgres_repo() -> Arc<dyn UserRepository> {
    let pool = init_postgres_pool().await;
    Arc::new(PostgresUserRepository::new(pool))
}

// ============================================================================
// Test Helpers
// ============================================================================

fn generate_test_username() -> String {
    let uuid_short = Uuid::new_v4()
        .to_string()
        .split('-')
        .next()
        .unwrap()
        .to_string();
    format!("user_{}", uuid_short)
}

// ============================================================================
// Shared Test Implementations
// ============================================================================

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

// ============================================================================
// SQLite Tests
// ============================================================================

#[tokio::test]
async fn test_sqlite_create_and_get_user() {
    let repo = create_sqlite_repo().await;
    test_create_and_get_user_impl(repo).await;
}

// ============================================================================
// PostgreSQL Tests
// ============================================================================

#[tokio::test]
async fn test_postgres_create_and_get_user() {
    // Skip test if DATABASE_URL is not set
    if std::env::var("DATABASE_URL").is_err() {
        eprintln!("⚠️  Skipping PostgreSQL test: DATABASE_URL not set");
        eprintln!("   Set DATABASE_URL to run PostgreSQL tests:");
        eprintln!("   export DATABASE_URL=postgres://postgres:postgres@localhost:5432/sultan_test");
        return;
    }

    let repo = create_postgres_repo().await;
    test_create_and_get_user_impl(repo).await;
}
