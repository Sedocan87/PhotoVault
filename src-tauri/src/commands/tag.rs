use crate::models::tag::Tag;
use crate::services::tag::TagService;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn add_tag(
    pool: State<'_, SqlitePool>,
    photo_id: i64,
    tag_name: String,
) -> Result<(), String> {
    TagService::add_tag(&pool, photo_id, tag_name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_tags(pool: State<'_, SqlitePool>) -> Result<Vec<Tag>, String> {
    TagService::get_all_tags(&pool)
        .await
        .map_err(|e| e.to_string())
}
