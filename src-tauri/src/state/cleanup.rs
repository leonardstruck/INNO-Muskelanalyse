use super::AppState;
use diesel::prelude::*;
use log::error;
use uuid::Uuid;

impl AppState {
    pub fn vacuum(&self, project_id: &Uuid) {
        let mut state = self.0.lock().unwrap();
        let window_state = state.windows.get_mut(project_id).unwrap();
        let connection = window_state.connection.as_mut().unwrap();

        let vacuum_command = diesel::sql_query("VACUUM").execute(connection);
        if let Err(err) = vacuum_command {
            error!("Failed to vacuum database: {:?}", err);
        }
    }
}
