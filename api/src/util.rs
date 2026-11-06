use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};
use axum_extra::extract::cookie::Cookie;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    Ok(Argon2::default()
        .hash_password(password.as_bytes())?
        .to_string())
}

pub fn verify_password(password: &str, hashed: &str) -> Result<bool, anyhow::Error> {
    Ok(PasswordHash::new(hashed).map(|hash| {
        Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok()
    })?)
}

pub fn expiry(secs: u64) -> u64 {
    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        // If the clock drifts before 1970 (UNIX_EPOCH), default to 0 instead of crashing
        .map(|duration| duration.as_secs())
        .unwrap_or(0);

    now_secs.saturating_add(secs)
}

pub fn refresh_cookie(value: String) -> Cookie<'static> {
    Cookie::build(("refresh", value))
        .http_only(true)
        .secure(true)
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .path("/auth")
        .build()
}
