use super::{AppState, AppStatePointer, Captcha, Msg};
use rocket::{
    fs::NamedFile, get, response::status::NotFound, serde::json::Json,
    tokio::sync::RwLockReadGuard, State,
};

#[get("/new?<len>&<auth>")]
pub async fn new_captcha(
    len: usize,
    auth: String,
    app_state: &State<AppStatePointer>,
) -> Result<Json<Captcha>, Json<Msg>> {
    let mut app_state = app_state.write().await;
    match app_state.authed(&auth) {
        true => (),
        false => return Err(Json(Msg::new("Not authorized"))),
    };

    let temp_dir = app_state.temp_dir();
    let captcha = Captcha::new(len, temp_dir).await;
    app_state.add_captcha(captcha.clone());

    Ok(Json(captcha))
}

#[get("/image?<id>&<auth>")]
pub async fn captcha_img(
    id: String,
    auth: String,
    app_state: &State<AppStatePointer>,
) -> Result<NamedFile, NotFound<Json<Msg>>> {
    let app_state = app_state.read().await;
    match app_state.authed(&auth) {
        true => (),
        false => return Err(NotFound(Json(Msg::new("Not authorized")))),
    }

    let temp_dir = app_state.temp_dir();

    let file_path = temp_dir.path().join(id.clone() + ".png");
    let file = NamedFile::open(&file_path).await;

    match file {
        Ok(file) => Ok(file),
        Err(_) => Err(NotFound(Json(Msg::new("Captcha not found")))),
    }
}

// Delete the captcha image from the server
// TODO: Make this a DELETE request instead of GET
//       because it modifies the server state but for debugging purposes it's fine
//       DONT FORGET TO REMOVE THE NOTE IN THE HELP MESSAGE
#[get("/delete?<id>&<auth>")]
pub async fn delete_captcha(
    id: String,
    auth: String,
    app_state: &State<AppStatePointer>,
) -> Json<Msg> {
    let app_state = app_state.read().await;
    match app_state.authed(&auth) {
        true => (),
        false => return Json(Msg::new("Not authorized")),
    }

    let result = delete_captcha_by_id(&id, app_state).await;

    match result {
        Ok(_) => Json(Msg::new("Captcha deleted")),
        Err(_) => Json(Msg::new("Captcha not found")),
    }
}

// Verify the captcha code
// TODO: Make this a POST request instead of GET
//       because it modifies the server state but for debugging purposes it's fine
//       DONT FORGET TO REMOVE THE NOTE IN THE HELP MESSAGE
#[get("/verify?<id>&<code>&<auth>")]
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

    dbg!(&captcha);

    match captcha.expired() {
        true => {
            delete_captcha_by_id(&id, app_state).await.unwrap();
            return Json(Msg::new("Captcha expired"));
        }
        false => (),
    }

    let result = match captcha.verify(&code) {
        true => "Captcha verified",
        false => "Captcha not verified",
    };

    delete_captcha_by_id(&id, app_state).await.unwrap();

    Json(Msg::new(result))
}

// deletes the captcha image from the server by id
async fn delete_captcha_by_id(
    id: &str,
    app_state: RwLockReadGuard<'_, AppState>,
) -> Result<(), std::io::Error> {
    let temp_dir = app_state.temp_dir();

    let file_path = temp_dir.path().join(id.to_string() + ".png");
    std::fs::remove_file(file_path)
}

#[get("/help")]
pub async fn help() -> &'static str {
    "API v1 Documentation

Welcome to the documentation for API v1. This API provides the following endpoints for managing captchas:

Create a New Captcha
  - Endpoint: GET /api/v1/captcha/new?len=<len>&auth=<auth_token>
  - Description: Creates a new captcha with the given length
  - Returns:
    - A Captcha object with the captcha image id and expiration time
    - A error message
  - Parameters:
    - len: Length of the captcha code
    - auth_token: Your auth token

Retrieve Captcha Image
  - Endpoint: GET /api/v1/captcha/image?id=<captcha_id>&auth=<auth_token>
  - Description: Returns the captcha image with the given captcha id
  - Returns:
    - The captcha image as a PNG file
    - A error message
  - Parameters:
    - captcha_id: Id of the captcha image obtained from /api/v1/captcha/new
    - auth_token: Your auth token

Delete Captcha
  - Endpoint: DELETE /api/v1/captcha/delete?id=<captcha_id>&auth=<auth_token>
  - Description: Deletes the captcha image with the given captcha id
  - Returns:
    - A status message
  - Parameters:
    - captcha_id: Id of the captcha image obtained from /api/v1/captcha/new
    - auth_token: Your auth token
  - Note: This is currently a GET request for debugging purposes during API development

Verify Captcha Code
  - Endpoint: POST /api/v1/captcha/verify?id=<captcha_id>&code=<code>&auth=<auth_token>
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
