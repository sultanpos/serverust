use std::env;
use time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub enum DatabaseType {
    Postgres,
    Sqlite,
}

impl DatabaseType {
    pub fn from_env() -> Self {
        let db_type = env::var("DATABASE_TYPE")
            .unwrap_or_else(|_| "sqlite".to_string())
            .to_lowercase();

        match db_type.as_str() {
            "postgres" | "postgresql" => DatabaseType::Postgres,
            "sqlite" => DatabaseType::Sqlite,
            _ => {
                tracing::warn!("Unknown DATABASE_TYPE '{}', defaulting to sqlite", db_type);
                DatabaseType::Sqlite
            }
        }
    }
}

#[derive(Clone)]
pub struct AppConfig {
    pub jwt_secret: String,
    pub access_token_ttl: Duration,
    pub refresh_token_ttl: Duration,
    pub database_type: DatabaseType,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database_type = DatabaseType::from_env();

        let refresh_token_ttl_days: i64 = env::var("REFRESH_TOKEN_TTL_DAYS")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .expect("REFRESH_TOKEN_TTL_DAYS must be a valid number");

        let access_token_ttl_secs: i64 = env::var("ACCESS_TOKEN_TTL_SECS")
            .unwrap_or_else(|_| "900".to_string())
            .parse()
            .expect("ACCESS_TOKEN_TTL_SECS must be a valid number");

        Self {
            jwt_secret,
            access_token_ttl: Duration::seconds(access_token_ttl_secs),
            refresh_token_ttl: Duration::days(refresh_token_ttl_days),
            database_type,
            database_url,
        }
    }
}
