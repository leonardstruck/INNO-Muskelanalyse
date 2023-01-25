use crate::data::PoolState;
use diesel::prelude::*;

#[tauri::command]
pub async fn get_cases(state: tauri::State<'_, PoolState>) -> Result<String, String> {
    use crate::models::case::Case;
    use crate::schema::cases::dsl::*;

    let mut connection = state.0.get().unwrap();

    let results = cases
        .load::<Case>(&mut connection)
        .expect("Error loading cases");

    Ok(serde_json::to_string(&results).unwrap())
}
