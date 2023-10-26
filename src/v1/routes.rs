use super::{captcha::Level, AppStatePointer, Captcha, Response};
use crate::conf_get;
use rocket::{get, post, response::Responder, serde::json::Json, State};
use uuid::Uuid;

#[derive(Responder)]
#[response(status = 200, content_type = "image/png")]
pub struct CaptchaImage(pub Vec<u8>);

// Create a new captcha
#[get("/new?<level>&<auth>")]
pub async fn new_captcha(
    auth: String,
    level: Option<u32>,
    app_state: &State<AppStatePointer>,
) -> Result<Json<Captcha>, Json<Response>> {
    let mut app_state = app_state.write().await;
    match app_state.authed(&auth) {
        true => (),
        false => {
            let mut response = Response::new();
            response.set_error("Not authorized");

            return Err(Json(response));
        }
    };

    let config = app_state.config();
    let captcha_level = conf_get!(&config, "CAPTCHA_LEVEL", u8);
    let captcha_level = Level::from(captcha_level);

    let level = match level {
        Some(level) => match level {
            1..=3 => Level::Easy(level as u8),
            4..=6 => Level::Normal(level as u8),
            7..=9 => Level::Hard(level as u8),
            _ => {
                let mut response = Response::new();
                response.set_error("Invalid captcha level");

                return Err(Json(response));
            }
        },
        None => captcha_level,
    };

    let config = app_state.config();
    let captcha = Captcha::new(level, config).await;

    app_state.add_captcha(captcha.clone());

    Ok(Json(captcha))
}

// Request a captcha image
#[get("/image?<id>&<auth>")]
pub async fn captcha_image(
    id: String,
    auth: String,
    app_state: &State<AppStatePointer>,
) -> Result<CaptchaImage, Json<Response>> {
    let mut app_state = app_state.write().await;
    match app_state.authed(&auth) {
        true => (),
        false => {
            let mut response = Response::new();
            response.set_error("Not authorized");

            return Err(Json(response));
        }
    }

    let captcha = match app_state.captchas().get(&id) {
        Some(captcha) => captcha,
        None => {
            let mut response = Response::new();
            response.set_warning("Captcha not found");

            return Err(Json(response));
        }
    };

    match captcha.expired() {
        true => {
            app_state.remove_captcha(&id);

            let mut response = Response::new();
            response.set_warning("Captcha expired");

            return Err(Json(response));
        }
        false => (),
    }

    Ok(CaptchaImage(captcha.image().clone()))
}

// Request a captcha image url
#[get("/image_url?<id>&<auth>")]
pub async fn captcha_image_url(
    id: String,
    auth: String,
    app_state: &State<AppStatePointer>,
) -> Option<Json<Response>> {
    let mut app_state = app_state.write().await;
    match app_state.authed(&auth) {
        true => (),
        false => {
            let mut response = Response::new();
            response.set_error("Not authorized");

            return Some(Json(response));
        }
    }

    let captcha = match app_state.captchas().get(&id) {
        Some(captcha) => captcha,
        None => {
            let mut response = Response::new();
            response.set_warning("Captcha not found");

            return Some(Json(response));
        }
    };

    match captcha.expired() {
        true => {
            app_state.remove_captcha(&id);
            let mut response = Response::new();
            response.set_warning("Captcha expired");

            return Some(Json(response));
        }
        false => (),
    }

    let unique_id = Uuid::new_v4().to_string() + &Uuid::new_v4().to_string();
    app_state.add_url(&id, &unique_id);
    let base_url = conf_get!(app_state.config(), "BASE_URL", String);
    let url = format!("{}/api/v1/img/{}", base_url, unique_id);

    let mut response = Response::new();
    response.set_url(&url);

    Some(Json(response))
}

#[get("/img/<id>")]
pub async fn captcha_image_url_redirect(
    id: String,
    app_state: &State<AppStatePointer>,
) -> Option<CaptchaImage> {
    let mut app_state = app_state.write().await;
    let captcha_id = match app_state.urls().get(&id) {
        Some(captcha_id) => captcha_id.clone(),
        None => return None,
    };
    let captcha = match app_state.captchas().get(captcha_id.as_str()) {
        Some(captcha) => captcha,
        None => return None,
    };

    match captcha.expired() {
        true => {
            app_state.remove_captcha(&captcha_id);
            return None;
        }
        false => (),
    }

    Some(CaptchaImage(captcha.image().clone()))
}

// Verify the captcha code
#[post("/verify?<id>&<code>&<auth>")]
pub async fn verify_captcha(
    id: String,
    code: String,
    auth: String,
    app_state: &State<AppStatePointer>,
) -> Json<Response> {
    let mut app_state = app_state.write().await;
    match app_state.authed(&auth) {
        true => (),
        false => {
            let mut response = Response::new();
            response.set_error("Not authorized");

            return Json(response);
        }
    }

    let captcha = match app_state.captchas().get(&id) {
        Some(captcha) => captcha,
        None => {
            let mut response = Response::new();
            response.set_warning("Captcha not found");

            return Json(response);
        }
    };

    match captcha.expired() {
        true => {
            app_state.remove_captcha(&id);

            let mut response = Response::new();
            response.set_warning("Captcha expired");

            return Json(response);
        }
        false => (),
    }

    let result = match captcha.verify(&code) {
        true => "Captcha verified",
        false => "Captcha not verified",
    };

    app_state.remove_captcha(&id);

    let mut response = Response::new();
    response.set_ok(result);

    Json(response)
}

#[get("/help")]
pub async fn help() -> &'static str {
    "API v1 Documentation

Welcome to the documentation for API v1. This API provides the following endpoints for managing captchas:

Create a New Captcha
  - Endpoint: GET /api/v1/new&auth=<auth_token>&level=<level>
  - Description: Creates a new captcha with the given length
  - Returns:
    - A Captcha object with the captcha image id and expiration time
    - A error message
  - Parameters:
    - level: Difficulty level of the captcha: (Optional)
      | Level | Description |
      | ----- | ----------- |
      | 1-3   | Easy        |
      | 4-6   | Normal      |
      | 7-9   | Hard        |
    - auth_token: Your auth token

Get Captcha Image
  - Endpoint: GET /api/v1/image?id=<captcha_id>&auth=<auth_token>
  - Description: Returns the captcha image
  - Returns:
    - The captcha image
    - A error/warning message
  - Parameters:
    - captcha_id: Id of the captcha to get the image of
    - auth_token: Your auth token

Get Captcha Image URL
    - Endpoint: GET /api/v1/image_url?id=<captcha_id>&auth=<auth_token>
    - Description: Returns the captcha image url
    - Returns:
        - A url to the captcha image in a JSON object
        - A error/warning message
    - Parameters:
        - captcha_id: Id of the captcha to get the image url of
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
