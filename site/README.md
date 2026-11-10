# viewer.lifetech.systems

LifeTech Viewer's website: a PHP site (PHP 8.5) served by CloudPanel.

- `index.php`: the public page (features, downloads, links). Its download
  section lists the builds in the site's `downloads/` folder, named
  `LifeTech-Viewer-<version>-<platform>.<ext>` with a `.sha256` beside
  each (`windows-x64.zip`, `macos-arm64.dmg`, `linux-x64.tar.gz`): the
  newest of each platform is offered.
- `assets/`: styles, logos and demo screenshots.

Nothing secret is in this folder.

- `changelog.php`: the repository's `CHANGELOG.md` as a page; the deploy
  copies that file next to it as `changelog.md`.

Deploy: copy `../CHANGELOG.md` to `viewer.lifetech.systems/changelog.md`,
then `viewer.lifetech.systems/` into the site's folder
(`rsync -rlt --omit-dir-times`, without `--delete`: the server keeps
folders that are not in this repository).
