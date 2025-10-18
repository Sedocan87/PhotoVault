mod common;

use photovault::commands::get_config;
use photovault::services::config::{self, AppConfig};
use std::path::PathBuf;
use tempfile::tempdir;

#[tokio::test]
async fn test_get_config() {
    // This test now uses a temporary directory for the config file,
    // and the database is handled in-memory by the helper.
    let home_dir = tempdir().unwrap();
    std::env::set_var("HOME", home_dir.path());

    let config_dir = config::get_app_config_dir().unwrap();
    let config_path = config_dir.join(config::CONFIG_FILE_NAME);

    let mut test_config = AppConfig::default();
    test_config.primary_drive = Some(PathBuf::from("/test/primary"));
    test_config.backup_drive = Some(PathBuf::from("/test/backup"));

    config::save_config_to_path(&test_config, &config_path)
        .await
        .unwrap();

    let loaded_config = get_config().await.unwrap();
    assert_eq!(
        loaded_config.primary_drive,
        Some(PathBuf::from("/test/primary"))
    );
    assert_eq!(
        loaded_config.backup_drive,
        Some(PathBuf::from("/test/backup"))
    );
}
