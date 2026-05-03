use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Default API URL when none is configured.
const DEFAULT_API_URL: &str = "https://rustgym.dev";

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
    pub fn config_path() -> Result<PathBuf, String> {
        let home = dirs::home_dir()
            .ok_or_else(|| "Could not determine home directory".to_string())?;
        Ok(home.join(".rustgym").join("config.toml"))
    }

    /// Load config from disk, returning defaults if file doesn't exist.
    pub fn load() -> Result<Self, String> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path).map_err(|e| {
            format!(
                "Failed to read config file '{}': {}",
                path.display(),
                e
            )
        })?;
        let config: CliConfig = toml::from_str(&content).map_err(|e| {
            format!(
                "Failed to parse config file '{}': {}",
                path.display(),
                e
            )
        })?;
        Ok(config)
    }

    /// Save config to disk, creating parent directories if needed.
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                format!(
                    "Failed to create config directory '{}': {}",
                    parent.display(),
                    e
                )
            })?;
        }
        let content = toml::to_string_pretty(self).map_err(|e| {
            format!("Failed to serialize config: {}", e)
        })?;
        std::fs::write(&path, content).map_err(|e| {
            format!(
                "Failed to write config file '{}': {}",
                path.display(),
                e
            )
        })?;
        Ok(())
    }

    /// Clear the stored token and persist the change to disk.
    /// Used when a session expires (401 response).
    pub fn clear_token(&mut self) -> Result<(), String> {
        self.token = None;
        self.save()
    }

    /// Resolve the API URL using the priority: CLI override > config value > default.
    ///
    /// - If `cli_override` is `Some` and non-empty, it wins.
    /// - Otherwise, if `self.api_url` is `Some` and non-empty, use that.
    /// - Otherwise, fall back to the default "https://rustgym.dev".
    pub fn resolve_api_url(&self, cli_override: Option<&str>) -> String {
        if let Some(url) = cli_override {
            if !url.is_empty() {
                return url.to_string();
            }
        }

        if let Some(ref url) = self.api_url {
            if !url.is_empty() {
                return url.clone();
            }
        }

        DEFAULT_API_URL.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_api_url_cli_override_wins() {
        let config = CliConfig {
            token: None,
            api_url: Some("https://staging.rustgym.dev".to_string()),
        };
        let result = config.resolve_api_url(Some("https://local.test"));
        assert_eq!(result, "https://local.test");
    }

    #[test]
    fn resolve_api_url_config_value_used_when_no_override() {
        let config = CliConfig {
            token: None,
            api_url: Some("https://staging.rustgym.dev".to_string()),
        };
        let result = config.resolve_api_url(None);
        assert_eq!(result, "https://staging.rustgym.dev");
    }

    #[test]
    fn resolve_api_url_default_when_nothing_set() {
        let config = CliConfig {
            token: None,
            api_url: None,
        };
        let result = config.resolve_api_url(None);
        assert_eq!(result, "https://rustgym.dev");
    }

    #[test]
    fn resolve_api_url_empty_override_falls_through() {
        let config = CliConfig {
            token: None,
            api_url: Some("https://staging.rustgym.dev".to_string()),
        };
        let result = config.resolve_api_url(Some(""));
        assert_eq!(result, "https://staging.rustgym.dev");
    }

    #[test]
    fn resolve_api_url_empty_config_falls_to_default() {
        let config = CliConfig {
            token: None,
            api_url: Some("".to_string()),
        };
        let result = config.resolve_api_url(None);
        assert_eq!(result, "https://rustgym.dev");
    }

    #[test]
    fn clear_token_sets_none() {
        // We can't easily test save() without a real filesystem,
        // but we can verify the token is cleared in memory.
        let mut config = CliConfig {
            token: Some("my-token".to_string()),
            api_url: None,
        };
        // clear_token will attempt to save, which may fail in test env,
        // but the token field should still be None after the call.
        let _ = config.clear_token();
        assert_eq!(config.token, None);
    }

    #[test]
    fn config_path_returns_expected_suffix() {
        // Just verify the path ends with the expected components.
        let path = CliConfig::config_path().unwrap();
        assert!(path.ends_with(".rustgym/config.toml"));
    }
}
