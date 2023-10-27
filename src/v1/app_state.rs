use super::Captcha;
use crate::{conf_get, conf_set};
use hashbrown::HashMap;
use rocket::tokio::sync::RwLock;
use url::Url;

type CaptchaId = String;
type CaptchaUrl = String;
type AuthToken = String;

pub type ConfigKey = String;
pub type ConfigValue = String;

pub type AppStatePointer = RwLock<AppState>;
pub struct AppState {
    auth_token: AuthToken,
    captchas: HashMap<CaptchaId, Captcha>,
    urls: HashMap<CaptchaUrl, CaptchaId>,

    config: HashMap<ConfigKey, ConfigValue>,
}

impl AppState {
    pub fn new() -> AppStatePointer {
        dotenv::dotenv().ok();

        let mut config = HashMap::new();
        conf_set!(config, "AUTH_TOKEN", String);
        conf_set!(config, "BASE_URL", Url);
        conf_set!(config, "CAPTCHA_EXPIRE_TIME", u64);
        conf_set!(config, "CAPTCHA_LENGTH", usize);
        conf_set!(config, "CAPTCHA_WIDTH", usize);
        conf_set!(config, "CAPTCHA_HEIGHT", usize);

        dbg!(&config);
        // dbg url
        dbg!(conf_get!(config, "BASE_URL", Url));
        let auth_token = conf_get!(config, "AUTH_TOKEN", String);
        let captchas = HashMap::new();
        let urls = HashMap::new();

        let app_state = AppState {
            auth_token,
            captchas,
            urls,

            config,
        };

        RwLock::new(app_state)
    }

    pub fn config(&self) -> &HashMap<ConfigKey, ConfigValue> {
        &self.config
    }

    pub fn authed(&self, token: &str) -> bool {
        self.auth_token == token
    }

    pub fn captchas(&self) -> &HashMap<CaptchaId, Captcha> {
        &self.captchas
    }

    pub fn add_captcha(&mut self, captcha: Captcha) {
        self.captchas.insert(captcha.id().to_string(), captcha);
    }

    pub fn remove_captcha(&mut self, captcha_id: &str) {
        self.captchas.remove(captcha_id);

        let mut urls_to_remove = Vec::new();
        for (url, id) in &self.urls {
            if id == captcha_id {
                urls_to_remove.push(url.to_string());
            }
        }

        for url in urls_to_remove {
            self.urls.remove(&url);
        }
    }

    pub fn urls(&self) -> &HashMap<CaptchaUrl, CaptchaId> {
        &self.urls
    }

    pub fn add_url(&mut self, url: &str, captcha_id: &str) {
        self.urls.insert(captcha_id.to_string(), url.to_string());
    }
}
