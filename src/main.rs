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
    "Welcome to Captcha API

This API provides a simple way to generate captcha images for your website.

- To access API v1, navigate to /api/v1. For example, /api/v1/captcha/new.
- Need help or information on available endpoints? Visit /api/v1/help.

Please note that this API is a work in progress, and your feedback is valuable to me. If you encounter any issues or have suggestions for improvement, please report them on our GitHub repository: https://github.com/Staninna/captcha/issues."
}
