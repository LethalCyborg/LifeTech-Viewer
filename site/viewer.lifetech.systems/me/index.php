<?php
declare(strict_types=1);
// The private download area: a password, then the files of
// private/downloads (see _auth.php).
define('LTV_PRIVATE', true);
require __DIR__ . '/_auth.php';

ltv_headers();
ltv_session();
$error = null;

if ($_SERVER['REQUEST_METHOD'] === 'POST' && ($_POST['action'] ?? '') === 'signin') {
    $ip = ltv_client_ip();
    if (!ltv_csrf_ok()) {
        $error = 'Your session expired. Please try again.';
    } elseif (ltv_locked_out($ip)) {
        $error = 'Too many attempts. Please wait 15 minutes and try again.';
    } else {
        $password = (string) ($_POST['password'] ?? '');
        if (password_verify($password, ltv_config()['password_hash'])) {
            ltv_clear_failures($ip);
            session_regenerate_id(true);
            $_SESSION['in'] = true;
            $_SESSION['csrf'] = bin2hex(random_bytes(32));
            header('Location: /me/', true, 303);
            exit;
        }
        ltv_note_failure($ip);
        usleep(400000);
        $error = 'That password is not right.';
    }
}

$signedIn = ltv_signed_in();
$files = $signedIn ? ltv_downloads() : [];
?><!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="robots" content="noindex, nofollow">
<title><?= $signedIn ? 'Your downloads' : 'Sign in' ?> · LifeTech Viewer</title>
<link rel="icon" type="image/png" href="/assets/favicon.png">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Sora:wght@600;700;800&display=swap" rel="stylesheet">
<link rel="stylesheet" href="/assets/site.css?v=4">
</head>
<body>
<header class="top">
  <div class="wrap bar">
    <a class="brand" href="/"><img src="/assets/mark.png" alt="" width="36" height="36">LifeTech <span>Viewer</span></a>
    <nav class="nav" aria-label="Main">
<?php if ($signedIn): ?>
      <form class="inline" method="post" action="/me/logout.php">
        <input type="hidden" name="csrf" value="<?= e($_SESSION['csrf']) ?>">
        <button class="btn btn-ghost btn-sm" type="submit">Sign out</button>
      </form>
<?php else: ?>
      <a class="btn btn-ghost btn-sm" href="/">Back to site</a>
<?php endif; ?>
    </nav>
  </div>
</header>

<?php if (!$signedIn): ?>
<main class="gate">
  <div class="wrap">
    <form class="login" method="post" action="/me/" autocomplete="on">
      <img src="/assets/mark.png" alt="" width="64" height="64">
      <h1>Private downloads</h1>
      <p class="sub">Sign in to download your personal LifeTech Viewer.</p>
      <input type="hidden" name="action" value="signin">
      <input type="hidden" name="csrf" value="<?= e($_SESSION['csrf']) ?>">
      <input type="text" name="username" value="owner" autocomplete="username" hidden>
      <label for="password">Password</label>
      <input id="password" name="password" type="password" autocomplete="current-password" required autofocus>
<?php if ($error): ?>
      <p class="msg bad" role="alert"><?= e($error) ?></p>
<?php endif; ?>
      <button class="btn btn-glow" type="submit">Sign in</button>
    </form>
  </div>
</main>
<?php else: ?>
<main class="area">
  <div class="wrap">
    <div class="area-head">
      <div>
        <span class="eyebrow">Personal build</span>
        <h2 class="title">Your downloads</h2>
        <p>Your own LifeTech Viewer. The newest version is at the top.</p>
      </div>
    </div>

<?php if (!$files): ?>
    <p class="empty">No download yet. The next build will appear here.</p>
<?php else: ?>
    <div class="files">
<?php foreach ($files as $i => $f): ?>
      <div class="file<?= $i === 0 ? ' latest' : '' ?>">
        <div class="ic"><svg viewBox="0 0 24 24" aria-hidden="true"><path d="M12 3v12M7 10l5 5 5-5M5 21h14"/></svg></div>
        <div>
          <div class="name"><?= e($f['name']) ?></div>
          <div class="meta"><?= e(ltv_size($f['size'])) ?> · <?= e(gmdate('j M Y, H:i', $f['time'])) ?> UTC<?= $i === 0 ? ' · newest' : '' ?>
<?php if ($f['sha256']): ?><br>SHA-256 <code><?= e($f['sha256']) ?></code><?php endif; ?></div>
        </div>
        <a class="btn <?= $i === 0 ? 'btn-glow' : 'btn-ghost' ?> btn-sm" href="/me/download.php?f=<?= rawurlencode($f['name']) ?>">Download</a>
      </div>
<?php endforeach; ?>
    </div>
<?php endif; ?>

    <h3 class="sect">Installing on your Mac</h3>
    <div class="steps">
      <div class="step">
        <h3>Open the disk image</h3>
        <p>Double-click the downloaded <code>.dmg</code>, then drag <b>LifeTech Viewer</b> onto the <b>Applications</b> folder.</p>
      </div>
      <div class="step">
        <h3>First start</h3>
        <p>In Applications, right-click LifeTech Viewer and choose <b>Open</b>, then <b>Open</b> again. If macOS still refuses, go to System Settings › Privacy &amp; Security and click <b>Open Anyway</b>.</p>
      </div>
      <div class="step">
        <h3>Log in</h3>
        <p>Log in with your Second Life account as usual. LifeTech Sync, which carries your chats, settings and places between computers, comes with a later build.</p>
      </div>
    </div>
    <p class="note">This build is for Macs with Apple Silicon (M1 or later). It is signed for your use only, which is why macOS asks before the first start.</p>
  </div>
</main>
<?php endif; ?>
</body>
</html>
