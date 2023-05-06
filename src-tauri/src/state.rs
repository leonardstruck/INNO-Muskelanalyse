use crate::queues::import::ImportQueue;
use multi_map::MultiMap;
use std::{path::PathBuf, sync::Mutex};
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
    pub import_queue: ImportQueue,
}
