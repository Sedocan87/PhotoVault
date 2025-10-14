use super::super::services::file_ops::FileOperationService;
use tempfile::tempdir;
use std::fs::File;
use std::io::Write;

#[tokio::test]
async fn test_scan_directory() {
    let dir = tempdir().unwrap();
    let service = FileOperationService::new(dir.path().to_path_buf());

    // Create a dummy image file
    let file_path = dir.path().join("test.jpg");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"fake image data").unwrap();

    let photos = service.scan_directory(dir.path()).await.unwrap();
    assert_eq!(photos.len(), 1);
    assert_eq!(photos[0].filename, "test.jpg");
}