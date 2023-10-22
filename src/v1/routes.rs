use super::{AppStatePointer, Captcha, ErrorMsg};
use rocket::{delete, fs::NamedFile, get, response::status::NotFound, serde::json::Json, State};
use serde::Serialize;

#[get("/captcha/new?<length>")]
pub async fn new_captcha(length: usize, app_state: &State<AppStatePointer>) -> Json<Captcha> {
    let captcha = Captcha::new(length, app_state).await;
    Json(captcha)
}

#[derive(Serialize)]
pub struct CaptchaResponse {
    msg: String,
    id: String,
}

#[get("/captcha/image?<captcha_id>")]
pub async fn get_captcha_img(
    captcha_id: String,
    app_state: &State<AppStatePointer>,
) -> Result<NamedFile, NotFound<Json<ErrorMsg>>> {
    let app_state = app_state.read().await;
    let temp_dir = app_state.temp_dir();

    let file_path = temp_dir.path().join(captcha_id.clone() + ".png");
    let file = NamedFile::open(&file_path).await;

    match file {
        Ok(file) => Ok(file),
        Err(_) => Err(NotFound(Json(ErrorMsg::new(
            "Captcha not found".to_string(),
            Some(captcha_id),
        )))),
    }
}

// Delete the captcha image from the server
#[delete("/captcha/delete?<captcha_id>")]
pub async fn delete_captcha(
    captcha_id: String,
    app_state: &State<AppStatePointer>,
) -> Json<ErrorMsg> {
    let app_state = app_state.read().await;
    let temp_dir = app_state.temp_dir();

    let file_path = temp_dir.path().join(captcha_id.clone() + ".png");
    let result = std::fs::remove_file(file_path);

    match result {
        Ok(_) => Json(ErrorMsg::new(
            "Captcha deleted".to_string(),
            Some(captcha_id),
        )),
        Err(_) => Json(ErrorMsg::new(
            "Captcha not found".to_string(),
            Some(captcha_id),
        )),
    }
}

#[get("/help")]
pub async fn help() -> &'static str {
    "GET /api/v1/captcha/new?length=<length>
    Creates a new captcha with the given length
    Returns a Captcha object with the captcha code and captcha image id

GET /api/v1/captcha/image?captcha_id=<captcha_id>
    Returns the captcha image with the given captcha id

DELETE /api/v1/captcha/delete?captcha_id=<captcha_id>
    Deletes the captcha image with the given captcha id"
}
