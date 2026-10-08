use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub postgres_user: String,
    pub postgres_port: u16,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        Ok(Self {
            port: env::var("API_PORT").unwrap_or("3000".into()).parse()?,
            postgres_user: env::var("POSTGRES_USER").unwrap_or("postgres".into()),
            postgres_port: env::var("POSTGRES_PORT").unwrap_or("5432".into()).parse()?,
        })
    }
}
