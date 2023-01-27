use crate::data::{get_connection, PoolState};
use diesel::prelude::*;

#[tauri::command]
pub async fn get_segments(
    state: tauri::State<'_, PoolState>,
    micrograph_id: Option<String>,
) -> Result<String, String> {
    use crate::models::segment::Segment;
    use crate::schema::segments::dsl;

    let results = match micrograph_id {
        Some(micrograph_id) => dsl::segments
            .filter(dsl::micrograph_id.eq(micrograph_id))
            .load::<Segment>(&mut get_connection(state).unwrap()),
        None => dsl::segments.load::<Segment>(&mut get_connection(state).unwrap()),
    };

    match results {
        Ok(results) => Ok(serde_json::to_string(&results).unwrap()),
        Err(e) => Err("Error loading segments: ".to_string() + &e.to_string()),
    }
}

#[tauri::command]
pub async fn get_segment(
    state: tauri::State<'_, PoolState>,
    segment_id: String,
) -> Result<String, String> {
    use crate::models::segment::Segment;
    use crate::schema::segments::dsl;

    let results = dsl::segments
        .filter(dsl::uuid.eq(segment_id))
        .load::<Segment>(&mut get_connection(state).unwrap());

    // check if segment exists
    match results {
        Ok(results) => {
            if results.len() == 0 {
                return Err("Segment not found".to_string());
            }

            Ok(serde_json::to_string(&results.first()).unwrap())
        }
        Err(e) => return Err("Error loading segment: ".to_string() + &e.to_string()),
    }
}
