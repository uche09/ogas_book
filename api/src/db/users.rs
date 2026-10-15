use super::PgPool;
use crate::{error::ApiError, services::users::{NewUser, UserCredentials}};
use sqlx::{query, query_as};

pub async fn email_already_exist(email: &str, pool: &PgPool) -> Result<bool, ApiError> {
    Ok(
        query!(
            "SELECT id FROM users WHERE email = $1",
            email.to_lowercase()
        )
        .fetch_optional(pool)
        .await?
        .is_some()
    )
}

pub async fn create_user(
    pool: &PgPool,
    user: UserCredentials,
) -> Result<NewUser, ApiError> {
    Ok(
        query_as!(
            NewUser,
            "INSERT INTO users \
            (first_name, last_name, other_name, email, password_hash, is_admin) \
            VALUES ($1, $2, $3, $4, $5, $6) RETURNING first_name, email as \"email!\"",
            user.first_name,
            user.last_name,
            user.other_name,
            user.email,
            user.password,
            user.is_admin
        )
        .fetch_one(pool)
        .await?
    )
}
