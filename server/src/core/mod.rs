pub mod error;
pub mod response;

use std::sync::Arc;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub addr: std::net::SocketAddr,
}

#[derive(Deserialize, Debug)]
pub struct PGConfig {
    pub host: String,
    pub db: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct JWTConfig {
    pub secret: String,
    pub expires: i64,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub pg: PGConfig,
    pub jwt: JWTConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let cfg = config::Config::builder()
            .add_source(
                config::Environment::default()
                    .try_parsing(true)
                    .separator("__"),
            )
            .build()?;
        cfg.try_deserialize()
    }
}

pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
}

use once_cell::sync::Lazy;

pub static APP_CONFIG: Lazy<Arc<AppConfig>> = Lazy::new(|| {
    dotenv::dotenv().ok();
    Arc::new(AppConfig::from_env().expect("App initialize fail!"))
});
