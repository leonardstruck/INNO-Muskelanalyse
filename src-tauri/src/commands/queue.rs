use crate::queues::import::ImportQueue;
use crate::queues::preprocessing::PreprocessingQueue;
use crate::queues::QueueLengths;

#[tauri::command]
pub async fn queue_get_status(
    import_queue: tauri::State<'_, ImportQueue>,
    preprocessing_queue: tauri::State<'_, PreprocessingQueue>,
) -> Result<QueueLengths, String> {
    // get queue pressure
    let lengths = QueueLengths {
        import_queue: import_queue.len(),
        preprocessing_queue: preprocessing_queue.len(),
    };

    Ok(lengths)
}
