use std::{fs, future::Future};

use diesel::prelude::*;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::schema::{
    case_micrographs::{self, dsl::*},
    micrographs::{self, dsl::*},
};

use crate::models::case::Case;

use crate::data::PoolState;

use ::uuid::Uuid;

//
// Models
//

#[derive(Queryable, Serialize, TS, Identifiable, Insertable)]
#[diesel(primary_key(uuid))]
#[ts(export)]
pub struct Micrograph {
    pub uuid: String,
    pub name: String,
    pub path: Option<String>,
    pub import_path: Option<String>,
    pub thumbnail_path: Option<String>,
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
    pub thumbnail_path: Option<String>,
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
pub async fn get_micrographs(
    state: tauri::State<'_, PoolState>,
    query_case_id: Option<i32>,
) -> Result<String, String> {
    let mut connection = state.0.clone().get().unwrap();

    let mut results = Vec::new();

    if query_case_id.is_some() {
        let case_micrographs_result = case_micrographs
            .filter(case_id.eq(query_case_id.unwrap()))
            .load::<CaseMicrograph>(&mut connection);

        match case_micrographs_result {
            Ok(case_micrographs_result) => {
                for case_micrograph in case_micrographs_result {
                    let micrograph = micrographs
                        .find(case_micrograph.micrograph_id)
                        .first::<Micrograph>(&mut connection);

                    match micrograph {
                        Ok(micrograph) => results.push(micrograph),
                        Err(error) => return Err(error.to_string()),
                    }
                }
            }
            Err(error) => return Err(error.to_string()),
        }
    } else {
        results = micrographs.load::<Micrograph>(&mut connection).unwrap();
    }

    Ok(serde_json::to_string(&results).unwrap())
}

#[tauri::command]
pub async fn import_micrographs(
    app: tauri::AppHandle,
    state: tauri::State<'_, PoolState>,
    micrograph_paths: Vec<String>,
    link_to_case: Option<i32>,
) -> Result<(), ()> {
    let mut connection = state.0.clone().get().unwrap();

    for micrograph_path in micrograph_paths {
        let file_size_from_file = fs::metadata(micrograph_path.clone()).unwrap().len() as i32;
        let file_mime_type = mime_guess::from_path(micrograph_path.clone())
            .first()
            .unwrap()
            .to_string();

        // get filename from path
        let filename = micrograph_path
            .clone()
            .split("/")
            .last()
            .unwrap()
            .to_string();

        let new_micrograph = NewMicrograph {
            uuid: Uuid::new_v4().to_string(),
            name: filename,
            import_path: Some(micrograph_path.clone()),
            path: None,
            thumbnail_path: None,
            file_size: file_size_from_file,
            file_type: file_mime_type,
            status: "new".to_string(),
        };

        // Insert micrograph and get the ID
        diesel::insert_into(micrographs)
            .values(&new_micrograph)
            .execute(&mut connection)
            .expect("Error inserting micrograph");

        if link_to_case.is_some() {
            let case_micrograph = NewCaseMicrograph {
                case_id: link_to_case.unwrap(),
                micrograph_id: new_micrograph.uuid.clone(),
            };

            diesel::insert_into(case_micrographs)
                .values(&case_micrograph)
                .execute(&mut connection)
                .expect("Failed to link micrograph to case");
        }

        // start a new background task to process the micrograph
        let micrograph_id_copy = new_micrograph.uuid.clone();
        let app_clone = app.clone();

        tauri::async_runtime::spawn_blocking(move || {
            _ = crate::events::micrograph::move_micrograph(app_clone, micrograph_id_copy);
        });
    }

    Ok(())
}
