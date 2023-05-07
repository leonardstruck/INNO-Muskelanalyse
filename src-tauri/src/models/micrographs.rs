use std::path::PathBuf;

use crate::schema::micrographs;
use chrono::NaiveDateTime;
use diesel::{
    backend::RawValue,
    deserialize::FromSql,
    serialize::{Output, ToSql},
    sql_types::Text,
    sqlite::Sqlite,
    AsChangeset, AsExpression, FromSqlRow, Identifiable, Insertable, Queryable,
};
use serde::Serialize;
use tauri::AppHandle;
use ts_rs::TS;

#[derive(Queryable, Debug, Identifiable, Serialize, AsChangeset)]
#[diesel(primary_key(uuid))]
pub struct Micrograph {
    pub uuid: String,
    pub name: String,
    pub import_path: String,
    pub thumbnail_img: Vec<u8>,
    pub display_img: Vec<u8>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub status: Status,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug, Serialize)]
#[diesel(table_name = micrographs)]
pub struct NewMicrograph {
    pub uuid: String,
    pub name: String,
    pub import_path: String,
    pub thumbnail_img: Vec<u8>,
    pub display_img: Vec<u8>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub status: Status,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Debug)]
#[diesel(table_name = micrographs)]
pub struct MicrographWithUuidAndStatus {
    pub uuid: String,
    pub status: Status,
}

#[derive(TS, Serialize)]
#[ts(export)]
pub struct CachedMicrograph {
    pub uuid: String,
    pub name: String,
    pub import_path: String,
    pub thumbnail_img: Option<PathBuf>,
    pub display_img: Option<PathBuf>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub status: Status,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Micrograph {
    pub fn to_cache(&self, app: &AppHandle) -> CachedMicrograph {
        CachedMicrograph {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            import_path: self.import_path.clone(),
            thumbnail_img: self.get_thumbnail_path(app),
            display_img: self.get_display_path(app),
            width: self.width,
            height: self.height,
            status: self.status,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    fn get_thumbnail_path(&self, app: &AppHandle) -> Option<std::path::PathBuf> {
        // check if thumbnail exists
        if self.thumbnail_img.is_empty() {
            return None;
        }

        // generate thumbnail path
        let path = app
            .path_resolver()
            .app_cache_dir()
            .unwrap()
            .join("thumbnails")
            .join(format!("{}.png", self.uuid));

        // check if thumbnail already exists
        if path.exists() {
            return Some(path);
        }

        // create necessary directories
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        // load thumbnail from database and write to file
        std::fs::write(&path, self.thumbnail_img.clone()).unwrap();

        Some(path)
    }

    fn get_display_path(&self, app: &AppHandle) -> Option<std::path::PathBuf> {
        // check if display image exists
        if self.display_img.is_empty() {
            return None;
        }

        // generate display image path
        let path = app
            .path_resolver()
            .app_cache_dir()
            .unwrap()
            .join("display_images")
            .join(format!("{}.png", self.uuid));

        // check if display image already exists
        if path.exists() {
            return Some(path);
        }

        // create necessary directories
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        // load display image from database and write to file
        std::fs::write(&path, self.display_img.clone()).unwrap();

        Some(path)
    }
}

#[derive(AsExpression, FromSqlRow, Debug, Copy, Clone, Serialize, TS, PartialEq)]
#[ts(export)]
#[diesel(sql_type = Text)]
pub enum Status {
    Pending,
    Imported,
    Segmented,
    Error,
    Done,
}

impl ToSql<Text, Sqlite> for Status {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        let s = match self {
            Status::Pending => "pending",
            Status::Imported => "imported",
            Status::Segmented => "segmented",
            Status::Error => "error",
            Status::Done => "done",
        };

        ToSql::<Text, Sqlite>::to_sql(s, out)
    }
}

impl FromSql<Text, Sqlite> for Status {
    fn from_sql(bytes: RawValue<Sqlite>) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "pending" => Ok(Status::Pending),
            "imported" => Ok(Status::Imported),
            "segmented" => Ok(Status::Segmented),
            "error" => Ok(Status::Error),
            "done" => Ok(Status::Done),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
