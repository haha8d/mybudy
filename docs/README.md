# MyBudy Documentation

## Overview

MyBudy is an AI assistant application with a floating button interface, built with Tauri (Rust + Web frontend).

## Architecture

### Backend (Rust)

- **main.rs**: Application entry point
- **commands/**: Tauri command handlers for frontend communication
- **config/**: Configuration management (JSON files in ~/.mybudy/)
- **db/**: SQLite database for chat history
- **float_window/**: Floating button window management
- **models/**: AI model API integration
- **screen_capture/**: Screenshot functionality
- **voice/**: Voice recognition integration

### Frontend (JavaScript)

- **main.js**: Main application entry
- **float.js**: Floating button logic
- **components/**: UI components (App, Sidebar, ChatArea)
- **stores/**: State management (chatStore, configStore)
- **styles/**: CSS styles

## Features

1. **Floating Button**
   - Always on top
   - Draggable
   - Click to open main window
   - Drag-and-drop file support

2. **Chat Interface**
   - Sidebar with chat list
   - Main chat area
   - Message history
   - Multiple model support

3. **Local Capabilities**
   - File reading/writing
   - Directory listing
   - Screen capture
   - Application launching

4. **Configuration**
   - `~/.mybudy/config.json`: App settings
   - `~/.mybudy/models.json`: AI model configs
   - `~/.mybudy/soul.md`: AI personality

## Building

```bash
# Install dependencies
npm install

# Development
npm run tauri-dev

# Build
npm run tauri-build
```

## Testing

```bash
./scripts/test.sh
```
