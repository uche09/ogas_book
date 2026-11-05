use crate::{config::Config, error::ApiError, util::hash_password};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: PgPool,
    pub config: Arc<Config>,
    pub dummy_hash: Arc<String>,
}

impl AppState {
    pub async fn new(pg_pool: PgPool, config: Config) -> Result<Self, ApiError> {
        let dummy_hash =
            tokio::task::spawn_blocking(move || hash_password("jUsT!a!dUmMy!pAsSw0rD?"))
                .await
                .map_err(|e| anyhow::anyhow!("Error hashing dummy password: {}", e))?
                .map_err(|e| anyhow::anyhow!("Error hashing dummy password: {}", e))?;

        Ok(Self {
            pg_pool,
            config: Arc::new(config),
            dummy_hash: Arc::new(dummy_hash),
        })
    }
}
