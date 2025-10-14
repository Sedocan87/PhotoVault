-- Add migration script here
-- Photos table
CREATE TABLE photos (
    id INTEGER PRIMARY KEY,
    path TEXT UNIQUE NOT NULL,
    filename TEXT NOT NULL,
    file_hash TEXT,          -- SHA256 for duplicate detection
    file_size INTEGER,
    date_taken DATETIME,
    date_added DATETIME DEFAULT CURRENT_TIMESTAMP,
    width INTEGER,
    height INTEGER,
    format TEXT                       -- JPEG, PNG, HEIC, etc.
);

-- Albums
CREATE TABLE albums (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Photo-Album mapping
CREATE TABLE photo_album (
    photo_id INTEGER,
    album_id INTEGER,
    PRIMARY KEY (photo_id, album_id),
    FOREIGN KEY (photo_id) REFERENCES photos(id) ON DELETE CASCADE,
    FOREIGN KEY (album_id) REFERENCES albums(id) ON DELETE CASCADE
);

-- Tags
CREATE TABLE tags (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

-- Photo-Tag mapping
CREATE TABLE photo_tag (
    photo_id INTEGER,
    tag_id INTEGER,
    PRIMARY KEY (photo_id, tag_id),
    FOREIGN KEY (photo_id) REFERENCES photos(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- Sync journal for recovery
CREATE TABLE sync_operations (
    id TEXT PRIMARY KEY,
    operation_type TEXT NOT NULL,     -- move, delete, rename, etc.
    params TEXT, -- Using TEXT to store JSON data
    status TEXT DEFAULT 'pending',    -- pending, completed, failed
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    error_message TEXT
);

-- Add indexes for performance
CREATE INDEX idx_photos_date_taken ON photos (date_taken);
CREATE INDEX idx_photos_path ON photos (path);
CREATE INDEX idx_tags_name ON tags (name);
CREATE INDEX idx_albums_name ON albums (name);