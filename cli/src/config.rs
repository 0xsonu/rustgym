use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// CLI configuration stored in ~/.rustgym/config.toml
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CliConfig {
    /// Stored authentication token
    pub token: Option<String>,
    /// API base URL
    pub api_url: Option<String>,
}

impl CliConfig {
    /// Get the path to the config file (~/.rustgym/config.toml)
    pub fn config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let home = dirs::home_dir().ok_or("Could not determine home directory")?;
        Ok(home.join(".rustgym").join("config.toml"))
    }

    /// Load config from disk, returning defaults if file doesn't exist.
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)?;
        let config: CliConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save config to disk, creating parent directories if needed.
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }
}
