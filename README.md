# MyBudy

AI Assistant with floating button interface - inspired by Lobe Chat design.

## Features

- **Floating Button**: Circular, draggable, always-on-top button for quick access
- **Chat Interface**: Lobe Chat-style sidebar and conversation view
- **Multi-Model Support**: Kimi, OpenAI, DeepSeek, and more
- **Local Capabilities**:
  - Read local files
  - Screen capture
  - Launch system applications
  - SQLite chat history storage
- **Voice Wake**: System native speech recognition
- **Configuration**:
  - `~/.mybudy/config.json` - App settings
  - `~/.mybudy/soul.md` - AI personality
  - `~/.mybudy/models.json` - Model configurations

## Project Structure

```
mybudy/
├── src-tauri/          # Tauri Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/   # Tauri commands
│   │   ├── config/     # Configuration management
│   │   ├── db/         # Database (SQLite)
│   │   ├── models/     # AI model integration
│   │   ├── float_window/   # Floating button window
│   │   ├── screen_capture/ # Screen capture functionality
│   │   └── voice/          # Voice recognition
│   ├── migrations/     # Database migrations
│   └── icons/          # App icons
├── src/                # Frontend code
│   ├── components/     # UI components
│   ├── stores/         # State management
│   ├── styles/         # CSS styles
│   ├── main.js         # Main app entry
│   └── float.js        # Floating button entry
├── scripts/            # Build scripts
└── docs/               # Documentation
```

## Development

### Prerequisites

- Node.js (v18+)
- Rust (latest stable)
- Tauri CLI

### Setup

```bash
# Install dependencies
./scripts/build.sh install

# Run in development mode
./scripts/build.sh dev
```

### Building

```bash
# Build for current platform
./scripts/build.sh build

# Build for specific platforms
./scripts/build.sh build-windows   # Windows (.msi)
./scripts/build.sh build-macos     # macOS Intel (.dmg)
./scripts/build.sh build-macos-arm # macOS ARM (.dmg)
./scripts/build.sh build-linux     # Linux (.AppImage/.deb)
```

### Testing

```bash
./scripts/test.sh
```

## Configuration

### Default Config (`~/.mybudy/config.json`)

```json
{
  "theme": "dark",
  "language": "zh-CN",
  "float_button_position": { "x": 0, "y": 0 },
  "auto_start": false,
  "shortcuts": {
    "show_window": "CmdOrCtrl+Shift+M",
    "voice_wake": "CmdOrCtrl+Shift+V",
    "screenshot": "CmdOrCtrl+Shift+S"
  },
  "voice_enabled": true,
  "default_model": "kimi"
}
```

### Models Config (`~/.mybudy/models.json`)

```json
{
  "providers": [
    {
      "id": "kimi",
      "name": "Kimi",
      "provider": "kimi",
      "api_key": "",
      "api_url": "https://api.moonshot.cn/v1",
      "models": ["moonshot-v1-8k", "moonshot-v1-32k", "moonshot-v1-128k"],
      "default_model": "moonshot-v1-8k",
      "enabled": true
    }
  ]
}
```

## License

MIT
