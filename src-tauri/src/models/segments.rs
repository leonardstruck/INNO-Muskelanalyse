use crate::schema::segments;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Queryable};

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(uuid))]
pub struct Segment {
    pub uuid: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub binary_img: String,
    pub location_x: Option<i32>,
    pub location_y: Option<i32>,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub measured_length: Option<f32>,
    pub measured_width: Option<f32>,
    pub measured_angle: Option<f32>,
    pub micrograph_id: String,
    pub status: String,
}
