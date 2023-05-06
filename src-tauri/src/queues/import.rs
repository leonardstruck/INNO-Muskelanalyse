use std::{collections::VecDeque, thread};

use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::state::MutableAppState;

pub struct ImportQueueItem {
    pub micrograph_uuid: String,
}

#[derive(Default)]
pub struct ImportQueue {
    queue: VecDeque<ImportQueueItem>,
    handle: Option<tauri::async_runtime::JoinHandle<()>>,
}

impl ImportQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            handle: None,
        }
    }

    pub fn push(&mut self, item: ImportQueueItem) {
        self.queue.push_back(item);
    }

    pub fn pop(&mut self) -> Option<ImportQueueItem> {
        self.queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn remove(&mut self, uuid: &str) {
        self.queue.retain(|item| item.micrograph_uuid != uuid);
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn populate(&mut self, connection: &mut SqliteConnection) {
        use crate::models::micrographs::{MicrographWithUuidAndStatus, Status};
        use crate::schema::micrographs::dsl::*;
        use diesel::prelude::*;

        let unfinished_micrographs = micrographs
            .select((uuid, status))
            .filter(status.is(Status::Pending))
            .load::<MicrographWithUuidAndStatus>(connection)
            .unwrap();

        for micrograph in unfinished_micrographs {
            self.push(ImportQueueItem {
                micrograph_uuid: micrograph.uuid,
            });
        }
    }

    pub fn start(&mut self, app: tauri::AppHandle, window_id: uuid::Uuid) {
        // check if queue is already running
        if self.handle.is_some() {
            return;
        }

        // start queue
        let handle = tauri::async_runtime::spawn(async move {
            use crate::models::micrographs::{Micrograph, Status};
            use crate::schema::micrographs::dsl::*;

            use tauri::Manager;

            loop {
                let item = {
                    // get state
                    let state = app.state::<MutableAppState>();
                    let mut state = state.0.lock().unwrap();

                    // get window
                    let window = state.windows.get_mut(&window_id).unwrap();

                    // get next item
                    window.import_queue.pop()
                };

                // if there is no next item, wait and try again
                if item.is_none() {
                    println!("No item in import queue, waiting...");
                    // sleep for 1 second
                    thread::sleep(std::time::Duration::from_secs(10));
                    continue;
                }

                let item = item.unwrap();

                println!("Importing micrograph {}", item.micrograph_uuid);

                let micrograph = {
                    // get state
                    let state = app.state::<MutableAppState>();
                    let mut state = state.0.lock().unwrap();

                    // get window
                    let window = state.windows.get_mut(&window_id).unwrap();

                    // get connection
                    let connection = window.connection.as_mut().unwrap();

                    let micrograph = micrographs
                        .filter(uuid.eq(item.micrograph_uuid.clone()))
                        .first::<Micrograph>(connection);

                    // if there is an error, print it and continue
                    if micrograph.is_err() {
                        println!("Error getting micrograph: {:?}", micrograph);
                        continue;
                    }

                    let micrograph = micrograph.unwrap() as Micrograph;

                    micrograph
                };

                // check if micrograph is already imported
                if micrograph.status != Status::Pending {
                    println!("Micrograph already imported");
                    continue;
                }

                // check if import_path is still valid
                if !std::path::Path::new(&micrograph.import_path).exists() {
                    println!("Import path does not exist");
                    continue;
                }

                let thumbnail = match crate::image_manipulation::generate_tumbnail(
                    micrograph.import_path.clone(),
                ) {
                    Ok(thumbnail) => thumbnail,
                    Err(e) => {
                        println!("Error generating thumbnail: {:?}", e);
                        continue;
                    }
                };

                // update micrograph
                let update_result = {
                    use crate::schema::micrographs::dsl::*;

                    // get state
                    let state = app.state::<MutableAppState>();
                    let mut state = state.0.lock().unwrap();

                    // get window
                    let window = state.windows.get_mut(&window_id).unwrap();

                    // get connection
                    let connection = window.connection.as_mut().unwrap();

                    diesel::update(micrographs.filter(uuid.eq(item.micrograph_uuid)))
                        .set((status.eq(Status::Imported), thumbnail_img.eq(thumbnail)))
                        .execute(connection)
                };

                println!("Micrograph: {:?}", micrograph);
            }
        });

        self.handle = Some(handle);
    }
}
