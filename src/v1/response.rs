use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    ok: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warn: Option<String>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            ok: None,
            warn: None,
            error: None,
        }
    }

    pub fn set_ok(&mut self, ok: &str) {
        self.ok = Some(ok.to_string());
    }

    pub fn set_warn(&mut self, warning: &str) {
        self.warn = Some(warning.to_string());
    }

    pub fn set_error(&mut self, error: &str) {
        self.error = Some(error.to_string());
    }
}
