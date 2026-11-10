<?php
declare(strict_types=1);
// The private download area's sign-in. Nothing secret lives in the site's
// folder: the password's hash and the files are in `ltv-private`, next to
// the site's folder in htdocs, which the web server never serves (its root
// is htdocs/viewer.lifetech.systems):
//   ltv-private/config.php  return ['password_hash' => '…'];  (password_hash())
//   ltv-private/downloads/  the files offered (a .sha256 next to each one)
//   ltv-private/state/      sign-in attempts, for the rate limit
if (!defined('LTV_PRIVATE')) {
    http_response_code(404);
    exit;
}

const LTV_SESSION_IDLE = 8 * 3600;
const LTV_FAILS_PER_IP = 5;          // in LTV_FAIL_WINDOW
const LTV_FAILS_GLOBAL = 40;         // all addresses together
const LTV_FAIL_WINDOW = 15 * 60;
const LTV_DOWNLOAD_TYPES = ['dmg', 'zip', 'tar.gz', 'tar.zst', 'exe', 'msi'];

function ltv_private_dir(): string
{
    // htdocs/viewer.lifetech.systems/me → htdocs/ltv-private
    return dirname(__DIR__, 2) . '/ltv-private';
}

function ltv_config(): array
{
    $file = ltv_private_dir() . '/config.php';
    $config = is_file($file) ? require $file : null;
    if (!is_array($config) || empty($config['password_hash'])) {
        http_response_code(503);
        exit('The private area is not set up yet.');
    }
    return $config;
}

function ltv_headers(): void
{
    header('Content-Type: text/html; charset=utf-8');
    header('Cache-Control: private, no-store, max-age=0');
    header('X-Robots-Tag: noindex, nofollow');
    header('X-Content-Type-Options: nosniff');
    header('X-Frame-Options: DENY');
    header('Referrer-Policy: same-origin');
    header("Content-Security-Policy: default-src 'self'; style-src 'self' https://fonts.googleapis.com; font-src https://fonts.gstatic.com; img-src 'self'; form-action 'self'; frame-ancestors 'none'; base-uri 'none'");
}

function ltv_session(): void
{
    session_name('ltv_me');
    session_set_cookie_params([
        'lifetime' => 0,
        'path' => '/me/',
        'secure' => true,
        'httponly' => true,
        'samesite' => 'Strict',
    ]);
    ini_set('session.use_strict_mode', '1');
    session_start();
    $now = time();
    if (!empty($_SESSION['in']) && ($now - ($_SESSION['seen'] ?? 0)) > LTV_SESSION_IDLE) {
        $_SESSION = [];
        session_regenerate_id(true);
    }
    $_SESSION['seen'] = $now;
    if (empty($_SESSION['csrf'])) {
        $_SESSION['csrf'] = bin2hex(random_bytes(32));
    }
}

function ltv_signed_in(): bool
{
    return !empty($_SESSION['in']);
}

function ltv_csrf_ok(): bool
{
    $sent = $_POST['csrf'] ?? '';
    return is_string($sent) && hash_equals($_SESSION['csrf'] ?? '', $sent);
}

/// The visitor's address: Cloudflare's header only when the request came
/// through Cloudflare (CF-Ray), else the connection's.
function ltv_client_ip(): string
{
    $ip = $_SERVER['REMOTE_ADDR'] ?? '0.0.0.0';
    $cf = $_SERVER['HTTP_CF_CONNECTING_IP'] ?? '';
    if ($cf !== '' && isset($_SERVER['HTTP_CF_RAY']) && filter_var($cf, FILTER_VALIDATE_IP)) {
        $ip = $cf;
    }
    return $ip;
}

/// Runs `$change` on the attempts file under a lock and returns its result.
function ltv_attempts(callable $change): mixed
{
    $dir = ltv_private_dir() . '/state';
    if (!is_dir($dir)) {
        @mkdir($dir, 0770, true);
    }
    $fh = fopen($dir . '/attempts.json', 'c+');
    if ($fh === false) {
        $none = [];
        return $change($none);
    }
    flock($fh, LOCK_EX);
    $data = json_decode(stream_get_contents($fh) ?: '[]', true);
    $data = is_array($data) ? $data : [];
    $cutoff = time() - LTV_FAIL_WINDOW;
    foreach ($data as $key => $times) {
        $data[$key] = array_values(array_filter((array) $times, fn($t) => is_int($t) && $t > $cutoff));
        if (!$data[$key]) {
            unset($data[$key]);
        }
    }
    $result = $change($data);
    ftruncate($fh, 0);
    rewind($fh);
    fwrite($fh, json_encode($data));
    flock($fh, LOCK_UN);
    fclose($fh);
    return $result;
}

function ltv_locked_out(string $ip): bool
{
    $key = hash('sha256', $ip);
    return ltv_attempts(function (array &$data) use ($key): bool {
        $all = array_sum(array_map('count', $data));
        return count($data[$key] ?? []) >= LTV_FAILS_PER_IP || $all >= LTV_FAILS_GLOBAL;
    });
}

function ltv_note_failure(string $ip): void
{
    $key = hash('sha256', $ip);
    ltv_attempts(function (array &$data) use ($key): void {
        $data[$key][] = time();
    });
}

function ltv_clear_failures(string $ip): void
{
    $key = hash('sha256', $ip);
    ltv_attempts(function (array &$data) use ($key): void {
        unset($data[$key]);
    });
}

/// The files offered, newest first: name, size, date and checksum.
function ltv_downloads(): array
{
    $dir = ltv_private_dir() . '/downloads';
    $out = [];
    foreach (is_dir($dir) ? scandir($dir) : [] as $name) {
        $path = $dir . '/' . $name;
        if ($name[0] === '.' || !is_file($path) || !ltv_offered($name)) {
            continue;
        }
        $sum = is_file($path . '.sha256') ? strtok((string) file_get_contents($path . '.sha256'), " \t\n") : null;
        $out[] = ['name' => $name, 'size' => filesize($path), 'time' => filemtime($path), 'sha256' => $sum ?: null];
    }
    usort($out, fn($a, $b) => $b['time'] <=> $a['time']);
    return $out;
}

function ltv_offered(string $name): bool
{
    if ($name !== basename($name) || str_contains($name, "\0")) {
        return false;
    }
    foreach (LTV_DOWNLOAD_TYPES as $ext) {
        if (str_ends_with(strtolower($name), '.' . $ext)) {
            return true;
        }
    }
    return false;
}

function ltv_size(int $bytes): string
{
    return $bytes >= 1 << 30 ? sprintf('%.1f GB', $bytes / (1 << 30)) : sprintf('%.0f MB', $bytes / (1 << 20));
}

function e(string $s): string
{
    return htmlspecialchars($s, ENT_QUOTES | ENT_SUBSTITUTE, 'UTF-8');
}
