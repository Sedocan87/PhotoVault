use crate::services::config::AppConfig;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub primary_connected: bool,
    pub backup_connected: bool,
    pub last_sync: Option<DateTime<Utc>>,
    pub is_in_sync: bool,
    pub pending_operations: u32,
}

impl Default for SyncStatus {
    fn default() -> Self {
        Self {
            primary_connected: false,
            backup_connected: false,
            last_sync: None,
            is_in_sync: false,
            pending_operations: 0,
        }
    }
}

/// Verifies the connection status of the primary and backup drives by checking if the paths exist and are directories.
pub async fn verify_sync_status(config: &AppConfig) -> Result<SyncStatus> {
    let mut status = SyncStatus::default();

    if let Some(primary_path) = &config.primary_drive {
        if primary_path.is_dir() {
            status.primary_connected = true;
        }
    }

    if let Some(backup_path) = &config.backup_drive {
        if backup_path.is_dir() {
            status.backup_connected = true;
        }
    }

    // For Phase 2, is_in_sync is true if both are connected.
    status.is_in_sync = status.primary_connected && status.backup_connected;

    Ok(status)
}