use std::{collections::VecDeque, sync::Mutex, thread};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::{models::micrographs::Status, state::MutableAppState};

#[derive(Debug, Clone)]
pub struct ImportQueueItem {
    pub project_uuid: String,
    pub micrograph_uuid: String,
}

pub struct ImportQueue(pub Mutex<ImportQueueInner>);

pub struct ImportQueueInner {
    items: VecDeque<ImportQueueItem>,
    handle: Option<tauri::async_runtime::JoinHandle<()>>,
    app_handle: AppHandle,
}

impl ImportQueue {
    pub fn new(app_handle: AppHandle) -> Self {
        let mut queue = Self(Mutex::new(ImportQueueInner {
            items: VecDeque::new(),
            handle: None,
            app_handle,
        }));
        queue.start();

        queue
    }

    pub fn push(&self, item: ImportQueueItem) {
        let mut queue = self.0.lock().unwrap();
        queue.items.push_back(item);
    }

    pub fn remove(&self, project_uuid: &str, micrograph_uuid: &str) {
        let mut queue = self.0.lock().unwrap();
        queue.items.retain(|item| {
            item.project_uuid != project_uuid || item.micrograph_uuid != micrograph_uuid
        });
    }

    pub fn len(&self) -> usize {
        let queue = self.0.lock().unwrap();
        queue.items.len()
    }

    pub fn start(&mut self) {
        // check if the queue is already running (handle isSome)
        let is_running = {
            let queue = self.0.lock().unwrap();
            queue.handle.is_some()
        };

        // return if isRunning
        if is_running {
            return;
        }

        let app_handle = {
            let queue = self.0.lock().unwrap();
            queue.app_handle.app_handle()
        };

        let handle = tauri::async_runtime::spawn(runner(app_handle));

        {
            let mut queue = self.0.lock().unwrap();
            queue.handle = Some(handle);
        }
    }

    fn pop_front(&self) -> Option<ImportQueueItem> {
        let mut queue = self.0.lock().unwrap();
        queue.items.pop_front()
    }
}

async fn runner(app_handle: AppHandle) {
    loop {
        let item = {
            let queue = app_handle.state::<self::ImportQueue>();
            queue.pop_front()
        };

        if item.is_none() {
            println!("Import Queue is empty. Waiting...");
            thread::sleep(std::time::Duration::from_secs(10));
            continue;
        }

        let item = item.unwrap();

        tauri::async_runtime::spawn(process_item(app_handle.app_handle(), item.clone()));
    }
}

async fn process_item(app_handle: AppHandle, item: ImportQueueItem) {
    let app_state = app_handle.state::<MutableAppState>();
    let project_id = Uuid::parse_str(&item.project_uuid).unwrap();
    let micrograph_id = Uuid::parse_str(&item.micrograph_uuid).unwrap();

    let result: Result<(), String> = {
        let micrograph = app_state.get_micrograph(project_id, micrograph_id).unwrap();

        let thumbnail =
            match crate::image_manipulation::generate_thumbnail(micrograph.import_path.clone()) {
                Err(e) => Err(format!("Failed to generate thumbnail: {:?}", e)),
                Ok(thumbnail) => Ok(thumbnail),
            }
            .unwrap();

        let display =
            match crate::image_manipulation::generate_display(micrograph.import_path.clone()) {
                Err(e) => Err(format!("Failed to generate display image: {:?}", e)),
                Ok(display) => Ok(display),
            }
            .unwrap();

        app_state
            .store_display_image(project_id, micrograph_id, display)
            .unwrap();
        app_state
            .store_thumbnail(project_id, micrograph_id, thumbnail)
            .unwrap();

        Ok(())
    };

    match result {
        Ok(_) => {
            println!(
                "Successfully imported micrograph {}",
                item.micrograph_uuid.clone()
            );
            app_state
                .update_micrograph_status(project_id, micrograph_id, Status::Imported)
                .unwrap();
        }
        Err(e) => {
            println!("Failed to import micrograph {}: {}", micrograph_id, e);
            app_state
                .update_micrograph_status(project_id, micrograph_id, Status::Error)
                .unwrap();
        }
    }
}
