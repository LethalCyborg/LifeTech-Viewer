# Building LifeTech Viewer

LifeTech Viewer is written in Rust. One command builds it on every system;
packaging it (an installer, an application, an archive) takes one more
step per system. The ready-made versions are on
<https://viewer.lifetech.systems/#download>.

Everywhere:

- **Rust**, stable, from <https://rustup.rs>. The repository's
  `rust-toolchain.toml` selects the right toolchain on its own.
- **Git**, to clone the repository with its history: the viewer takes its
  version number from the Git tags (`v1.1.0` → `1.1.0`; between two tags,
  `1.1.0+3.gabcdef123`). A source archive without Git can be built with
  `AURORA_VERSION_TAG=v1.1.0`.
- A graphics card with **Vulkan** (Windows, Linux) or **Metal** (macOS,
  Apple Silicon).
- About 15 GB of free disk space for the build folder, and some patience
  for the first build: 10 to 30 minutes, depending on the computer.

```bash
git clone https://github.com/LethalCyborg/LifeTech-Viewer
cd LifeTech-Viewer
```

`cargo build --release` is the optimized build that is published; it takes
longer than the default development build (`cargo run -p lifetech-viewer`),
which behaves the same and is the one to use while working on the code.
`AURORA_DEMO=1` starts the viewer in its offline demo, without logging in.

---

## Windows

### Requirements

1. **Visual Studio 2022 Build Tools** (or Visual Studio 2022), with the
   **Desktop development with C++** workload: the MSVC compiler, the
   Windows SDK and CMake. <https://visualstudio.microsoft.com/downloads/>
   (Tools for Visual Studio › Build Tools).
2. **Rust**: run `rustup-init.exe` from <https://rustup.rs> and keep the
   default (MSVC) toolchain.
3. **Git for Windows**: <https://git-scm.com/download/win>.
4. To package: **PowerShell 7** (`winget install Microsoft.PowerShell`) and,
   for the installer, **Inno Setup 6** (`winget install JRSoftware.InnoSetup`).

### Build

In a terminal (PowerShell):

```powershell
cargo build --release --locked -p lifetech-viewer
.\target\release\lifetech-viewer.exe
```

### Package: a folder, a zip and an installer

```powershell
$version = ((.\target\release\lifetech-viewer.exe --version) -replace '^LifeTech Viewer ', '')
$dir = "LifeTech-Viewer-$version-windows-x64"
pwsh ./scripts/package.ps1 -Exe target/release/lifetech-viewer.exe -Out $dir
Compress-Archive -Path $dir -DestinationPath "$dir.zip"
& "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe" "/DAppVersion=$version" "/DSourceDir=$PWD\$dir" "/DOutputName=$dir-setup" "/O$PWD" packaging\windows\lifetech-viewer.iss
```

This gives `LifeTech-Viewer-<version>-windows-x64-setup.exe`: it installs
for the current user without administrator rights
(`%LOCALAPPDATA%\Programs\LifeTech Viewer`), with a Start menu entry, an
optional desktop shortcut and an uninstaller in Settings › Apps.

An installer that is not signed with a code signing certificate makes
Windows SmartScreen ask first: **More info** › **Run anyway**.

---

## Linux

### Requirements

Debian and Ubuntu (24.04 or later):

```bash
sudo apt install git build-essential cmake clang pkg-config curl \
  libasound2-dev libudev-dev libxkbcommon-dev libwayland-dev \
  libx11-dev libxcursor-dev libxrandr-dev libxi-dev libvulkan1 mesa-vulkan-drivers
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Arch Linux:

```bash
sudo pacman -S --needed git base-devel rustup cmake clang pkgconf alsa-lib libxkbcommon \
  wayland libx11 libxcursor libxrandr libxi vulkan-icd-loader
rustup default stable
```

Fedora:

```bash
sudo dnf install git gcc gcc-c++ cmake clang pkgconf-pkg-config alsa-lib-devel systemd-devel \
  libxkbcommon-devel wayland-devel libX11-devel libXcursor-devel libXrandr-devel libXi-devel vulkan-loader
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build

```bash
cargo build --release --locked -p lifetech-viewer
./target/release/lifetech-viewer
```

### Package: an archive with a menu entry

```bash
version="$(./target/release/lifetech-viewer --version | sed 's/^LifeTech Viewer //')"
dir="LifeTech-Viewer-$version-linux-x64"
mkdir -p "$dir/assets/emoji" "$dir/assets/dictionaries"
cp target/release/lifetech-viewer LICENSE NOTICE.md README.md packaging/linux/lifetech-viewer.desktop "$dir/"
cp assets/branding/lifetech-icon.png "$dir/lifetech-viewer.png"
cp assets/emoji/LICENSE-OFL.txt "$dir/assets/emoji/"
cp crates/lifetech-viewer/assets/dictionaries/README.md "$dir/assets/dictionaries/"
tar -czf "$dir.tar.gz" "$dir"
```

To add it to your applications menu, put the folder where you keep it, then
copy `lifetech-viewer.desktop` to `~/.local/share/applications/` with its
`Exec=` line pointing at the program, and `lifetech-viewer.png` to
`~/.local/share/icons/hicolor/256x256/apps/`.

---

## macOS (Apple Silicon)

### Requirements

```bash
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
brew install powershell   # to build the application (https://brew.sh)
```

### Build

```bash
cargo build --release --locked -p lifetech-viewer
./target/release/lifetech-viewer
```

### Package: the application and a disk image

```bash
pwsh ./scripts/package-macos.ps1 -Exe target/release/lifetech-viewer -Out "LifeTech Viewer.app"
codesign --force --sign - "LifeTech Viewer.app"          # ad hoc signature
version="$("LifeTech Viewer.app/Contents/MacOS/lifetech-viewer" --version | sed 's/^LifeTech Viewer //')"
mkdir dmg && ditto "LifeTech Viewer.app" "dmg/LifeTech Viewer.app" && ln -s /Applications dmg/Applications
hdiutil create -volname "LifeTech Viewer" -srcfolder dmg -fs HFS+ -format UDZO "LifeTech-Viewer-$version-macos-arm64.dmg"
```

An application signed ad hoc opens on the Mac that built it. Downloaded on
another Mac, macOS asks first: right-click it in Applications › **Open**,
or System Settings › Privacy & Security › **Open Anyway**. Without that
step, it needs a Developer ID signature and Apple's notarization (an Apple
Developer account); see [MACOS.md](MACOS.md#signing-for-the-maintainers).

---

## Checks

Before a pull request: `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`
and `cargo test --workspace` (or `pwsh ./scripts/check.ps1`, which runs them).
