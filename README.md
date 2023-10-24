# Captcha

This is a simple captcha-API that can be used to generate captchas and validate them. It is written in Rust and uses the [Rocket](https://rocket.rs/) framework the captcha is generated using the [captcha](https://crates.io/crates/captcha) crate.

## Installation

In order to install the captcha-API, you need to have [Rust](https://www.rust-lang.org/) installed. Then, clone this repository and run `cargo build --release`. This will create an executable in `target/release/captcha`. You can then run this executable to start the captcha-API.

There is alse a example app written in PHP that uses the captcha-API. To run this example app, you need to have [PHP](https://www.php.net/) installed. Then, clone this repository and run `php -S localhost:8001`. This will start a webserver on port 8001. Then run the captcha-API as described above. You can then open `http://localhost:8001/` in your browser to see the example app.

## Usage

### Generate a captcha

To generate a captcha, send a `GET` request to `/api/v1/new`. With the following query parameters:

- `level`: The difficulty level of the captcha. This is a value between 1-9. The difficulty level increases every 3 levels. The default value can be set in the `.env` file.
- `auth`: The authentication token.

The response will be a JSON object with the following structure or an error message:

```json
{
    "image_id": "...",
    "expire_time": 1234567890,
}
```

### Get a captcha image

To get a captcha image, send a `GET` request to `/api/v1/image`. With the following query parameters:

- `id`: The image id of the captcha. Got from the `/api/v1/new` endpoint.
- `auth`: The authentication token.

The response will be the captcha image in PNG format.

### Get a captcha image url

To get a captcha image url, send a `GET` request to `/api/v1/image_url`. With the following query parameters:

- `id`: The image id of the captcha.
- `auth`: The authentication token.

The response will be a JSON object with the following structure or an error message:

```json
{
    "msg": "url",
}
```

### Validate a captcha

To validate a captcha, send a `POST` request to `/api/v1/captcha/verify`. With the following query parameters:

- `id`: The image id of the captcha. Got from the `/api/v1/new` endpoint.
- `auth`: The authentication token.
- `code`: The code of the captcha.

The response will be a JSON object with a message indicating whether the captcha was valid or not.

```json
{
    "msg": "message",
}
```

<!-- TODO: Add example captcha image's. -->

## License

This project is licensed under the [MIT License](LICENSE).
