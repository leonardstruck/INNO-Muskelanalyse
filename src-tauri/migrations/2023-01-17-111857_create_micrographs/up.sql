-- Your SQL goes here
CREATE TABLE micrographs (
  uuid TEXT PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  path TEXT,
  import_path TEXT,
  thumbnail_path TEXT,
  file_size INTEGER NOT NULL,
  file_type TEXT NOT NULL,

  status TEXT CHECK (status IN ('new', 'imported', 'segmented', 'error', 'ok')) NOT NULL DEFAULT 'new',
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE case_micrographs (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  case_id INTEGER NOT NULL,
  micrograph_id TEXT NOT NULL,
  FOREIGN KEY (case_id) REFERENCES cases(id) ON DELETE CASCADE,
  FOREIGN KEY (micrograph_id) REFERENCES micrograph(id) ON DELETE CASCADE 
);