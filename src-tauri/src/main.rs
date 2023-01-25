#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod data;
mod events;
mod models;
mod schema;

use dotenvy::dotenv;
use tauri::Manager;

use data::PoolState;

fn main() {
    // load .env file
    // create .env file if it doesn't exist

    let env_exists = std::path::Path::new(".env").exists();
    if !env_exists {
        std::fs::write(".env", "").unwrap_or_default();
    }

    dotenv().unwrap();

    tauri::Builder::default()
        .manage(PoolState)
        .setup(|app| {
            let app_handle = app.handle();

            // get connection pool and store in state
            let pool = data::get_connection_pool(app_handle.clone());
            let mut connection = pool.get().unwrap();
            app.manage(PoolState(pool));

            data::run_migrations(&mut connection);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![crate::commands::case::get_cases])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
