use log::{debug, info};
use std::{collections::VecDeque, sync::Mutex, thread};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PreprocessingQueueItem {
    pub project_uuid: Uuid,
    pub micrograph_uuid: Uuid,
}

pub struct PreprocessingQueue(pub Mutex<PreprocessingQueueInner>);

pub struct PreprocessingQueueInner {
    items: VecDeque<PreprocessingQueueItem>,
    app_handle: AppHandle,
}

impl PreprocessingQueue {
    pub fn new(app_handle: AppHandle) -> Self {
        Self(Mutex::new(PreprocessingQueueInner {
            items: VecDeque::new(),
            app_handle,
        }))
    }

    pub fn push(&self, item: PreprocessingQueueItem) {
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

        info!("Initialized preprocessing queue");
    }

    fn pop_front(&self) -> Option<PreprocessingQueueItem> {
        let mut queue = self.0.lock().unwrap();
        queue.items.pop_front()
    }
}

async fn runner(app_handle: AppHandle) {
    let workers = 4;
    let queue = app_handle.state::<self::PreprocessingQueue>();

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
            debug!("No items in preprocessing queue. Sleeping for 10 seconds.");
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

async fn process_item(app_handle: AppHandle, item: PreprocessingQueueItem) {}
