use super::{ConfigKey, ConfigValue};
use captcha::{CaptchaName, Difficulty};
use chrono::{DateTime, Duration, Utc};
use hashbrown::HashMap;
use serde::Serialize;
use std::ops::Add;
use uuid::Uuid;

// TODO: Improve Levels are currently used it is confusing
pub enum Level {
    Easy(u8),
    Normal(u8),
    Hard(u8),
}

impl From<u8> for Level {
    fn from(level: u8) -> Self {
        match level {
            1..=3 => Level::Easy(level),
            4..=6 => Level::Normal(level),
            7..=9 => Level::Hard(level),
            _ => Level::Easy(1),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Captcha {
    #[serde(skip_serializing)]
    image: Vec<u8>,
    #[serde(skip_serializing)]
    code: String,
    id: String,
    expire_time: DateTime<Utc>,
}

impl Captcha {
    pub async fn new(level: Level, config: &HashMap<ConfigKey, ConfigValue>) -> Self {
        let difficulty = match &level {
            Level::Easy(_) => Difficulty::Easy,
            Level::Normal(_) => Difficulty::Medium,
            Level::Hard(_) => Difficulty::Hard,
        };

        // Name is based on the inner value u8 of the enum
        let name_id = match level {
            Level::Easy(n) => n,
            Level::Normal(n) => n,
            Level::Hard(n) => n,
        };

        let name = match name_id {
            1 | 4 | 7 => CaptchaName::Amelia,
            2 | 5 | 8 => CaptchaName::Lucy,
            3 | 6 | 9 => CaptchaName::Mila,
            _ => unreachable!(),
        };

        let captcha = captcha::by_name(difficulty, name);
        let code = captcha.chars().iter().collect::<String>();
        let image = captcha.as_png().expect("Failed to generate captcha image");

        let id =
            (Uuid::new_v4().to_string() + &Uuid::new_v4().to_string()).replace("-", "") + &code;

        let captcha_expire_time = config.get("CAPTCHA_EXPIRE_TIME").unwrap();
        let expire_time = Utc::now().add(Duration::seconds(captcha_expire_time.parse().unwrap()));

        Self {
            code,
            image,
            id,
            expire_time,
        }
    }

    pub fn expired(&self) -> bool {
        Utc::now() > self.expire_time
    }

    pub fn verify(&self, code: &str) -> bool {
        self.code == code
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn image(&self) -> &Vec<u8> {
        &self.image
    }
}
