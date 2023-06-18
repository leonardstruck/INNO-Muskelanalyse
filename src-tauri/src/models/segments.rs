use std::path::PathBuf;

use crate::schema::segments;
use chrono::NaiveDateTime;
use diesel::{
    backend::Backend,
    deserialize::FromSql,
    serialize::{Output, ToSql},
    sql_types::Text,
    sqlite::Sqlite,
    AsChangeset, AsExpression, FromSqlRow, Identifiable, Insertable, Queryable,
};
use serde::Serialize;
use tauri::AppHandle;
use ts_rs::TS;

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(uuid))]
pub struct Segment {
    pub uuid: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub binary_img: Vec<u8>,
    pub location_x: Option<i32>,
    pub location_y: Option<i32>,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub measured_length: Option<f32>,
    pub measured_width: Option<f32>,
    pub measured_angle: Option<f32>,
    pub micrograph_id: String,
    pub status: Status,
    pub measured_midpoint_x: Option<f32>,
    pub measured_midpoint_y: Option<f32>,
}

#[derive(AsChangeset)]
#[diesel(table_name = segments)]
pub struct SegmentChangeset {
    pub uuid: String,
    pub location_x: Option<i32>,
    pub location_y: Option<i32>,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub measured_length: Option<f32>,
    pub measured_width: Option<f32>,
    pub measured_angle: Option<f32>,
    pub measured_midpoint_x: Option<f32>,
    pub measured_midpoint_y: Option<f32>,
    pub status: Status,
}

#[derive(TS, Serialize)]
#[ts(export)]
pub struct CachedSegment {
    pub uuid: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub binary_img: PathBuf,
    pub location_x: Option<i32>,
    pub location_y: Option<i32>,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub measured_length: Option<f32>,
    pub measured_width: Option<f32>,
    pub measured_angle: Option<f32>,
    pub measured_midpoint_x: Option<f32>,
    pub measured_midpoint_y: Option<f32>,
    pub micrograph_id: String,
    pub status: Status,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = segments)]
pub struct NewSegment {
    pub uuid: String,
    pub binary_img: Vec<u8>,
    pub location_x: Option<i32>,
    pub location_y: Option<i32>,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub measured_length: Option<f32>,
    pub measured_width: Option<f32>,
    pub measured_angle: Option<f32>,
    pub measured_midpoint_x: Option<f32>,
    pub measured_midpoint_y: Option<f32>,
    pub micrograph_id: String,
    pub status: Status,
}

#[derive(AsExpression, FromSqlRow, Debug, Copy, Clone, Serialize, TS, PartialEq)]
#[ts(export)]
#[diesel(sql_type = Text)]
pub enum Status {
    New,
    Verified,
    Error,
    Ok,
}

impl ToSql<Text, Sqlite> for Status {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        let s = match self {
            Status::New => "new",
            Status::Verified => "verified",
            Status::Error => "error",
            Status::Ok => "ok",
        };

        ToSql::<Text, Sqlite>::to_sql(s, out)
    }
}

impl FromSql<Text, Sqlite> for Status {
    fn from_sql<'a>(bytes: <Sqlite as Backend>::RawValue<'a>) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "new" => Ok(Status::New),
            "verified" => Ok(Status::Verified),
            "error" => Ok(Status::Error),
            "ok" => Ok(Status::Ok),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl Segment {
    pub fn to_cache(&self, app: &AppHandle) -> CachedSegment {
        CachedSegment {
            uuid: self.uuid.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
            binary_img: self.get_segment_path(app),
            location_x: self.location_x.clone(),
            location_y: self.location_y.clone(),
            height: self.height.clone(),
            width: self.width.clone(),
            measured_length: self.measured_length.clone(),
            measured_width: self.measured_width.clone(),
            measured_angle: self.measured_angle.clone(),
            measured_midpoint_x: self.measured_midpoint_x.clone(),
            measured_midpoint_y: self.measured_midpoint_y.clone(),
            micrograph_id: self.micrograph_id.clone(),
            status: self.status.clone(),
        }
    }

    fn get_segment_path(&self, app: &AppHandle) -> PathBuf {
        // generate segment path
        let path = app
            .path_resolver()
            .app_cache_dir()
            .unwrap()
            .join("segments")
            .join(format!("{}.png", self.uuid));

        // check if segment already exists
        if path.exists() {
            return path;
        }

        // create necessary directories
        let _ = std::fs::create_dir_all(path.parent().unwrap());

        // load segment from database and save to disk
        std::fs::write(&path, &self.binary_img).unwrap();

        path
    }
}
