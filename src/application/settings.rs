use config::{Config, ConfigError, Environment};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::env;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::from_env().expect("Failed to setup settings");
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Server {
    pub port: u16,
}

impl Default for Server {
    fn default() -> Self {
        Self { port: 3000 }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Logger {
    pub level: String,
    pub sql_explain: String,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            level: "debug".to_string(),
            sql_explain: "off".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Database {
    pub url: String,
    pub max_connections: u32,
    pub password: String,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            url: "postgres://postgres:password@localhost:5430/develop_db".to_string(),
            max_connections: 4,
            password: "default_password".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Auth {
    pub secret: String,
}

impl Default for Auth {
    fn default() -> Self {
        Self {
            secret: "default_secret".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub run_mode: String,
    pub auth: Auth,
    pub database: Database,
    pub server: Server,
    pub logger: Logger,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            run_mode: "develop".to_string(),
            auth: Auth::default(),
            database: Database::default(),
            server: Server::default(),
            logger: Logger::default(),
        }
    }
}

impl Settings {
    pub fn from_env() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "develop".to_string());
        tracing::debug!("Loading environment file for run mode: {}", run_mode);

        let config = Config::builder()
            .add_source(
                Environment::default()
                    .separator("__")
                    .try_parsing(true)
                    .ignore_empty(true),
            )
            .build()?;

        match config.try_deserialize() {
            Ok(settings) => {
                tracing::info!("Successfully loaded settings");
                Ok(settings)
            }
            Err(e) => {
                tracing::error!("Failed to deserialize settings: {}", e);
                Err(e)
            }
        }
    }
}
