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

#[tauri::command]
pub async fn create_case(
    state: tauri::State<'_, PoolState>,
    case: String,
) -> Result<String, String> {
    use crate::models::case::NewCase;
    use crate::schema::cases::dsl;

    let mut connection = get_connection(state).unwrap();

    let new_case: NewCase = serde_json::from_str(&case).unwrap();

    let results = diesel::insert_into(dsl::cases)
        .values(&new_case)
        .execute(&mut connection);

    match results {
        Ok(results) => Ok(serde_json::to_string(&results).unwrap()),
        Err(e) => Err("Error creating case: ".to_string() + &e.to_string()),
    }
}

#[tauri::command]
pub async fn delete_case(state: tauri::State<'_, PoolState>, id: i32) -> Result<String, String> {
    use crate::schema::cases::dsl;

    let mut connection = get_connection(state).unwrap();

    let results = diesel::delete(dsl::cases.filter(dsl::id.eq(id))).execute(&mut connection);

    match results {
        Ok(results) => Ok(serde_json::to_string(&results).unwrap()),
        Err(e) => Err("Error deleting case: ".to_string() + &e.to_string()),
    }
}
