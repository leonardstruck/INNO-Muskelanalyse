use diesel::Connection;
use tauri::Manager;
use uuid::Uuid;

use crate::{
    models::micrographs::Status,
    queues::import::{ImportQueue, ImportQueueItem},
    state::{MutableAppState, WindowState},
};

#[tauri::command]
pub async fn open_project(
    app: tauri::AppHandle,
    state: tauri::State<'_, MutableAppState>,
    import_queue: tauri::State<'_, ImportQueue>,
    path: String,
) -> Result<(), String> {
    // check if there's already a window with this project path
    let existing_window = state.is_project_already_open(path.clone());

    if existing_window {
        return Err("The selected project is currently open in another window. Please close the project before attempting to open it again.".into());
    }

    let mut connection = {
        // create connection to database
        let path = std::path::Path::new(&path);
        Some(diesel::SqliteConnection::establish(path.to_str().unwrap()).unwrap())
    };

    // run diesel migrations
    match crate::migrations::run_migrations(connection.as_mut().unwrap()) {
        Ok(_) => {}
        Err(error) => return Err(format!("The project file is corrupted: {}", error)),
    }

    // generate new window id
    let id = {
        use crate::schema::config::dsl::*;
        use diesel::prelude::*;

        // check if the project id is already in database
        let existing_id = config
            .select(value)
            .filter(key.eq("project_id"))
            .first::<String>(connection.as_mut().unwrap())
            .optional()
            .unwrap();

        // generate new id if it doesn't exist
        let project_id = match existing_id {
            Some(other_id) => Uuid::parse_str(&other_id).unwrap(),
            None => {
                let new_id = Uuid::new_v4();
                let new_config = crate::models::config::NewConfig {
                    key: "project_id".into(),
                    value: new_id.to_string(),
                };
                diesel::insert_into(crate::schema::config::table)
                    .values(new_config)
                    .execute(connection.as_mut().unwrap())
                    .unwrap();
                new_id
            }
        };

        project_id
    };

    let file_name = {
        // get file name from path without extension
        let path = std::path::Path::new(&path);
        path.file_stem().unwrap().to_str().unwrap().to_string()
    };

    // add window to windows
    let new_window = WindowState {
        id: id.clone(),
        project_path: path.clone().into(),
        file_name: file_name.clone(),
        connection,
    };

    state.add_window(new_window, path);

    // load pending micrographs from database and add them to queue
    let pending_micrographs = state
        .get_micrographs_by_status(id.clone(), Status::Pending)
        .unwrap();

    for micrograph in pending_micrographs {
        import_queue.push(ImportQueueItem {
            project_uuid: id.clone().to_string(),
            micrograph_uuid: micrograph.uuid,
        })
    }

    // open new window
    let _new_window =
        tauri::WindowBuilder::new(&app, id.to_string(), tauri::WindowUrl::App("main".into()))
            .title(format!("Project: {}", file_name))
            .min_inner_size(650.0, 500.0)
            .theme(Some(tauri::Theme::Dark))
            .build()
            .unwrap();

    if app.windows().get("welcome").is_some() {
        app.windows().get("welcome").unwrap().close().unwrap();
    }

    Ok(())
}
