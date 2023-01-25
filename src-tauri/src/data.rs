use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct PoolState(pub Pool<ConnectionManager<SqliteConnection>>);

pub fn run_migrations(connection: &mut diesel::sqlite::SqliteConnection) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub fn get_connection_pool(
    app_handle: tauri::AppHandle,
) -> Pool<ConnectionManager<SqliteConnection>> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("Failed to get app data dir");

    // check if app_dir exists and create it if not
    if !app_dir.exists() {
        std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
    }

    let database_url = app_dir
        .join("database.sqlite")
        .to_str()
        .unwrap()
        .to_string();

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .max_size(1)
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn get_connection(
    state: tauri::State<'_, PoolState>,
) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, String> {
    state
        .0
        .get()
        .map_err(|e| format!("Could not get connection from pool: {}", e))
}
