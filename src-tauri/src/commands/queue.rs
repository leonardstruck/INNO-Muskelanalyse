use crate::queues::QueueLengths;
use crate::state::MutableAppState;

#[tauri::command]
pub async fn queue_get_status(
    app: tauri::AppHandle,
    window: tauri::Window,
    state: tauri::State<'_, MutableAppState>,
) -> Result<QueueLengths, String> {
    // lock state
    let state = state.0.lock().unwrap();

    // get window state
    let window_state = state
        .windows
        .get(&uuid::Uuid::parse_str(window.label()).unwrap())
        .unwrap();

    // get queue pressure
    let lengths = QueueLengths {
        import_queue: window_state.import_queue.len(),
    };

    Ok(lengths)
}
