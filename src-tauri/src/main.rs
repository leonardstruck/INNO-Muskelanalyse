#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod data;
mod models;
mod schema;

use dotenvy::dotenv;

fn main() {
    // load .env file
    dotenv().ok();

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let mut connection = data::establish_connection(app_handle);
            data::run_migrations(&mut connection);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            models::case::get_cases,
            models::case::get_case,
            models::case::create_case,
            models::case::delete_case,
            models::micrograph::get_micrographs_by_case,
            models::micrograph::import_micrographs
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
