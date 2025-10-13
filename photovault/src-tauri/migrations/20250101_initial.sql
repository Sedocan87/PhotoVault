-- Photos table
CREATE TABLE photos (
    id INTEGER PRIMARY KEY,
    path TEXT UNIQUE NOT NULL,
    filename TEXT NOT NULL,
    file_hash TEXT NOT NULL,          -- SHA256 for duplicate detection
    file_size INTEGER,
    date_taken DATETIME,
    date_added DATETIME DEFAULT CURRENT_TIMESTAMP,
    width INTEGER,
    height INTEGER,
    format TEXT                       -- JPEG, PNG, HEIC, etc.
);