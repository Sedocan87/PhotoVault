use crate::models::photo::Photo;
use crate::services::filter::{filter_photos, search_photos, FilterCriteria};
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn filter_photos_command(
    pool: State<'_, SqlitePool>,
    criteria: FilterCriteria,
) -> Result<Vec<Photo>, String> {
    filter_photos(&pool, criteria)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_photos_command(
    pool: State<'_, SqlitePool>,
    query: String,
) -> Result<Vec<Photo>, String> {
    search_photos(&pool, query).await.map_err(|e| e.to_string())
}
