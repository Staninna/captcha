use super::{ConfigKey, ConfigValue, Filters};
use crate::conf_get;
use chrono::{DateTime, Duration, Utc};
use hashbrown::HashMap;
use serde::Serialize;
use std::ops::Add;
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct Captcha {
    #[serde(skip_serializing)]
    image: Vec<u8>,
    #[serde(skip_serializing)]
    code: String,

    id: String,
    expire_time: DateTime<Utc>,
    url: String,
}

impl Captcha {
    pub fn new(
        config: &HashMap<ConfigKey, ConfigValue>,
        length: Option<u32>,
        width: Option<u32>,
        height: Option<u32>,
        filters: Option<&mut Filters>,
    ) -> Result<Self, String> {
        let mut captcha = captcha::Captcha::new();

        let length = length.unwrap_or(conf_get!(&config, "CAPTCHA_LENGTH", u32));
        captcha.add_chars(length);

        if let Some(filters) = filters {
            while let Ok(_) = filters.next_filter(&mut captcha) {}
        }

        let width = width.unwrap_or(conf_get!(&config, "CAPTCHA_WIDTH", u32));
        let height = height.unwrap_or(conf_get!(&config, "CAPTCHA_HEIGHT", u32));
        captcha.view(width, height);

        let code = captcha.chars_as_string();
        let image = match captcha.as_png() {
            Some(image) => image,
            None => return Err("Failed to generate captcha image".to_string()),
        };

        let id = (Uuid::new_v4().to_string() + &Uuid::new_v4().to_string()).replace("-", "");

        let captcha_expire_time = config.get("CAPTCHA_EXPIRE_TIME").unwrap();
        let expire_time = Utc::now().add(Duration::seconds(captcha_expire_time.parse().unwrap()));

        let url_id = Uuid::new_v4().to_string();
        let url = format!("{}api/v1/img/{}", config.get("BASE_URL").unwrap(), url_id);

        Ok(Self {
            image,
            code,
            id,
            expire_time,
            url,
        })
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

    pub fn url(&self) -> &str {
        &self.url
    }
}
