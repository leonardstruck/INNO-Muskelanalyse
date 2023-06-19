use super::{Processor, ProcessorState};
use crate::models::micrographs::Status;
use crate::state::AppState;
use log::{debug, error};
use tauri::Manager;

impl Processor {
    pub fn import_micrograph(&self) {
        let app = self.app.app_handle();
        let project_id = self.project_id.clone();
        let micrograph_id = self.micrograph_id.clone();

        tauri::async_runtime::spawn(async move {
            debug!("Importing micrograph: {:?}", micrograph_id);

            let state = app.state::<AppState>();

            match state.get_micrograph(&project_id, &micrograph_id) {
                Ok(micrograph) => {
                    match crate::image_manipulation::generate_thumbnail(
                        micrograph.import_path.clone(),
                    ) {
                        Ok(thumbnail) => {
                            debug!("Generated thumbnail for micrograph: {:?}", micrograph_id);
                            state
                                .store_thumbnail(&project_id, &micrograph_id, thumbnail)
                                .unwrap();
                        }
                        Err(err) => {
                            error!("Failed to generate thumbnail: {:?}", err);
                            return;
                        }
                    };

                    match crate::image_manipulation::generate_display(
                        micrograph.import_path.clone(),
                    ) {
                        Ok(display) => {
                            debug!("Generated display for micrograph: {:?}", micrograph_id);
                            state
                                .store_display_image(&project_id, &micrograph_id, display)
                                .unwrap();
                        }
                        Err(err) => {
                            error!("Failed to generate display: {:?}", err);
                            return;
                        }
                    };

                    match crate::image_manipulation::get_dimensions(micrograph.import_path.clone())
                    {
                        Ok(dimensions) => {
                            debug!("Got dimensions for micrograph: {:?}", micrograph_id);
                            state
                                .store_dimensions(&project_id, &micrograph_id, dimensions)
                                .unwrap();
                        }
                        Err(err) => {
                            error!("Failed to get dimensions: {:?}", err);
                            return;
                        }
                    }

                    // update micrograph status
                    state
                        .update_micrograph_status(&project_id, &micrograph_id, Status::Imported)
                        .unwrap();

                    // kick off preprocessing
                    let processor_state = app.state::<ProcessorState>();

                    let processor = processor_state.0.get(&micrograph_id.to_string());

                    match processor {
                        Some(processor) => {
                            processor.preprocess();
                        }
                        None => {
                            debug!("No processor found for micrograph: {:?}", micrograph_id);
                        }
                    }

                    // send event to project window to update micrographs
                    app.emit_to(&project_id.to_string(), "UPDATE_MICROGRAPHS", ())
                        .unwrap();
                }
                Err(err) => {
                    error!("Failed to get micrograph: {:?}", err);
                }
            }
        });
    }
}
