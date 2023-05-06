#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

mod commands;
mod image_manipulation;
mod menu;
mod migrations;
mod models;
mod queues;
mod schema;
mod state;
mod utils;

fn main() {
    tauri::Builder::default()
        .menu(menu::create_menu())
        .on_menu_event(menu::menu_event_handler)
        .on_window_event(|event| {
            use tauri::WindowEvent;

            if let WindowEvent::CloseRequested { .. } = event.event() {
                // get state
                let app = event.window().app_handle();
                let state = app.state::<state::MutableAppState>();

                // lock state
                let mut state = state.0.lock().unwrap();

                let id = uuid::Uuid::parse_str(event.window().label()).unwrap();

                // remove window from windows
                state.windows.remove(&id);
            }
        })
        .manage(state::MutableAppState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            crate::commands::window::open_project,
            crate::commands::micrographs::get_micrographs,
            crate::commands::micrographs::import_micrographs,
            crate::commands::micrographs::delete_micrograph,
            crate::commands::queue::queue_get_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
