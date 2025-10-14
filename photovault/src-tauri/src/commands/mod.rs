use crate::models::photo::Photo;
use crate::services::file_ops::FileOperationService;
use std::path::PathBuf;

#[tauri::command]
pub async fn scan_library(primary_path: String) -> Result<Vec<Photo>, String> {
    let service = FileOperationService::new(PathBuf::from(primary_path));
    service
        .scan_directory(&service.primary_path.clone())
        .await
        .map_err(|e| e.to_string())
}

use crate::models::operation::Operation;
use crate::services::config::AppConfig;
use crate::services::sync_engine::SyncEngine;
use crate::services::sync_status::{verify_sync_status, SyncStatus};
use tokio::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub sync_engine: Mutex<SyncEngine>,
}

#[tauri::command]
pub async fn get_photos(_limit: i64, _offset: i64) -> Result<Vec<Photo>, String> {
    // This will be implemented in a later phase, once the database is fully integrated.
    Ok(vec![])
}

#[tauri::command]
pub async fn set_drive_paths(primary: String, backup: Option<String>) -> Result<(), String> {
    let mut config = AppConfig::load().map_err(|e| e.to_string())?;
    config.primary_drive = PathBuf::from(primary);
    config.backup_drive = backup.map(PathBuf::from);
    config.save().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_config() -> Result<AppConfig, String> {
    AppConfig::load().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn verify_sync_status_command() -> Result<SyncStatus, String> {
    verify_sync_status().await
}

#[tauri::command]
pub async fn move_photos(
    photo_ids: Vec<i64>,
    target_path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut sync_engine = state.sync_engine.lock().await;
    for photo_id in photo_ids {
        // In a real implementation, we would get the photo path from the database.
        let from_path = PathBuf::from(format!("/fake/path/{}.jpg", photo_id));
        let op = Operation::Move {
            from: from_path,
            to: PathBuf::from(&target_path),
        };
        sync_engine.execute_operation(op).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_photos(photo_ids: Vec<i64>, state: State<'_, AppState>) -> Result<(), String> {
    let mut sync_engine = state.sync_engine.lock().await;
    for photo_id in photo_ids {
        // In a real implementation, we would get the photo path from the database.
        let path = PathBuf::from(format!("/fake/path/{}.jpg", photo_id));
        let op = Operation::Delete { path };
        sync_engine.execute_operation(op).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn rename_photo(
    photo_id: i64,
    new_name: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut sync_engine = state.sync_engine.lock().await;
    // In a real implementation, we would get the photo path from the database.
    let path = PathBuf::from(format!("/fake/path/{}.jpg", photo_id));
    let op = Operation::Rename { path, new_name };
    sync_engine.execute_operation(op).await.map_err(|e| e.to_string())
}