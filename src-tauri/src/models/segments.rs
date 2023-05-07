use crate::schema::segments;
use chrono::NaiveDateTime;
use diesel::{
    backend::RawValue,
    deserialize::FromSql,
    serialize::{Output, ToSql},
    sql_types::Text,
    sqlite::Sqlite,
    AsExpression, FromSqlRow, Identifiable, Insertable, Queryable,
};
use serde::Serialize;
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
    fn from_sql(bytes: RawValue<Sqlite>) -> diesel::deserialize::Result<Self> {
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
