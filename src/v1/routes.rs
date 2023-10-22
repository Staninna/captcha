use super::{Captcha, ErrorMsg};
use rocket::{delete, fs::NamedFile, get, response::status::NotFound, serde::json::Json, State};
use serde::Serialize;
use tempfile::TempDir;

#[get("/captcha/new?<len>")]
pub async fn new_captcha(len: usize, state: &State<TempDir>) -> Json<Captcha> {
    let captcha = Captcha::new(len, state);
    Json(captcha)
}

#[derive(Serialize)]
pub struct CaptchaResponse {
    msg: String,
    id: String,
}

#[get("/captcha/image?<id>")]
pub async fn get_captcha_img(
    id: String,
    state: &State<TempDir>,
) -> Result<NamedFile, NotFound<Json<ErrorMsg>>> {
    let file_path = state.path().join(id.clone() + ".png");
    dbg!(&file_path);

    let file = NamedFile::open(&file_path).await;

    match file {
        Ok(file) => Ok(file),
        Err(_) => Err(NotFound(Json(ErrorMsg::new(
            "Captcha not found".to_string(),
            Some(id),
        )))),
    }
}

// Delete the captcha image from the server
#[delete("/captcha/delete?<id>")]
pub async fn delete_captcha(id: String, state: &State<TempDir>) -> Json<ErrorMsg> {
    let file_path = state.path().join(id.clone() + ".png");

    let result = std::fs::remove_file(file_path);

    match result {
        Ok(_) => Json(ErrorMsg::new("Captcha deleted".to_string(), None)),
        Err(_) => Json(ErrorMsg::new("Captcha not found".to_string(), Some(id))),
    }
}
