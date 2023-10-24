# Captcha

This is a simple captcha-API that can be used to generate captchas and validate them. It is written in Rust and uses the [Rocket](https://rocket.rs/) framework. <!-- TODO: the captcha is generated using the [captcha](https://crates.io/crates/captcha) crate. -->

## Installation

In order to install the captcha-API, you need to have [Rust](https://www.rust-lang.org/) installed. Then, clone this repository and run `cargo build --release`. This will create an executable in `target/release/captcha`. You can then run this executable to start the captcha-API.

## Usage

### Generate a captcha

To generate a captcha, send a `GET` request to `/api/vi/captcha/new`. With the following query parameters:

- `len`: The length of the captcha.
- `auth`: The authentication token.

The response will be a JSON object with the following structure:

```json
{
    "image_id": "...",
    "expire_time": 1234567890,
}
```

### Get a captcha image

To get a captcha image, send a `GET` request to `/api/vi/captcha/image`. With the following query parameters:

- `id`: The image id of the captcha.
- `auth`: The authentication token.

The response will be the captcha image in PNG format.

### Get a captcha image url

To get a captcha image url, send a `GET` request to `/api/vi/captcha/image-url`. With the following query parameters:

- `id`: The image id of the captcha.
- `auth`: The authentication token.

The response will be a plain text url to the captcha image. Note that the url can only be used once. After that, the image will be deleted.

### Validate a captcha

To validate a captcha, send a `GET` request to `/api/vi/captcha/verify`. With the following query parameters:

- `id`: The image id of the captcha.
- `auth`: The authentication token.
- `code`: The code of the captcha.

The response will be a JSON object with a message indicating whether the captcha was valid or not.

<!-- TODO: Add example captcha image's. -->
