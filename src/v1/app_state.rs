use super::Captcha;
use hashbrown::HashMap;
use rocket::tokio::sync::RwLock;

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
        let auth_token = dotenv::var("AUTH_TOKEN").expect("AUTH_TOKEN must be set");
        config.insert("AUTH_TOKEN".to_string(), auth_token.clone());
        let base_url = dotenv::var("BASE_URL").expect("BASE_URL must be set");
        config.insert("BASE_URL".to_string(), base_url.clone());
        let captcha_expire_time = dotenv::var("CAPTCHA_EXPIRE_TIME")
            .expect("CAPTCHA_EXPIRE_TIME must be set")
            .parse::<i64>()
            .expect("CAPTCHA_EXPIRE_TIME must be an integer");
        config.insert(
            "CAPTCHA_EXPIRE_TIME".to_string(),
            captcha_expire_time.to_string(),
        );
        let captcha_level = dotenv::var("CAPTCHA_LEVEL").expect("CAPTCHA_LEVEL must be set");
        config.insert("CAPTCHA_LEVEL".to_string(), captcha_level.clone());

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
