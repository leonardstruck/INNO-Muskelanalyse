-- Your SQL goes here
CREATE TABLE segments (
    uuid TEXT PRIMARY KEY NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    path TEXT NOT NULL,
    location_x INTEGER,
    location_y INTEGER,
    height INTEGER,
    width INTEGER,
    measured_length FLOAT,
    measured_width FLOAT,
    measured_angle FLOAT,
    micrograph_id TEXT NOT NULL,
    FOREIGN KEY (micrograph_id) REFERENCES micrographs(uuid) ON DELETE CASCADE
);