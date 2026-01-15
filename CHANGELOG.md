# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-01-15

### Added
- 🚀 Initial release
- WhatsApp Web wrapper using Tauri 2.0
- System tray support with show/hide functionality
- Close to tray behavior (X button minimizes, doesn't quit)
- Windows startup integration with proper app name
- Native notification support (enabled by default)
- Single instance enforcement
- Custom WhatsApp icon
- Proper Start Menu integration
- Task Manager shows correct app name

### Technical
- Built with Tauri 2.0 and Rust
- Uses WebView2 (Edge) for rendering
- Memory usage: ~30 MB (vs ~700 MB for official Electron app)
- Install size: ~5 MB
