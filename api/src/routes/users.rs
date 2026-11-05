use super::*;
use crate::{models::users as user_models, services::auth};

// pub fn create_router() -> Router<AppState> {
//     Router::new().route("/users", post(create_user))
// }

// async fn create_user(
//     State(state): State<AppState>,
//     Json(payload): Json<user_models::RegCredentials>,
// ) -> Result<impl IntoResponse, ApiError> {
//     info!("creating new user");
//     let new_user = auth::create_new_user(payload, state).await?;

//     info!(user = ?new_user, "Created new user");
//     let body = Json(new_user);
//     Ok((StatusCode::CREATED, body))
// }
