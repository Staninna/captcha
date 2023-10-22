use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorMsg {
    msg: String,
    captcha_id: Option<String>,
}

impl ErrorMsg {
    pub fn new(msg: String, captcha_id: Option<String>) -> Self {
        Self { msg, captcha_id }
    }
}
