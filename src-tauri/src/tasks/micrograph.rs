use diesel::prelude::*;
use tauri::Manager;

pub fn move_micrograph(app: &tauri::AppHandle, micrograph_id: String) {
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
        panic!("Micrograph not found in import location");
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
}

pub fn generate_thumbnail(app: &tauri::AppHandle, micrograph_id: String) {
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

    // create path to thumbnail
    let thumbnail_path = app
        .path_resolver()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join("micrographs")
        .join(micrograph.uuid.to_string())
        .join("thumbnail.png");

    let file = std::fs::File::open(&micrograph.path.unwrap()).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut image_reader = image::io::Reader::new(reader)
        .with_guessed_format()
        .expect("Failed to read micrograph");

    image_reader.no_limits();

    let image = image_reader.decode().expect("Failed to decode micrograph");

    // get aspect ratio of micrograph
    let aspect_ratio = image.width() as f32 / image.height() as f32;

    // calculate thumbnail size
    let thumbnail_size = if aspect_ratio > 1.0 {
        (512, (512.0 / aspect_ratio) as u32)
    } else {
        (((512.0 * aspect_ratio) as u32), 512)
    };

    // create thumbnail
    let thumbnail = image
        .resize(
            thumbnail_size.0,
            thumbnail_size.1,
            image::imageops::FilterType::Lanczos3,
        )
        .to_rgb16();

    // save thumbnail
    thumbnail
        .save_with_format(&thumbnail_path, image::ImageFormat::Png)
        .expect("Failed to save thumbnail");

    // update micrograph in database
    diesel::update(dsl::micrographs.find(micrograph.uuid))
        .set((
            dsl::thumbnail_path.eq(thumbnail_path.to_str().unwrap()),
            dsl::updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(&mut get_connection(state).unwrap())
        .unwrap();
}
