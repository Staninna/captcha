use crate::v1::{delete_captcha, get_captcha_img, new_captcha};
use rocket::{launch, routes};
use tempfile::tempdir;

mod v1;

#[launch]
fn rocket() -> _ {
    // TODO: Make so that the temp dir is deleted when the server is stopped
    let temp_dir = tempdir().unwrap();

    rocket::build()
        .mount(
            "/api/v1",
            routes![new_captcha, get_captcha_img, delete_captcha],
        )
        .manage(temp_dir)
}
