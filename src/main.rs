use crate::v1::{delete_captcha, get_captcha_img, help, new_captcha, AppState};
use rocket::{get, launch, routes};

// TODO: Add valid until field to captcha
//       if expired, delete the captcha image from the server
// TODO: Add way to authenticate requests to create, delete, and get captchas
// TODO: Give unique urls to each captcha image so they can be inserted into
//       html pages without leaking the auth token
// TODO: Add logging

mod v1;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount(
            "/api/v1",
            routes![new_captcha, get_captcha_img, delete_captcha, help],
        )
        .manage(AppState::new())
}

#[get("/")]
fn index() -> &'static str {
    "API v1 is located at /api/v1
For more information, go to /api/v1/help"
}
