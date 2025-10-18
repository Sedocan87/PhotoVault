use crate::{
    models::operation::Operation,
    services::duplicate::{DuplicateDetector, DuplicateGroup},
    AppState,
};
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub async fn find_duplicates(state: State<'_, AppState>) -> Result<Vec<DuplicateGroup>, String> {
    let db_pool = state.db_pool.lock().await;
    let pool = db_pool.as_ref().ok_or("Database not connected")?;
    DuplicateDetector::find_duplicates(pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_duplicates(
    photo_ids: Vec<i64>,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    let mut sync_engine_lock = state.sync_engine.lock().await;
    let sync_engine = sync_engine_lock
        .as_mut()
        .ok_or("Sync engine not initialized")?;

    let mut total_space_freed = 0;

    for photo_id in photo_ids {
        let photo = sync_engine
            .get_photo_by_id(photo_id)
            .await
            .map_err(|e| e.to_string())?;
        let path = PathBuf::from(&photo.path);
        total_space_freed += photo.file_size.unwrap_or(0);
        let op = Operation::Delete { path };
        sync_engine
            .execute_operation(&op)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(total_space_freed)
}
