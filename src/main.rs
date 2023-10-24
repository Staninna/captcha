use crate::v1::{help, new_captcha, verify_captcha, AppState};
use rocket::{get, launch, routes};

// TODO: Add logging

mod v1;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api/v1", routes![help, new_captcha, verify_captcha])
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
