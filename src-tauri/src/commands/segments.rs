use uuid::Uuid;

use crate::models::segments::CachedSegment;
use crate::state::AppState;

#[tauri::command]
pub async fn get_segments(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    micrograph_id: String,
    project_id: String,
) -> Result<Option<Vec<CachedSegment>>, String> {
    let project_id = Uuid::parse_str(&project_id).unwrap();
    let micrograph_id = Uuid::parse_str(&micrograph_id).unwrap();

    let segments = match state.get_segments_by_micrograph(&project_id, &micrograph_id) {
        Ok(segments) => segments,
        Err(_) => return Err("Failed to get segments".into()),
    };

    // convert to portable segments
    let portable_segments = segments
        .into_iter()
        .map(|segment| segment.to_cache(&app))
        .collect();

    Ok(Some(portable_segments))
}
