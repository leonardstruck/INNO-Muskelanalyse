use diesel::Connection;
use tauri::{Manager, Runtime};
use uuid::Uuid;

use crate::state::{MutableAppState, WindowState};

#[tauri::command]
pub async fn open_project<R: Runtime>(
    app: tauri::AppHandle<R>,
    state: tauri::State<'_, MutableAppState>,
    path: String,
) -> Result<(), String> {
    // lock state
    let mut state = state.0.lock().unwrap();

    // check if there's already a window with this project path
    let existing_window = state.windows.contains_key_alt(&path);

    if existing_window {
        return Err("The selected project is currently open in another window. Please close the project before attempting to open it again.".into());
    }

    // generate new window id
    let id = Uuid::new_v4();

    // add window to windows
    let new_window = WindowState {
        id,
        project_path: path.clone().into(),
        file_name: {
            // get file name from path without extension
            let path = std::path::Path::new(&path);
            path.file_stem().unwrap().to_str().unwrap().into()
        },
        connection: {
            // create connection to database
            let path = std::path::Path::new(&path);
            Some(diesel::SqliteConnection::establish(path.to_str().unwrap()).unwrap())
        },
    };

    // open new window
    let _new_window = tauri::WindowBuilder::new(
        &app,
        id.to_string(),
        tauri::WindowUrl::App("windows/default".into()),
    )
    .title(format!("Project: {}", new_window.file_name))
    .build()
    .unwrap();

    state.windows.insert(id, path, new_window);

    if app.windows().get("welcome").is_some() {
        app.windows().get("welcome").unwrap().close().unwrap();
    }

    Ok(())
}
