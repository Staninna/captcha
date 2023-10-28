<?php

function getReqest($url) {
    $curl = curl_init(BASE_URL . $url);
    curl_setopt($curl, CURLOPT_RETURNTRANSFER, true);
    $response = curl_exec($curl);
    curl_close($curl);
    return $response;
}

function postRequest($url) {
    $curl = curl_init(BASE_URL . $url);
    curl_setopt($curl, CURLOPT_RETURNTRANSFER, true);
    curl_setopt($curl, CURLOPT_POST, true);
    $response = curl_exec($curl);
    curl_close($curl);
    return $response;
}

function createNewCaptcha(
    $length,
    $width,
    $height,
    $filters,
    $authToken = AUTH_TOKEN,
) {
    $url = "/api/v1/new?auth=$authToken&len=$length&width=$width&height=$height&filters=$filters";
    $response = getReqest($url);
    return $response;
}

function verifyCaptchaCode($captchaId, $code, $authToken = AUTH_TOKEN) {
    $url = "/api/v1/verify?id=$captchaId&code=$code&auth=$authToken";
    $response = postRequest($url);
    return $response;
}

function head() {
    echo '<head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Captcha Example</title>
        <link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css">
    </head>';
}

// Authentication token
const AUTH_TOKEN = "secret";

// Base URL of the API
const BASE_URL = "http://localhost:8000";

// Captcha settings
const CAPTCHA_LENGTH = 6;
const CAPTCHA_WIDTH = 200;
const CAPTCHA_HEIGHT = 100;
const CAPTCHA_FILTERS = null;

session_start();
if ($_SERVER['REQUEST_METHOD'] === 'GET') {
    // Update session variables
    if (isset($_GET['length'])) {
        $_SESSION['length']  = $_GET['length'];
    }
    if (isset($_GET['width'])) {
        $_SESSION['width']   = $_GET['width'];
    }
    if (isset($_GET['height'])) {
        $_SESSION['height']  = $_GET['height'];
    }
    if (isset($_GET['filters'])) {
        $_SESSION['filters'] = $_GET['filters'];
    }


    // Generate new captcha
    $captcha = createNewCaptcha(
        length: $_SESSION['length'] ?? CAPTCHA_LENGTH,
        width: $_SESSION['width'] ?? CAPTCHA_WIDTH,
        height: $_SESSION['height'] ?? CAPTCHA_HEIGHT,
        filters: $_SESSION['filters'] ?? CAPTCHA_FILTERS,
    );

    // Decode captcha response
    $captcha = json_decode($captcha, true);

    // Check if captcha is generated successfully
    if (!isset($captcha['id']) && !isset($captcha['url']) && !isset($captcha['expire_time'])) {
        die("An error occured while generating captcha");
    }

    // Get captcha details
    $captchaId = $captcha['id'];
    $captchaUrl = $captcha['url'];
    $captchaExpiresAt = $captcha['expire_time'];
}

// Verify captcha
else if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    $captchaId = $_POST['captchaId'];
    $code = $_POST['code'];
    $verifyCaptcha = verifyCaptchaCode($captchaId, $code);

    // Decode captcha response
    $response = json_decode($verifyCaptcha, true);

    echo "<html lang='en'>";
    head();
    echo "<div class='container'>";
    if (isset($response['ok'])) {
        echo "<h1 style='color: green;'>Response is OK</h1>";
        echo "<div class='alert alert-success'>" . $response['ok'] . "</div>";
    } else if (isset($response['warn'])) {
        echo "<h1 style='color: orange;'>Response is Warning</h1>";
        echo "<div class='alert alert-warning'>" . $response['warn'] . "</div>";
    } else if (isset($response['error'])) {
        echo "<h1 style='color: red;'>Response is Error</h1>";
        echo "<div class='alert alert-danger'>" . $response['error'] . "</div>";
    } else {
        echo "<h1 style='color: red;'>Unknown Response</h1>";
        echo "<div class='alert alert-danger'>" . $response . "</div>";
    }
    echo "<a href='index.php'>Go Back</a>";
    exit();
}

?>

<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Example Captcha</title>
    <link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css">
</head>

<body>
    <div class="container">
        <h1>Capthca Example</h1>
        <div class="col-md-6">
            <h2>Captcha Image</h2>
            <img src="<?= $captchaUrl ?>" alt="Captcha Image">
            <hr>
            <h2>Verify Captcha</h2>
            <form method="POST" action="index.php">
                <div class="form-group">
                    <input type="hidden" name="captchaId" value="<?= $captchaId ?>">
                    <label for="code">Enter the CAPTCHA Code:</label>
                    <input type="text" name="code" id="code" class="form-control" required>
                </div>
                <button type="submit" class="btn btn-primary">Verify CAPTCHA</button>
            </form>
            <hr>
            <h3>Settings</h3>
            <form method="GET" action="index.php">
                <div class="form-group">
                    <label for="length">Length:</label>
                    <input type="number" name="length" id="length" class="form-control" value="<?= $_SESSION['length'] ?? CAPTCHA_LENGTH ?>">
                </div>
                <div class="form-group">
                    <label for="width">Width:</label>
                    <input type="number" name="width" id="width" class="form-control" value="<?= $_SESSION['width'] ?? CAPTCHA_WIDTH ?>">
                </div>
                <div class="form-group">
                    <label for="height">Height:</label>
                    <input type="number" name="height" id="height" class="form-control" value="<?= $_SESSION['height'] ?? CAPTCHA_HEIGHT ?>">
                </div>
                <div class="form-group">
                    <label for="filters">Filters:</label>
                    <input type="text" name="filters" id="filters" class="form-control" value="<?= $_SESSION['filters'] ?? CAPTCHA_FILTERS ?>">
                    <small class="form-text text-muted">For more info about the filters syntax goto <a href="<?= BASE_URL ?>/api/v1/help/filters" target="_blank"><?= BASE_URL ?>/api/v1/help/filters</a></small>
                </div>

                <button type="submit" class="btn btn-primary">Generate New Captcha</button>
            </form>
            <hr>
            <h3>Details</h3>
            <ul>
                <li>Captcha ID: <code><?= $captchaId ?></code></li>
                <li>Captcha Expires At: <code><?= $captchaExpiresAt ?></code></li>
                <li>Captcha URL: <code><?= $captchaUrl ?></code></li>
            </ul>
        </div>
    </div>
</body>

</html>
