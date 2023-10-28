<!-- TODO: Add badges -->
<!-- TODO: Create logo -->
<!-- TODO: Add docker support/image -->

# Captcha

Welcome to the Captcha API, a simple tool for generating and validating captchas. This API is written in Rust and powered by the [Rocket](https://rocket.rs/) framework, utilizing the [captcha](https://crates.io/crates/captcha) crate for captcha generation.

## Installation

To set up the Captcha API, ensure you have [Rust](https://www.rust-lang.org/) installed. Then, follow these steps:

1. Clone this repository.
2. Run `cargo build --release` to build the API executable, which will be located in `target/release/captcha`.
3. Start the Captcha API by running the executable.

Additionally, there is an example app written in PHP that demonstrates the usage of the Captcha API. To run this example app:

1. Make sure you have [PHP](https://www.php.net/) installed.
2. Run `php -S localhost:8001` to start a web server on port 8001.
3. Follow the previous steps to run the Captcha API using `cargo build --release` and `target/release/captcha`.
4. Access the example app by opening `http://localhost:8001/` in your web browser.

## Usage

### Generating a Captcha

To create a captcha, make a `GET` request to `/api/v1/new` with the following query parameters:

All optional parameters have default values defined in the [.env](.env) file.

- `auth`: The authentication token.
- `len`: The captcha code's length (optional).
- `width`: The captcha image's width (optional).
- `height`: The captcha image's height (optional).
- `filters`: Filters to apply to the captcha image (optional).

The response will be a JSON object like this:

```json
{
    "id": "8c6b00702ab445f29b25e8d09d0734c8c848a75d7dcd4ec19cae8c238836cf17",
    "expire_time": "2023-10-28T11:21:10.037393300Z",
    "url": "http://localhost:8000/api/v1/img/9ecdcd60-3148-49ae-8313-e820c8fcd713"
}
```

### Validating a Captcha

To validate a captcha, send a `POST` request to `/api/v1/verify` with the following query parameters:

- `id`: The captcha image ID obtained from the `/api/v1/new` endpoint.
- `auth`: The authentication token.
- `code`: The captcha code.

The response will be a JSON object with a message indicating whether the captcha was valid ("ok"), raised a warning ("warn"), or encountered an error ("error").

### Filters

Filters can be applied to modify the captcha image and make it more challenging for bots to solve. Available filters include:

- `dot`: Adds dots to the image.
- `grid`: Adds a grid to the image.
- `wave`: Adds a wave to the image.
- `noise`: Adds noise to the image.

### Filter Syntax

Filters are applied in the order they are specified in the `filters` query parameter, using the following syntax:

```txt
filter1:arg1,arg2,arg3;filter2:arg1,arg2;filter3:arg1,...
```

Please note that the `filters` query parameter must be valid; otherwise, the Captcha API will return an error message. Here are some valid examples of the `filters` query parameter:

#### Filter Examples

- `dot:10`
- `grid:10,10`
- `wave:10,10,h`
- `noise:0.1`
- `dot:10;grid:10,10;wave:10,10,h;noise:0.1`

## Configuration

You can configure the Captcha API using the [.env](.env) file, where you can set the following options:

```env
# Authentication token
AUTH_TOKEN=secret

# Base URL of the Captcha API
BASE_URL=http://localhost:8000

# Captcha settings
CAPTCHA_EXPIRE_TIME=300 # 5 minutes in seconds
CAPTCHA_LENGTH=6
CAPTCHA_WIDTH=200
CAPTCHA_HEIGHT=100
```

## License

This project is licensed under the MIT license. For more details, please read the [LICENSE](LICENSE) file.
