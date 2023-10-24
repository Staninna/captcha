use crate::v1::{
    captcha_img, captcha_img_url, captcha_img_url_redirect, delete_captcha, help, new_captcha,
    verify_captcha, AppState,
};
use rocket::{get, launch, routes};

// TODO: Give unique urls to each captcha image so they can be inserted into
//       html pages without leaking the auth token
// TODO: Add logging

mod v1;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, captcha_img_url_redirect])
        .mount("/api/v1", routes![help])
        .mount(
            "/api/v1/captcha",
            routes![
                new_captcha,
                captcha_img,
                delete_captcha,
                verify_captcha,
                captcha_img_url
            ],
        )
        .manage(AppState::new())
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to Captcha API Documentation

This API provides a simple way to generate captcha images for your website

- To access API v1, navigate to /api/v1. For example, you can use endpoints like /api/v1/captcha/new
- Need help or information on available endpoints? Visit /api/v1/help

Please note that this API is a work in progress, and your feedback is valuable to me. If you encounter any issues or have suggestions for improvement, please report them on GitHub: https://github.com/Staninna/captcha/issues"
}
