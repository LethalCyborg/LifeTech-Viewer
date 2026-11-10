#!/usr/bin/env python3
"""Bring Aurora Viewer's changes into LifeTech Viewer (docs/AURORA_UPDATES.md).

LifeTech Viewer started from Aurora Viewer's commit e3778b13 with every
package renamed (`aurora-*` to `lifetech-*`, `aurora_*` to `lifetech_*`).
The `aurora` branch holds Aurora's code with that renaming only: one commit
per Aurora version taken in. `main` has merged it, so git knows what is
Aurora's and what is LifeTech's.

    python3 scripts/aurora-update.py status
        Which Aurora commit the `aurora` branch is at, and how many newer
        commits Aurora has.
    python3 scripts/aurora-update.py import [<aurora ref>]
        Add Aurora's code at <aurora ref> (default: Aurora's main), renamed,
        as a new commit of the `aurora` branch. Then `git merge aurora`
        on main takes Aurora's changes and keeps LifeTech's.
    python3 scripts/aurora-update.py pick <aurora commit>
        Apply one Aurora commit only, renamed, to the working tree (three-way:
        clashes are marked in the files, as in a merge).

Nothing here changes the current branch or the working tree, except `pick`.
"""

import os
import re
import shutil
import subprocess
import sys
import tempfile

REMOTE = "aurora"
REMOTE_URL = "https://github.com/Aurora-Viewer/Aurora-Viewer.git"
BRANCH = "aurora"
TRAILER = "Aurora-Commit:"

# ---- the renaming, exactly as LifeTech Viewer's first commit made it ------
CRATES = ["assets", "audio", "i18n", "llsd", "media", "msg", "net", "prim", "render", "updater", "version", "viewer", "voice"]
# names that keep Aurora's spelling: its website and its media plugin pack
PROTECT = ["aurora-viewer.org", "aurora-media-plugins", "Aurora-Media-Plugins"]
EXTS = {".rs", ".toml", ".md", ".yml", ".yaml", ".ps1", ".py", ".json", ".desktop", ".plist", ".cmd", ".sh", ".lock", ".txt", ".wgsl", ".ftl", ".entitlements", ".strings"}
SKIP_DIRS = {"target", ".git", "node_modules"}
DASH = re.compile(r"\baurora-(" + "|".join(CRATES) + r")\b(?!\.org)")
SNAKE = re.compile(r"\baurora_(" + "|".join(CRATES) + r")\b")


def rename_text(text):
    keep = {}
    for i, p in enumerate(PROTECT):
        token = f"\x00P{i}\x00"
        text = text.replace(p, token)
        keep[token] = p
    text = DASH.sub(r"lifetech-\1", text)
    text = SNAKE.sub(r"lifetech_\1", text)
    text = text.replace("DEP_AURORA_VERSION_", "DEP_LIFETECH_VERSION_")
    for token, p in keep.items():
        text = text.replace(token, p)
    return text


def rename_tree(root):
    """Rename Aurora's code checked out at `root`, in place."""
    for here, dirs, files in os.walk(root):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for f in files:
            if os.path.splitext(f)[1] not in EXTS and f != "Cargo.lock":
                continue
            path = os.path.join(here, f)
            try:
                with open(path, encoding="utf-8", newline="") as fh:
                    text = fh.read()
            except (UnicodeDecodeError, OSError):
                continue
            new = rename_text(text)
            if new != text:
                with open(path, "w", encoding="utf-8", newline="") as fh:
                    fh.write(new)
    for c in CRATES:
        old = os.path.join(root, "crates", f"aurora-{c}")
        if os.path.isdir(old):
            os.rename(old, os.path.join(root, "crates", f"lifetech-{c}"))
    desktop = os.path.join(root, "packaging", "linux", "aurora-viewer.desktop")
    if os.path.exists(desktop):
        os.rename(desktop, os.path.join(root, "packaging", "linux", "lifetech-viewer.desktop"))


# ---- git -----------------------------------------------------------------
def git(*args, env=None, check=True, input=None):
    run = subprocess.run(["git", *args], capture_output=True, text=True, env=env, input=input)
    if check and run.returncode != 0:
        sys.exit(f"git {' '.join(args)} failed:\n{run.stderr.strip()}")
    return run.stdout.strip()


def fetch(ref):
    if git("remote", "get-url", REMOTE, check=False) == "":
        git("remote", "add", REMOTE, REMOTE_URL)
    git("fetch", "--no-tags", REMOTE, "main")
    return git("rev-parse", f"{REMOTE}/main" if ref is None else ref)


def renamed_tree(commit):
    """The git tree of Aurora's `commit`, renamed, written to the object store."""
    work = tempfile.mkdtemp(prefix="aurora-update-")
    index = os.path.join(work, ".index")
    files = os.path.join(work, "files")
    os.mkdir(files)
    env = dict(os.environ, GIT_INDEX_FILE=index)
    try:
        git("read-tree", commit, env=env)
        git("checkout-index", "-a", f"--prefix={files}/", env=env)
        rename_tree(files)
        os.remove(index)
        git("--work-tree", files, "add", "-A", "-f", ".", env=env)
        return git("write-tree", env=env)
    finally:
        shutil.rmtree(work, ignore_errors=True)


def imported_commit():
    """The Aurora commit the `aurora` branch is at, from its trailer."""
    message = git("log", "-1", "--format=%B", BRANCH, check=False)
    for line in message.splitlines():
        if line.startswith(TRAILER):
            return line.split(":", 1)[1].strip()
    return None


def status():
    at = imported_commit()
    if at is None:
        sys.exit("no `aurora` branch yet (docs/AURORA_UPDATES.md)")
    head = fetch(None)
    newer = git("rev-list", "--count", f"{at}..{head}", check=False) or "?"
    print(f"aurora branch: Aurora {at[:8]}; Aurora's main: {head[:8]}, {newer} newer commits")
    print(git("log", "--format=  %h %cs %s", f"{at}..{head}", check=False))


def import_(ref):
    commit = fetch(ref)
    tree = renamed_tree(commit)
    subject = git("log", "-1", "--format=%s", commit)
    message = f"Aurora Viewer {commit[:8]}, renamed for LifeTech\n\n{subject}\n\n{TRAILER} {commit}\n"
    parent = git("rev-parse", "--verify", "-q", BRANCH, check=False)
    args = ["commit-tree", tree] + (["-p", parent] if parent else [])
    new = git(*args, input=message)
    git("update-ref", f"refs/heads/{BRANCH}", new)
    print(f"aurora branch: Aurora {commit[:8]} ({subject})")
    print("next, on main: git merge aurora")


def pick(commit):
    fetch(None)
    commit = git("rev-parse", commit)
    before, after = renamed_tree(f"{commit}^"), renamed_tree(commit)
    patch = subprocess.run(["git", "diff", "--binary", before, after], capture_output=True).stdout
    run = subprocess.run(["git", "apply", "-3", "--index"], input=patch, capture_output=True)
    sys.stdout.write(run.stdout.decode() + run.stderr.decode())
    if run.returncode != 0:
        sys.exit("some parts clash: fix the marked files, then commit")
    print(f"Aurora {commit[:8]} applied; check it, then commit")


def main():
    args = sys.argv[1:]
    if not args or args[0] in ("-h", "--help"):
        print(__doc__)
    elif args[0] == "status":
        status()
    elif args[0] == "import":
        import_(args[1] if len(args) > 1 else None)
    elif args[0] == "pick" and len(args) == 2:
        pick(args[1])
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main()
