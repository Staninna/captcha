use std::path::PathBuf;

use super::{CHARACTERS, FONT_FILE, HEIGHT, WIDTH};
use image::Rgba;
use imageproc::{definitions::Image, drawing::draw_text};
use lazy_static::lazy_static;
use rand::Rng;
use rusttype::Scale;
use serde::Serialize;
use tempfile::TempDir;
use uuid::Uuid;

// TODO: Add valid until field to captcha
// if expired, delete the captcha image from the server
#[derive(Serialize)]
pub struct Captcha {
    code: String,
    image_id: String,

    // DEBUG: Remove this field
    image_path: PathBuf,
}

impl Captcha {
    pub async fn new(length: usize, temp_dir: &TempDir) -> Self {
        let code = Self::gen_code(length);
        let image_id = (Uuid::new_v4().to_string() + &Uuid::new_v4().to_string()).replace("-", "");

        let image = Self::gen_img(&code);

        let image_path = Self::save_img(&image, &image_id, temp_dir);

        Self {
            code,
            image_id,
            image_path,
        }
    }
}

// Private methods
impl Captcha {
    fn gen_code(length: usize) -> String {
        let mut code = String::new();
        let mut rng = rand::thread_rng();

        for _ in 0..length {
            let index = rng.gen_range(0..CHARACTERS.len());
            code.push(CHARACTERS[index]);
        }

        code
    }

    fn gen_img(code: &str) -> Image<Rgba<u8>> {
        lazy_static! {
            static ref FONT: rusttype::Font<'static> =
                rusttype::Font::try_from_bytes(FONT_FILE).unwrap();
        };

        let mut image = Image::new(WIDTH, HEIGHT);
        let mut rng = rand::thread_rng();

        // Random background noise
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let color = Rgba([
                    rng.gen_range(150..255),
                    rng.gen_range(150..255),
                    rng.gen_range(150..255),
                    255,
                ]);

                image.put_pixel(x, y, color);
            }
        }

        // Seperate image into parts for each character but with randomness
        let part_width = WIDTH / code.len() as u32;

        for (i, c) in code.chars().enumerate() {
            let scale = rng.gen_range(HEIGHT as f32 * 0.45..HEIGHT as f32 * 0.8);

            let x = (i as u32 * part_width) as i32;
            let y = (rng.gen_range(0..(HEIGHT as f32 - scale) as u32)) as i32;
            let scale = Scale::uniform(scale);

            // Random bright color
            let color = Rgba([
                rng.gen_range(100..150),
                rng.gen_range(100..150),
                rng.gen_range(100..150),
                rng.gen_range(200..255),
            ]);

            // Draw character
            image = draw_text(&mut image, color, x, y, scale, &FONT, &c.to_string());
        }

        image
    }

    fn save_img(image: &Image<Rgba<u8>>, image_id: &str, temp_dir: &TempDir) -> PathBuf {
        let file_path = temp_dir.path().join(image_id.to_string() + ".png");
        image.save(&file_path).unwrap();
        file_path
    }
}
