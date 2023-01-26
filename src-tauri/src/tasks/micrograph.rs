use diesel::prelude::*;
use tauri::Manager;

pub fn move_micrograph(app: &tauri::AppHandle, micrograph_id: String) -> Result<(), String> {
    use crate::data::{get_connection, PoolState};
    use crate::models::micrograph::Micrograph;
    use crate::schema::micrographs::dsl;

    // get state from app handle
    let state = app.state::<PoolState>();

    // fetch micrograph from database
    let micrograph = dsl::micrographs
        .filter(dsl::uuid.eq(micrograph_id))
        .first::<Micrograph>(&mut get_connection(state.clone()).unwrap())
        .unwrap();

    // create path to new micrograph
    let micograph_dir = app
        .path_resolver()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join("micrographs")
        .join(micrograph.uuid.to_string());

    // create directory for new micrograph
    std::fs::create_dir_all(&micograph_dir).expect("Failed to create micrograph dir");

    // check if micrograph is still in the import location
    let import_path = std::path::Path::new(&micrograph.import_path);
    if !import_path.exists() {
        return Err("Micrograph not found in import location".to_string());
    }

    // get extension of micrograph
    let extension = import_path.extension().unwrap();
    let new_path = micograph_dir.join(format!("{}.{}", "original", extension.to_str().unwrap()));

    // copy micrograph to new location
    std::fs::copy(&import_path, &new_path).expect("Failed to copy micrograph");

    // get file size of new micrograph
    let file_size = std::fs::metadata(&new_path).unwrap().len();

    // update micrograph in database
    diesel::update(dsl::micrographs.find(micrograph.uuid))
        .set((
            dsl::path.eq(new_path.to_str().unwrap()),
            dsl::file_size.eq(file_size as i32),
            dsl::updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(&mut get_connection(state).unwrap())
        .unwrap();

    Ok(())
}
