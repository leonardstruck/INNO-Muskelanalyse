use log::{debug, error, info};
use serde::Deserialize;
use std::{collections::VecDeque, sync::Mutex, thread};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::{models::segments::Status, state::MutableAppState};

#[derive(Debug, Clone)]
pub struct AnalysisQueueItem {
    pub project_uuid: Uuid,
    pub segment_uuid: Uuid,
}

pub struct AnalysisQueue(pub Mutex<AnalysisQueueInner>);

pub struct AnalysisQueueInner {
    items: VecDeque<AnalysisQueueItem>,
    app_handle: AppHandle,
}

impl AnalysisQueue {
    pub fn new(app_handle: AppHandle) -> Self {
        Self(Mutex::new(AnalysisQueueInner {
            items: VecDeque::new(),
            app_handle,
        }))
    }

    pub fn push(&self, item: AnalysisQueueItem) {
        let mut queue = self.0.lock().unwrap();
        queue.items.push_back(item.clone())
    }

    pub fn remove(&self, project_uuid: &Uuid, segment_uuid: &Uuid) {
        let mut queue = self.0.lock().unwrap();
        queue.items.retain(|item| {
            item.project_uuid != *project_uuid || item.segment_uuid != *segment_uuid
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

        info!("Initialized analysis queue");
    }

    fn pop_front(&self) -> Option<AnalysisQueueItem> {
        let mut queue = self.0.lock().unwrap();
        queue.items.pop_front()
    }
}

async fn runner(app_handle: AppHandle) {
    let workers = 20;
    let queue = app_handle.state::<self::AnalysisQueue>();

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
            debug!("No items in analysis queue. Sleeping for 10 seconds.");
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

async fn process_item(app_handle: AppHandle, item: AnalysisQueueItem) {
    let app_state = app_handle.state::<MutableAppState>();

    let segment = app_state.get_segment(&item.project_uuid, &item.segment_uuid);

    if segment.is_err() {
        error!(
            "Failed to get segment {} from project {}",
            item.segment_uuid, item.project_uuid
        );
        return;
    }

    let segment = segment.unwrap();

    if segment.is_none() {
        error!(
            "Segment {} from project {} does not exist",
            item.segment_uuid, item.project_uuid
        );
        return;
    }

    let segment = segment.unwrap();

    // load segment to cache
    let segment = segment.to_cache(&app_handle.app_handle());

    // spawn analysis process
    let analysis =
        tauri::api::process::Command::new(crate::utils::resolve_bin_path(&app_handle, "analysis"))
            .current_dir(crate::utils::resolve_bin_dir(&app_handle))
            .args(&[&segment.binary_img.to_str().unwrap()]);

    debug!(
        "Running analysis for segment {}: {:?}",
        segment.uuid, analysis
    );

    let analysis = analysis.output();

    if analysis.is_err() {
        error!(
            "Failed to run analysis for segment {}: {:?}",
            segment.uuid, analysis
        );
        return;
    }

    let analysis = analysis.unwrap();

    if !analysis.status.success() {
        error!(
            "Failed to run analysis for segment {}: {:?}",
            segment.uuid, analysis
        );
        return;
    }

    debug!(
        "Successfully ran analysis for segment {}: {:?}",
        segment.uuid, analysis.stdout
    );
}
