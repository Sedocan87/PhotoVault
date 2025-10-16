CREATE TABLE IF NOT EXISTS photo_albums (
    photo_id INTEGER NOT NULL,
    album_id INTEGER NOT NULL,
    PRIMARY KEY (photo_id, album_id),
    FOREIGN KEY (photo_id) REFERENCES photos (id),
    FOREIGN KEY (album_id) REFERENCES albums (id)
);