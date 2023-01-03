use diesel::prelude::*;
use inno_muskelanalyse::models::Case;

#[tauri::command]
pub async fn get_cases(app: tauri::AppHandle) -> Result<String, String> {
    use inno_muskelanalyse::schema::cases::dsl::*;

    let mut connection = super::connection::establish_connection(app);

    let results = cases
        .load::<Case>(&mut connection)
        .expect("Error loading cases");
    Ok(serde_json::to_string(&results).unwrap())
}
