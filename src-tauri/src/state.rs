use crate::models::queue::{QueueItem, QueueStatus};
use multi_map::MultiMap;
use std::{collections::VecDeque, path::PathBuf, sync::Mutex};
use uuid::Uuid;

#[derive(Default)]
pub struct MutableAppState(pub Mutex<AppState>);

#[derive(Default)]
pub struct AppState {
    pub windows: MultiMap<Uuid, String, WindowState>,
}

#[derive(Default)]
pub struct WindowState {
    pub id: Uuid,
    pub project_path: PathBuf,
    pub file_name: String,
    pub connection: Option<diesel::SqliteConnection>,
    pub queue: Queue,
    pub queue_status: QueueStatus,
}

#[derive(Default)]
pub struct Queue(pub VecDeque<QueueItem>);
