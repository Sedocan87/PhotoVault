use std::path::{Path, PathBuf};
use crate::models::photo::Photo;
use walkdir::WalkDir;
use image::io::Reader as ImageReader;

pub struct FileOperationService {
    pub primary_path: PathBuf,
}

impl FileOperationService {
    pub fn new(primary_path: String) -> Self {
        Self {
            primary_path: PathBuf::from(primary_path),
        }
    }

    pub async fn scan_directory(&self) -> Result<Vec<Photo>, String> {
        let mut photos = Vec::new();
        let supported_formats = Self::get_supported_formats();

        for entry in WalkDir::new(&self.primary_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if supported_formats.contains(&ext.to_lowercase().as_str()) {
                        if let Ok(photo) = Photo::from_path(path).await {
                            photos.push(photo);
                        }
                    }
                }
            }
        }
        Ok(photos)
    }

    pub fn get_supported_formats() -> Vec<&'static str> {
        vec!["jpg", "jpeg", "png", "gif", "heic", "heif", "webp"]
    }
}

impl Photo {
    pub async fn from_path(path: &Path) -> Result<Self, String> {
        let metadata = tokio::fs::metadata(path).await.map_err(|e| e.to_string())?;
        let file_size = Some(metadata.len() as i64);

        let filename = path.file_name().unwrap().to_str().unwrap().to_string();
        let path_str = path.to_str().unwrap().to_string();

        let (width, height, date_taken) = match ImageReader::open(path) {
            Ok(reader) => {
                let dimensions = reader.into_dimensions().unwrap_or((0, 0));
                // Metadata reading for date_taken is complex and will be added later.
                (Some(dimensions.0 as i64), Some(dimensions.1 as i64), None)
            },
            Err(_) => (None, None, None),
        };

        let format = path.extension().unwrap().to_str().unwrap().to_string();

        Ok(Photo {
            id: 0, // ID will be set by the database
            path: path_str,
            filename,
            file_size,
            date_taken,
            width,
            height,
            format,
        })
    }
}