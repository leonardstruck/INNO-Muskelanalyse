use crate::models::{
    micrographs::{Micrograph, NewMicrograph},
    segments::{NewSegment, Segment, SegmentChangeset},
};
use diesel::{associations::HasTable, prelude::*};
use log::error;
use multi_map::MultiMap;
use std::{path::PathBuf, sync::Mutex};
use uuid::Uuid;

#[derive(Default)]
pub struct MutableAppState(pub Mutex<AppState>);

#[derive(Default)]
pub struct AppState {
    pub windows: MultiMap<Uuid, String, WindowState>,
}

#[derive(Default)]
pub struct WindowState {
    pub id: Uuid,
    pub project_path: PathBuf,
    pub file_name: String,
    pub connection: Option<diesel::SqliteConnection>,
}

impl MutableAppState {
    // WINDOWS

    pub fn add_window(&self, window: WindowState, path: String) {
        let mut state = self.0.lock().unwrap();
        state.windows.insert(window.id, path, window)
    }

    pub fn remove_window(&self, project_id: &Uuid) {
        let mut state = self.0.lock().unwrap();
        state.windows.remove(&project_id);
    }

    pub fn is_project_already_open(&self, path: &String) -> bool {
        let state = self.0.lock().unwrap();
        state.windows.contains_key_alt(path)
    }

    // CLEANUP
    pub fn vacuum(&self, project_id: &Uuid) {
        let mut state = self.0.lock().unwrap();
        let window_state = state.windows.get_mut(project_id).unwrap();
        let connection = window_state.connection.as_mut().unwrap();

        let vacuum_command = diesel::sql_query("VACUUM").execute(connection);
        if let Err(err) = vacuum_command {
            error!("Failed to vacuum database: {:?}", err);
        }
    }

    // MICROGRAPH

    pub fn get_micrograph(
        &self,
        project_id: &Uuid,
        micrograph_id: &Uuid,
    ) -> Result<Micrograph, String> {
        use crate::schema::micrographs::dsl::*;

        let mut state = self.0.lock().unwrap();
        let window_state = state.windows.get_mut(project_id).unwrap();
        let connection = window_state.connection.as_mut().unwrap();

        micrographs
            .filter(uuid.eq(micrograph_id.to_string()))
            .first::<Micrograph>(connection)
            .map_err(|err| {
                format!(
                    "Failed to load micrograph {}: {:?}",
                    micrograph_id.to_string(),
                    err
                )
            })
    }

    pub fn get_micrographs(&self, project_id: &Uuid) -> Result<Vec<Micrograph>, String> {
        use crate::schema::micrographs::dsl::*;

        let mut state = self.0.lock().unwrap();
        let window_state = state.windows.get_mut(&project_id).unwrap();
        let connection = window_state.connection.as_mut().unwrap();

        micrographs
            .order(created_at.desc())
            .load::<Micrograph>(connection)
            .map_err(|err| format!("Failed to load micrographs: {:?}", err))
    }

    pub fn get_micrographs_by_status(
        &self,
        project_id: &Uuid,
        status_filter: crate::models::micrographs::Status,
    ) -> Result<Vec<Micrograph>, String> {
        use crate::schema::micrographs::dsl::*;

        let mut state = self.0.lock().unwrap();
        let window_state = state.windows.get_mut(&project_id).unwrap();
        let connection = window_state.connection.as_mut().unwrap();

        micrographs
            .order(created_at.desc())
            .filter(status.eq(status_filter))
            .load(connection)
            .map_err(|err| format!("Failed to load micrographs: {:?}", err))
    }

    pub fn delete_micrograph(
        &self,
        project_id: &Uuid,
        micrograph_id: &Uuid,
    ) -> Result<usize, String> {
        use crate::schema::micrographs::dsl::*;

        let result = {
            let mut state = self.0.lock().unwrap();
            let window_state = state.windows.get_mut(&project_id).unwrap();
            let connection = window_state.connection.as_mut().unwrap();

            diesel::delete(micrographs.filter(uuid.eq(micrograph_id.to_string())))
                .execute(connection)
                .map_err(|err| format!("Failed to delete micrograph: {:?}", err))
        };

        if result.is_ok() {
            self.vacuum(project_id);
        }

        result
    }

    pub fn add_micrograph(
        &self,
        project_id: &Uuid,
        new_micrograph: NewMicrograph,
    ) -> Result<usize, String> {
        use crate::schema::micrographs::dsl::*;

        let mut state = self.0.lock().unwrap();
        let window_state = state.windows.get_mut(&project_id).unwrap();
        let connection = window_state.connection.as_mut().unwrap();

        diesel::insert_into(micrographs::table())
            .values(&new_micrograph)
            .execute(connection)
            .map_err(|err| format!("Failed to insert micrograph: {:?}", err))
    }

    pub fn update_micrograph_status(
        &self,
        project_id: &Uuid,
        micrograph_id: &Uuid,
        new_status: crate::models::micrographs::Status,
    ) -> Result<usize, String> {
        use crate::schema::micrographs::dsl::*;

        let mut state = self.0.lock().unwrap();
        let window_state = state.windows.get_mut(&project_id).unwrap();
        let connection = window_state.connection.as_mut().unwrap();

        diesel::update(micrographs.filter(uuid.eq(micrograph_id.to_string())))
            .set(status.eq(new_status))
            .execute(connection)
            .map_err(|err| format!("Failed to update micrograph status: {:?}", err))
    }

    pub fn store_thumbnail(
        &self,
        project_id: &Uuid,
        micrograph_id: &Uuid,
        thumbnail: Vec<u8>,
    ) -> Result<usize, String> {
        use crate::schema::micrographs::dsl::*;

        let mut state = self.0.lock().unwrap();
        let window_state = state.windows.get_mut(&project_id).unwrap();
        let connection = window_state.connection.as_mut().unwrap();

        diesel::update(micrographs.filter(uuid.eq(micrograph_id.to_string())))
            .set(thumbnail_img.eq(thumbnail))
            .execute(connection)
            .map_err(|err| format!("Failed to store thumbnail in database: {:?}", err))
    }

    pub fn store_display_image(
        &self,
        project_id: &Uuid,
        micrograph_id: &Uuid,
        display_image: Vec<u8>,
    ) -> Result<usize, String> {
        use crate::schema::micrographs::dsl::*;

        let mut state = self.0.lock().unwrap();
        let window_state = state.windows.get_mut(&project_id).unwrap();
        let connection = window_state.connection.as_mut().unwrap();

        diesel::update(micrographs.filter(uuid.eq(micrograph_id.to_string())))
            .set(display_img.eq(display_image))
            .execute(connection)
            .map_err(|err| format!("Failed to store display image in database: {:?}", err))
    }

    // SEGMENTS

    pub fn add_segments(
        &self,
        project_id: &Uuid,
        new_segments: Vec<NewSegment>,
    ) -> Result<usize, String> {
        use crate::schema::segments::dsl::*;

        let mut state = self.0.lock().unwrap();
        let window_state = state.windows.get_mut(&project_id).unwrap();
        let connection = window_state.connection.as_mut().unwrap();
        let result = diesel::insert_into(segments)
            .values(new_segments)
            .execute(connection)
            .map_err(|err| format!("Failed to insert segments: {:?}", err));

        result
    }

    pub fn get_segment(
        &self,
        project_id: &Uuid,
        segment_id: &Uuid,
    ) -> Result<Option<Segment>, String> {
        use crate::schema::segments::dsl::*;

        let mut state = self.0.lock().unwrap();
        let window_state = state.windows.get_mut(&project_id).unwrap();
        let connection = window_state.connection.as_mut().unwrap();
        let result = segments
            .filter(uuid.eq(segment_id.to_string()))
            .first::<Segment>(connection)
            .optional()
            .map_err(|err| format!("Failed to load segment: {:?}", err));

        result
    }

    pub fn update_segment(
        &self,
        project_id: &Uuid,
        segment: &SegmentChangeset,
    ) -> Result<usize, String> {
        use crate::schema::segments::dsl::*;

        let mut state = self.0.lock().unwrap();
        let window_state = state.windows.get_mut(&project_id).unwrap();
        let connection = window_state.connection.as_mut().unwrap();

        diesel::update(segments.filter(uuid.eq(segment.uuid.to_string())))
            .set(segment)
            .execute(connection)
            .map_err(|err| format!("Failed to update segment: {:?}", err))
    }

    pub fn _get_segments(&self, project_id: &Uuid) -> Result<Vec<Segment>, String> {
        use crate::schema::segments::dsl::*;

        let mut state = self.0.lock().unwrap();
        let window_state = state.windows.get_mut(&project_id).unwrap();
        let connection = window_state.connection.as_mut().unwrap();
        let result = segments
            .order(created_at.desc())
            .load::<Segment>(connection)
            .map_err(|err| format!("Failed to load segments: {:?}", err));

        result
    }

    pub fn _get_segments_by_micrograph(
        &self,
        project_id: &Uuid,
        micrograph_uuid: &Uuid,
    ) -> Result<Vec<Segment>, String> {
        use crate::schema::segments::dsl::*;

        let mut state = self.0.lock().unwrap();
        let window_state = state.windows.get_mut(&project_id).unwrap();
        let connection = window_state.connection.as_mut().unwrap();
        let result = segments
            .order(created_at.desc())
            .filter(micrograph_id.eq(micrograph_uuid.to_string()))
            .load::<Segment>(connection)
            .map_err(|err| format!("Failed to load segments: {:?}", err));

        result
    }

    pub fn get_segments_by_status(
        &self,
        project_id: &Uuid,
        status_filter: crate::models::segments::Status,
    ) -> Result<Vec<Segment>, String> {
        use crate::schema::segments::dsl::*;

        let mut state = self.0.lock().unwrap();
        let window_state = state.windows.get_mut(&project_id).unwrap();
        let connection = window_state.connection.as_mut().unwrap();
        let result = segments
            .order(created_at.desc())
            .filter(status.eq(status_filter))
            .load::<Segment>(connection)
            .map_err(|err| format!("Failed to load segments: {:?}", err));

        result
    }
}
