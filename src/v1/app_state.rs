use rocket::tokio::sync::RwLock;
use tempfile::{tempdir, TempDir};

pub type AppStatePointer = RwLock<AppState>;

pub struct AppState {
    temp_dir: TempDir,
}

impl AppState {
    pub fn new() -> AppStatePointer {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let app_state = AppState { temp_dir };

        RwLock::new(app_state)
    }

    pub fn temp_dir(&self) -> &TempDir {
        &self.temp_dir
    }
}
