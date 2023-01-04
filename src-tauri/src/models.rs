use crate::schema::cases;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Case {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = cases)]
pub struct NewCase {
    pub name: String,
    pub description: String,
}
