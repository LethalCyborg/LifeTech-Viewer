<?php
declare(strict_types=1);
// viewer.lifetech.systems: LifeTech Viewer's page. Static content; PHP for
// the year, the cache headers and the list of downloads (downloads/).
header('Content-Type: text/html; charset=utf-8');
header('Cache-Control: public, max-age=300');
header('X-Content-Type-Options: nosniff');
header('Referrer-Policy: strict-origin-when-cross-origin');
$year = date('Y');
$source = 'https://github.com/LethalCyborg/LifeTech-Viewer';

/// The newest public build of each platform in downloads/:
/// LifeTech-Viewer-<version>-<platform>.<ext>, with its .sha256 beside it.
function ltv_builds(): array
{
    $platforms = [
        'windows-x64-setup' => ['exe', 'Windows', 'Installer · 64-bit, Windows 10 and 11'],
        'macos-arm64' => ['dmg', 'macOS', 'Apple Silicon (M1 or later)'],
        'linux-x64' => ['tar.gz', 'Linux', '64-bit, recent distributions'],
    ];
    $out = [];
    foreach ($platforms as $platform => [$ext, $system, $detail]) {
        $best = null;
        foreach (glob(__DIR__ . "/downloads/LifeTech-Viewer-*-$platform.$ext") ?: [] as $path) {
            if (!is_file("$path.sha256")) {
                continue;
            }
            if ($best === null || filemtime($path) > filemtime($best)) {
                $best = $path;
            }
        }
        if ($best === null) {
            continue;
        }
        $name = basename($best);
        $version = substr($name, strlen('LifeTech-Viewer-'), -strlen("-$platform.$ext"));
        $out[] = [
            'system' => $system,
            'detail' => $detail,
            'name' => $name,
            'version' => $version,
            'size' => sprintf('%.0f MB', filesize($best) / (1 << 20)),
            'sha256' => strtok((string) file_get_contents("$best.sha256"), " \t\n") ?: '',
        ];
    }
    return $out;
}
$builds = ltv_builds();
?><!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>LifeTech Viewer · Second Life, made simple</title>
<meta name="description" content="LifeTech Viewer is a Second Life viewer by LifeTech Systems, made for newcomers: choose your look on day one, a clear interface, and everything you need to find your way.">
<meta name="theme-color" content="#07090e">
<link rel="icon" type="image/png" href="/assets/favicon.png">
<link rel="apple-touch-icon" href="/assets/apple-touch-icon.png">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Sora:wght@600;700;800&display=swap" rel="stylesheet">
<link rel="stylesheet" href="/assets/site.css?v=6">
<meta property="og:title" content="LifeTech Viewer">
<meta property="og:description" content="A Second Life viewer made for newcomers, by LifeTech Systems.">
<meta property="og:image" content="https://viewer.lifetech.systems/assets/logo.png">
</head>
<body>
<a class="skip" href="#main">Skip to content</a>

<header class="top">
  <div class="wrap bar">
    <a class="brand" href="/"><img src="/assets/mark.png" alt="" width="36" height="36">LifeTech <span>Viewer</span></a>
    <nav class="nav" aria-label="Main">
      <a class="opt" href="#features">Features</a>
      <a class="opt" href="#download">Download</a>
      <a class="opt" href="/changelog.php">Changelog</a>
      <a class="opt" href="<?= $source ?>" rel="noopener">Source</a>
      <a class="btn btn-ghost btn-sm" href="https://lifetech.systems">lifetech.systems</a>
    </nav>
  </div>
</header>

<main id="main">
  <section class="hero">
    <div class="wrap">
      <div>
        <img class="hero-logo" src="/assets/logo.png" alt="LifeTech Viewer" width="600" height="431">
        <span class="eyebrow">A Second Life viewer</span>
        <h1 style="margin-top:14px">Second Life, <span class="grad">made simple.</span></h1>
        <p class="lead">LifeTech Viewer is made for people who are new to Second Life. Pick a look the first time you log in, find your way around a clear interface, and spend your time exploring instead of learning menus.</p>
        <div class="cta">
          <a class="btn btn-glow" href="#features">
            <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M12 5v14M5 12l7 7 7-7"/></svg>
            See what's inside
          </a>
          <a class="btn btn-ghost" href="<?= $source ?>" rel="noopener">
            <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M9 19c-4.3 1.4-4.3-2.5-6-3m12 5v-3.5c0-1 .1-1.4-.5-2 2.8-.3 5.5-1.4 5.5-6a4.6 4.6 0 0 0-1.3-3.2 4.2 4.2 0 0 0-.1-3.2s-1.1-.3-3.5 1.3a12.3 12.3 0 0 0-6.2 0C6.5 2.8 5.4 3.1 5.4 3.1a4.2 4.2 0 0 0-.1 3.2A4.6 4.6 0 0 0 4 9.5c0 4.6 2.7 5.7 5.5 6-.6.6-.6 1.2-.5 2V21"/></svg>
            Open source on GitHub
          </a>
        </div>
      </div>
      <figure class="shot" style="margin:0">
        <img src="/assets/picker.webp" alt="The Choose your look panel: starter characters listed under Everyday and Fantasy, with Cancel and Keep this look buttons" width="352" height="460">
      </figure>
    </div>
  </section>

  <section id="features" class="alt">
    <div class="wrap">
      <div class="head">
        <span class="eyebrow">Features</span>
        <h2 style="margin-top:12px">Everything a newcomer needs, <span class="grad">nothing in the way.</span></h2>
        <p>Built on the fast, modern Aurora Viewer engine (Rust and Vulkan), with LifeTech's own touches for the first days in Second Life.</p>
      </div>

      <div class="features">
        <article class="feature">
          <figure><img src="/assets/picker.webp" alt="" loading="lazy" width="352" height="460"></figure>
          <div class="txt">
            <h3>Choose your look on day one</h3>
            <p>The first time you log in, LifeTech Viewer shows the free mesh characters every account gets. Click one to try it on, then keep it or go back. You can open it again any time from the hanger button.</p>
          </div>
        </article>
        <article class="feature">
          <figure><img src="/assets/people.webp" alt="" loading="lazy" width="520" height="555"></figure>
          <div class="txt">
            <h3>See who's around</h3>
            <p>People near you, grouped by how far your voice carries. A house badge shows who is on the same parcel as you, next to the friend, voice, typing and sitting badges.</p>
          </div>
        </article>
        <article class="feature">
          <figure class="fit"><img src="/assets/minimap.webp" alt="" loading="lazy" width="312" height="222"></figure>
          <div class="txt">
            <h3>A mini-map that shows height</h3>
            <p>Avatars above you carry a ^ in their dot, those below a v, so you know whether to look up or down. Your view cone and compass stay where you expect them.</p>
          </div>
        </article>
        <article class="feature">
          <figure><img src="/assets/chest.webp" alt="" loading="lazy" width="290" height="684"></figure>
          <div class="txt">
            <h3>The Build Chest</h3>
            <p>Keep your favourite textures, materials and objects in one place. Drag them onto a surface to apply them, or into the world to rez them.</p>
          </div>
        </article>
      </div>

      <div class="small">
        <div class="tile">
          <svg viewBox="0 0 24 24" aria-hidden="true"><rect x="3" y="4" width="18" height="14" rx="2"/><path d="M7 20h10M9 9l2 2 4-4"/></svg>
          <h3>Web and media built in</h3>
          <p>Web pages, parcel media and music work from the first start, with nothing extra to install.</p>
        </div>
        <div class="tile">
          <svg viewBox="0 0 24 24" aria-hidden="true"><circle cx="12" cy="12" r="9"/><path d="M12 7v5l3 2"/></svg>
          <h3>Fast and smooth</h3>
          <p>A modern Vulkan renderer that uses your graphics card to the full, for high frame rates in busy places.</p>
        </div>
        <div class="tile">
          <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M12 3l8 4v5c0 5-3.5 8-8 9-4.5-1-8-4-8-9V7z"/><path d="M9 12l2 2 4-4"/></svg>
          <h3>Open and honest</h3>
          <p>Free software under the GNU GPL v3. Every line of code is public, and bug reports go straight to the project.</p>
        </div>
      </div>
    </div>
  </section>

  <section id="download">
    <div class="wrap">
      <div class="head">
        <span class="eyebrow">Download</span>
        <h2 style="margin-top:12px">Get LifeTech Viewer</h2>
      </div>
<?php if ($builds): ?>
      <div class="files">
<?php foreach ($builds as $b): ?>
        <div class="file">
          <div class="ic"><svg viewBox="0 0 24 24" aria-hidden="true"><path d="M12 3v12M7 10l5 5 5-5M5 21h14"/></svg></div>
          <div>
            <div class="name">LifeTech Viewer <?= htmlspecialchars($b['version']) ?> for <?= htmlspecialchars($b['system']) ?></div>
            <div class="meta"><?= htmlspecialchars($b['detail']) ?> · <?= htmlspecialchars($b['size']) ?><br>SHA-256 <code><?= htmlspecialchars($b['sha256']) ?></code></div>
          </div>
          <a class="btn btn-glow btn-sm" href="/downloads/<?= rawurlencode($b['name']) ?>" download>Download</a>
        </div>
<?php endforeach; ?>
      </div>
      <p class="note">macOS: the first time, right-click LifeTech Viewer in Applications and choose <b>Open</b> (or System Settings › Privacy &amp; Security › <b>Open Anyway</b>). Linux: unpack the archive and start <code>lifetech-viewer</code>. Windows: run the installer; if SmartScreen asks, choose <b>More info</b> › <b>Run anyway</b>.</p>
<?php endif; ?>
      <div class="download">
        <div class="card">
<?php if (!$builds): ?>
          <span class="pill">Coming soon</span>
          <h3>Public downloads</h3>
          <p>Ready-to-install versions for Windows, macOS and Linux are on their way. Until then, LifeTech Viewer can be built from its source code on GitHub.</p>
<?php else: ?>
          <h3>Source code</h3>
          <p>LifeTech Viewer is free software under the GNU GPL v3: its source code, and every version's, is on GitHub.</p>
<?php endif; ?>
          <div class="cta">
            <a class="btn btn-glow" href="<?= $source ?>" rel="noopener">View the source</a>
            <a class="btn btn-ghost" href="<?= $source ?>/issues/new/choose" rel="noopener">Report a bug</a>
          </div>
        </div>
        <div class="card">
          <h3>Need help?</h3>
          <p>Visit the LifeTech store in Second Life, or see everything else LifeTech makes for Second Life.</p>
          <div class="cta">
            <a class="btn btn-ghost" href="https://lifetech.systems">lifetech.systems</a>
          </div>
        </div>
      </div>
    </div>
  </section>
</main>

<footer>
  <div class="wrap">
    <span>© <?= $year ?> LifeTech Systems. LifeTech Viewer is based on <a href="https://github.com/Aurora-Viewer/Aurora-Viewer" rel="noopener">Aurora Viewer</a>, free software under the GNU GPL v3.</span>
    <span>Second Life® is a trademark of Linden Research, Inc. LifeTech Systems is not affiliated with Linden Lab.</span>
  </div>
</footer>
</body>
</html>
