use crate::{
    db::{sessions, users as users_db},
    error::ApiError,
    models::users as user_models,
    services::tokens::{TokenService, UserClaims},
    state::AppState,
    util,
};
use anyhow::Context;

use axum_extra::extract::cookie::CookieJar;
use uuid::Uuid;
use validator::{Validate, ValidationError, ValidationErrors};

const ISS: &str = "ogasbook-auth";
const AUD_USER: &str = "ogasbook-user";

pub async fn create_new_user(
    mut payload: user_models::RegCredentials,
    state: AppState,
) -> Result<user_models::NewUser, ApiError> {
    payload.email = payload.email.trim().to_lowercase();
    payload.first_name = payload.first_name.trim().to_lowercase();
    payload.last_name = payload.last_name.trim().to_lowercase();
    payload.other_name = payload.other_name.map(|s| s.trim().to_lowercase());
    payload.validate()?;

    if users_db::get_user(&payload.email, &state.pg_pool)
        .await?
        .is_some()
    {
        let mut err = ValidationError::new("Duplicate email detected");
        err.message = Some("An account already exist with this email".into());

        let mut val_err = ValidationErrors::new();
        val_err.add("Duplicate email detected", err);
        return Err(ApiError::Validation(val_err));
    }

    // The argon2 crate uses a memory heavy password hashing algorithm, making the hashed password difficult
    // and expensive to crack even with the processing speed of a GPU.
    //
    // Hence the hashing process is being offloaded into another async thread so it doesn't block the server
    payload.password =
        tokio::task::spawn_blocking(move || util::hash_password(&payload.password.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("Error hashing password: {}", e))?
            .map_err(|e| anyhow::anyhow!("Error hashing password: {}", e))?;

    // No admin can be created on users route
    payload.is_admin = Some(false);

    users_db::create_user(&state.pg_pool, payload).await
}

pub async fn user_login(
    mut payload: user_models::LoginCredentials,
    state: AppState,
    jar: CookieJar,
) -> Result<(CookieJar, user_models::LoginResponse), ApiError> {
    payload.email = payload.email.trim().to_lowercase();
    payload.validate()?;

    let user = users_db::get_user(&payload.email, &state.pg_pool).await?;

    let hash = user
        .clone()
        // default to dummy passwaord hash if no user found
        .map_or(state.dummy_hash.to_string(), |u| u.password_hash);

    let password_is_valid =
        tokio::task::spawn_blocking(move || util::verify_password(&payload.password, &hash))
            .await
            .map_err(|e| anyhow::anyhow!("Error verifying password hash: {}", e))?
            .map_err(|e| anyhow::anyhow!("Error verifying password hash: {}", e))?;

    let no_user = user.is_none();
    let user = user.filter(|_| password_is_valid).ok_or_else(|| {
        if no_user {
            ApiError::Unauthorized("Failed login attempt with unknown email".to_string())
        } else {
            ApiError::Unauthorized("Failed login attempt incorrect password".to_string())
        }
    })?;

    let sid = Uuid::new_v4();
    let refresh_hash = TokenService::create_refresh_token()?;
    sessions::create_new_session(&state.pg_pool, sid, user.id, refresh_hash.clone()).await?;

    let memberships = users_db::memberships(&state.pg_pool, user.id).await?;

    let user_token = TokenService::create_token(
        &UserClaims {
            iss: ISS.into(),
            aud: AUD_USER.into(),
            sub: user.id,
            sid,
            exp: util::expiry(300),
        },
        &state.config.jwt_secret,
    )
    .context("Failed to sign user token")?;

    Ok((
        jar.add(util::refresh_cookie(refresh_hash)),
        user_models::LoginResponse {
            user_token,
            memberships,
        },
    ))
}
