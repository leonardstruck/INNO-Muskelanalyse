use std::{collections::HashMap, sync::Mutex};

use serde::Serialize;
use uuid::Uuid;

mod analysis;
mod import;
mod populate;
mod preprocess;

#[derive(Default)]
pub struct ProcessorState(pub Mutex<InnerProcessorState>);

#[derive(Default)]
pub struct InnerProcessorState {
    pub processors: HashMap<String, Processor>,
}

#[derive(Serialize, Clone)]
pub enum Status {
    Preparing,
    Processing,
    Complete,
}

pub struct Processor {
    app: tauri::AppHandle,
    pub micrograph_id: Uuid,
    pub project_id: Uuid,
    pub status: Status,
}

impl ProcessorState {
    fn add_processor(&self, processor: Processor) {
        let mut state = self.0.lock().unwrap();

        state
            .processors
            .entry(processor.micrograph_id.to_string())
            .or_insert(processor);
    }
}

impl Processor {
    pub fn new(micrograph_id: &Uuid, project_id: &Uuid, app: tauri::AppHandle) -> Self {
        let mut processor = Processor {
            app,
            micrograph_id: micrograph_id.to_owned(),
            project_id: project_id.to_owned(),
            status: Status::Preparing,
        };

        processor.populate();

        processor
    }
}
