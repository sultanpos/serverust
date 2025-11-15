/// Shared test utilities for all integration tests
///
/// This module provides database pool initialization that can be reused
/// across all repository test files. This is the idiomatic Rust approach
/// for shared test utilities.
use once_cell::sync::Lazy;
use sqlx::{PgPool, SqlitePool};
use tokio::sync::Mutex;

// ============================================================================
// Shared Database Pools - Created Once for All Tests
// ============================================================================

/// SQLite pool - created once and reused across all tests
pub static SQLITE_POOL: Lazy<Mutex<Option<SqlitePool>>> = Lazy::new(|| Mutex::new(None));

/// PostgreSQL pool - created once and reused across all tests
pub static POSTGRES_POOL: Lazy<Mutex<Option<PgPool>>> = Lazy::new(|| Mutex::new(None));

// ============================================================================
// Pool Initialization Functions
// ============================================================================

/// Initialize SQLite pool (runs once per test session)
///
/// This function creates an in-memory SQLite database and runs migrations.
/// Subsequent calls return the same pool instance.
///
/// # Example
/// ```ignore
/// use common::init_sqlite_pool;
///
/// #[tokio::test]
/// async fn test_my_feature() {
///     let pool = init_sqlite_pool().await;
///     let repo = MyRepository::new(pool);
/// }
/// ```
pub async fn init_sqlite_pool() -> SqlitePool {
    let mut pool = SQLITE_POOL.lock().await;

    if let Some(existing_pool) = pool.as_ref() {
        return existing_pool.clone();
    }

    let new_pool = sqlx::SqlitePool::connect(":memory:")
        .await
        .expect("Failed to create in-memory SQLite database");

    // Run migrations
    sqlx::migrate!("./migrations-sqlite")
        .run(&new_pool)
        .await
        .expect("Failed to run SQLite migrations");

    *pool = Some(new_pool.clone());
    new_pool
}

/// Initialize PostgreSQL pool (runs once per test session)
///
/// This function connects to a PostgreSQL database and runs migrations.
/// Requires `DATABASE_URL` environment variable to be set.
/// Subsequent calls return the same pool instance.
///
/// # Environment Variables
/// - `DATABASE_URL`: PostgreSQL connection string
///   Example: `postgres://postgres:postgres@localhost:5432/sultan_test`
///
/// # Example
/// ```ignore
/// use common::init_postgres_pool;
///
/// #[tokio::test]
/// async fn test_my_feature() {
///     let pool = init_postgres_pool().await;
///     let repo = MyRepository::new(pool);
/// }
/// ```
pub async fn init_postgres_pool() -> PgPool {
    let mut pool = POSTGRES_POOL.lock().await;

    if let Some(existing_pool) = pool.as_ref() {
        return existing_pool.clone();
    }

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set for PostgreSQL tests");

    let new_pool = sqlx::PgPool::connect(&database_url)
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

    // Run migrations once
    sqlx::migrate!("./migrations")
        .run(&new_pool)
        .await
        .expect("Failed to run PostgreSQL migrations");

    *pool = Some(new_pool.clone());
    new_pool
}
