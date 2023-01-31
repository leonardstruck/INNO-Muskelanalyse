use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::schema::segments;

#[derive(Queryable, Serialize, Identifiable, TS)]
#[diesel(primary_key(uuid))]
#[ts(export)]
pub struct Segment {
    pub uuid: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub filename: String,
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

#[derive(Insertable, Deserialize)]
#[diesel(table_name = segments)]
pub struct NewSegment {
    pub uuid: String,
    pub filename: String,
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

#[derive(Deserialize, Debug)]
pub struct SegmentationResponse {
    pub path: String,
    pub y: i32,
    pub x: i32,
    pub height: i32,
    pub width: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisResponse {
    pub path: String,
    pub direction_a: f32,
    pub direction_b: f32,
    pub angle: f32,
}
