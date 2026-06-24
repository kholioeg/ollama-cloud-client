use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_endpoint: String,
    pub api_key: String,
    pub db_path: String,
    pub log_level: String,
}

impl Config {
    fn expand_tilde(path: &str) -> String {
        if path.starts_with('~') {
            if let Some(home) = dirs::home_dir() {
                let expanded = path.replacen('~', home.to_string_lossy().as_ref(), 1);
                return expanded;
            }
        }
        path.to_string()
    }

    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        let api_endpoint = env::var("OLLAMA_API_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:11434".to_string());

        let api_key = env::var("OLLAMA_API_KEY")
            .map_err(|_| Error::ConfigError("OLLAMA_API_KEY environment variable not set".to_string()))?;

        let db_path = env::var("DB_PATH")
            .unwrap_or_else(|_| {
                let home = dirs::home_dir().expect("Could not determine home directory");
                home.join(".ollama-cloud-client")
                    .join("conversations.db")
                    .to_string_lossy()
                    .to_string()
            });
        
        // Expand tilde in path if present
        let db_path = Self::expand_tilde(&db_path);

        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

        Ok(Config {
            api_endpoint,
            api_key,
            db_path,
            log_level,
        })
    }
}
