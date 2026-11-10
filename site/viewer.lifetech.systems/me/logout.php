<?php
declare(strict_types=1);
define('LTV_PRIVATE', true);
require __DIR__ . '/_auth.php';

ltv_session();
if ($_SERVER['REQUEST_METHOD'] === 'POST' && ltv_csrf_ok()) {
    $_SESSION = [];
    session_regenerate_id(true);
    session_destroy();
}
header('Location: /me/', true, 303);
