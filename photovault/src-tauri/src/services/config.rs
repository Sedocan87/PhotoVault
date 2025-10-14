use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub primary_drive: PathBuf,
    pub backup_drive: Option<PathBuf>,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            primary_drive: PathBuf::new(),
            backup_drive: None,
        }
    }

    pub fn load() -> Result<Self, io::Error> {
        let path = config_path()?;
        if path.exists() {
            let content = fs::read_to_string(path)?;
            serde_json::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        } else {
            Ok(Self::new())
        }
    }

    pub fn save(&self) -> Result<(), io::Error> {
        let path = config_path()?;
        let content = serde_json::to_string_pretty(self).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())
    }
}

fn config_path() -> Result<PathBuf, io::Error> {
    let config_dir = dirs::home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?.join(".photovault");
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }
    Ok(config_dir.join("config.json"))
}