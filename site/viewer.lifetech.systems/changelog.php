<?php
declare(strict_types=1);
// viewer.lifetech.systems/changelog.php: the repository's CHANGELOG.md, as a
// page. The deploy copies it next to this file as changelog.md (site/README.md).
// Its Markdown is a small subset: ## and ### headings, "- " items (with
// indented continuation lines), paragraphs, **bold**, `code` and links.
header('Content-Type: text/html; charset=utf-8');
header('Cache-Control: public, max-age=300');
header('X-Content-Type-Options: nosniff');
header('Referrer-Policy: strict-origin-when-cross-origin');
$year = date('Y');
$source = 'https://github.com/LethalCyborg/LifeTech-Viewer';

function cl_inline(string $text): string
{
    global $source;
    $html = htmlspecialchars($text, ENT_QUOTES | ENT_SUBSTITUTE, 'UTF-8');
    $html = preg_replace('/`([^`]+)`/', '<code>$1</code>', $html);
    $html = preg_replace('/\*\*([^*]+)\*\*/', '<b>$1</b>', $html);
    // [text](url): repository paths lead to the files on GitHub
    $html = preg_replace_callback('/\[([^\]]+)\]\(([^)\s]+)\)/', function (array $m) use ($source): string {
        $url = html_entity_decode($m[2], ENT_QUOTES, 'UTF-8');
        if (!preg_match('#^https?://#', $url)) {
            $url = $source . '/blob/main/' . ltrim($url, './');
        }
        return '<a href="' . htmlspecialchars($url, ENT_QUOTES) . '" rel="noopener">' . $m[1] . '</a>';
    }, $html);
    // <https://…>
    return preg_replace('#&lt;(https?://[^&\s]+)&gt;#', '<a href="$1" rel="noopener">$1</a>', $html);
}

/// The changelog's Markdown as HTML.
function cl_render(string $md): string
{
    $out = '';
    $para = [];
    $items = [];
    $flush = function () use (&$out, &$para, &$items): void {
        if ($para) {
            $out .= '<p>' . cl_inline(implode(' ', $para)) . "</p>\n";
            $para = [];
        }
        if ($items) {
            $out .= "<ul>\n";
            foreach ($items as $item) {
                $out .= '<li>' . cl_inline($item) . "</li>\n";
            }
            $out .= "</ul>\n";
            $items = [];
        }
    };
    foreach (preg_split('/\R/', $md) as $line) {
        if (str_starts_with($line, '# ')) {
            continue; // the page has its own title
        }
        if (preg_match('/^(#{2,3}) (.*)$/', $line, $m)) {
            $flush();
            $tag = strlen($m[1]) === 2 ? 'h2' : 'h3';
            $out .= "<$tag>" . cl_inline($m[2]) . "</$tag>\n";
        } elseif (str_starts_with($line, '- ')) {
            if ($para) {
                $flush();
            }
            $items[] = substr($line, 2);
        } elseif ($items && preg_match('/^\s+\S/', $line)) {
            $items[count($items) - 1] .= ' ' . trim($line);
        } elseif (trim($line) === '') {
            $flush();
        } else {
            if ($items) {
                $flush();
            }
            $para[] = trim($line);
        }
    }
    $flush();
    return $out;
}

$file = __DIR__ . '/changelog.md';
$body = is_file($file) ? cl_render((string) file_get_contents($file)) : '<p>The changelog is on <a href="' . $source . '/blob/main/CHANGELOG.md">GitHub</a>.</p>';
?><!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Changelog · LifeTech Viewer</title>
<meta name="description" content="What changed in each version of LifeTech Viewer, compared with Aurora Viewer.">
<meta name="theme-color" content="#07090e">
<link rel="icon" type="image/png" href="/assets/favicon.png">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Sora:wght@600;700;800&display=swap" rel="stylesheet">
<link rel="stylesheet" href="/assets/site.css?v=6">
</head>
<body>
<main id="main" class="changelog">
  <div class="wrap">
    <a class="back" href="/">← LifeTech Viewer</a>
    <span class="eyebrow">Changelog</span>
    <h1>What's new</h1>
    <?= $body ?>
    <div class="cta">
      <a class="btn btn-glow" href="/#download">Download</a>
      <a class="btn btn-ghost" href="<?= $source ?>" rel="noopener">Source code</a>
    </div>
  </div>
</main>
<footer>
  <div class="wrap">
    <span>© <?= $year ?> LifeTech Systems. LifeTech Viewer is based on <a href="https://github.com/Aurora-Viewer/Aurora-Viewer" rel="noopener">Aurora Viewer</a>, free software under the GNU GPL v3.</span>
    <span>Second Life® is a trademark of Linden Research, Inc. LifeTech Systems is not affiliated with Linden Lab.</span>
  </div>
</footer>
</body>
</html>
