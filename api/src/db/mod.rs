pub mod sessions;
pub mod users;

pub use crate::error::ApiError;
pub use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgQueryResult},
    query, query_as,
};
pub use uuid::Uuid;

pub async fn make_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(database_url)
        .await
}
