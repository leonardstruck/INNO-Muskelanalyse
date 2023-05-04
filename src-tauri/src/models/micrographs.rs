use crate::schema::micrographs;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Queryable};

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(uuid))]
pub struct Micrograph {
    pub uuid: String,
    pub name: String,
    pub path: Option<String>,
    pub import_path: String,
    pub thumbnail_img: Option<Vec<u8>>,
    pub display_img: Option<Vec<u8>>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
