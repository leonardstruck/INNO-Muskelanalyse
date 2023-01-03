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

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
