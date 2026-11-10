# viewer.lifetech.systems

LifeTech Viewer's website: a PHP site (PHP 8.5) served by CloudPanel.

- `index.php`: the public page (features, download, links).
- `me/`: the private download area, behind a password.
- `sync/api.php`: LifeTech Sync's server (the personal build's sync): a
  store of blobs the viewer encrypts, per account, behind a token derived
  from the owner's passphrase. Data in `ltv-private/sync/`; new accounts
  only while `'sync' => ['registration_open' => true, 'max_accounts' => 1]`
  is in `config.php`.
- `assets/`: styles, logos and demo screenshots.

Nothing secret is in this folder. On the server, next to the site's folder
(`htdocs/ltv-private`, outside the web root):

- `config.php`: `return ['password_hash' => '…'];`, made with PHP's
  `password_hash()` (Argon2id). To change the password, make a new hash and
  replace it.
- `downloads/`: the files offered in the private area, each with its
  `.sha256` next to it.
- `state/`: sign-in attempts, for the rate limit (5 failures per address,
  40 in all, per 15 minutes).

Deploy: copy `viewer.lifetech.systems/` into the site's folder
(`rsync -rlt --omit-dir-times`).
