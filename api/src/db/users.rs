use super::*;
use crate::models::{
    business,
    users::{NewUser, RegCredentials, User},
};

pub async fn get_user(email: &str, pool: &PgPool) -> Result<Option<User>, ApiError> {
    Ok(
        query_as!(
            User,
            "SELECT id, first_name, last_name, other_name, email, password_hash FROM users WHERE email = $1",
            email.to_lowercase()
        )
        .fetch_optional(pool)
        .await?
    )
}

pub async fn create_user(pool: &PgPool, user: RegCredentials) -> Result<NewUser, ApiError> {
    Ok(query_as!(
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
    .await?)
}

pub async fn memberships(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<business::MembershipView>, ApiError> {
    Ok(query_as!(
        business::MembershipView,
        r#"SELECT m.business_id, b.name, m.role AS "role: business::Role", m.branch_id
            FROM business_members m JOIN businesses b ON b.id = m.business_id
            WHERE m.user_id = $1"#,
        user_id
    )
    .fetch_all(pool)
    .await?)
}
