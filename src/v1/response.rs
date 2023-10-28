use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    ok: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warning: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            ok: None,
            error: None,
            warning: None,
            url: None,
        }
    }

    pub fn set_ok(&mut self, ok: &str) {
        self.ok = Some(ok.to_string());
    }

    pub fn set_error(&mut self, error: &str) {
        self.error = Some(error.to_string());
    }

    pub fn set_warning(&mut self, warning: &str) {
        self.warning = Some(warning.to_string());
    }
}
