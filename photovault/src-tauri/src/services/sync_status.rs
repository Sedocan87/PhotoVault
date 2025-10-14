use crate::db::manager::DatabaseManager;
use crate::services::config::AppConfig;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncStatus {
    pub primary_connected: bool,
    pub backup_connected: bool,
    pub last_sync: Option<DateTime<Utc>>,
    pub is_in_sync: bool,
    pub pending_operations: u32,
}

pub async fn verify_sync_status() -> Result<SyncStatus, String> {
    let config = AppConfig::load().map_err(|e| e.to_string())?;
    let primary_path = config.primary_drive;
    let backup_path = config.backup_drive;

    let primary_connected = primary_path.exists();
    let backup_connected = backup_path.as_ref().map_or(false, |p| p.exists());

    let (is_in_sync, pending_operations, last_sync) = if primary_connected && backup_connected {
        let _manager = DatabaseManager::new(primary_path, backup_path).await.map_err(|e| e.to_string())?;
        // In a real implementation, we would check the database for pending operations
        // and the last sync time. For now, we'll just return some dummy data.
        (true, 0, Some(Utc::now()))
    } else {
        (false, 0, None)
    };

    Ok(SyncStatus {
        primary_connected,
        backup_connected,
        last_sync,
        is_in_sync,
        pending_operations,
    })
}