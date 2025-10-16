use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tokio::fs as async_fs;

pub const CONFIG_DIR_NAME: &str = ".photovault";
pub const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AppConfig {
    pub primary_drive: Option<PathBuf>,
    pub backup_drive: Option<PathBuf>,
}

/// Returns the path to the application's config directory.
pub fn get_app_config_dir() -> Result<PathBuf> {
    let home_dir = home::home_dir().context("Failed to get user's home directory")?;
    Ok(home_dir.join(CONFIG_DIR_NAME))
}

/// Asynchronously loads the application configuration from the specified path.
pub async fn load_config_from_path(config_path: &Path) -> Result<AppConfig> {
    if !config_path.exists() {
        return Ok(AppConfig::default());
    }

    let config_str = async_fs::read_to_string(config_path)
        .await
        .context("Failed to read config file")?;

    serde_json::from_str(&config_str).context("Failed to parse config file")
}

/// Asynchronously saves the application configuration to the specified path.
pub async fn save_config_to_path(config: &AppConfig, config_path: &Path) -> Result<()> {
    if let Some(parent) = config_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }
    }

    let config_str = serde_json::to_string_pretty(config).context("Failed to serialize config")?;

    async_fs::write(config_path, config_str)
        .await
        .context("Failed to write to config file")?;

    Ok(())
}