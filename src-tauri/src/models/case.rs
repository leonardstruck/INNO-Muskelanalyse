use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::schema::cases;

//
// Models
//

#[derive(Queryable, Serialize, Identifiable, TS)]
#[ts(export)]
pub struct Case {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = cases)]
pub struct NewCase {
    pub name: String,
    pub description: String,
}
