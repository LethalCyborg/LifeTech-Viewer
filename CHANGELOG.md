# Changelog

What LifeTech Viewer changes compared with
[Aurora Viewer](https://github.com/Aurora-Viewer/Aurora-Viewer), the viewer
it is based on, version by version. Aurora's own improvements come in as
they are released (see "Aurora Viewer updates" in each version).

## 1.1.0 — 2026-11-11

### Camera and controls
- **Alt + W / A / S / D move the camera**, as in the Legacy Viewer: Alt+A
  and Alt+D orbit around, Alt+W and Alt+S move closer and farther, Alt+E
  and Alt+C swing over and under. Ctrl+Alt+W / S swing over and under,
  Ctrl+Alt+Shift+W / A / D pan. The avatar no longer walks or turns while
  Alt is held. They follow your own key bindings and keyboard layout.

### Windows and dialogs
- **A window you open comes to the front**, from a menu, a button or a
  shortcut, so **Ctrl+W closes it**. Before, a window opened again could
  stay behind the others and Ctrl+W closed another one.
- **Ctrl+W on a dialog acts like Escape**: a dialog that Escape closes
  closes; one that needs an answer stays.
- **Ctrl+W ignores a script's dialog** (llDialog, llTextBox) once it has
  been on screen for 5 seconds, as its Ignore button does. Pressed sooner,
  it does nothing, so a dialog is never thrown away unread.

### Chat and notifications
- **Friends online / offline**: the line in nearby chat no longer counts as
  unread chat (no number beside Nearby chat for it). The toast and the
  nearby chat line are two separate switches (Preferences › Chat): either,
  both or none.

### Mini-map and building
- **Mini-map height marks**: an avatar higher than you shows a ^ inside its
  dot, a lower one a v; the direction pointer stays.
- **The Build Chest**: the texture library's new name, and its inventory
  folder's (an existing "LifeTech Library" or "Aurora Library" folder is
  renamed).

### Media
- **Video on prims keeps its colours**: video played through the GStreamer
  plugin (Linux) had red and blue swapped; frames now follow each plugin's
  own pixel format. Web pages are unchanged.

### macOS
- **Resource monitor**: the processor, memory and thread readings now work
  on macOS (they showed "—").
- The log notes the frame pacing every 30 seconds (time waiting for the
  graphics card and for the frame limiter), to diagnose a low frame rate.

### Aurora Viewer updates
- Aurora Viewer's changes up to its commit `0d701e61`.

## 1.0.0 — 2026-11-10

The first LifeTech Viewer, from Aurora Viewer at its commit `e3778b13`.

### A viewer for newcomers
- **Choose your look on day one**: on an account's very first login, a
  picker offers the starter characters of the Library, tried on in place,
  and worn as a whole outfit once chosen.
- **People › Near you** marks the avatars standing on the same parcel as
  you (beside the existing "sitting" mark).
- **Web and media support** (profiles, search, media on prims) is installed
  with the viewer by default, without a separate step.

### Name and look
- The LifeTech name, the LT logo and icons, everywhere: windows, menus,
  About, the system's notifications, the installers.
- A new login screen background: the LifeTech network.

### Simpler
- **VR view is removed**, with its toolbar button and settings.
- No news feed, wiki or update checks pointing to Aurora Viewer's: LifeTech
  publishes its own versions.

### Help and reporting
- **Report a bug**, crash reports and the About window's links lead to
  LifeTech Viewer's own repository
  (<https://github.com/LethalCyborg/LifeTech-Viewer>).

### For developers
- Aurora Viewer's updates are merged through a renamed `aurora` branch
  (`scripts/aurora-update.py`, [docs/AURORA_UPDATES.md](docs/AURORA_UPDATES.md)).
- Building on each system: [docs/BUILDING.md](docs/BUILDING.md).
