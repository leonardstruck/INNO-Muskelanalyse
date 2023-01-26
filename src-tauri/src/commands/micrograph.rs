use crate::data::{get_connection, PoolState};
use diesel::prelude::*;

#[tauri::command]
pub async fn get_micrographs(
    state: tauri::State<'_, PoolState>,
    case_id: Option<i32>,
) -> Result<String, String> {
    let mut connection = get_connection(state).unwrap();

    let results = match case_id {
        Some(case_id) => {
            use crate::models::micrograph::CaseMicrograph;
            use crate::schema::case_micrographs::dsl as cmdsl;

            let linked_micrographs = cmdsl::case_micrographs
                .filter(cmdsl::case_id.eq(case_id))
                .load::<CaseMicrograph>(&mut connection);

            match linked_micrographs {
                Ok(linked_micrographs) => {
                    let mut micrograph_ids = Vec::new();
                    for micrograph in linked_micrographs {
                        micrograph_ids.push(micrograph.micrograph_id);
                    }

                    use crate::models::micrograph::Micrograph;
                    use crate::schema::micrographs::dsl;

                    dsl::micrographs
                        .filter(dsl::uuid.eq_any(micrograph_ids))
                        .load::<Micrograph>(&mut connection)
                }
                Err(e) => Err(e),
            }
        }
        None => {
            use crate::models::micrograph::Micrograph;
            use crate::schema::micrographs::dsl;

            dsl::micrographs.load::<Micrograph>(&mut connection)
        }
    };

    match results {
        Ok(results) => Ok(serde_json::to_string(&results).unwrap()),
        Err(e) => Err("Error loading micrographs: ".to_string() + &e.to_string()),
    }
}

#[tauri::command]
pub async fn import_micrographs(
    app: tauri::AppHandle,
    state: tauri::State<'_, PoolState>,
    case_id: Option<i32>,
    micrograph_paths: Vec<String>,
) -> Result<String, String> {
    extern crate futures;
    use futures::future;

    let result = future::try_join_all(micrograph_paths.into_iter().map(|path| {
        let state = state.clone();
        let app_clone = app.clone();
        async move {
            use crate::models::micrograph::NewMicrograph;
            use crate::schema::micrographs::dsl;

            // get the file name from the path (use / and \ as separators to support both unix and windows)
            let file_name = path
                .split("/")
                .last()
                .unwrap()
                .split("\\")
                .last()
                .unwrap()
                .to_string();

            let new_micrograph = NewMicrograph {
                uuid: uuid::Uuid::new_v4().to_string(),
                import_path: path.clone(),
                name: file_name,
                status: "new".to_string(),

                file_size: None,
                file_type: None,
                path: None,
                thumbnail_path: None,
            };

            let inserted_micrograph = diesel::insert_into(dsl::micrographs)
                .values(&new_micrograph)
                .execute(&mut get_connection(state.clone()).unwrap());

            match inserted_micrograph {
                Ok(_) => {
                    let uuid = new_micrograph.uuid.clone();

                    // Spawn a thread to process the micrograph
                    tauri::async_runtime::spawn(async move {
                        crate::tasks::micrograph::move_micrograph(&app_clone, uuid.clone());
                        crate::tasks::micrograph::generate_thumbnail(&app_clone, uuid);
                    });

                    // If a case ID was provided, link the micrograph to the case
                    if let Some(case_id) = case_id {
                        use crate::models::micrograph::NewCaseMicrograph;
                        use crate::schema::case_micrographs::dsl as cmdsl;

                        let new_case_micrograph = NewCaseMicrograph {
                            case_id,
                            micrograph_id: new_micrograph.uuid.clone(),
                        };

                        let inserted_case_micrograph = diesel::insert_into(cmdsl::case_micrographs)
                            .values(&new_case_micrograph)
                            .execute(&mut get_connection(state.clone()).unwrap());

                        match inserted_case_micrograph {
                            Ok(_) => Ok(()),
                            Err(e) => Err(e),
                        }
                    } else {
                        Ok(())
                    }
                }
                Err(e) => Err(e),
            }
        }
    }))
    .await;

    match result {
        Ok(_) => Ok("Successfully imported micrographs".to_string()),
        Err(e) => Err("Error loading micrographs: ".to_string() + &e.to_string()),
    }
}
