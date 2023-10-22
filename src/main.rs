use crate::v1::{delete_captcha, get_captcha_img, help, new_captcha};
use rocket::{get, launch, routes};
use tempfile::tempdir;

// TODO: Add valid until field to captcha
//       if expired, delete the captcha image from the server
// TODO: Add way to authenticate requests to create, delete, and get captchas
// TODO: Give unique urls to each captcha image so they can be inserted into
//       html pages without leaking the auth token
// TODO: Add logging

mod v1;

#[launch]
fn rocket() -> _ {
    // TODO: Make so that the temp dir is deleted when the server is stopped
    let temp_dir = tempdir().unwrap();

    rocket::build()
        .mount("/", routes![index])
        .mount(
            "/api/v1",
            routes![new_captcha, get_captcha_img, delete_captcha, help],
        )
        .manage(temp_dir)
}

#[get("/")]
fn index() -> &'static str {
    "API v1 is located at /api/v1
For more information, go to /api/v1/help"
}
