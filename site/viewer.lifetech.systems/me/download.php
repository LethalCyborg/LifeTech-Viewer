<?php
declare(strict_types=1);
// One file of private/downloads, to a signed-in visitor only.
define('LTV_PRIVATE', true);
require __DIR__ . '/_auth.php';

ltv_session();
$name = (string) ($_GET['f'] ?? '');
$path = ltv_private_dir() . '/downloads/' . $name;
if (!ltv_signed_in()) {
    header('Location: /me/', true, 303);
    exit;
}
if (!ltv_offered($name) || !is_file($path)) {
    http_response_code(404);
    exit('Not found.');
}
// the session is not needed while the file streams
session_write_close();
@set_time_limit(0);
while (ob_get_level() > 0) {
    ob_end_clean();
}
header('Content-Type: application/octet-stream');
header('Content-Length: ' . filesize($path));
header('Content-Disposition: attachment; filename="' . str_replace('"', '', $name) . '"');
header('Cache-Control: private, no-store, max-age=0');
header('X-Content-Type-Options: nosniff');
header('X-Robots-Tag: noindex, nofollow');
readfile($path);
