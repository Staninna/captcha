use super::CAPTCHA_EXPIRE_TIME;
use captcha::Captcha as CaptchaGen;
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use std::ops::Add;
use uuid::Uuid;

#[derive(Serialize, Clone, Debug)]
pub struct Captcha {
    #[serde(skip_serializing)]
    code: String,
    image_id: String,
    expire_time: DateTime<Utc>,
}

impl Captcha {
    pub async fn new(length: u32) -> Self {
        let mut captcha = CaptchaGen::new();
        captcha.add_chars(length as u32);
        let code = captcha.chars().iter().collect::<String>();
        dbg!(&code);

        let image_id =
            (Uuid::new_v4().to_string() + &Uuid::new_v4().to_string()).replace("-", "") + &code;

        let valid_till = Utc::now().add(Duration::seconds(CAPTCHA_EXPIRE_TIME));

        Self {
            code,
            image_id,
            expire_time: valid_till,
        }
    }

    pub fn expired(&self) -> bool {
        Utc::now() > self.expire_time
    }

    pub fn verify(&self, code: &str) -> bool {
        self.code == code
    }

    pub fn id(&self) -> &str {
        &self.image_id
    }
}
