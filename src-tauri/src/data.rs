use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn run_migrations(connection: &mut diesel::sqlite::SqliteConnection) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}

use diesel::prelude::*;

pub fn establish_connection(app_handle: tauri::AppHandle) -> SqliteConnection {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("Failed to get app data dir");

    let database_url = app_dir
        .join("database.sqlite")
        .to_str()
        .unwrap()
        .to_string();

    if cfg!(debug_assertions) {
        // check if DATABASE_URL equals database_url
        if std::env::var("DATABASE_URL").unwrap_or(String::new()) != database_url {
            println!("Please update DATABASE_URL in the .env file:");
            println!("DATABASE_URL=\"{}\"", database_url);
        }
    }

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
