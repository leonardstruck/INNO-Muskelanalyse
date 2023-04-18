use crate::{data::get_connection_from_app, models::micrograph::Micrograph};
use diesel::prelude::*;

pub fn move_micrograph(app: &tauri::AppHandle, micrograph: &Micrograph) -> Result<(), String> {
    use crate::schema::micrographs::dsl;

    // create path to new micrograph
    let micograph_dir = app
        .path_resolver()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join("micrographs")
        .join(micrograph.uuid.to_string());

    // create target directory
    std::fs::create_dir_all(&micograph_dir).expect("Failed to create micrograph dir");

    // verify that micrograph exists in import location
    let import_path = std::path::Path::new(&micrograph.import_path);
    if !import_path.exists() {
        panic!("Micrograph not found in import location");
    }

    // get extension of micrograph and create target path
    let extension = import_path.extension().unwrap();
    let new_path = micograph_dir.join(format!("{}.{}", "original", extension.to_str().unwrap()));

    // copy micrograph to new location
    std::fs::copy(&import_path, &new_path).expect("Failed to copy micrograph");

    // get file size of new micrograph
    let file_size = std::fs::metadata(&new_path).unwrap().len();

    // update micrograph in database
    diesel::update(dsl::micrographs.find(micrograph.uuid.clone()))
        .set((
            dsl::path.eq(new_path.to_str().unwrap()),
            dsl::file_size.eq(file_size as i32),
            dsl::updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(&mut get_connection_from_app(app).unwrap())
        .unwrap();

    Ok(())
}

pub fn generate_thumbnail(app: &tauri::AppHandle, micrograph: &Micrograph) -> Result<(), String> {
    use crate::schema::micrographs::dsl;

    // create path to micrograph dir
    let micrograph_dir = app
        .path_resolver()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join("micrographs")
        .join(micrograph.uuid.to_string());

    // create path to thumbnail
    let thumbnail_path = micrograph_dir.join("thumbnail.png");

    let file =
        std::fs::File::open(&micrograph.path.clone().unwrap()).expect("Failed to open micrograph");
    let reader = std::io::BufReader::new(file);

    let mut image_reader = image::io::Reader::new(reader)
        .with_guessed_format()
        .expect("Failed to read micrograph");

    // disable image limits to allow reading of large images
    image_reader.no_limits();

    let image = image_reader.decode().expect("Failed to decode micrograph");

    // guess mime type of micrograph
    let mime_type = mime_guess::from_path(&micrograph.path.clone().unwrap())
        .first()
        .unwrap();

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

    // create display image path
    let display_path = micrograph_dir.join("display.png");

    // calculate display image size
    let desired_size = 2048;
    let display_size = if aspect_ratio > 1.0 {
        (desired_size, (desired_size as f32 / aspect_ratio) as u32)
    } else {
        (((desired_size as f32 * aspect_ratio) as u32), desired_size)
    };

    // create display image
    let display_image = image
        .resize(
            display_size.0,
            display_size.1,
            image::imageops::FilterType::Lanczos3,
        )
        .to_rgb16();

    // save display image
    display_image
        .save_with_format(&display_path, image::ImageFormat::Png)
        .expect("Failed to save display image");

    // update micrograph in database
    diesel::update(dsl::micrographs.find(micrograph.uuid.clone()))
        .set((
            dsl::thumbnail_path.eq(thumbnail_path.to_str().unwrap()),
            dsl::display_path.eq(display_path.to_str().unwrap()),
            dsl::updated_at.eq(chrono::Utc::now().naive_utc()),
            dsl::width.eq(image.width() as i32),
            dsl::height.eq(image.height() as i32),
            dsl::file_type.eq(mime_type.to_string()),
            dsl::status.eq("imported"),
        ))
        .execute(&mut get_connection_from_app(app).unwrap())
        .unwrap();

    Ok(())
}
