# Changelog

What LifeTech Viewer changes compared with
[Aurora Viewer](https://github.com/Aurora-Viewer/Aurora-Viewer), the viewer
it is based on, version by version. Aurora's own improvements come in as
they are released (see "Aurora Viewer updates" in each version).

## 1.3.2 — 2026-11-18

### People
- **The LifeTech Team mark** (the glow on the name plate and the "LifeTech
  Team" tag in chat, People and profiles) now goes to LifeTech's own team.
  It used to mark Aurora's developers, a list carried over from Aurora.

## 1.3.1 — 2026-11-17

### Notecards
- **Find and replace**: the search button beside the Classic, Markdown
  and Edit tabs, or Ctrl+F, opens a bar with Find, Replace, Replace all
  and Match case, in all three views. Enter or F3 finds the next match,
  Shift+F3 the previous one, and the match is highlighted in the text.
  Ctrl+Z takes a replacement back, and items embedded in the notecard are
  never replaced away. In Markdown, Find searches the formatted text and
  a replacement changes the notecard's text underneath.

### Inventory
- **Home, End, Page Up and Page Down** move the selection to the first
  row, the last row or a page away; with Shift they extend it. With
  nothing selected, Down starts at the top and Up at the bottom.
- **The wide inventory keeps the keyboard**: after a click on a folder in
  its left-hand column, typing letters and the arrow keys still move
  through the list.

## 1.3.0 — 2026-11-17

### Aurora Viewer updates
Aurora's work up to 16 November 2026: about 110 new features and 150
fixes. Among them:

- **LifeTech Weather**: rain, snow that settles, wind, thunder and
  northern lights, drawn for you only or set by the region.
- **The Director**: a camera track and a recorder to film with.
- **Avatar Studio**: a focused view to edit your avatar, with an animation
  editor and a compact poser; move several poser joints together.
- **Outfits**: a gallery showcase in Appearance, with automatic portraits.
- **Chat**: the Omnifilter, conversation options (sort tabs by type,
  "(You)", system messages in brackets, moderator styles), ((OOC))
  brackets closed for you, nearby chat from beyond chat range dimmed,
  blocked residents' chat hidden or greyed.
- **People**: type-ahead in the friend lists, radar marks with your own
  colour, a region badge, ban or unban someone from all your groups.
- **Building**: an SLua editor, save objects as Collada or OBJ, beacons,
  Build › Options, "Select faces with this texture".
- **Camera and avatar**: camera roll shortcuts, Look at Last Chatter
  (Ctrl+\), face the nearest avatar and the Compass, Animation Speed menu,
  draw distance up to 1024 m.
- **And more**: a startup splash, script recovery after a crash, cancel a
  teleport in progress, the Milky Way from real star data, more photo LUTs
  built in, god mode for OpenSim grids.

### Kept from LifeTech
Everything LifeTech added stays as it was: Choose your look, the Build
Chest, the parcel badge, the Alt camera keys, Ctrl+W, usernames in
brackets and inventory type-ahead. LifeTech still has no VR.

## 1.2.0 — 2026-11-16

### Chat
- **Usernames in brackets**: when someone's display name differs from
  their username, conversations show both, the username in brackets:
  "Tess ✦ (tesstouch)". Everyone whose display name is their username
  shows once, as before. In every chat style (Aurora, Compact, Bubbles and
  Legacy), nearby chat, group chats and IMs alike.

### Inventory
- **Type to jump**: typing letters in the inventory selects the next
  folder or item whose name starts with them, as in the Legacy Viewer.
  Type the same letter again for the next match; after a one-second pause,
  the next letter starts a new word. The list has the keyboard when the
  inventory opens, so typing jumps instead of searching; click the search
  field to search.

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
