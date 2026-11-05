use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub db_url: String,
    pub jwt_secret: Vec<u8>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        Ok(Self {
            port: env::var("API_PORT").unwrap_or("3000".into()).parse()?,
            db_url: env::var("DATABASE_URL").expect("Add \"DATABASE_URL\" to env"),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or("SeCrEt".to_string())
                .as_bytes()
                .to_owned(),
        })
    }
}
