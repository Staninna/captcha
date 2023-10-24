// TODO: Make configurable on runtime (via env vars)
pub const BASE_URL: &str = "http://localhost:8000";

pub const CHARACTERS: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', //
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', //
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', //
    'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', //
    'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', //
    'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', //
    'Y', 'Z',
];
pub const WIDTH: u32 = 100 * 3;
pub const HEIGHT: u32 = 40 * 3;
pub const FONT_FILE: &[u8] = include_bytes!("../../fonts/opensans.ttf");

pub const CAPTCHA_EXPIRE_TIME: i64 = 60 * 5; // 5 minutes
