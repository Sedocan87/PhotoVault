use crate::models::operation::Operation;
use crate::models::photo::Photo;
use anyhow::Result;
use sqlx::SqlitePool;

pub struct SyncEngine {
    pub primary_db: SqlitePool,
    backup_db: Option<SqlitePool>,
    operation_queue: Vec<Operation>,
}

impl SyncEngine {
    pub fn new(primary_db: SqlitePool, backup_db: Option<SqlitePool>) -> Self {
        Self {
            primary_db,
            backup_db,
            operation_queue: Vec::new(),
        }
    }

    pub async fn execute_operation(&mut self, op: &Operation) -> Result<()> {
        let mut tx = self.primary_db.begin().await?;
        self.execute_on_db(&mut tx, op).await?;

        if let Some(backup_db) = &self.backup_db {
            let mut backup_tx = backup_db.begin().await?;
            let backup_result = self.execute_on_db(&mut backup_tx, op).await;
            if backup_result.is_err() {
                tx.rollback().await?;
                backup_tx.rollback().await?;
                self.operation_queue.push(op.clone());
                return backup_result;
            }
            backup_tx.commit().await?;
        } else {
            self.operation_queue.push(op.clone());
        }
        tx.commit().await?;
        Ok(())
    }

    async fn execute_on_db(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        op: &Operation,
    ) -> Result<()> {
        let op_json = serde_json::to_string(op)?;
        let op_id = sqlx::query!(
            "INSERT INTO sync_operations (operation, status) VALUES (?, 'pending')",
            op_json
        )
        .execute(&mut **tx)
        .await?
        .last_insert_rowid();

        match op {
            Operation::AddPhoto { photo } => {
                sqlx::query!(
                    "INSERT INTO photos (path, filename, file_size, date_taken, width, height, format) VALUES (?, ?, ?, ?, ?, ?, ?)",
                    photo.path,
                    photo.filename,
                    photo.file_size,
                    photo.date_taken,
                    photo.width,
                    photo.height,
                    photo.format
                )
                .execute(&mut **tx)
                .await?;
            }
            Operation::Move { from, to } => {
                let from_str = from.to_str().unwrap();
                let to_str = to.to_str().unwrap();
                sqlx::query!(
                    "UPDATE photos SET path = ? WHERE path = ?",
                    to_str,
                    from_str
                )
                .execute(&mut **tx)
                .await?;
            }
            Operation::Delete { path } => {
                let path_str = path.to_str().unwrap();
                sqlx::query!("DELETE FROM photos WHERE path = ?", path_str)
                    .execute(&mut **tx)
                    .await?;
            }
            Operation::Rename { path, new_name } => {
                let path_str = path.to_str().unwrap();
                sqlx::query!(
                    "UPDATE photos SET filename = ? WHERE path = ?",
                    new_name,
                    path_str
                )
                .execute(&mut **tx)
                .await?;
            }
            Operation::CreateAlbum { name } => {
                sqlx::query!("INSERT INTO albums (name) VALUES (?)", name)
                    .execute(&mut **tx)
                    .await?;
            }
            Operation::DeleteAlbum { album_id } => {
                // First, delete associations in photo_albums
                sqlx::query!("DELETE FROM photo_albums WHERE album_id = ?", album_id)
                    .execute(&mut **tx)
                    .await?;
                // Then, delete the album itself
                sqlx::query!("DELETE FROM albums WHERE id = ?", album_id)
                    .execute(&mut **tx)
                    .await?;
            }
            Operation::AddToAlbum { photo_id, album_id } => {
                sqlx::query!(
                    "INSERT INTO photo_albums (photo_id, album_id) VALUES (?, ?)",
                    photo_id,
                    album_id
                )
                .execute(&mut **tx)
                .await?;
            }
            Operation::AddTag { photo_id, tag_name } => {
                let tag = sqlx::query!("SELECT id FROM tags WHERE name = ?", tag_name)
                    .fetch_optional(&mut **tx)
                    .await?;
                let tag_id = match tag {
                    Some(tag) => tag.id.unwrap(),
                    None => sqlx::query!("INSERT INTO tags (name) VALUES (?)", tag_name)
                        .execute(&mut **tx)
                        .await?
                        .last_insert_rowid(),
                };
                sqlx::query!(
                    "INSERT INTO photo_tags (photo_id, tag_id) VALUES (?, ?)",
                    photo_id,
                    tag_id
                )
                .execute(&mut **tx)
                .await?;
            }
        }
        sqlx::query!(
            "UPDATE sync_operations SET status = 'completed' WHERE id = ?",
            op_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    pub async fn flush_queue(&mut self) -> Result<()> {
        if self.backup_db.is_none() {
            return Ok(());
        }

        let ops_to_flush = self.operation_queue.clone();
        self.operation_queue.clear();

        for op in ops_to_flush {
            let mut backup_tx = self.backup_db.as_mut().unwrap().begin().await?;
            if self.execute_on_db(&mut backup_tx, &op).await.is_err() {
                self.operation_queue.push(op);
                backup_tx.rollback().await?;
            } else {
                backup_tx.commit().await?;
            }
        }
        Ok(())
    }

    pub fn queue_len(&self) -> usize {
        self.operation_queue.len()
    }

    pub async fn get_photo_by_id(&self, photo_id: i64) -> Result<Photo> {
        let photo = sqlx::query_as::<_, Photo>("SELECT * FROM photos WHERE id = ?")
            .bind(photo_id)
            .fetch_one(&self.primary_db)
            .await?;
        Ok(photo)
    }

    pub async fn get_photos(&self, limit: i64, offset: i64) -> Result<Vec<Photo>> {
        let photos = sqlx::query_as::<_, Photo>("SELECT * FROM photos LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.primary_db)
            .await?;
        Ok(photos)
    }

    pub async fn get_photos_by_album_id(&self, album_id: i64) -> Result<Vec<Photo>> {
        let photos = sqlx::query_as::<_, Photo>(
            "SELECT p.* FROM photos p JOIN photo_albums pa ON p.id = pa.photo_id WHERE pa.album_id = ?",
        )
        .bind(album_id)
        .fetch_all(&self.primary_db)
        .await?;
        Ok(photos)
    }

    pub async fn add_photo(&mut self, photo: Photo) -> Result<()> {
        let op = Operation::AddPhoto { photo };
        self.execute_operation(&op).await
    }

    pub async fn scan_library(&mut self, library_path: &str) -> Result<()> {
        for entry in walkdir::WalkDir::new(library_path) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let path = entry.path();
                let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                if ["jpg", "jpeg", "png", "gif"].contains(&extension.to_lowercase().as_str()) {
                    let path_str = path.to_str().unwrap().to_string();
                    let existing_photo = sqlx::query("SELECT id FROM photos WHERE path = ?")
                        .bind(&path_str)
                        .fetch_optional(&self.primary_db)
                        .await?;

                    if existing_photo.is_none() {
                        let photo = Photo::new_from_path(path.to_path_buf())?;
                        self.add_photo(photo).await?;
                    }
                }
            }
        }
        Ok(())
    }
}
