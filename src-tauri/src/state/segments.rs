use super::AppState;
use crate::models::segments::{NewSegment, Segment, SegmentChangeset};
use diesel::prelude::*;
use uuid::Uuid;

impl AppState {
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

    pub fn get_segments_by_micrograph(
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
