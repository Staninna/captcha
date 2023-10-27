use std::ops::Add;

use super::{ConfigKey, ConfigValue};
use crate::conf_get;
use captcha::filters::{Dots, Grid, Noise, Wave};
use chrono::{DateTime, Duration, Utc};
use hashbrown::HashMap;
use serde::Serialize;
use uuid::Uuid;

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
    pub fn new(
        config: &HashMap<ConfigKey, ConfigValue>,
        length: Option<u32>,
        width: Option<u32>,
        height: Option<u32>,

        // Filters
        dots: Option<Vec<Dots>>,
        grids: Option<Vec<Grid>>,
        waves: Option<Vec<Wave>>,
        noises: Option<Vec<Noise>>,
    ) -> Self {
        let mut captcha = captcha::Captcha::new();

        let length = length.unwrap_or(conf_get!(&config, "CAPTCHA_LENGTH", u32));
        captcha.add_chars(length);

        let dots = dots.unwrap_or(vec![]);
        for dot in dots {
            captcha.apply_filter(dot);
        }

        let grids = grids.unwrap_or(vec![]);
        for grid in grids {
            captcha.apply_filter(grid);
        }

        let waves = waves.unwrap_or(vec![]);
        for wave in waves {
            captcha.apply_filter(wave);
        }

        let noises = noises.unwrap_or(vec![]);
        for noise in noises {
            captcha.apply_filter(noise);
        }

        let width = width.unwrap_or(conf_get!(&config, "CAPTCHA_WIDTH", u32));
        let height = height.unwrap_or(conf_get!(&config, "CAPTCHA_HEIGHT", u32));
        captcha.view(width, height);

        let code = captcha.chars_as_string();
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
