use uuid::Uuid;

use crate::models::{
    micrographs::{Micrograph, MicrographWithUuidAndStatus},
    queue::{QueueInfo, QueueItem},
};

#[tauri::command]
pub async fn get_queue_info(
    app: tauri::AppHandle,
    window: tauri::Window,
) -> Result<QueueInfo, String> {
    use tauri::Manager;

    let window_id = Uuid::parse_str(window.label()).unwrap();

    // get state
    let state = app.state::<crate::state::MutableAppState>();

    // lock state
    let mut state = state.0.lock().unwrap();

    // get window
    let window_state = state.windows.get_mut(&window_id).unwrap();

    // get queue status
    let queue_status = &mut window_state.queue_status;

    Ok(QueueInfo {
        status: queue_status.clone(),
        current_item: None,
    })
}

#[tauri::command]
pub async fn queue_warmup(app: tauri::AppHandle, window: tauri::Window) -> Result<(), String> {
    use crate::schema::micrographs::dsl::*;
    use diesel::prelude::*;
    use tauri::Manager;

    let window_id = Uuid::parse_str(window.label()).unwrap();

    // get state
    let state = app.state::<crate::state::MutableAppState>();

    // lock state
    let mut state = state.0.lock().unwrap();

    // get window
    let window_state = state.windows.get_mut(&window_id).unwrap();

    // get queue status
    let queue_status = &mut window_state.queue_status;

    // check if queue is already warming up
    if *queue_status == crate::models::queue::QueueStatus::Warmup {
        println!("Queue is already warming up");
        // return Ok(());
    }

    // set queue status to warming up
    *queue_status = crate::models::queue::QueueStatus::Warmup;

    // get connection
    let connection = window_state.connection.as_mut().unwrap();

    // get micrographs that aren't done or erroneus
    let unfinished_micrographs: Vec<MicrographWithUuidAndStatus> = micrographs
        .select((uuid, status))
        .filter(status.is_not(crate::models::micrographs::Status::Done))
        .filter(status.is_not(crate::models::micrographs::Status::Error))
        .load::<MicrographWithUuidAndStatus>(connection)
        .unwrap();

    // iterate through unfinished micrographs
    for micrograph in unfinished_micrographs {
        use crate::models::micrographs::Status;
        use crate::models::queue::QueueType;
        match micrograph.status {
            Status::Pending => {
                // first add Import to queue
                let queue_item = QueueItem {
                    id: micrograph.uuid,
                    message: "starting import".into(),
                    progress: 0.0,
                    queue_type: QueueType::Import,
                };

                window_state.queue.0.push_back(queue_item);
            }
            _ => {
                println!("Unimplemented micrograph status: {:?}", micrograph.status);
                return Err("Unimplemented micrograph status".into());
            }
        }
    }

    // set queue status
    *queue_status = crate::models::queue::QueueStatus::Idle;

    Ok(())
}
