use log::debug;
use tauri::Manager;
use uuid::Uuid;

use crate::state::AppState;

use super::Processor;
use super::ProcessorState;

impl ProcessorState {
    pub fn populate(&self, app: &tauri::AppHandle, project_id: &Uuid) -> Result<(), String> {
        // get app state
        let state = app.state::<AppState>();

        // get all micrographs that are unprocessed
        let micrographs = state
            .get_micrographs_by_status_neg(project_id, crate::models::micrographs::Status::Done)?;

        if micrographs.len() == 0 {
            debug!("No micrographs to populate");
            return Ok(());
        }

        // add a processor for each micrograph
        for micrograph in micrographs {
            debug!("Adding processor for micrograph: {:?}", micrograph.uuid);
            self.add_processor(Processor::new(
                &Uuid::parse_str(&micrograph.uuid).unwrap(),
                &project_id,
                app.app_handle(),
            ))
        }

        Ok(())
    }
}

impl Processor {
    pub fn populate(&self) {
        use crate::models::micrographs::Status;

        // get app state
        let state = self.app.state::<AppState>();

        debug!(
            "Populating processor for micrograph: {:?}",
            self.micrograph_id
        );

        let status = state
            .get_micrograph_status(&self.project_id, &self.micrograph_id)
            .unwrap();

        // check if micrograph is already imported
        match status {
            Status::Pending => {
                debug!(
                    "Micrograph {} is pending, importing it now",
                    self.micrograph_id
                );
                self.import_micrograph();
            }
            Status::Imported => {
                debug!(
                    "Micrograph {} is imported, preprocessing it now",
                    self.micrograph_id
                );
                self.preprocess();
            }
            Status::Segmented => {
                debug!(
                    "Micrograph {} is segmented, skipping preprocessing",
                    self.micrograph_id
                );
            }
            Status::Done => {
                debug!(
                    "Micrograph {} is done, skipping preprocessing",
                    self.micrograph_id
                );
            }
            _ => {
                debug!(
                    "Micrograph {} is in an unknown state, skipping preprocessing",
                    self.micrograph_id
                );
            }
        }
    }
}
