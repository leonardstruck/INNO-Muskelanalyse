-- Your SQL goes here
CREATE TABLE micrographs (
    uuid TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    import_path TEXT NOT NULL,
    thumbnail_img BLOB,
    display_img BLOB,
    width INTEGER,
    height INTEGER,
    status TEXT CHECK (
        status IN (
            'pending',
            'imported',
            'segmented',
            'error',
            'done'
        )
    ) NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE segments (
    uuid TEXT PRIMARY KEY NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    binary_img TEXT NOT NULL,
    location_x INTEGER,
    location_y INTEGER,
    height INTEGER,
    width INTEGER,
    measured_length FLOAT,
    measured_width FLOAT,
    measured_angle FLOAT,
    micrograph_id TEXT NOT NULL,
    status TEXT CHECK (
        status IN ('new', 'verified', 'error', 'ok')
    ) NOT NULL DEFAULT 'new',
    FOREIGN KEY (micrograph_id) REFERENCES micrographs(uuid) ON DELETE CASCADE
);