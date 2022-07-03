use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
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
