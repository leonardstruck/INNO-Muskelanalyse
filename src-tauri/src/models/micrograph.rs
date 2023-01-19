use std::fs;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::schema::{
    case_micrographs::{self, dsl::*},
    micrographs::{self, dsl::*},
};

use crate::models::case::Case;

use ::uuid::Uuid;

//
// Models
//

#[derive(Queryable, Serialize, TS)]
#[ts(export)]
pub struct Micrograph {
    pub uuid: String,
    pub name: String,
    pub path: Option<String>,
    pub import_path: Option<String>,
    pub file_size: i32,
    pub file_type: String,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = micrographs)]
pub struct NewMicrograph {
    pub uuid: String,
    pub name: String,
    pub path: Option<String>,
    pub import_path: Option<String>,
    pub file_size: i32,
    pub file_type: String,
    pub status: String,
}

#[derive(Queryable, Associations, Serialize)]
#[diesel(belongs_to(Case))]
pub struct CaseMicrograph {
    pub id: i32,
    pub case_id: i32,
    pub micrograph_id: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = case_micrographs)]
pub struct NewCaseMicrograph {
    pub case_id: i32,
    pub micrograph_id: String,
}

//
// Operations
//

#[tauri::command]
pub async fn get_micrographs(app: tauri::AppHandle) -> Result<String, String> {
    let mut connection = crate::data::establish_connection(app);

    let results = micrographs.load::<Micrograph>(&mut connection);

    match results {
        Ok(results) => Ok(serde_json::to_string(&results).unwrap()),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub async fn get_micrograph_cases(
    app: tauri::AppHandle,
    query_micrograph_id: String,
) -> Result<String, String> {
    let mut connection = crate::data::establish_connection(app);

    let results = case_micrographs
        .filter(micrograph_id.eq(query_micrograph_id))
        .load::<CaseMicrograph>(&mut connection);

    match results {
        Ok(results) => Ok(serde_json::to_string(&results).unwrap()),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub async fn get_micrographs_by_case(
    app: tauri::AppHandle,
    query_case_id: i32,
) -> Result<String, String> {
    let mut connection = crate::data::establish_connection(app);

    let results = case_micrographs
        .filter(case_id.eq(query_case_id))
        .load::<CaseMicrograph>(&mut connection);

    match results {
        Ok(results) => {
            let mut found_micrographs = Vec::new();

            for case_micrograph in results {
                let micrograph = micrographs
                    .find(case_micrograph.micrograph_id)
                    .first::<Micrograph>(&mut connection);

                match micrograph {
                    Ok(micrograph) => found_micrographs.push(micrograph),
                    Err(error) => return Err(error.to_string()),
                }
            }

            Ok(serde_json::to_string(&found_micrographs).unwrap())
        }
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub async fn import_micrographs(
    app: tauri::AppHandle,
    micrograph_paths: Vec<String>,
    link_to_case: Option<i32>,
) {
    let mut connection = crate::data::establish_connection(app);

    for micrograph_path in micrograph_paths {
        let file_size_from_file = fs::metadata(micrograph_path.clone()).unwrap().len() as i32;
        let file_mime_type = mime_guess::from_path(micrograph_path.clone())
            .first()
            .unwrap()
            .to_string();

        let new_micrograph = NewMicrograph {
            uuid: Uuid::new_v4().to_string(),
            name: micrograph_path.clone(),
            import_path: Some(micrograph_path.clone()),
            path: None,
            file_size: file_size_from_file,
            file_type: file_mime_type,
            status: "new".to_string(),
        };

        // Insert micrograph and get the ID
        let result = diesel::insert_into(micrographs)
            .values(&new_micrograph)
            .execute(&mut connection);

        match result {
            Ok(_) => {
                if link_to_case.is_some() {
                    let case_micrograph = NewCaseMicrograph {
                        case_id: link_to_case.unwrap(),
                        micrograph_id: new_micrograph.uuid,
                    };

                    let result = diesel::insert_into(case_micrographs)
                        .values(&case_micrograph)
                        .execute(&mut connection);

                    match result {
                        Ok(_) => {}
                        Err(error) => println!("Error inserting case micrograph: {}", error),
                    }
                }
            }
            Err(error) => println!("Error inserting micrograph: {}", error),
        }
    }
}
