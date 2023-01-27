-- Your SQL goes here
CREATE TABLE micrographs (
  uuid TEXT PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  path TEXT,
  import_path TEXT NOT NULL,
  thumbnail_path TEXT,
  display_path TEXT,
  file_size INTEGER,
  file_type TEXT,
  width INTEGER,
  height INTEGER,
  status TEXT CHECK (
    status IN ('new', 'imported', 'segmented', 'error', 'ok')
  ) NOT NULL DEFAULT 'new',
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE case_micrographs (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  case_id INTEGER NOT NULL,
  micrograph_id TEXT NOT NULL,
  FOREIGN KEY (case_id) REFERENCES cases(id) ON DELETE CASCADE,
  FOREIGN KEY (micrograph_id) REFERENCES micrographs(uuid) ON DELETE CASCADE
);