use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Case {
    pub id: i32,
    pub name: String,
    pub description: String,
}
