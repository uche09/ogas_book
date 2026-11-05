pub use crate::{error::ApiError, state::AppState};
pub use axum::extract::State;
pub use axum::{Json, http::StatusCode, response::IntoResponse};
pub use axum::{
    Router,
    routing::{get, post},
};
pub use serde_json::json;
pub use tracing::info;

mod auth;
mod users;

pub fn create_router(state: AppState) -> Router {
    let auth = Router::new().nest("/auth", auth::create_router());

    Router::new()
        .route("/health", get(health_check))
        .merge(auth)
        // .merge(users::create_router())
        .with_state(state)
}

pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "Server is running",
    }))
}
