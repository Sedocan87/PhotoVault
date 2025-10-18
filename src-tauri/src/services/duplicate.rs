use sha2::{Digest, Sha256};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::models::photo::Photo;
use sqlx::{Pool, Sqlite};
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum DuplicateError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub struct DuplicateDetector;

impl DuplicateDetector {
    pub async fn find_duplicates(
        pool: &Pool<Sqlite>,
    ) -> Result<Vec<DuplicateGroup>, DuplicateError> {
        let mut photos = sqlx::query_as::<_, Photo>("SELECT * FROM photos")
            .fetch_all(pool)
            .await?;

        for photo in &mut photos {
            if photo.file_hash.is_none() {
                let path = Path::new(&photo.path);
                if path.exists() {
                    let hash = Self::hash_file(path).await?;
                    Self::cache_hash(pool, photo.id, hash.clone()).await?;
                    photo.file_hash = Some(hash);
                }
            }
        }

        let mut groups: HashMap<String, Vec<Photo>> = HashMap::new();
        for photo in photos {
            if let Some(hash) = &photo.file_hash {
                groups.entry(hash.clone()).or_default().push(photo);
            }
        }

        let duplicate_groups = groups
            .into_iter()
            .filter(|(_, photos)| photos.len() > 1)
            .map(|(hash, photos)| {
                let size = photos.iter().map(|p| p.file_size.unwrap_or(0)).sum();
                DuplicateGroup { hash, photos, size }
            })
            .collect();

        Ok(duplicate_groups)
    }

    pub async fn hash_file(path: &Path) -> Result<String, std::io::Error> {
        let mut file = File::open(path).await?;
        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024];

        loop {
            let n = file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        let hash = hasher.finalize();
        Ok(hex::encode(hash))
    }

    pub async fn cache_hash(
        pool: &Pool<Sqlite>,
        photo_id: i64,
        hash: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE photos SET file_hash = ? WHERE id = ?")
            .bind(hash)
            .bind(photo_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DuplicateGroup {
    pub hash: String,
    pub photos: Vec<Photo>,
    pub size: i64,
}
