use crate::v1::{
    captcha_image_url_redirect, filter_docs, help, new_captcha, verify_captcha, AppState,
};
use rocket::{get, launch, routes};

mod v1;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount(
            "/api/v1",
            routes![
                help,
                new_captcha,
                verify_captcha,
                captcha_image_url_redirect,
                filter_docs
            ],
        )
        .manage(AppState::new())
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to Captcha API Documentation

This API provides a simple way to generate captcha images for your website

- To access API v1, navigate to /api/v1. For example, you can use endpoints like /api/v1/new
- Need help or information on available endpoints? Visit /api/v1/help
- Need help with filters? Visit /api/v1/help/filters

If you encounter any issues or have suggestions for improvement, please report them on GitHub: https://github.com/Staninna/captcha/issues"
}

#[macro_export]
macro_rules! conf_get {
    ($config:expr, $key:expr, $type:ty) => {
        $config
            .get($key)
            .expect(&format!("{} must be set", $key))
            .parse::<$type>()
            .expect(&format!("{} must be a {}", $key, stringify!($type)))
    };
}

#[macro_export]
macro_rules! conf_set {
    ($config:ident, $env_var:literal, $type:ty) => {
        let value = dotenv::var($env_var).expect(&format!("{} must be set", $env_var));
        let parsed_value: $type = value.parse().expect(&format!(
            "{} must be a valid {}",
            $env_var,
            stringify!($type)
        ));
        $config.insert($env_var.to_string(), parsed_value.to_string())
    };
}
