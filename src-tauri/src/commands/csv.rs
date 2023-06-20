use tauri::api::dialog::message;
use uuid::Uuid;

use crate::state::AppState;

#[derive(serde::Serialize)]
struct Record {
    pub uuid: String,
    pub location_x: Option<i32>,
    pub location_y: Option<i32>,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub measured_length: Option<f32>,
    pub measured_width: Option<f32>,
    pub measured_angle: Option<f32>,
    pub measured_midpoint_x: Option<f32>,
    pub measured_midpoint_y: Option<f32>,
}

#[tauri::command]
pub async fn export_csv(
    window: tauri::Window,
    micrograph_id: String,
    path: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // get project id
    let project_id = match Uuid::parse_str(window.label()) {
        Ok(project_id) => project_id,
        Err(_) => return Err("Invalid project id".into()),
    };

    let micrograph_uuid = match Uuid::parse_str(&micrograph_id) {
        Ok(micrograph_uuid) => micrograph_uuid,
        Err(_) => return Err("Invalid micrograph id".into()),
    };

    // get segments
    let segments = match state.get_segments_by_micrograph(&project_id, &micrograph_uuid) {
        Ok(segments) => segments,
        Err(_) => return Err("Failed to get segments".into()),
    };

    // create csv writer
    let mut wtr = match csv::Writer::from_path(path) {
        Ok(wtr) => wtr,
        Err(_) => return Err("Failed to create csv writer".into()),
    };

    // iterate over segments
    for segment in segments {
        let record = Record {
            uuid: segment.uuid.to_string(),
            location_x: segment.location_x,
            location_y: segment.location_y,
            height: segment.height,
            width: segment.width,
            measured_length: segment.measured_length,
            measured_width: segment.measured_width,
            measured_angle: segment.measured_angle,
            measured_midpoint_x: segment.measured_midpoint_x,
            measured_midpoint_y: segment.measured_midpoint_y,
        };

        // write record
        if let Err(_) = wtr.serialize(record) {
            return Err("Failed to write record".into());
        }
    }

    // flush writer
    if let Err(_) = wtr.flush() {
        return Err("Failed to flush writer".into());
    }

    message(
        Some(&window),
        "Export successful",
        "CSV file has been exported successfully",
    );

    Ok(())
}
