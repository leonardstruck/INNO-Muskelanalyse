use serde::Serialize;
use ts_rs::TS;

pub mod import;

#[derive(Debug, TS, Serialize)]
#[ts(export)]
pub struct QueueLengths {
    pub import_queue: usize,
}
