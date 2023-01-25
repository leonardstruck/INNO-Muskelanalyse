#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod data;
mod events;
mod models;
mod schema;

use tauri::Manager;

use data::PoolState;

fn main() {
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
        .invoke_handler(tauri::generate_handler![
            crate::commands::case::get_cases,
            crate::commands::case::get_case,
            crate::commands::case::create_case,
            crate::commands::case::delete_case,
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
