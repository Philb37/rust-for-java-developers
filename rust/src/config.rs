use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerInfo,
    pub postgres: PostgreSQLInfo
}

#[derive(Serialize, Deserialize)]
pub struct ServerInfo {
    pub port: u16
}

#[derive(Serialize, Deserialize)]
pub struct PostgreSQLInfo {
    pub url: String,
    pub schema: String,
    pub pool: PoolConfig
}

#[derive(Serialize, Deserialize)]
pub struct PoolConfig {
    pub max_size: usize,
    pub acquire_timeout_secs: u64,
    pub create_timeout_secs: u64
}

impl AppConfig {
    pub fn build() -> Result<AppConfig, ConfigError> {

        let config = Config::builder()
            .add_source(File::with_name("config.yml"))
            // TALK : Enables checking env variables in a real environment (not local), for example APP_SERVER__PORT would map to server.port.
            .add_source(
                config::Environment::with_prefix("APP")  // only APP_* vars are read
                    .prefix_separator("_") 
                    .separator("__")                     // "__" maps to the yaml nesting dot
            )
            .build()?;

        let app_config = config
            .try_deserialize()?;

        Ok(app_config)
    }
}