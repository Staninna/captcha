use super::{AppStatePointer, Captcha, Msg};
use rocket::{fs::NamedFile, get, response::status::NotFound, serde::json::Json, State};
use serde::Serialize;

#[get("/captcha/new?<len>&<auth>")]
pub async fn new_captcha(
    len: usize,
    auth: String,
    app_state: &State<AppStatePointer>,
) -> Result<Json<Captcha>, Json<Msg>> {
    let app_state = app_state.read().await;
    match app_state.authed(&auth) {
        true => (),
        false => return Err(Json(Msg::new("Not authorized"))),
    };

    let temp_dir = app_state.temp_dir();
    let captcha = Captcha::new(len, temp_dir).await;

    Ok(Json(captcha))
}

#[derive(Serialize)]
pub struct CaptchaResponse {
    msg: String,
    id: String,
}

#[get("/captcha/image?<id>&<auth>")]
pub async fn get_captcha_img(
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
#[get("/captcha/delete?<id>&<auth>")]
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

    let temp_dir = app_state.temp_dir();

    let file_path = temp_dir.path().join(id.clone() + ".png");
    let result = std::fs::remove_file(file_path);

    match result {
        Ok(_) => Json(Msg::new("Captcha deleted")),
        Err(_) => Json(Msg::new("Captcha not found")),
    }
}

#[get("/help")]
pub async fn help() -> &'static str {
    "GET /api/v1/captcha/new?len=<len>&auth=<auth_token>
- Description: Creates a new captcha with the given length.
- Returns: A Captcha object with the captcha code and captcha image id.
- Parameters:
    - len: Length of the captcha code.
    - auth_token: Your auth token.

GET /api/v1/captcha/image?id=<captcha_id>&auth=<auth_token>
- Description: Returns the captcha image with the given captcha id.
- Parameters:
    - captcha_id: Id of the captcha image obtained from /api/v1/captcha/new.
    - auth_token: Your auth token.

DELETE /api/v1/captcha/delete?id=<captcha_id>&auth=<auth_token>
- Description: Deletes the captcha image with the given captcha id.
- Parameters:
    - captcha_id: Id of the captcha image obtained from /api/v1/captcha/new.
    - auth_token: Your auth token.
- Note: This is a GET request for debugging purposes during API development.

GET /api/v1/help
    - Description: Returns this help message."
}
