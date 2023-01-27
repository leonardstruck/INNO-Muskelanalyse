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

    let file = std::fs::File::open(&micrograph.path.clone().unwrap()).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut image_reader = image::io::Reader::new(reader)
        .with_guessed_format()
        .expect("Failed to read micrograph");

    image_reader.no_limits();

    let image = image_reader.decode().expect("Failed to decode micrograph");

    // guess mime type of micrograph
    let mime_type = mime_guess::from_path(&micrograph.path.unwrap())
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

    // update micrograph in database
    diesel::update(dsl::micrographs.find(micrograph.uuid))
        .set((
            dsl::thumbnail_path.eq(thumbnail_path.to_str().unwrap()),
            dsl::updated_at.eq(chrono::Utc::now().naive_utc()),
            dsl::width.eq(image.width() as i32),
            dsl::height.eq(image.height() as i32),
            dsl::file_type.eq(mime_type.to_string()),
            dsl::status.eq("imported"),
        ))
        .execute(&mut get_connection(state).unwrap())
        .unwrap();
}

pub async fn segment_micrograph(app: &tauri::AppHandle, micrograph_id: String) {
    use crate::data::{get_connection, PoolState};
    use crate::models::micrograph::Micrograph;
    use crate::models::segment::SegmentationResponse;
    use crate::schema::micrographs::dsl;

    // get state from app handle
    let state = app.state::<PoolState>();

    // fetch micrograph from database
    let micrograph = dsl::micrographs
        .filter(dsl::uuid.eq(micrograph_id))
        .first::<Micrograph>(&mut get_connection(state.clone()).unwrap())
        .unwrap();

    // create folder for micrograph segments
    let segment_dir = app
        .path_resolver()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join("micrographs")
        .join(micrograph.uuid.to_string())
        .join("segments");

    std::fs::create_dir_all(&segment_dir).expect("Failed to create segment dir");

    // create segmentation sidecar
    let segmentation = tauri::api::process::Command::new_sidecar("segmentation")
        .expect("Failed to create segmentation sidecar")
        .args(&[
            micrograph.path.unwrap(),
            segment_dir.to_str().unwrap().to_string(),
        ])
        .output()
        .expect("Failed to run segmentation");

    // deserialize segmentation output and save to vector
    let segments: Vec<SegmentationResponse> = serde_json::from_str(&segmentation.stdout)
        .expect("Failed to deserialize segmentation output");

    // create vector of insertable segments
    let insertable_segments: Vec<crate::models::segment::NewSegment> = segments
        .iter()
        .map(|segment| crate::models::segment::NewSegment {
            uuid: uuid::Uuid::new_v4().to_string(),
            micrograph_id: micrograph.uuid.to_string(),
            path: segment.path.clone(),
            location_x: Some(segment.x),
            location_y: Some(segment.y),
            width: Some(segment.width),
            height: Some(segment.height),
            measured_angle: None,
            measured_length: None,
            measured_width: None,
            status: "new".to_string(),
        })
        .collect();

    // insert segments into database
    diesel::insert_into(crate::schema::segments::table)
        .values(&insertable_segments)
        .execute(&mut get_connection(state.clone()).unwrap())
        .expect("Failed to insert segments into database");

    // update micrograph in database
    diesel::update(dsl::micrographs.find(micrograph.uuid))
        .set((
            dsl::status.eq("segmented"),
            dsl::updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(&mut get_connection(state).unwrap())
        .unwrap();
}
