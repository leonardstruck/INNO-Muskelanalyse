use super::{AppState, WindowState};
use uuid::Uuid;

impl AppState {
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
}
