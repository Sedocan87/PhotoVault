use sqlx::{SqlitePool, Error};
use anyhow::Result;
use tokio::fs;

pub mod models;
use models::operation::Operation;

use std::path::PathBuf;

pub struct SyncEngine {
    pub primary_db: SqlitePool,
    pub backup_db: Option<SqlitePool>,
    pub operation_queue: Vec<Operation>,
    pub is_syncing: bool,
    primary_path: PathBuf,
    backup_path: Option<PathBuf>,
}

impl SyncEngine {
    pub async fn new(primary_db: SqlitePool, backup_db: Option<SqlitePool>, primary_path: PathBuf, backup_path: Option<PathBuf>) -> Self {
        Self {
            primary_db,
            backup_db,
            operation_queue: Vec::new(),
            is_syncing: false,
            primary_path,
            backup_path,
        }
    }

    pub fn queue_len(&self) -> usize {
        self.operation_queue.len()
    }

    pub async fn execute_operation(&mut self, op: Operation) -> Result<()> {
        self.is_syncing = true;
        // Log operation to journal
        self.log_operation(&op, "pending").await?;

        match self.execute_on_both(&op).await {
            Ok(_) => {
                self.log_operation(&op, "completed").await?;
                self.is_syncing = false;
                Ok(())
            }
            Err(e) => {
                self.log_operation(&op, "failed").await?;
                // If backup is disconnected, queue the operation
                if self.backup_db.is_none() {
                    self.handle_backup_disconnected(op).await?;
                }
                self.is_syncing = false;
                Err(e.into())
            }
        }
    }

    pub async fn execute_on_both(&mut self, op: &Operation) -> Result<(), Error> {
        // In a real scenario, you would perform the operation on the primary filesystem and database,
        // and then on the backup filesystem and database.
        // For now, we'll just simulate this with database queries.
        println!("Executing operation on primary: {:?}", op);
        // Placeholder for primary execution
        sqlx::query("SELECT 1").execute(&self.primary_db).await?;


        // In a real scenario, you would perform the operation on the primary filesystem and database,
        // and then on the backup filesystem and database.
        self.execute_fs_operation(op, Some(&self.primary_path)).await.map_err(|e| sqlx::Error::Io(e))?;

        if let Some(backup_db) = &self.backup_db {
            println!("Executing operation on backup: {:?}", op);
            self.execute_fs_operation(op, self.backup_path.as_deref()).await.map_err(|e| sqlx::Error::Io(e))?;
            self.log_operation_to_db(op, "completed", backup_db).await?;
        } else {
            // This is where we would detect the backup is disconnected.
            return Err(Error::Io(std::io::Error::new(std::io::ErrorKind::NotConnected, "Backup drive not connected")));
        }

        Ok(())
    }

    pub async fn handle_backup_disconnected(&mut self, op: Operation) -> Result<()> {
        println!("Backup disconnected, queueing operation: {:?}", op);
        self.operation_queue.push(op);
        Ok(())
    }

    pub async fn flush_queue(&mut self) -> Result<()> {
        if self.backup_db.is_some() {
            self.is_syncing = true;
            println!("Flushing operation queue...");
            let ops_to_flush: Vec<Operation> = self.operation_queue.drain(..).collect();
            let mut still_failed = Vec::new();

            for op in ops_to_flush {
                if self.execute_on_backup(&op).await.is_err() {
                    still_failed.push(op);
                }
            }
            self.operation_queue = still_failed;
            println!("Queue flushed.");
            self.is_syncing = false;
        }
        Ok(())
    }

    async fn execute_on_backup(&self, op: &Operation) -> Result<(), Error> {
        if let Some(backup_db) = &self.backup_db {
            println!("Executing operation on backup from queue: {:?}", op);
            self.execute_fs_operation(op, self.backup_path.as_deref()).await.map_err(|e| sqlx::Error::Io(e))?;
            self.log_operation_to_db(op, "completed", backup_db).await?;
            Ok(())
        } else {
            Err(Error::Io(std::io::Error::new(std::io::ErrorKind::NotConnected, "Backup drive not connected")))
        }
    }

    async fn log_operation(&self, op: &Operation, status: &str) -> Result<()> {
        self.log_operation_to_db(op, status, &self.primary_db).await.map_err(|e| e.into())
    }

    async fn log_operation_to_db(&self, op: &Operation, status: &str, db: &SqlitePool) -> Result<(), sqlx::Error> {
        let op_type = match op {
            Operation::Move { .. } => "Move",
            Operation::Delete { .. } => "Delete",
            Operation::Rename { .. } => "Rename",
            Operation::CreateAlbum { .. } => "CreateAlbum",
            Operation::AddToAlbum { .. } => "AddToAlbum",
            Operation::AddTag { .. } => "AddTag",
        };

        let params = serde_json::to_string(op).map_err(|e| sqlx::Error::Decode(e.into()))?;
        let id = uuid::Uuid::new_v4().to_string();

        sqlx::query!(
            "INSERT INTO sync_operations (id, operation_type, params, status) VALUES (?, ?, ?, ?)",
            id,
            op_type,
            params,
            status
        )
        .execute(db)
        .await?;

        Ok(())
    }

    async fn execute_fs_operation(&self, op: &Operation, base_path: Option<&std::path::Path>) -> std::io::Result<()> {
        let base_path = base_path.unwrap_or_else(|| std::path::Path::new("/"));

        match op {
            Operation::Move { from, to } => {
                let from = base_path.join(from);
                let to = base_path.join(to);
                let dest_file = to.join(from.file_name().unwrap());
                fs::rename(from, dest_file).await?;
            }
            Operation::Delete { path } => {
                let path = base_path.join(path);
                fs::remove_file(path).await?;
            }
            Operation::Rename { path, new_name } => {
                let path = base_path.join(path);
                let new_path = path.with_file_name(new_name);
                fs::rename(path, new_path).await?;
            }
            // Other operations are database-only
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::{SqlitePoolOptions, SqlitePool};
    use std::path::PathBuf;

    async fn setup_test_dbs() -> (SqlitePool, SqlitePool) {
        let primary_db = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create primary pool.");
        let backup_db = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create backup pool.");

        sqlx::migrate!("../migrations")
            .run(&primary_db)
            .await
            .expect("Failed to run migrations on primary db");
        sqlx::migrate!("../migrations")
            .run(&backup_db)
            .await
            .expect("Failed to run migrations on backup db");

        (primary_db, backup_db)
    }

    #[tokio::test]
    async fn test_execute_operation_successfully() {
        let (primary_db, backup_db) = setup_test_dbs().await;
        let mut engine = SyncEngine::new(primary_db, Some(backup_db), PathBuf::from("/"), Some(PathBuf::from("/"))).await;

        let op = Operation::CreateAlbum {
            name: "Test Album".to_string(),
        };

        let result = engine.execute_operation(op).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_backup_disconnected_queues_operation() {
        let (primary_db, _) = setup_test_dbs().await;
        let mut engine = SyncEngine::new(primary_db, None, PathBuf::from("/"), None).await;

        let op = Operation::CreateAlbum {
            name: "Test Album".to_string(),
        };

        let result = engine.execute_operation(op).await;
        assert!(result.is_err()); // Should fail because backup is disconnected
        assert_eq!(engine.queue_len(), 1);
    }

    #[tokio::test]
    async fn test_flush_queue_on_reconnect() {
        let (primary_db, backup_db) = setup_test_dbs().await;

        // 1. Start with backup disconnected and queue an operation
        let mut engine = SyncEngine::new(primary_db.clone(), None, PathBuf::from("/"), None).await;
        let op = Operation::CreateAlbum { name: "Test Album".to_string() };
        let _ = engine.execute_operation(op).await;
        assert_eq!(engine.queue_len(), 1);

        // 2. "Reconnect" the backup and flush the queue
        let mut engine_reconnected = SyncEngine::new(primary_db, Some(backup_db), PathBuf::from("/"), Some(PathBuf::from("/"))).await;
        engine_reconnected.operation_queue = engine.operation_queue; // Transfer the queue

        let flush_result = engine_reconnected.flush_queue().await;
        assert!(flush_result.is_ok());
        assert_eq!(engine_reconnected.queue_len(), 0);
    }

    #[tokio::test]
    async fn test_integration_sequence_of_operations() {
        let (primary_db, backup_db) = setup_test_dbs().await;

        // Create dummy files and directories for testing
        let primary_temp_dir = tempfile::tempdir().unwrap();
        let backup_temp_dir = tempfile::tempdir().unwrap();

        let primary_import_dir = primary_temp_dir.path().join("import");
        let primary_photos_dir = primary_temp_dir.path().join("photos");
        fs::create_dir_all(&primary_import_dir).await.unwrap();
        fs::create_dir_all(&primary_photos_dir).await.unwrap();
        let primary_photo1_path = primary_import_dir.join("photo1.jpg");
        fs::write(&primary_photo1_path, b"test data").await.unwrap();

        let backup_import_dir = backup_temp_dir.path().join("import");
        let backup_photos_dir = backup_temp_dir.path().join("photos");
        fs::create_dir_all(&backup_import_dir).await.unwrap();
        fs::create_dir_all(&backup_photos_dir).await.unwrap();
        let backup_photo1_path = backup_import_dir.join("photo1.jpg");
        fs::write(&backup_photo1_path, b"test data").await.unwrap();

        let mut engine = SyncEngine::new(primary_db, Some(backup_db.clone()), primary_temp_dir.path().to_path_buf(), Some(backup_temp_dir.path().to_path_buf())).await;


        // 1. Create an album
        let op1 = Operation::CreateAlbum { name: "Vacation".to_string() };
        assert!(engine.execute_operation(op1).await.is_ok());

        // 2. Add a photo (simulated)
        let op2 = Operation::Move { from: PathBuf::from("import/photo1.jpg"), to: PathBuf::from("photos") };
        assert!(engine.execute_operation(op2).await.is_ok());

        // 3. Rename the photo
        let op3 = Operation::Rename { path: PathBuf::from("photos/photo1.jpg"), new_name: "photo_renamed.jpg".to_string() };
         assert!(engine.execute_operation(op3).await.is_ok());
         assert!(!primary_photos_dir.join("photo1.jpg").exists());
         assert!(primary_photos_dir.join("photo_renamed.jpg").exists());
         assert!(!backup_photos_dir.join("photo1.jpg").exists());
         assert!(backup_photos_dir.join("photo_renamed.jpg").exists());


        // 4. Delete the photo
        let op4 = Operation::Delete { path: PathBuf::from("photos/photo_renamed.jpg") };
        assert!(engine.execute_operation(op4).await.is_ok());
        assert!(!primary_photos_dir.join("photo_renamed.jpg").exists());
        assert!(!backup_photos_dir.join("photo_renamed.jpg").exists());


        // Verify the operation log on the primary db
        let primary_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sync_operations WHERE status = 'completed'")
            .fetch_one(&engine.primary_db)
            .await
            .unwrap();
        assert_eq!(primary_count.0, 4);

        // Verify the operation log on the backup db
        let backup_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sync_operations WHERE status = 'completed'")
            .fetch_one(&backup_db)
            .await
            .unwrap();
        assert_eq!(backup_count.0, 4);
    }
}