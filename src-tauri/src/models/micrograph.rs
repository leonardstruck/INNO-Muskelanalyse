use diesel::backend::RawValue;
use diesel::deserialize::FromSql;
use diesel::{prelude::*, AsExpression, FromSqlRow};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::case::Case;

use crate::schema::{case_micrographs, micrographs};

use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;

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
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
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

#[derive(Queryable, Serialize, TS, Identifiable, Insertable)]
#[diesel(primary_key(uuid))]
#[ts(export)]
pub struct Micrograph {
    pub uuid: String,
    pub name: String,
    pub path: Option<String>,
    pub import_path: String,
    pub thumbnail_path: Option<String>,
    pub display_path: Option<String>,
    pub file_size: Option<i32>,
    pub file_type: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub status: Status,
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
    pub display_path: Option<String>,
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
