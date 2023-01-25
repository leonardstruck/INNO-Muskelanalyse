use crate::data::{get_connection, PoolState};
use diesel::prelude::*;

#[tauri::command]
pub async fn get_cases(state: tauri::State<'_, PoolState>) -> Result<String, String> {
    use crate::models::case::Case;
    use crate::schema::cases::dsl::*;

    let mut connection = get_connection(state).unwrap();

    let results = cases.load::<Case>(&mut connection);

    match results {
        Ok(results) => Ok(serde_json::to_string(&results).unwrap()),
        Err(e) => Err("Error loading cases: ".to_string() + &e.to_string()),
    }
}

#[tauri::command]
pub async fn get_case(state: tauri::State<'_, PoolState>, id: i32) -> Result<String, String> {
    use crate::models::case::Case;
    use crate::schema::cases::dsl;

    let mut connection = get_connection(state).unwrap();

    let results = dsl::cases
        .filter(dsl::id.eq(id))
        .load::<Case>(&mut connection);

    match results {
        Ok(results) => Ok(serde_json::to_string(&results.first()).unwrap()),
        Err(e) => Err("Error loading case: ".to_string() + &e.to_string()),
    }
}
