use diesel::prelude::*;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::case::Case;

use crate::schema::{case_micrographs, micrographs};

#[derive(Queryable, Serialize, TS, Identifiable, Insertable)]
#[diesel(primary_key(uuid))]
#[ts(export)]
pub struct Micrograph {
    pub uuid: String,
    pub name: String,
    pub path: Option<String>,
    pub import_path: String,
    pub thumbnail_path: Option<String>,
    pub file_size: Option<i32>,
    pub file_type: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
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
    pub import_path: String,
    pub thumbnail_path: Option<String>,
    pub file_size: Option<i32>,
    pub file_type: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
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
