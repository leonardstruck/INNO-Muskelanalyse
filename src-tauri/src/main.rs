#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use queues::{analysis::AnalysisQueue, import::ImportQueue, preprocessing::PreprocessingQueue};
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

                let project_id = uuid::Uuid::parse_str(event.window().label()).unwrap();

                state.remove_window(&project_id);
            }
        })
        .setup(|app| {
            env_logger::init();

            // init queues
            let import_queue = ImportQueue::new(app.app_handle());
            app.manage(import_queue);

            let preprocessing_queue = PreprocessingQueue::new(app.app_handle());
            app.manage(preprocessing_queue);

            let analysis_queue = AnalysisQueue::new(app.app_handle());
            app.manage(analysis_queue);

            // start queues
            let import_queue = app.state::<ImportQueue>();
            import_queue.start();

            let preprocessing_queue = app.state::<PreprocessingQueue>();
            preprocessing_queue.start();

            let analysis_queue = app.state::<AnalysisQueue>();
            analysis_queue.start();

            Ok(())
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
