pub use axum::{Json, response::IntoResponse, http::StatusCode};
pub use axum::{Router, routing::{get, post}};
pub use serde_json::json;
pub use crate::{error::ApiError, state::AppState};
pub use axum::extract::State;
pub use tracing::info;

mod users;

pub fn create_router(state: AppState) -> Router {
    Router::new().route("/health", get(health_check))
    .merge(users::create_router(state))
}

pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "Server is running",
    }))
}
