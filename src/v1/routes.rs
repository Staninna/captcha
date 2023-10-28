use super::{AppStatePointer, Captcha, Filters, Response};
use rocket::{get, post, response::Responder, serde::json::Json, State};

#[derive(Responder)]
#[response(status = 200, content_type = "image/png")]
pub struct CaptchaImage(pub Vec<u8>);

// Create a new captcha
#[get("/new?<len>&<width>&<height>&<filters>&<auth>")]
pub async fn new_captcha(
    auth: String,
    len: Option<u32>,
    width: Option<u32>,
    height: Option<u32>,
    filters: Option<String>,
    app_state: &State<AppStatePointer>,
) -> Result<Json<Captcha>, Json<Response>> {
    let mut app_state = app_state.write().await;
    app_state.clear_expired();

    match app_state.authed(&auth) {
        true => (),
        false => {
            let mut response = Response::new();
            response.set_error("Not authorized");

            return Err(Json(response));
        }
    };

    let mut filters = match filters {
        Some(filters) => {
            let filters_obj = Filters::new(&filters);

            if let Err(err) = &filters_obj {
                let mut response = Response::new();
                if !err.is_empty() {
                    response.set_error(&format!("Failed to parse filters: {}", err));
                    return Err(Json(response));
                }
            }

            Some(filters_obj.unwrap())
        }
        None => None,
    };

    let config = app_state.config();
    let captcha = Captcha::new(config, len, width, height, filters.as_mut());

    let url = captcha.url();
    let url_id = url.split("/").last().unwrap().to_string();
    app_state.add_url(&url_id, captcha.id());

    app_state.add_captcha(captcha.clone());

    Ok(Json(captcha))
}

#[get("/img/<id>")]
pub async fn captcha_image_url_redirect(
    id: String,
    app_state: &State<AppStatePointer>,
) -> Option<CaptchaImage> {
    let mut app_state = app_state.write().await;
    app_state.clear_expired();

    let captcha_id = match app_state.urls().get(&id) {
        Some(url) => url,
        None => return None,
    };

    let captcha = match app_state.captchas().get(captcha_id) {
        Some(captcha) => captcha,
        None => return None,
    };

    match captcha.expired() {
        true => {
            app_state.remove_captcha(&id);
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
    app_state.clear_expired();

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
            response.set_warn("Captcha not found");

            return Json(response);
        }
    };

    match captcha.expired() {
        true => {
            app_state.remove_captcha(&id);

            let mut response = Response::new();
            response.set_warn("Captcha expired");

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
  - Endpoint: GET /api/v1/new&auth=<auth_token>&len=<length>&width=<width>&height=<height>&filters=<filters>
  - Description: Creates a new captcha with the given length
  - Returns:
    - A Captcha object with the captcha image id, expiration time and image url
    - A error message
  - Parameters:
    - length: Length of the captcha code             (Optional)
    - width: Width of the captcha image              (Optional)
    - height: Height of the captcha image            (Optional)
    - filters: Filters to apply to the captcha image (Optional)
               see /api/v1/help/filters for more information on filters
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

  If you encounter any issues or have suggestions for improvement, please report them on GitHub: https://github.com/Staninna/captcha/issues"
}

// Returns documentation for filters
#[get("/help/filters")]
pub async fn filter_docs() -> &'static str {
    "Filters Documentation

Welcome to the documentation for filters. This documentation provides information on the filters how filters are formatted and how to use them.

Filters are used to add noise to the captcha image. The filter string is a semicolon-separated list of filters. Each filter is a colon-separated list of the filter type and the filter arguments. The filter arguments are comma-separated.
<filter_type>:<filter_arg1>,<filter_arg2>,<filter_arg3>;...

The following filters are available:
  - dot: Adds dots to the image
    - Arguments:
      - n: Number of dots to add
  - grid: Adds a grid to the image
    - Arguments:
      - x_gap: Horizontal gap between grid lines
      - y_gap: Vertical gap between grid lines
  - wave: Adds a wave to the image
    - Arguments:
      - f: Frequency of the wave
      - amp: Amplitude of the wave
      - direction: Direction of the wave (v/h)
  - noise: Adds noise to the image
    - Arguments:
      - prob: Probability of a pixel being set to black

Example's
  - Add 10 dots to the image:
    - dot:10
  - Add a grid with a horizontal gap of 10 and a vertical gap of 5:
    - grid:10,5
  - Add a wave with a frequency of 0.1 and an amplitude of 5:
    - wave:0.1,5,v
  - Add noise with a probability of 0.1:
    - noise:0.1
  - Add 10 dots, a grid with a horizontal gap of 10 and a vertical gap of 5, a wave with a frequency of 0.1 and an amplitude of 5, and noise with a probability of 0.1:
    - dot:10;grid:10,5;wave:0.1,5,v;noise:0.1

If you encounter any issues or have suggestions for improvement, please report them on GitHub: https://github.com/Staninna/captcha/issues
"
}
