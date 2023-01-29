use diesel::prelude::*;
use tauri::Manager;

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
            // parse filename from path
            filename: std::path::Path::new(&segment.path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
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

pub async fn analyze_segments(app: &tauri::AppHandle, micrograph_id: String) {
    use crate::data::{get_connection, PoolState};
    use crate::models::micrograph::Micrograph;
    use crate::models::segment::AnalysisResponse;
    use crate::schema::micrographs::dsl;
    use crate::schema::segments::dsl as segment_dsl;

    // get state from app handle
    let state = app.state::<PoolState>();

    // fetch micrograph from database
    let micrograph = dsl::micrographs
        .filter(dsl::uuid.eq(micrograph_id))
        .first::<Micrograph>(&mut get_connection(state.clone()).unwrap())
        .unwrap();

    // check if micrograph has been segmented
    if micrograph.status != "segmented" {
        return;
    }

    // generate path to micrograph segments
    let segment_dir = app
        .path_resolver()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join("micrographs")
        .join(micrograph.uuid.to_string())
        .join("segments");

    // run analysis sidecar
    let analysis = tauri::api::process::Command::new_sidecar("analysis")
        .expect("Failed to create analysis sidecar")
        .args(&["-d".to_string(), segment_dir.to_str().unwrap().to_string()])
        .output()
        .expect("Failed to run analysis");

    // replace single quotes with double quotes
    let analysis_escaped = analysis.stdout.replace("'", "\"");

    // parse analysis output
    let analysis_output: Vec<AnalysisResponse> =
        serde_json::from_str(&analysis_escaped).expect("Failed to parse analysis output");

    // update segments in database
    for segment in analysis_output {
        let length = if segment.direction_a > segment.direction_b {
            segment.direction_a
        } else {
            segment.direction_b
        };

        let width = if segment.direction_a < segment.direction_b {
            segment.direction_a
        } else {
            segment.direction_b
        };

        diesel::update(segment_dsl::segments.filter(segment_dsl::filename.eq(segment.path)))
            .set((
                segment_dsl::measured_angle.eq(segment.angle),
                segment_dsl::measured_length.eq(length),
                segment_dsl::measured_width.eq(width),
                segment_dsl::status.eq("ok"),
                segment_dsl::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(&mut get_connection(state.clone()).unwrap())
            .unwrap();
    }

    // update micrograph in database
    diesel::update(dsl::micrographs.find(micrograph.uuid))
        .set((
            dsl::status.eq("ok"),
            dsl::updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(&mut get_connection(state).unwrap())
        .unwrap();
}
