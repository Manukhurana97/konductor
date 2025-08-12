# Konductor

A Tauri application with system tray functionality for Linux.

## Features

- **System Tray Integration**: The application runs in the system tray on Linux
- **Tray Menu**: Right-click the tray icon to access menu options:
  - Show Konductor: Shows the main application window
  - Hide Konductor: Hides the main application window
  - Quit: Exits the application
- **Click to Toggle**: Left-click the tray icon to show/hide the main window

## Development

### Prerequisites

- Node.js and npm
- Rust and Cargo
- Tauri CLI

### Running the Application

1. Install dependencies:
   ```bash
   npm install
   ```

2. Start the development server:
   ```bash
   npm run tauri dev
   ```

### Building for Production

```bash
npm run tauri build
```

## System Tray Implementation

The system tray functionality is implemented using the `tray-icon` crate and includes:

- **Tray Icon**: Uses the application's 32x32 icon
- **Context Menu**: Provides show/hide/quit options
- **Event Handling**: Responds to left-click and menu item clicks
- **Window Management**: Controls the main application window visibility

### Files

- `src-tauri/src/tray_linux.rs`: System tray implementation
- `src-tauri/src/main.rs`: Main application with tray integration
- `src-tauri/tauri.conf.json`: Tauri configuration with window settings

## Platform Support

Currently tested and working on:
- Linux (Ubuntu 22.04+)

## License

[Add your license here]


