-- Add indexes to photos table for faster filtering
CREATE INDEX idx_photos_date_taken ON photos(date_taken);
CREATE INDEX idx_photos_width ON photos(width);
CREATE INDEX idx_photos_height ON photos(height);
CREATE INDEX idx_photos_file_size ON photos(file_size);