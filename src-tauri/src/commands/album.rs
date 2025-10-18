use crate::models::album::Album;
use crate::services::album::AlbumService;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn create_album(state: State<'_, AppState>, name: String) -> Result<(), String> {
    let album_service = AlbumService::new(&state.sync_engine);
    album_service
        .create_album(name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_albums(state: State<'_, AppState>) -> Result<Vec<Album>, String> {
    let album_service = AlbumService::new(&state.sync_engine);
    album_service.get_albums().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_photos_to_album(
    state: State<'_, AppState>,
    photo_ids: Vec<i64>,
    album_id: i64,
) -> Result<(), String> {
    let album_service = AlbumService::new(&state.sync_engine);
    album_service
        .add_photos_to_album(photo_ids, album_id)
        .await
        .map_err(|e| e.to_string())
}

use crate::models::photo::Photo;

#[tauri::command]
pub async fn delete_album(state: State<'_, AppState>, album_id: i64) -> Result<(), String> {
    let album_service = AlbumService::new(&state.sync_engine);
    album_service
        .delete_album(album_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_photos_by_album(
    state: State<'_, AppState>,
    album_id: i64,
) -> Result<Vec<Photo>, String> {
    let album_service = AlbumService::new(&state.sync_engine);
    album_service
        .get_photos_by_album_id(album_id)
        .await
        .map_err(|e| e.to_string())
}
