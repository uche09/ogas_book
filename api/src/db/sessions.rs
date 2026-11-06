use super::*;
use uuid::Uuid;

pub async fn create_new_session(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    refresh_hash: String,
) -> Result<PgQueryResult, ApiError> {
    Ok(query!(
        "INSERT INTO sessions (id, user_id, refresh_hash, expires_at)\
            VALUES ($1, $2, $3, now() + interval '30 days')",
        id,
        user_id,
        refresh_hash
    )
    .execute(pool)
    .await?)
}
