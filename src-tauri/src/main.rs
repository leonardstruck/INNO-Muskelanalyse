#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri_plugin_log::{LogTarget};

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
        .menu(menu::create_menu())
        .on_menu_event(menu::menu_event_handler)
        .on_window_event(|event| {
            use tauri::WindowEvent;

            if let WindowEvent::CloseRequested { .. } = event.event() {
                // skip if the window is a viewer
                if event.window().label().starts_with("viewer") {
                    return;
                }
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
            crate::commands::resolve_requirements::check_requirements,
            crate::commands::processor::get_processor_status,
            crate::commands::window::open_project,
            crate::commands::micrographs::get_micrographs,
            crate::commands::micrographs::get_micrograph,
            crate::commands::micrographs::import_micrographs,
            crate::commands::micrographs::delete_micrograph,
            crate::commands::segments::get_segments,
            crate::commands::csv::export_csv,
        ])
        .plugin(tauri_plugin_log::Builder::default().targets([
            LogTarget::LogDir,
            LogTarget::Stdout,
            LogTarget::Webview,
        ]).build())    
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
