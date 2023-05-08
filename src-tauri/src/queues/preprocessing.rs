use log::{debug, error, info};
use serde::Deserialize;
use std::{collections::VecDeque, sync::Mutex, thread};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::{
    models::segments::{NewSegment, Status},
    state::MutableAppState,
};

#[derive(Deserialize, Debug)]
struct PreprocessingResultItem {
    path: String,
    y: usize,
    x: usize,
    height: usize,
    width: usize,
}

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

async fn process_item(app_handle: AppHandle, item: PreprocessingQueueItem) {
    let app_state = app_handle.state::<MutableAppState>();

    let micrograph = app_state.get_micrograph(&item.project_uuid, &item.micrograph_uuid);

    if micrograph.is_err() {
        error!(
            "Failed to get micrograph {} from project {}",
            item.micrograph_uuid, item.project_uuid
        );
        return;
    }

    let micrograph = micrograph.unwrap();

    // validate that the original file exists
    let file_exists = std::path::Path::new(&micrograph.import_path).exists();

    if !file_exists {
        error!(
            "Original file {} for micrograph {} does not exist",
            micrograph.import_path, micrograph.uuid
        );
        return;
    }

    let cache_dir = app_handle
        .path_resolver()
        .app_cache_dir()
        .unwrap()
        .join("preprocessing")
        .join(&item.project_uuid.to_string())
        .join(&item.micrograph_uuid.to_string());

    // ensure cache directory exists
    std::fs::create_dir_all(&cache_dir).unwrap();

    let preprocessing = tauri::api::process::Command::new(crate::utils::resolve_bin_path(
        &app_handle,
        "preprocessing",
    ))
    .current_dir(crate::utils::resolve_bin_dir(&app_handle))
    .args(&[
        micrograph.import_path.replace("\\", "/").as_str(),
        cache_dir.to_str().unwrap().replace("\\", "/").as_str(),
    ]);

    debug!(
        "Running preprocessing for {}: {:?}",
        micrograph.uuid, preprocessing
    );

    let preprocessing = preprocessing.output();

    let preprocessing = match preprocessing {
        Ok(preprocessing) => preprocessing,
        Err(err) => {
            error!(
                "Failed to run preprocessing for {}: {}",
                micrograph.uuid, err
            );
            return;
        }
    };

    if !preprocessing.status.success() {
        error!(
            "Preprocessing for {} failed: {} {}",
            micrograph.uuid, &preprocessing.stderr, &preprocessing.stdout
        );
        return;
    }

    debug!("Preprocessing for {} finished", micrograph.uuid);

    // parse preprocessing results
    let preprocessing_results: Result<Vec<PreprocessingResultItem>, _> =
        serde_json::from_str(&preprocessing.stdout);

    if preprocessing_results.is_err() {
        error!(
            "Failed to parse preprocessing results for {}: {}",
            micrograph.uuid,
            preprocessing_results.err().unwrap()
        );
        return;
    }

    let preprocessing_results = preprocessing_results.unwrap();

    // add segments to database
    let mut segments = Vec::<NewSegment>::new();

    for result in preprocessing_results {
        let binary_img = std::fs::read(&result.path).unwrap();

        let segment = NewSegment {
            micrograph_id: micrograph.uuid.clone(),
            location_x: Some(result.x as i32),
            location_y: Some(result.y as i32),
            width: Some(result.width as i32),
            height: Some(result.height as i32),
            measured_angle: None,
            measured_width: None,
            measured_length: None,
            measured_midpoint_x: None,
            measured_midpoint_y: None,
            status: Status::New,
            uuid: Uuid::new_v4().to_string(),
            binary_img,
        };

        segments.push(segment);
    }

    let segments = app_state.add_segments(&item.project_uuid, segments);

    if segments.is_err() {
        error!(
            "Failed to add segments for {}: {}",
            micrograph.uuid,
            segments.err().unwrap()
        );
        return;
    }

    // update micrograph status
    let _result = app_state.update_micrograph_status(
        &item.project_uuid,
        &item.micrograph_uuid,
        crate::models::micrographs::Status::Segmented,
    );

    // send event to project window to update micrographs
    app_handle
        .emit_to(&item.project_uuid.to_string(), "UPDATE_MICROGRAPHS", ())
        .unwrap();
}
