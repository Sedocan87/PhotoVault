CREATE TABLE IF NOT EXISTS photos (
    id INTEGER PRIMARY KEY NOT NULL,
    path TEXT NOT NULL UNIQUE,
    filename TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    date_taken TEXT,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    format TEXT NOT NULL
);