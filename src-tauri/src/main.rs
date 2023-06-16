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
mod processor;
mod schema;
mod state;
mod utils;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            env_logger::init();
            Ok(())
        })
        .menu(menu::create_menu())
        .on_menu_event(menu::menu_event_handler)
        .on_window_event(|event| {
            use tauri::WindowEvent;

            if let WindowEvent::CloseRequested { .. } = event.event() {
                // get state
                let app = event.window().app_handle();
                let state = app.state::<state::AppState>();

                let project_id = uuid::Uuid::parse_str(event.window().label()).unwrap();

                state.remove_window(&project_id);
            }
        })
        .manage(state::AppState(Default::default()))
        .manage(processor::ProcessorState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            crate::commands::window::open_project,
            crate::commands::micrographs::get_micrographs,
            crate::commands::micrographs::import_micrographs,
            crate::commands::micrographs::delete_micrograph
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
