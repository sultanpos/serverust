use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, instrument};

use crate::{
    application::{app_error::AppResult, user_service::UserService},
    web::app_state::AppState,
};

// ============================================================================
// DTOs (Request/Response models)
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: SecretString,
}

#[derive(Debug, Clone, Serialize)]
struct RegisterResponse {
    success: bool,
}

// ============================================================================
// HTTP Handlers
// ============================================================================

/// Register a new user
#[instrument(skip(user_service, payload))]
async fn register(
    State(user_service): State<Arc<UserService>>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<impl IntoResponse> {
    info!("Register endpoint called");
    
    user_service
        .register_user(&payload.username, &payload.email, &payload.password)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse { success: true }),
    ))
}

// ============================================================================
// Router
// ============================================================================

pub fn user_router() -> Router<AppState> {
    Router::new().route("/register", post(register))
}
