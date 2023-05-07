use log::{debug, error, info};
use std::{collections::VecDeque, sync::Mutex, thread};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::{models::micrographs::Status, state::MutableAppState};

#[derive(Debug, Clone)]
pub struct ImportQueueItem {
    pub project_uuid: Uuid,
    pub micrograph_uuid: Uuid,
}

pub struct ImportQueue(pub Mutex<ImportQueueInner>);

pub struct ImportQueueInner {
    items: VecDeque<ImportQueueItem>,
    app_handle: AppHandle,
}

impl ImportQueue {
    pub fn new(app_handle: AppHandle) -> Self {
        Self(Mutex::new(ImportQueueInner {
            items: VecDeque::new(),
            app_handle,
        }))
    }

    pub fn push(&self, item: ImportQueueItem) {
        let mut queue = self.0.lock().unwrap();
        queue.items.push_back(item.clone())
    }

    pub fn remove(&self, project_uuid: &Uuid, micrograph_uuid: &Uuid) {
        let mut queue = self.0.lock().unwrap();
        queue.items.retain(|item| {
            item.project_uuid != *project_uuid || item.micrograph_uuid != *micrograph_uuid
        });
    }

    pub fn len(&self) -> usize {
        let queue = self.0.lock().unwrap();
        queue.items.len()
    }

    pub fn start(&self) {
        let app_handle = {
            let queue = self.0.lock().unwrap();
            queue.app_handle.app_handle()
        };

        tauri::async_runtime::spawn(runner(app_handle));

        info!("Initialized import queue");
    }

    fn pop_front(&self) -> Option<ImportQueueItem> {
        let mut queue = self.0.lock().unwrap();
        queue.items.pop_front()
    }
}

async fn runner(app_handle: AppHandle) {
    let workers = 4;
    let queue = app_handle.state::<self::ImportQueue>();

    loop {
        // get maximum {workers} items from queue
        let items = {
            let mut items = Vec::new();

            for _ in 0..workers {
                if let Some(item) = queue.pop_front() {
                    items.push(item);
                }
            }

            items
        };

        if items.is_empty() {
            debug!("No items in import queue. Sleeping for 10 seconds.");
            thread::sleep(std::time::Duration::from_secs(10));
            continue;
        }

        // process items in parallel
        let mut handles = Vec::new();

        for item in items {
            let app_handle = app_handle.clone();
            let handle = tauri::async_runtime::spawn(async move {
                process_item(app_handle, item).await;
            });

            handles.push(handle);
        }

        // wait for all items to be processed
        for handle in handles {
            let _ = handle.await;
        }
    }
}

async fn process_item(app_handle: AppHandle, item: ImportQueueItem) {
    let app_state = app_handle.state::<MutableAppState>();

    debug!("Processing {:?}", item);

    let result: Result<(), String> = {
        let micrograph = app_state
            .get_micrograph(&item.project_uuid, &item.micrograph_uuid)
            .unwrap();

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
            .store_display_image(&item.project_uuid, &item.micrograph_uuid, display)
            .unwrap();
        app_state
            .store_thumbnail(&item.project_uuid, &item.micrograph_uuid, thumbnail)
            .unwrap();

        Ok(())
    };

    match result {
        Ok(_) => {
            debug!("Successfully imported {:?}", &item);
            // send event to project window to update micrographs
            app_handle
                .emit_to(&item.project_uuid.to_string(), "UPDATE_MICROGRAPHS", ())
                .unwrap();
            app_state
                .update_micrograph_status(
                    &item.project_uuid,
                    &item.micrograph_uuid,
                    Status::Imported,
                )
                .unwrap();

            // add micrograph to preprocessing queue
            let preprocessing_queue =
                app_handle.state::<crate::queues::preprocessing::PreprocessingQueue>();

            preprocessing_queue.push(crate::queues::preprocessing::PreprocessingQueueItem {
                project_uuid: item.project_uuid,
                micrograph_uuid: item.micrograph_uuid,
            });
        }
        Err(e) => {
            error!("Failed to import {:?}, {:?}", &item, e);
            app_state
                .update_micrograph_status(&item.project_uuid, &item.micrograph_uuid, Status::Error)
                .unwrap();
        }
    }
}
