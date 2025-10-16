use crate::models::album::Album;
use crate::models::operation::Operation;
use crate::services::sync_engine::SyncEngine;
use anyhow::{anyhow, Result};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AlbumService {
    sync_engine: Arc<Mutex<Option<SyncEngine>>>,
}

impl AlbumService {
    pub fn new(sync_engine: Arc<Mutex<Option<SyncEngine>>>) -> Self {
        Self { sync_engine }
    }

    pub async fn create_album(&self, name: String) -> Result<()> {
        let op = Operation::CreateAlbum { name };
        let mut sync_engine = self.sync_engine.lock().await;
        if let Some(sync_engine) = &mut *sync_engine {
            sync_engine.execute_operation(&op).await
        } else {
            Err(anyhow!("Sync engine not initialized"))
        }
    }

    pub async fn add_photos_to_album(&self, photo_ids: Vec<i64>, album_id: i64) -> Result<()> {
        let mut sync_engine = self.sync_engine.lock().await;
        if let Some(sync_engine) = &mut *sync_engine {
            for photo_id in photo_ids {
                let op = Operation::AddToAlbum { photo_id, album_id };
                sync_engine.execute_operation(&op).await?;
            }
            Ok(())
        } else {
            Err(anyhow!("Sync engine not initialized"))
        }
    }

    pub async fn get_albums(&self) -> Result<Vec<Album>> {
        let sync_engine = self.sync_engine.lock().await;
        if let Some(sync_engine) = &*sync_engine {
            let albums = sqlx::query_as::<_, Album>("SELECT * FROM albums")
                .fetch_all(&sync_engine.primary_db)
                .await?;
            Ok(albums)
        } else {
            Err(anyhow!("Sync engine not initialized"))
        }
    }

    pub async fn delete_album(&self, album_id: i64) -> Result<()> {
        let op = Operation::DeleteAlbum { album_id };
        let mut sync_engine = self.sync_engine.lock().await;
        if let Some(sync_engine) = &mut *sync_engine {
            sync_engine.execute_operation(&op).await
        } else {
            Err(anyhow!("Sync engine not initialized"))
        }
    }

    pub async fn get_photos_by_album_id(&self, album_id: i64) -> Result<Vec<crate::models::photo::Photo>> {
        let sync_engine = self.sync_engine.lock().await;
        if let Some(sync_engine) = &*sync_engine {
            sync_engine.get_photos_by_album_id(album_id).await
        } else {
            Err(anyhow!("Sync engine not initialized"))
        }
    }
}