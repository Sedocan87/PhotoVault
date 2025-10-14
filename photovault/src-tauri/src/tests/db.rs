use crate::db::manager::DatabaseManager;
use tempfile::tempdir;

#[tokio::test]
async fn test_database_manager_new() {
    let dir = tempdir().unwrap();
    let primary_path = dir.path().join("primary.db");
    let backup_path = dir.path().join("backup.db");

    let manager = DatabaseManager::new(primary_path.clone(), Some(backup_path.clone())).await;
    assert!(manager.is_ok());
    assert!(primary_path.exists());
    assert!(backup_path.exists());
}