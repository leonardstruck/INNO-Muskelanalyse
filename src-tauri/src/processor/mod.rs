use serde::Serialize;
use uuid::Uuid;

mod analysis;
mod import;
mod populate;
mod preprocess;

#[derive(Default)]
pub struct ProcessorState(pub chashmap::CHashMap<String, Processor>);

#[derive(Serialize, Clone)]
pub enum Status {
    Preparing,
}

pub struct Processor {
    app: tauri::AppHandle,
    pub micrograph_id: Uuid,
    pub project_id: Uuid,
    pub status: Status,
    pub total_jobs: Option<usize>,
    pub completed_jobs: Option<usize>,
}

impl ProcessorState {
    fn add_processor(&self, processor: Processor) {
        self.0
            .insert(processor.micrograph_id.to_string(), processor);
    }
}

impl Processor {
    pub fn new(micrograph_id: &Uuid, project_id: &Uuid, app: tauri::AppHandle) -> Self {
        let mut processor = Processor {
            app,
            micrograph_id: micrograph_id.to_owned(),
            project_id: project_id.to_owned(),
            status: Status::Preparing,
            total_jobs: None,
            completed_jobs: None,
        };

        processor.populate();

        processor
    }
}
