use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Serialize, TS, Default, Clone, PartialEq)]
#[ts(export)]
pub enum QueueStatus {
    Idle,
    Running,
    Paused,
    Warmup,
    #[default]
    Uninitialized,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub enum QueueType {
    Import,
    Segmentation,
    Analysis,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct QueueInfo {
    pub status: QueueStatus,
    pub current_item: Option<QueueItem>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct QueueItem {
    pub id: String,
    pub queue_type: QueueType,
    pub progress: f32,
    pub message: String,
}
