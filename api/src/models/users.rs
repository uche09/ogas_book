use super::*;
use crate::{models::business::MembershipView, validations::users::is_strong_password};
use axum::{Json, response::IntoResponse};
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct RegCredentials {
    #[validate(length(min = 3, max = 70))]
    pub first_name: String,

    #[validate(length(min = 3, max = 70))]
    pub last_name: String,

    #[validate(length(min = 3, max = 70))]
    pub other_name: Option<String>,

    #[validate(email(message = "must be a valid email address"))]
    pub email: String,

    #[validate(custom(function = is_strong_password))]
    pub password: String,
    pub is_admin: Option<bool>,
}

#[derive(Deserialize, Validate, Debug)]
pub struct LoginCredentials {
    #[validate(email(message = "must be a valid email address"))]
    pub email: String,

    #[validate(custom(function = is_strong_password))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user_token: String,
    pub memberships: Vec<MembershipView>,
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[derive(Serialize, sqlx::FromRow, Debug)]
pub struct NewUser {
    pub first_name: String,
    pub email: String,
}

#[derive(Serialize, sqlx::FromRow, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub other_name: Option<String>,
    pub email: String,
    pub password_hash: String,
}
