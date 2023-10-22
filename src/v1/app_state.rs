use rocket::tokio::sync::RwLock;
use tempfile::{tempdir, TempDir};

pub type AppStatePointer = RwLock<AppState>;
pub struct AppState {
    temp_dir: TempDir,
    auth_tokens: Vec<String>,
}

impl AppState {
    pub fn new() -> AppStatePointer {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let auth_tokens = vec!["TOKEN".to_string()];

        let app_state = AppState {
            temp_dir,
            auth_tokens,
        };

        RwLock::new(app_state)
    }

    pub fn temp_dir(&self) -> &TempDir {
        &self.temp_dir
    }

    pub fn authed(&self, token: &str) -> bool {
        self.auth_tokens.contains(&token.to_string())
    }
}
