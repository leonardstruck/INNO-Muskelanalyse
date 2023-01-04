use diesel::prelude::*;
use inno_muskelanalyse::models::{self, Case};

#[tauri::command]
pub async fn get_cases(app: tauri::AppHandle) -> Result<String, String> {
    use inno_muskelanalyse::schema::cases::dsl::*;

    let mut connection = super::connection::establish_connection(app);

    let results = cases.load::<Case>(&mut connection);

    match results {
        Ok(results) => Ok(serde_json::to_string(&results).unwrap()),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub async fn get_case(app: tauri::AppHandle, case_id: i32) -> Result<String, String> {
    use inno_muskelanalyse::schema::cases::dsl::*;

    let mut connection = super::connection::establish_connection(app);

    let result = cases.find(case_id).first::<Case>(&mut connection);

    match result {
        Ok(result) => Ok(serde_json::to_string(&result).unwrap()),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub async fn create_case(app: tauri::AppHandle, case_obj: String) -> Result<String, String> {
    use inno_muskelanalyse::schema::cases::dsl::*;

    let mut connection = super::connection::establish_connection(app);

    let case: models::NewCase = serde_json::from_str(&case_obj).unwrap();

    let new_case = diesel::insert_into(cases)
        .values(&case)
        .execute(&mut connection);

    match new_case {
        Ok(new_case) => Ok(serde_json::to_string(&new_case).unwrap()),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub async fn delete_case(app: tauri::AppHandle, case_id: i32) -> Result<String, String> {
    use inno_muskelanalyse::schema::cases::dsl::*;

    let mut connection = super::connection::establish_connection(app);

    let result = diesel::delete(cases.find(case_id)).execute(&mut connection);

    match result {
        Ok(result) => Ok(serde_json::to_string(&result).unwrap()),
        Err(error) => Err(error.to_string()),
    }
}
