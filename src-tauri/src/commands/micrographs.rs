use log::{debug, error};
use uuid::Uuid;

use crate::{
    models::micrographs::{CachedMicrograph, NewMicrograph},
    processor::ProcessorState,
    state::AppState,
};

#[tauri::command]
pub async fn get_micrographs(
    app: tauri::AppHandle,
    window: tauri::Window,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<CachedMicrograph>, String> {
    // get window id
    let id = Uuid::parse_str(window.label()).unwrap();

    let micrographs = state.get_micrographs(&id).unwrap();

    // convert to portable micrographs
    let portable_micrographs = micrographs
        .into_iter()
        .map(|micrograph| micrograph.to_cache(&app))
        .collect();

    Ok(portable_micrographs)
}

#[tauri::command]
pub async fn get_micrograph(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    micrograph_id: String,
    project_id: String,
) -> Result<Option<CachedMicrograph>, String> {
    let project_id = Uuid::parse_str(&project_id).unwrap();
    let micrograph_id = Uuid::parse_str(&micrograph_id).unwrap();

    let micrographs = match state.get_micrograph(&project_id, &micrograph_id) {
        Ok(micrographs) => micrographs,
        Err(_) => return Err("Failed to get micrograph".into()),
    };

    // convert to portable micrograph
    let portable_micrograph = micrographs.to_cache(&app);

    Ok(Some(portable_micrograph))
}

#[tauri::command]
pub async fn import_micrographs(
    app: tauri::AppHandle,
    window: tauri::Window,
    state: tauri::State<'_, AppState>,
    processor: tauri::State<'_, ProcessorState>,
    files: Vec<String>,
) -> Result<(), String> {
    let project_uuid = Uuid::parse_str(window.label()).unwrap();

    debug!("Importing micrographs: {:?}", files);

    // insert micrographs into database
    for file in files {
        let micrograph_uuid = Uuid::new_v4();

        let micrograph = NewMicrograph {
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
            import_path: file.clone(),
            name: {
                // get file name from path without extension
                let path = std::path::Path::new(&file);
                path.file_stem().unwrap().to_str().unwrap().into()
            },
            status: crate::models::micrographs::Status::Pending,
            uuid: micrograph_uuid.to_string(),
            display_img: Vec::new(),
            thumbnail_img: Vec::new(),
            height: None,
            width: None,
        };

        state.add_micrograph(&project_uuid, micrograph).unwrap();
    }

    // populate processor
    match processor.populate(&app, &project_uuid) {
        Ok(_) => {}
        Err(e) => {
            error!("Error populating processor: {}", e);
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn delete_micrograph(
    window: tauri::Window,
    state: tauri::State<'_, AppState>,
    id: uuid::Uuid,
) -> Result<(), String> {
    let project_id = Uuid::parse_str(window.label()).unwrap();

    state.delete_micrograph(&project_id, &id).unwrap();

    Ok(())
}
