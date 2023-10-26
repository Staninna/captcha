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

function createNewCaptcha($authToken, $level = 2) {
    $url = "/api/v1/new?auth=$authToken&level=$level";
    $response = getReqest($url);
    return json_decode($response, true);
}

function getCaptchaImageURL($captchaId, $authToken) {
    $url = "/api/v1/image_url?id=$captchaId&auth=$authToken";
    $response = getReqest($url);
    return $response;
}

function verifyCaptchaCode($captchaId, $code, $authToken) {
    $url = "/api/v1/verify?id=$captchaId&code=$code&auth=$authToken";
    $response = postRequest($url);
    return $response;
}

const BASE_URL = "http://127.0.0.1:8000";
const AUTH_TOKEN = "TOKEN";

session_start();
if (!isset($_SESSION['level'])) {
    $_SESSION['level'] = 2;
}

if (isset($_GET['level'])) {
    $_SESSION['level'] = $_GET['level'];
}

if ($_SERVER['REQUEST_METHOD'] === 'GET') {
    $captchaId = createNewCaptcha(AUTH_TOKEN, $_SESSION['level'])['id'];
    $captchaImageUrl = getCaptchaImageURL($captchaId, AUTH_TOKEN);
    $response = json_decode($captchaImageUrl, true);
    $captchaImageUrl = $response['url'];
} else if (
    $_SERVER['REQUEST_METHOD'] === 'POST' &&
    isset($_POST['code']) && isset($_POST['captchaId'])
) {
    $code = $_POST['code'];
    $captchaId = $_POST['captchaId'];
    $response = verifyCaptchaCode($captchaId, $code, AUTH_TOKEN);
    $response = json_decode($response, true);

    echo '<link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css">';
    if ($response['ok'] == 'Captcha verified') {
        echo '<h1>Captcha verified</h1>';
    } else {
        echo '<h1>Captcha not verified</h1>';
    }

    echo '<a href="index.php">Go back</a>';
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
        <div class="col-md-6">
            <h2>Capthca Image</h2>
            <img src="<?= $captchaImageUrl ?>" alt="CAPTCHA">
            <form method="GET" action="index.php">
                <div class="form-group">
                    <label for="level">Select CAPTCHA Level:</label>
                    <select name="level" id="level" class="form-control">
                        <?php
                        // Generate options for levels 1 to 9
                        for ($i = 1; $i <= 9; $i++) {
                            if ($_SESSION['level'] == $i) {
                                echo "<option value='$i' selected>$i</option>";
                            } else {
                                echo "<option value='$i'>$i</option>";
                            }
                        }
                        ?>
                    </select>
                </div>
                <button type="submit" class="btn btn-primary">Set CAPTCHA Level</button>
                <a href="index.php">
                    <button type="button" class="btn btn-primary">Generate New CAPTCHA</button>
                </a>
            </form>

            <form method="POST" action="index.php">
                <div class="col-md-6">
                    <h2>Verify Captcha</h2>
                    <div class="form-group">
                        <input type="hidden" name="captchaId" value="<?= $captchaId ?>">
                        <label for="code">Enter the CAPTCHA Code:</label>
                        <input type="text" name="code" id="code" class="form-control" required>
                    </div>
                    <button type="submit" class="btn btn-primary">Verify CAPTCHA</button>
                </div>
            </form>
        </div>
    </div>
</body>

</html>
