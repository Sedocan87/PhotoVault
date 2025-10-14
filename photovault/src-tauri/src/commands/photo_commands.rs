use tauri::State;
use sync_logic::{SyncEngine, Operation};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;

pub struct AppState {
    pub sync_engine: Mutex<SyncEngine>,
}

#[tauri::command]
pub async fn flush_sync_queue(state: State<'_, AppState>) -> Result<(), String> {
    let mut sync_engine = state.sync_engine.lock().await;
    sync_engine.flush_queue().await.map_err(|e| e.to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStatus {
    pub pending_operations: u32,
    pub is_sync_in_progress: bool,
}

#[tauri::command]
pub async fn move_photos(source_paths: Vec<String>, target_path: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut sync_engine = state.sync_engine.lock().await;
    for source_path in source_paths {
        let op = Operation::Move {
            from: PathBuf::from(source_path),
            to: PathBuf::from(&target_path),
        };
        sync_engine.execute_operation(op).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_photos(paths: Vec<String>, state: State<'_, AppState>) -> Result<(), String> {
    let mut sync_engine = state.sync_engine.lock().await;
    for path in paths {
        let op = Operation::Delete {
            path: PathBuf::from(path),
        };
        sync_engine.execute_operation(op).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn rename_photo(path: String, new_name: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut sync_engine = state.sync_engine.lock().await;
    let op = Operation::Rename {
        path: PathBuf::from(path),
        new_name,
    };
    sync_engine.execute_operation(op).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_sync_queue_status(state: State<'_, AppState>) -> Result<QueueStatus, String> {
    let sync_engine = state.sync_engine.lock().await;
    Ok(QueueStatus {
        pending_operations: sync_engine.queue_len() as u32,
        is_sync_in_progress: sync_engine.is_syncing,
    })
}