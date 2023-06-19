-- Your SQL goes here
CREATE TABLE config (
    id INTEGER PRIMARY KEY NOT NULL,
    key TEXT NOT NULL UNIQUE,
    value TEXT NOT NULL
);