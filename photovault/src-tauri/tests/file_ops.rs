use photovault::services::file_ops::FileOperationService;
use std::fs::File;
use tempfile::tempdir;

#[tokio::test]
async fn test_scan_directory() {
    let dir = tempdir().unwrap();
    let path = dir.path();

    // Create some dummy files
    File::create(path.join("image1.jpg")).unwrap();
    File::create(path.join("image2.png")).unwrap();
    File::create(path.join("document.txt")).unwrap();

    let service = FileOperationService::new(path.to_str().unwrap().to_string());
    let photos = service.scan_directory().await.unwrap();

    assert_eq!(photos.len(), 2);
    assert!(photos.iter().any(|p| p.filename == "image1.jpg"));
    assert!(photos.iter().any(|p| p.filename == "image2.png"));
}