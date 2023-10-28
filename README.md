<!-- TODO: Add badges -->
<!-- TODO: Create logo -->
<!-- TODO: Add docker support/image -->

# Captcha

This is a simple captcha-API that can be used to generate captchas and validate them. It is written in Rust and uses the [Rocket](https://rocket.rs/) framework the captcha is generated using the [captcha](https://crates.io/crates/captcha) crate.

## Installation

In order to install the captcha-API, you need to have [Rust](https://www.rust-lang.org/) installed. Then, clone this repository and run `cargo build --release`. This will create an executable in `target/release/captcha`. You can then run this executable to start the captcha-API.

There is alse a example app written in PHP that uses the captcha-API. To run this example app, you need to have [PHP](https://www.php.net/) installed. Then, run `php -S localhost:8001`. This will start a webserver on port 8001. Then run the captcha-API as described above(`cargo build --release`, `target/release/captcha`). You can then open `http://localhost:8001/` in your browser to see the example app.

## Usage

### Generate a captcha

To generate a captcha, send a `GET` request to `/api/v1/new`. With the following query parameters:

- `auth`: The authentication token.
- `len`: The length of the captcha code.                (Optional)
- `width`: The width of the captcha image.              (Optional)
- `height`: The height of the captcha image.            (Optional)
- `filters`: The filters to apply to the captcha image. (Optional)
All optional parameters have a default value defined in the [.env](.env) file.

The response will be a JSON object with the following structure or an error message:

```json
{
    "id":"8c6b00702ab445f29b25e8d09d0734c8c848a75d7dcd4ec19cae8c238836cf17",
    "expire_time":"2023-10-28T11:21:10.037393300Z",
    "url":"http://localhost:8000/api/v1/img/9ecdcd60-3148-49ae-8313-e820c8fcd713"
}
```

### Validate a captcha

To validate a captcha, send a `POST` request to `/api/v1/verify`. With the following query parameters:

- `id`: The image id of the captcha. Got from the `/api/v1/new` endpoint.
- `auth`: The authentication token.
- `code`: The code of the captcha.

The response will be a JSON object with a message indicating whether the captcha was valid or not. ("ok", "warn" or "error")

```json
{
    "ok": "ok msg",
    "warn": "warn msg",
    "error": "error msg"
}
```

### Filters

Filters can be used to modify the captcha image. To make it harder for bots to solve the captcha.

The following filters are available:

- `dot`: Adds dots to the image.
  Arguments:
  - `n`: The number of dots to add.
- `grid`: Adds a grid to the image.
    Arguments:
  - `x_gap`: The horizontal gap between the grid lines.
  - `y_gap`: The vertical gap between the grid lines.
- `wave`: Adds a wave to the image.
    Arguments:
  - `f`: The frequency of the wave.
  - `amp`: The amplitude of the wave.
  - `dir`: The direction of the wave. (h or v)
- `noise`: Adds noise to the image.
    Arguments:
  - `prob`: The probability of a pixel being set to black.

### Filter syntax

The filters are applied in the order they are specified in the `filters` query parameter. The syntax for the `filters` query parameter is as follows:

Begin with a filter name, followed by a colon and then the arguments for the filter. The arguments are separated by commas. Multiple filters are separated by semicolons.

```txt
filter1:arg1,arg2,arg3;filter2:arg1,arg2;filter3:arg1,...
```

Keep in mind that the `filters` query parameter needs to be valid otherwise the captcha-API will return an error message. The following are valid examples of the `filters` query parameter

#### Filter examples

```txt


```txt
dot:10
grid:10,10
wave:10,10,h
noise:0.1
dot:10;grid:10,10;wave:10,10,h;noise:0.1
dot:10;grid:10,10;wave:10,10,h;noise:0.1;dot:10;grid:10,10;wave:10,10,h;noise:0.1
```

## Configuration

The captcha-API can be configured using the [.env](.env) file. The following options are available:

```env
# Authentication token
AUTH_TOKEN=secret

# Base url of the captcha-API
BASE_URL=http://localhost:8000

# Captcha settings
CAPTCHA_EXPIRE_TIME=300 # 5 minutes in seconds
CAPTCHA_LENGTH=6
CAPTCHA_WIDTH=200
CAPTCHA_HEIGHT=100
```

## License

This project is licensed under the MIT license. See the [LICENSE](LICENSE) file for more details.
