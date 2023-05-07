use uuid::Uuid;

use crate::{
    models::micrographs::{NewMicrograph, PortableMicrograph},
    queues::{
        import::{ImportQueue, ImportQueueItem},
        preprocessing::PreprocessingQueue,
    },
    state::MutableAppState,
};

#[tauri::command]
pub async fn get_micrographs(
    app: tauri::AppHandle,
    window: tauri::Window,
    state: tauri::State<'_, MutableAppState>,
) -> Result<Vec<PortableMicrograph>, String> {
    // get window id
    let id = Uuid::parse_str(window.label()).unwrap();

    let micrographs = state.get_micrographs(&id).unwrap();

    // convert to portable micrographs
    let portable_micrographs = micrographs
        .into_iter()
        .map(|micrograph| micrograph.to_portable(&app))
        .collect();

    Ok(portable_micrographs)
}

#[tauri::command]
pub async fn import_micrographs(
    window: tauri::Window,
    state: tauri::State<'_, MutableAppState>,
    import_queue: tauri::State<'_, ImportQueue>,
    files: Vec<String>,
) -> Result<(), String> {
    let project_uuid = Uuid::parse_str(window.label()).unwrap();
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

        import_queue.push(ImportQueueItem {
            project_uuid,
            micrograph_uuid,
        });
    }

    Ok(())
}

#[tauri::command]
pub async fn delete_micrograph(
    window: tauri::Window,
    state: tauri::State<'_, MutableAppState>,
    import_queue: tauri::State<'_, ImportQueue>,
    preprocessing_queue: tauri::State<'_, PreprocessingQueue>,
    id: uuid::Uuid,
) -> Result<(), String> {
    let project_id = Uuid::parse_str(window.label()).unwrap();

    state.delete_micrograph(&project_id, &id).unwrap();
    import_queue.remove(&project_id, &id);
    preprocessing_queue.remove(&project_id, &id);

    Ok(())
}
