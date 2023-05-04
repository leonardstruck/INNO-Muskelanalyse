use diesel::prelude::*;
use tauri::Manager;
use uuid::Uuid;

use crate::models::micrographs::{NewMicrograph, PortableMicrograph};

#[tauri::command]
pub async fn get_micrographs(
    app: tauri::AppHandle,
    window: tauri::Window,
) -> Result<Vec<PortableMicrograph>, String> {
    use crate::models::micrographs::Micrograph;

    use crate::schema::micrographs::dsl::*;

    // get window id
    let id = Uuid::parse_str(window.label()).unwrap();

    // get state
    let state = app.state::<crate::state::MutableAppState>();
    let mut state = state.0.lock().unwrap();

    // get window
    let window = state.windows.get_mut(&id).unwrap();

    // get connection
    let connection = window.connection.as_mut().unwrap();

    // get micrographs
    let results = micrographs
        .load::<Micrograph>(connection)
        .expect("Error loading micrographs");

    // convert to portable micrographs
    let portable_micrographs = results
        .into_iter()
        .map(|micrograph| micrograph.to_portable(&app))
        .collect();

    Ok(portable_micrographs)
}

#[tauri::command]
pub async fn import_micrographs(
    app: tauri::AppHandle,
    window: tauri::Window,
    files: Vec<String>,
) -> Result<(), String> {
    println!("importing micrographs: {:?}", files);

    let id = Uuid::parse_str(window.label()).unwrap();

    // get state
    let state = app.state::<crate::state::MutableAppState>();
    let mut state = state.0.lock().unwrap();

    // get window
    let window = state.windows.get_mut(&id).unwrap();

    // get connection
    let connection = window.connection.as_mut().unwrap();

    // insert micrographs into database
    for file in files {
        let micrograph = NewMicrograph {
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
            import_path: file.clone(),
            name: {
                // get file name from path without extension
                let path = std::path::Path::new(&file);
                path.file_stem().unwrap().to_str().unwrap().into()
            },
            status: "pending".into(),
            uuid: Uuid::new_v4().to_string(),
            display_img: Vec::new(),
            thumbnail_img: Vec::new(),
            height: None,
            width: None,
        };

        diesel::insert_into(crate::schema::micrographs::table)
            .values(&micrograph)
            .execute(connection)
            .expect("Error saving new micrograph");
    }

    Ok(())
}
