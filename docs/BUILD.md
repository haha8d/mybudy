# MyBudy Build & Package Guide

## Prerequisites

### All Platforms
- Node.js 18+ 
- Rust (latest stable)
- Tauri CLI: `cargo install tauri-cli`

### Platform-Specific

#### Windows
- Microsoft Visual Studio C++ Build Tools
- Windows SDK

#### macOS
- Xcode Command Line Tools
- For ARM builds: Rust target `aarch64-apple-darwin`

#### Linux
- `libwebkit2gtk-4.0-dev`
- `libssl-dev`
- `libayatana-appindicator3-dev`

## Build Commands

### Development
```bash
npm run tauri-dev
```

### Production Build
```bash
npm run build
npm run tauri-build
```

## Packaging

### Windows (.msi)
```bash
./scripts/build.sh build-windows
```
Output: `src-tauri/target/release/bundle/msi/*.msi`

### macOS (.dmg)
```bash
# Intel
./scripts/build.sh build-macos

# Apple Silicon
./scripts/build.sh build-macos-arm

# Universal binary
./scripts/build.sh build-macos-universal
```
Output: `src-tauri/target/release/bundle/dmg/*.dmg`

### Linux
```bash
./scripts/build.sh build-linux
```
Outputs:
- `.AppImage`: `src-tauri/target/release/bundle/appimage/*.AppImage`
- `.deb`: `src-tauri/target/release/bundle/deb/*.deb`

## CI/CD Setup

### GitHub Actions Example

```yaml
name: Build

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        platform: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: 18
      - uses: dtolnay/rust-action@stable
      - run: npm install
      - run: npm run tauri-build
      - uses: actions/upload-artifact@v3
        with:
          name: bundles
          path: src-tauri/target/release/bundle/
```

## Distribution

### Auto-Updater
Tauri supports auto-updates via:
- GitHub Releases
- Custom update server

Configure in `tauri.conf.json`:
```json
{
  "updater": {
    "active": true,
    "endpoints": ["https://myserver.com/updates"],
    "dialog": true,
    "pubkey": "YOUR_PUBLIC_KEY"
  }
}
```

## Code Signing

### Windows
Use `signtool` or AzureSignTool:
```bash
tauri sign --sign-identity "CN=My Company"
```

### macOS
Use Apple Developer certificate:
```bash
tauri sign --sign-identity "Developer ID Application: My Company"
```
