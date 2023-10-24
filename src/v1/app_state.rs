use super::Captcha;
use hashbrown::HashMap;
use rocket::tokio::sync::RwLock;
use tempfile::{tempdir, TempDir};

pub type AppStatePointer = RwLock<AppState>;
pub struct AppState {
    temp_dir: TempDir,
    auth_tokens: Vec<String>,
    captchas: HashMap<String, Captcha>,
    image_urls: HashMap<String, String>,
}

impl AppState {
    pub fn new() -> AppStatePointer {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let auth_tokens = vec!["TOKEN".to_string()];
        let captchas = HashMap::new();
        let image_urls = HashMap::new();

        let app_state = AppState {
            temp_dir,
            auth_tokens,
            captchas,
            image_urls,
        };

        RwLock::new(app_state)
    }

    pub fn temp_dir(&self) -> &TempDir {
        &self.temp_dir
    }

    pub fn captchas(&self) -> &HashMap<String, Captcha> {
        &self.captchas
    }

    pub fn authed(&self, token: &str) -> bool {
        self.auth_tokens.contains(&token.to_string())
    }

    pub fn add_captcha(&mut self, captcha: Captcha) {
        self.captchas.insert(captcha.id().to_string(), captcha);
    }

    pub fn add_image_url(&mut self, id: &str, url: &str) {
        self.image_urls.insert(id.to_string(), url.to_string());
    }

    pub fn get_image_url(&self, id: &str) -> Option<&String> {
        self.image_urls.get(id)
    }

    pub fn remove_image_url(&mut self, id: &str) {
        self.image_urls.remove(id);
    }
}
