# OpenCap

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Lightweight, open-source, private screenshot tool. Only ~15 MB installed. No accounts, no cloud, no telemetry.

<!-- TODO: Add screenshot or demo GIF here -->
<!-- ![OpenCap Screenshot](docs/screenshot.png) -->

## Features

- **Region capture** — click and drag to select any area of your screen
- **Full screen capture** — right-click to grab the entire screen
- **Clipboard support** — screenshots are automatically copied to your clipboard
- **Auto-save** — images are saved to your Pictures folder with timestamps
- **HiDPI aware** — works correctly on high-density displays
- **~15 MB installed** — native Rust backend, minimal resource usage
- **Fully private** — everything stays on your machine, no network calls

## Download

Grab the latest installer from the [Releases](../../releases/latest) page:

- **Windows:** `.exe` (NSIS installer) or `.msi`
- **macOS:** `.dmg`
- **Linux:** `.deb` or `.AppImage`

## Build from Source

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://www.rust-lang.org/tools/install) (1.77+)
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for your platform

### Steps

```bash
git clone https://github.com/sprintrstudio/openCap.git
cd openCap
npm install
npm run tauri -- build
```

Installers will be in `src-tauri/target/release/bundle/`.

For development:

```bash
npm run tauri -- dev
```

## Usage

1. Launch OpenCap
2. **Drag** to select a region, or **right-click** for full screen
3. Press **Escape** to cancel
4. The screenshot is saved and copied to your clipboard automatically

## License

[MIT](LICENSE)
