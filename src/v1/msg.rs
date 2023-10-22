use serde::Serialize;

#[derive(Serialize)]
pub struct Msg {
    msg: String,
}

impl Msg {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}
