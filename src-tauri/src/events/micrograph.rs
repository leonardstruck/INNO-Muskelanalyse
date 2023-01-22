use std::path::Path;

use crate::data::PoolState;
use crate::models::micrograph::Micrograph;
use crate::schema::micrographs::{self, dsl::*};
use diesel::prelude::*;

use ::image::GenericImageView;
use tauri::Manager;

pub fn move_micrograph(app: tauri::AppHandle, micrograph_uuid: String) -> Result<(), String> {
    let state = app.state::<PoolState>();
    let mut connection = state.0.clone().get().unwrap();

    let micrograph = micrographs
        .filter(micrographs::dsl::uuid.eq(micrograph_uuid))
        .first::<Micrograph>(&mut connection)
        .expect("Error loading micrograph");

    // create micrograph path
    let micrograph_dir = app
        .path_resolver()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join("micrographs")
        .join(micrograph.uuid.to_string());

    // create micrograph directory
    std::fs::create_dir_all(&micrograph_dir).expect("Failed to create micrograph directory");

    // check if micrograph path is valid
    let import_path_string = micrograph.import_path.unwrap();
    let path_to_import = std::path::Path::new(import_path_string.as_str());
    if !&path_to_import.exists() || !&path_to_import.is_file() {
        return Err("Micrograph path is invalid".to_string());
    }

    // get extension of micrograph
    let extension = path_to_import.extension().unwrap().to_str().unwrap();

    let new_file_path = micrograph_dir.join(format!("{}.{}", "original", extension));

    // copy micrograph to micrograph directory
    std::fs::copy(&path_to_import, &new_file_path).expect("Failed to copy micrograph");

    // update micrograph path
    diesel::update(micrographs.find(micrograph.uuid.to_string()))
        .set(micrographs::dsl::import_path.eq(new_file_path.to_str().unwrap().to_string()))
        .execute(&mut connection)
        .expect("Error updating micrograph");

    // create thumbnail
    let store_thumbnail_path = micrograph_dir.join(format!("{}.{}", "thumbnail", "png"));
    let thumbnail_result = create_thumbnail(&new_file_path, &store_thumbnail_path);
    if thumbnail_result.is_err() {
        return Err(thumbnail_result.err().unwrap());
    }

    // update thumbnail path
    diesel::update(micrographs.find(micrograph.uuid.to_string()))
        .set(
            micrographs::dsl::thumbnail_path.eq(store_thumbnail_path.to_str().unwrap().to_string()),
        )
        .execute(&mut connection)
        .expect("Error updating micrograph");

    // update status
    diesel::update(micrographs.find(micrograph.uuid.to_string()))
        .set(micrographs::dsl::status.eq("imported"))
        .execute(&mut connection)
        .expect("Error updating micrograph");

    Ok(())
}

pub fn create_thumbnail(
    original_path: &Path,
    path_to_store_thumbnail: &Path,
) -> Result<(), String> {
    let file = std::fs::File::open(original_path).expect("Failed to open file");
    let reader = std::io::BufReader::new(file);

    let mut image_reader = image::io::Reader::new(reader)
        .with_guessed_format()
        .expect("Failed to read image");

    image_reader.no_limits();

    let image = image_reader.decode().expect("Failed to decode image");

    // get aspect ratio of image
    let (width, height) = image.dimensions();
    let aspect_ratio = width as f32 / height as f32;

    // calculate thumbnail size
    let thumbnail_size = 400;
    let thumbnail_width = (thumbnail_size as f32 * aspect_ratio) as u32;
    let thumbnail_height = thumbnail_size;

    // create thumbnail
    let thumbnail = image
        .resize(
            thumbnail_width,
            thumbnail_height,
            image::imageops::FilterType::Lanczos3,
        )
        .to_rgba8();

    // write thumbnail to file
    thumbnail
        .save(path_to_store_thumbnail)
        .expect("Failed to save thumbnail");

    Ok(())
}
