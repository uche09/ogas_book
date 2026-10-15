use super::*;
use crate::{ services::users};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/users", post(create_user))
        .with_state(state)
}



async fn create_user(State(state): State<AppState> ,Json(payload): Json<users::UserCredentials>) -> Result<impl IntoResponse, ApiError> {
    info!("creating new user");
    let new_user = users::create_new_user(payload, state).await?;

    info!(user = ?new_user, "Created new user");
    let body = Json(new_user);
    Ok((StatusCode::CREATED, body))
}