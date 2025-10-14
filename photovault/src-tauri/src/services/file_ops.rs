use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::models::photo::Photo;
use chrono::{DateTime, Utc};
use image::GenericImageView;

pub struct FileOperationService {
    pub primary_path: PathBuf,
}

impl FileOperationService {
    pub fn new(primary_path: PathBuf) -> Self {
        Self { primary_path }
    }

    pub async fn scan_directory(&self, path: &Path) -> Result<Vec<Photo>, std::io::Error> {
        let mut photos = Vec::new();
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if self.is_supported_format(path) {
                if let Ok(metadata) = self.read_metadata(path).await {
                    photos.push(metadata);
                }
            }
        }
        Ok(photos)
    }

    pub fn get_supported_formats() -> Vec<&'static str> {
        vec!["jpg", "jpeg", "png", "gif", "bmp", "ico"]
    }

    fn is_supported_format(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            return Self::get_supported_formats().contains(&ext.to_lowercase().as_str());
        }
        false
    }

    pub async fn read_metadata(&self, path: &Path) -> Result<Photo, std::io::Error> {
        let metadata = std::fs::metadata(path)?;
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();
        let file_size = metadata.len();
        let date_taken: Option<DateTime<Utc>> = metadata.created().ok().map(DateTime::from);

        let (width, height, format) = match image::open(path) {
            Ok(img) => {
                let (w, h) = img.dimensions();
                let format = image::guess_format(&std::fs::read(path).unwrap()).map_or("".to_string(), |f| f.extensions_str()[0].to_string());
                (w, h, format)
            },
            Err(_) => (0, 0, "".to_string()),
        };

        Ok(Photo {
            id: 0, // ID will be set by the database
            path: path.to_str().unwrap().to_string(),
            filename,
            file_size,
            date_taken,
            width,
            height,
            format,
        })
    }
}