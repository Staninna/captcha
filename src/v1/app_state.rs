use super::Captcha;
use hashbrown::HashMap;
use rocket::tokio::sync::RwLock;

pub type AppStatePointer = RwLock<AppState>;
pub struct AppState {
    auth_tokens: Vec<String>,
    captchas: HashMap<String, Captcha>,
}

impl AppState {
    pub fn new() -> AppStatePointer {
        let auth_tokens = vec!["TOKEN".to_string()];
        let captchas = HashMap::new();

        let app_state = AppState {
            auth_tokens,
            captchas,
        };

        RwLock::new(app_state)
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
}
