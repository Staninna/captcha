use super::{AppStatePointer, Captcha, Msg};
use rocket::{get, post, serde::json::Json, State};

#[get("/new?<len>&<auth>")]
pub async fn new_captcha(
    len: u32,
    auth: String,
    app_state: &State<AppStatePointer>,
) -> Result<Json<Captcha>, Json<Msg>> {
    let mut app_state = app_state.write().await;
    match app_state.authed(&auth) {
        true => (),
        false => return Err(Json(Msg::new("Not authorized"))),
    };

    let captcha = Captcha::new(len).await;
    app_state.add_captcha(captcha.clone());

    Ok(Json(captcha))
}

// Verify the captcha code
#[post("/verify?<id>&<code>&<auth>")]
pub async fn verify_captcha(
    id: String,
    code: String,
    auth: String,
    app_state: &State<AppStatePointer>,
) -> Json<Msg> {
    let app_state = app_state.read().await;
    match app_state.authed(&auth) {
        true => (),
        false => return Json(Msg::new("Not authorized")),
    }

    let captcha = match app_state.captchas().get(&id) {
        Some(captcha) => captcha,
        None => return Json(Msg::new("Captcha not found")),
    };

    match captcha.expired() {
        true => {
            // delete_captcha_by_id(&id, app_state).await.unwrap();
            return Json(Msg::new("Captcha expired"));
        }
        false => (),
    }

    let result = match captcha.verify(&code) {
        true => "Captcha verified",
        false => "Captcha not verified",
    };

    // delete_captcha_by_id(&id, app_state).await.unwrap();

    Json(Msg::new(result))
}

#[get("/help")]
pub async fn help() -> &'static str {
    "API v1 Documentation

Welcome to the documentation for API v1. This API provides the following endpoints for managing captchas:

Create a New Captcha
  - Endpoint: GET /api/v1/new?len=<len>&auth=<auth_token>
  - Description: Creates a new captcha with the given length
  - Returns:
    - A Captcha object with the captcha image id and expiration time
    - A error message
  - Parameters:
    - len: Length of the captcha code
    - auth_token: Your auth token

Verify Captcha Code
  - Endpoint: POST /api/v1/verify?id=<captcha_id>&code=<code>&auth=<auth_token>
  - Description: Verify the captcha code
  - Returns:
    - A status message
  - Parameters:
    - captcha_id: Id of the captcha to verify
    - code: The code to verify against
    - auth_token: Your auth token
  - Note: This is currently a GET request for debugging purposes during API development

API Help
  - Endpoint: GET /api/v1/help
  - Description: Returns this help message

Please note that this API is a work in progress, and your feedback is valuable to me. If you encounter any issues or have suggestions for improvement, please report them on GitHub: https://github.com/Staninna/captcha/issues"
}
