use serde::Serialize;
use ts_rs::TS;

pub mod import;
pub mod preprocessing;

#[derive(Debug, TS, Serialize)]
#[ts(export)]
pub struct QueueLengths {
    pub import_queue: usize,
    pub preprocessing_queue: usize,
}
