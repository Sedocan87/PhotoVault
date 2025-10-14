use crate::db::manager::DatabaseManager;
use crate::models::operation::Operation;
use crate::services::sync_engine::SyncEngine;
use tempfile::tempdir;

#[tokio::test]
async fn test_sync_engine_queueing() {
    let dir = tempdir().unwrap();
    let primary_path = dir.path().join("primary.db");
    let backup_path = dir.path().join("backup.db");

    // Create the initial database manager with both databases connected
    let db_manager = DatabaseManager::new(primary_path.clone(), Some(backup_path.clone()))
        .await
        .unwrap();
    let mut sync_engine = SyncEngine::new(db_manager);

    // Disconnect the backup database by closing the connection pool
    sync_engine.db_manager.backup_db.as_ref().unwrap().close().await;

    // Execute an operation, which should fail on the backup and be queued
    let op = Operation::CreateAlbum {
        name: "Test Album".to_string(),
    };
    let result = sync_engine.execute_operation(op.clone()).await;
    assert!(result.is_err());

    // Check that the operation is in the queue
    assert_eq!(sync_engine.operation_queue.len(), 1);

    // Reconnect the backup database by creating a new database manager
    let db_manager = DatabaseManager::new(primary_path.clone(), Some(backup_path.clone()))
        .await
        .unwrap();
    sync_engine.db_manager = db_manager;

    // Flush the queue, which should now succeed
    let result = sync_engine.flush_queue().await;
    assert!(result.is_ok());

    // Check that the queue is empty
    assert_eq!(sync_engine.operation_queue.len(), 0);
}