# Taking Aurora Viewer's updates

LifeTech Viewer is based on Aurora Viewer
(<https://github.com/Aurora-Viewer/Aurora-Viewer>, GPLv3). Aurora keeps
improving; this is how LifeTech takes the changes it wants without losing its
own.

## How it works

- LifeTech Viewer started from Aurora's commit `e3778b13` (2026-11-09) with
  every package renamed: `aurora-*` became `lifetech-*` (folders, crates,
  the program) and `aurora_*` became `lifetech_*` in the code.
- The **`aurora` branch** holds Aurora's code with that renaming only, one
  commit per Aurora version taken in (its message ends with
  `Aurora-Commit: <Aurora's commit>`). Nothing LifeTech-specific is ever
  committed to it.
- **`main`** has merged the `aurora` branch, so git knows which changes are
  Aurora's and which are LifeTech's. Merging a newer `aurora` brings only
  what Aurora changed since, and keeps everything LifeTech changed.
- `scripts/aurora-update.py` makes the renamed copies. It never changes the
  current branch or the files you are working on (except `pick`, below).

## See what Aurora has done since

```bash
python3 scripts/aurora-update.py status
```

It lists Aurora's commits that LifeTech has not taken yet.

## Take everything up to now

```bash
git switch main
python3 scripts/aurora-update.py import
git merge aurora
```

`import` takes Aurora's latest `main` (or a commit or tag you name:
`import v0.8.0`). Git then merges:

- a file only Aurora changed takes Aurora's change;
- a file only LifeTech changed keeps LifeTech's;
- lines both changed are a **conflict**: git marks them in the file
  (`<<<<<<<`, `=======`, `>>>>>>>`). Keep LifeTech's, Aurora's, or combine
  them, remove the marks, then `git add` the file;
- a file LifeTech deleted (the VR files) and Aurora changed: keep it
  deleted with `git rm <file>`.

Then build and test before committing the merge:

```bash
cargo build -p lifetech-viewer
cargo test --workspace
git commit
git push origin main aurora
```

To give up a merge half way: `git merge --abort`.

## Take one Aurora change only

To take a single fix without the rest of Aurora's commits:

```bash
python3 scripts/aurora-update.py pick <Aurora commit>
```

It applies that commit, renamed, to your files (conflicts are marked the same
way). Check it, build, test and commit. A later full `import` and merge
knows nothing of the pick: git usually sees the same change on both sides
and keeps it once.

## What clashes, and what does not

Most of Aurora (renderer, network, inventory, building, sound) is untouched
by LifeTech and merges by itself. Clashes come where both changed the same
lines: `app.rs`, the theme, the login screen, the toolbar, the People list,
the translations (`*.ftl`). LifeTech's own features live in files of their
own where possible (`world/starter_looks.rs`, `ui/starter_picker.rs`,
`app/starter_picker.rs`, `world/brand_folder.rs`), which keeps clashes rare.

Aurora's new interface texts arrive with "Aurora" in them: rename them in
the merge (the "LifeTech" texts are LifeTech's edits, not the script's).
