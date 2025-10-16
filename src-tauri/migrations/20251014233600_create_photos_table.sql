CREATE TABLE IF NOT EXISTS photos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    filename TEXT NOT NULL,
    file_size INTEGER,
    date_taken DATETIME,
    width INTEGER,
    height INTEGER,
    format TEXT NOT NULL
);