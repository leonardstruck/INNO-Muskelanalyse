use serde::Serialize;

use crate::processor::ProcessorState;

#[derive(Serialize)]
pub struct ProcessorStatus {
    total_jobs: Option<usize>,
    completed_jobs: Option<usize>,
}

#[tauri::command]
pub async fn get_processor_status(
    processor_state: tauri::State<'_, ProcessorState>,
    micrograph_id: String,
) -> Result<ProcessorStatus, String> {
    let processor = match processor_state.0.get(&micrograph_id) {
        Some(processor) => processor,
        None => return Err("Processor not found".into()),
    };

    Ok(ProcessorStatus {
        total_jobs: processor.total_jobs,
        completed_jobs: processor.completed_jobs,
    })
}
