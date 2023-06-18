use crate::processor::{ProcessorState, ProcessorStatus};

#[tauri::command]
pub async fn get_processor_status(
    app: tauri::AppHandle,
    processor_state: tauri::State<'_, ProcessorState>,
    micrograph_id: String,
) -> Result<ProcessorStatus, String> {
    let uuid = uuid::Uuid::parse_str(&micrograph_id).unwrap();
    let status = processor_state.get_status(&uuid);

    match status {
        Some(status) => Ok(status),
        None => Err("Processor not found".to_string()),
    }
}
