<?php
declare(strict_types=1);
// LifeTech Sync: a store of encrypted blobs, one set per Second Life
// account. The viewer encrypts everything before sending it, so this server
// never sees a chat, a setting or even a file name: only opaque ids, version
// numbers and bytes.
//
//   GET  api.php?op=ping
//   POST api.php?op=register&account=<uuid>           (Authorization: Bearer <token>)
//   GET  api.php?op=index&account=<uuid>              → {"blobs":{"<id>":{"v":n,"size":n}}}
//   GET  api.php?op=get&account=<uuid>&id=<id>        → the blob's bytes, X-LTS-Version
//   POST api.php?op=put&account=<uuid>&id=<id>&base=<v>  body: the blob's bytes
//        → {"v":n}; 409 {"v":current} when base is not the current version
//
// The token is derived from the account's sync passphrase in the viewer;
// only its SHA-256 is kept here. An account exists once registered, and
// registering is only possible while `registration_open` is on in
// ltv-private/config.php ('sync' => [...]), for at most `max_accounts`.
// Data: ltv-private/sync/<account>/{meta.json,index.json,blobs/<id>}.

const LTS_ID = '/^[0-9a-f]{64}$/';
const LTS_UUID = '/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/';
const LTS_MAX_BLOB = 32 * 1024 * 1024;
const LTS_MAX_TOTAL = 2 * 1024 * 1024 * 1024;
const LTS_MAX_BLOBS = 20000;
const LTS_FAILS = 10;           // bad tokens per address in LTS_FAIL_WINDOW
const LTS_FAIL_WINDOW = 15 * 60;

header('Cache-Control: private, no-store, max-age=0');
header('X-Content-Type-Options: nosniff');
header('X-Robots-Tag: noindex, nofollow');

function lts_private(): string
{
    // htdocs/viewer.lifetech.systems/sync → htdocs/ltv-private
    return dirname(__DIR__, 2) . '/ltv-private';
}

function lts_fail(int $code, string $message, array $extra = []): never
{
    http_response_code($code);
    header('Content-Type: application/json');
    echo json_encode(['error' => $message] + $extra);
    exit;
}

function lts_json(array $data): never
{
    header('Content-Type: application/json');
    echo json_encode($data, JSON_UNESCAPED_SLASHES);
    exit;
}

function lts_config(): array
{
    $file = lts_private() . '/config.php';
    $config = is_file($file) ? require $file : [];
    $sync = is_array($config) ? ($config['sync'] ?? []) : [];
    return [
        'registration_open' => (bool) ($sync['registration_open'] ?? false),
        'max_accounts' => (int) ($sync['max_accounts'] ?? 1),
    ];
}

function lts_client_ip(): string
{
    $ip = $_SERVER['REMOTE_ADDR'] ?? '0.0.0.0';
    $cf = $_SERVER['HTTP_CF_CONNECTING_IP'] ?? '';
    if ($cf !== '' && isset($_SERVER['HTTP_CF_RAY']) && filter_var($cf, FILTER_VALIDATE_IP)) {
        $ip = $cf;
    }
    return $ip;
}

/// Bad-token attempts per address, under a lock: `$note` adds one.
function lts_throttled(bool $note = false): bool
{
    $dir = lts_private() . '/state';
    if (!is_dir($dir)) {
        @mkdir($dir, 0770, true);
    }
    $fh = @fopen($dir . '/sync-attempts.json', 'c+');
    if ($fh === false) {
        return false;
    }
    flock($fh, LOCK_EX);
    $data = json_decode(stream_get_contents($fh) ?: '[]', true);
    $data = is_array($data) ? $data : [];
    $key = hash('sha256', lts_client_ip());
    $cutoff = time() - LTS_FAIL_WINDOW;
    foreach ($data as $k => $times) {
        $data[$k] = array_values(array_filter((array) $times, fn($t) => is_int($t) && $t > $cutoff));
        if (!$data[$k]) {
            unset($data[$k]);
        }
    }
    if ($note) {
        $data[$key][] = time();
    }
    $throttled = count($data[$key] ?? []) >= LTS_FAILS;
    ftruncate($fh, 0);
    rewind($fh);
    fwrite($fh, json_encode($data));
    flock($fh, LOCK_UN);
    fclose($fh);
    return $throttled;
}

function lts_token(): string
{
    // (X-LTS-Token too: some proxies drop Authorization on its way to PHP)
    $auth = $_SERVER['HTTP_AUTHORIZATION'] ?? $_SERVER['REDIRECT_HTTP_AUTHORIZATION'] ?? '';
    if ($auth === '' && isset($_SERVER['HTTP_X_LTS_TOKEN'])) {
        $auth = 'Bearer ' . $_SERVER['HTTP_X_LTS_TOKEN'];
    }
    if (!preg_match('/^Bearer ([0-9a-f]{64})$/', $auth, $m)) {
        lts_fail(401, 'missing token');
    }
    return $m[1];
}

function lts_account_dir(string $account): string
{
    return lts_private() . '/sync/' . $account;
}

/// The account's folder once the token is checked; 401 otherwise.
function lts_authorised(string $account): string
{
    if (lts_throttled()) {
        lts_fail(429, 'too many attempts');
    }
    $token = lts_token();
    $dir = lts_account_dir($account);
    $meta = is_file("$dir/meta.json") ? json_decode((string) file_get_contents("$dir/meta.json"), true) : null;
    if (!is_array($meta) || !hash_equals((string) ($meta['token_sha256'] ?? ''), hash('sha256', $token))) {
        lts_throttled(true);
        usleep(300000);
        lts_fail(401, 'unknown account or wrong token');
    }
    return $dir;
}

/// Runs `$change` on the account's index under its lock; `$change` may
/// return a value and change the index, which is then written atomically.
function lts_with_index(string $dir, callable $change): mixed
{
    $lock = fopen("$dir/.lock", 'c');
    flock($lock, LOCK_EX);
    $index = is_file("$dir/index.json") ? json_decode((string) file_get_contents("$dir/index.json"), true) : [];
    $index = is_array($index) ? $index : [];
    $before = $index;
    $result = $change($index);
    if ($index !== $before) {
        $tmp = "$dir/index.json." . bin2hex(random_bytes(6));
        file_put_contents($tmp, json_encode($index, JSON_UNESCAPED_SLASHES));
        rename($tmp, "$dir/index.json");
    }
    flock($lock, LOCK_UN);
    fclose($lock);
    return $result;
}

$op = (string) ($_GET['op'] ?? '');
if ($op === 'ping') {
    lts_json(['service' => 'lifetech-sync', 'version' => 1]);
}
$account = strtolower((string) ($_GET['account'] ?? ''));
if (!preg_match(LTS_UUID, $account)) {
    lts_fail(400, 'bad account');
}
$method = $_SERVER['REQUEST_METHOD'] ?? 'GET';

switch ($op) {
    case 'register':
        if ($method !== 'POST') {
            lts_fail(405, 'POST only');
        }
        if (lts_throttled()) {
            lts_fail(429, 'too many attempts');
        }
        $token = lts_token();
        $dir = lts_account_dir($account);
        if (is_file("$dir/meta.json")) {
            // registering again with the same passphrase is harmless
            $meta = json_decode((string) file_get_contents("$dir/meta.json"), true);
            if (is_array($meta) && hash_equals((string) ($meta['token_sha256'] ?? ''), hash('sha256', $token))) {
                lts_json(['registered' => true, 'existing' => true]);
            }
            lts_throttled(true);
            lts_fail(409, 'this account already uses another passphrase');
        }
        $config = lts_config();
        $root = lts_private() . '/sync';
        $count = is_dir($root) ? count(glob("$root/*/meta.json") ?: []) : 0;
        if (!$config['registration_open'] || $count >= $config['max_accounts']) {
            lts_fail(403, 'registration is closed');
        }
        if (!is_dir("$dir/blobs") && !@mkdir("$dir/blobs", 0770, true)) {
            lts_fail(500, 'cannot create the account');
        }
        file_put_contents("$dir/meta.json", json_encode([
            'token_sha256' => hash('sha256', $token),
            'created' => gmdate('c'),
        ]));
        lts_json(['registered' => true, 'existing' => false]);

    case 'index':
        $dir = lts_authorised($account);
        $index = lts_with_index($dir, fn(array &$index) => $index);
        lts_json(['blobs' => (object) $index]);

    case 'get':
        $dir = lts_authorised($account);
        $id = (string) ($_GET['id'] ?? '');
        if (!preg_match(LTS_ID, $id)) {
            lts_fail(400, 'bad id');
        }
        [$version, $bytes] = lts_with_index($dir, function (array &$index) use ($dir, $id) {
            $entry = $index[$id] ?? null;
            $path = "$dir/blobs/$id";
            return [$entry['v'] ?? null, $entry && is_file($path) ? file_get_contents($path) : null];
        });
        if ($version === null || $bytes === null) {
            lts_fail(404, 'no such blob');
        }
        header('Content-Type: application/octet-stream');
        header('X-LTS-Version: ' . $version);
        header('Content-Length: ' . strlen($bytes));
        echo $bytes;
        exit;

    case 'put':
        if ($method !== 'POST') {
            lts_fail(405, 'POST only');
        }
        $dir = lts_authorised($account);
        $id = (string) ($_GET['id'] ?? '');
        $base = (int) ($_GET['base'] ?? -1);
        if (!preg_match(LTS_ID, $id)) {
            lts_fail(400, 'bad id');
        }
        $bytes = file_get_contents('php://input', false, null, 0, LTS_MAX_BLOB + 1);
        if ($bytes === false || $bytes === '' || strlen($bytes) > LTS_MAX_BLOB) {
            lts_fail(413, 'blob empty or too large');
        }
        $result = lts_with_index($dir, function (array &$index) use ($dir, $id, $base, $bytes) {
            $current = $index[$id]['v'] ?? 0;
            if ($base !== $current) {
                return ['conflict' => $current];
            }
            if (!isset($index[$id]) && count($index) >= LTS_MAX_BLOBS) {
                return ['full' => true];
            }
            $total = array_sum(array_map(fn($e) => (int) ($e['size'] ?? 0), $index)) - (int) ($index[$id]['size'] ?? 0);
            if ($total + strlen($bytes) > LTS_MAX_TOTAL) {
                return ['full' => true];
            }
            $tmp = "$dir/blobs/.$id." . bin2hex(random_bytes(6));
            if (file_put_contents($tmp, $bytes) !== strlen($bytes) || !rename($tmp, "$dir/blobs/$id")) {
                @unlink($tmp);
                return ['failed' => true];
            }
            $index[$id] = ['v' => $current + 1, 'size' => strlen($bytes), 't' => time()];
            return ['v' => $current + 1];
        });
        if (isset($result['conflict'])) {
            lts_fail(409, 'version conflict', ['v' => $result['conflict']]);
        }
        if (isset($result['full'])) {
            lts_fail(507, 'storage quota reached');
        }
        if (isset($result['failed'])) {
            lts_fail(500, 'write failed');
        }
        lts_json($result);

    default:
        lts_fail(400, 'unknown op');
}
