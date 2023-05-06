use crate::schema::config;
use diesel::{Insertable, Queryable};

#[derive(Queryable, Debug)]
pub struct Config {
    pub key: String,
    pub value: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = config)]
pub struct NewConfig {
    pub key: String,
    pub value: String,
}
