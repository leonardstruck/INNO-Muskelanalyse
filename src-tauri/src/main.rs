#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod data;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let mut connection = data::connection::establish_connection(app_handle);
            data::migrations::run_migrations(&mut connection);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            data::cases::get_cases,
            data::cases::get_case,
            data::cases::create_case,
            data::cases::delete_case,
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
