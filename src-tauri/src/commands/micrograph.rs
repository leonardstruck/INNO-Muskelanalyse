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
