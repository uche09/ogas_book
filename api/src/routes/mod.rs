use axum::{Router, routing::get};
pub use axum::{Json, response::IntoResponse};
pub use serde_json::json;


pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
}

pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "Server is running",
    }))
}