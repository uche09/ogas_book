use crate::{
    error::ApiError, state::AppState, db::users,
    validations::users::is_strong_password
};
use argon2::{
    password_hash::{phc::PasswordHash, PasswordHasher, PasswordVerifier},
    Argon2,
};
use validator::{Validate, ValidationError, ValidationErrors};

use super::*;

fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    Ok(Argon2::default().hash_password(password.as_bytes())?.to_string())
    
}

fn verify_password(password: &str, hashed: &str) -> bool {
    PasswordHash::new(hashed)
        .map(|hash| Argon2::default().verify_password(password.as_bytes(), &hash).is_ok())
        .unwrap_or(false)
}

#[derive(Deserialize, Validate, Debug)]
pub struct UserCredentials {
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

#[derive(Serialize, sqlx::FromRow, Debug)]
pub struct NewUser {
    pub first_name: String,
    pub email: String,
}

pub async fn create_new_user(mut payload: UserCredentials, state: AppState) -> Result<NewUser, ApiError> {
    payload.validate()?;

    if users::email_already_exist(&payload.email, &state.pg_pool).await? {
        let mut err = ValidationError::new("Duplicate email detected");
        err.message = Some("An account already exist with this email".into());

        let mut val_err = ValidationErrors::new();
        val_err.add("Duplicate email detected", err);
        return Err(ApiError::Validation(val_err))
    }
 
    // The argon2 crate uses a memory heavy password hashing algorithm, making the hashed password difficult
    // and expensive to crack even with the processing speed of a GPU.
    //
    // Hence the hashing process is being offloaded into another async thread so it doesn't block the server
    payload.password = 
        tokio::task::spawn_blocking(move || hash_password(&payload.password.clone()))
        .await
        .map_err(|e| anyhow::anyhow!("Error hashing password: {}", e))?
        .map_err(|e| anyhow::anyhow!("Error hashing password: {}", e))?;

    // No admin can be created on users route
    payload.is_admin = Some(false);

    users::create_user(&state.pg_pool, payload).await
}