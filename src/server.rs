use axum::{Router, http};
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use sqlx::{Sqlite, migrate::MigrateDatabase, postgres::PgPoolOptions, sqlite::SqlitePoolOptions};
use std::{fs::File, sync::Arc};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

use crate::{
    application::user_service::UserService,
    config::{AppConfig, DatabaseType},
    crypto::Argon2PasswordHasher,
    persistence::{SqlUserRepository, user_repo::DbPool},
    web::{AppState, user_router},
};

// ============================================================================
// Database Connection
// ============================================================================

async fn init_db(config: &AppConfig) -> anyhow::Result<DbPool> {
    let database_url = &config.database_url;

    match config.database_type {
        DatabaseType::Postgres => {
            tracing::info!("Connecting to PostgreSQL database");
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(database_url)
                .await?;

            tracing::info!("Running PostgreSQL migrations");
            sqlx::migrate!("./migrations").run(&pool).await?;

            tracing::info!("Connected to PostgreSQL database");
            Ok(DbPool::Postgres(pool))
        }
        DatabaseType::Sqlite => {
            // Create database if it doesn't exist
            if !Sqlite::database_exists(database_url).await? {
                tracing::info!("Creating SQLite database at: {}", database_url);
                Sqlite::create_database(database_url).await?;
            }

            tracing::info!("Connecting to SQLite database");
            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect(database_url)
                .await?;

            tracing::info!("Running SQLite migrations");
            sqlx::migrate!("./migrations-sqlite").run(&pool).await?;

            tracing::info!("Connected to SQLite database");
            Ok(DbPool::Sqlite(pool))
        }
    }
}

// ============================================================================
// Logging Setup
// ============================================================================

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "clean_architecture=debug,tower_http=debug".into());

    // Console (pretty logs)
    let console_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_level(true)
        .pretty();

    // File (structured JSON logs)
    let file = File::create("app.log").expect("Cannot create log file");
    let json_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(file)
        .with_current_span(true)
        .with_span_list(true);

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .with(json_layer)
        .try_init()
        .ok();
}

// ============================================================================
// Application State Initialization
// ============================================================================

async fn init_app_state() -> anyhow::Result<AppState> {
    let config = AppConfig::from_env();

    // Initialize database
    let pool = init_db(&config).await?;

    // Create repository and service
    let user_repository = Arc::new(SqlUserRepository::new(pool));
    let password_hasher = Arc::new(Argon2PasswordHasher::default());
    let user_service = UserService::new(password_hasher, user_repository);

    Ok(AppState {
        config: Arc::new(config),
        user_service: Arc::new(user_service),
    })
}

// ============================================================================
// Server Creation
// ============================================================================

pub async fn create_app() -> anyhow::Result<Router> {
    init_tracing();

    let app_state = init_app_state().await?;

    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:5173"
                .parse::<http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([http::Method::POST, http::Method::GET])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true);

    let router = Router::new()
        .nest("/api/user", user_router())
        .with_state(app_state)
        .layer(cors)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &http::Request<_>| {
                let request_id = Uuid::new_v4();
                tracing::info_span!(
                    "http-request",
                    method = %request.method(),
                    uri = %request.uri(),
                    version = ?request.version(),
                    request_id = %request_id
                )
            }),
        );

    Ok(router)
}
