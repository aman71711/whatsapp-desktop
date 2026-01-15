# WhatsApp Desktop

A fast, lightweight, and open-source WhatsApp Desktop client built with [Tauri 2.0](https://tauri.app/).

![WhatsApp Desktop](https://img.shields.io/badge/WhatsApp-Desktop-25D366?style=for-the-badge&logo=whatsapp&logoColor=white)
![Tauri](https://img.shields.io/badge/Tauri-2.0-FFC131?style=for-the-badge&logo=tauri&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)
![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)

---

## 📊 Why This App? (Comparison with Official WhatsApp)

| Feature | Official WhatsApp | WhatsApp Desktop (This App) |
|---------|------------------|----------------------------|
| **Memory Usage** | ~700 MB 😰 | **~30 MB** ⚡ |
| **Technology** | Electron | Tauri + Rust |
| **Install Size** | ~500 MB | **~5 MB** |
| **Startup Speed** | Slow | **Instant** |
| **System Resources** | Heavy | **Minimal** |
| **Open Source** | ❌ No | ✅ **Yes** |
| **Telemetry** | Yes | ❌ **None** |
| **System Tray** | ✅ Yes | ✅ Yes |
| **Notifications** | ✅ Yes | ✅ Yes |
| **Windows Startup** | ✅ Yes | ✅ Yes |

> 💡 **This app uses 20x less memory** than the official WhatsApp Desktop while providing the same functionality!

### 📊 Memory Comparison

```
┌─────────────────────────────────────────────────────────────────┐
│ Task Manager - Memory Usage                                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  WhatsApp (Official)     ████████████████████████████  694.5 MB │
│  WhatsApp Desktop (This) ██                              30.3 MB │
│                                                                 │
│  💡 23x LESS MEMORY!                                            │
└─────────────────────────────────────────────────────────────────┘
```

---

## ✨ Features

- 🚀 **Lightning Fast** - Built with Tauri and Rust for minimal resource usage
- 💾 **Tiny Memory Footprint** - Uses only ~30 MB RAM vs ~700 MB for official app
- 📦 **Small Install Size** - Just ~5 MB installer
- 🔔 **Native Notifications** - Full Windows notification support (enabled by default)
- 📌 **System Tray** - Minimize to tray, click to show/hide
- 🚀 **Windows Startup** - Starts automatically with Windows (configurable)
- 🔒 **Privacy Focused** - No telemetry, no tracking, fully open source
- 🪟 **Native Experience** - Proper Windows integration with Start Menu shortcuts
- 🔄 **Single Instance** - Prevents multiple instances from running
- 📱 **Full WhatsApp Web Features** - All features of WhatsApp Web in a native window

---

## 📥 Installation

### Download Pre-built Binary

Download the latest release from the [Releases](https://github.com/user/whatsapp-desktop/releases) page.

**Available formats:**
- `WhatsApp Desktop_x.x.x_x64-setup.exe` - NSIS Installer (Recommended)
- `WhatsApp Desktop_x.x.x_x64_en-US.msi` - MSI Installer

### System Requirements
- Windows 10/11 (64-bit)
- WebView2 Runtime (usually pre-installed on Windows 10/11)

---

## ⚙️ Configuration

### 🚀 Windows Startup

The app automatically starts with Windows (minimized to system tray). 

**To disable Windows Startup:**

**Option 1: Task Manager**
1. Press `Ctrl + Shift + Esc` to open Task Manager
2. Go to **Startup** tab
3. Find **WhatsApp Desktop**
4. Right-click → **Disable**

**Option 2: Windows Settings**
1. Open **Settings** (Win + I)
2. Go to **Apps** → **Startup**
3. Toggle **WhatsApp Desktop** off

### 🔔 Notifications

Notifications are **enabled by default** and work natively through Windows.

**To manage notifications:**
1. Open **Settings** (Win + I)
2. Go to **System** → **Notifications**
3. Scroll down to find **WhatsApp Desktop**
4. Toggle on/off or customize notification settings

---

## 🖱️ How to Use

### System Tray Behavior
| Action | Result |
|--------|--------|
| **Left Click** on tray icon | Show/Hide window |
| **Right Click** on tray icon | Open context menu |
| **Close Button (X)** | Minimize to tray (doesn't quit) |

### Tray Menu Options
- **Show WhatsApp** - Bring window to front
- **Hide** - Minimize to system tray
- **Quit** - Completely exit the application

### Tips
- The app runs in the background when you close the window
- To fully quit, right-click the tray icon and select "Quit"
- You can pin the app to your taskbar for quick access

---

## 🛠️ Build from Source

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (v18 or higher)
- Windows 10/11 with Visual Studio Build Tools

### Build Steps

```bash
# Clone the repository
git clone https://github.com/user/whatsapp-desktop.git
cd whatsapp-desktop

# Install dependencies
npm install

# Development mode (with hot reload)
npm run dev

# Build for production
npm run build
```

The installers will be generated in:
- `src-tauri/target/release/bundle/nsis/` - NSIS installer
- `src-tauri/target/release/bundle/msi/` - MSI installer

---

## 📁 Project Structure

```
whatsapp-desktop/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   └── main.rs         # Main application logic
│   ├── icons/              # App icons (all sizes)
│   ├── capabilities/       # Tauri permissions
│   ├── Cargo.toml          # Rust dependencies
│   └── tauri.conf.json     # Tauri configuration
├── dist/                   # Frontend assets
├── .github/
│   └── workflows/          # GitHub Actions for releases
├── package.json            # Node.js configuration
├── LICENSE                 # MIT License
├── CHANGELOG.md            # Version history
└── README.md               # This file
```

---

## 🔧 Technologies

| Technology | Purpose |
|------------|---------|
| [Tauri 2.0](https://tauri.app/) | Lightweight app framework |
| [Rust](https://www.rust-lang.org/) | Fast, memory-safe backend |
| [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) | Windows native browser engine |
| [WhatsApp Web](https://web.whatsapp.com/) | Official WhatsApp web interface |

---

## 🔒 Privacy & Security

- **No Telemetry** - This app doesn't collect any data
- **No Tracking** - No analytics or user tracking
- **Open Source** - Full source code available for audit
- **Direct Connection** - Connects directly to WhatsApp Web servers
- **No Middleman** - Your messages go directly to WhatsApp, not through us

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/AmazingFeature`)
3. **Commit** your changes (`git commit -m 'Add AmazingFeature'`)
4. **Push** to the branch (`git push origin feature/AmazingFeature`)
5. **Open** a Pull Request

### Ideas for Contribution
- Linux/macOS support
- Custom themes
- Keyboard shortcuts
- Performance improvements
- Bug fixes

---

## 📋 Roadmap

- [x] Basic WhatsApp Web wrapper
- [x] System tray support
- [x] Close to tray behavior
- [x] Windows startup integration
- [x] Native notifications
- [x] Single instance enforcement
- [x] Proper app name in Task Manager
- [ ] Custom keyboard shortcuts
- [ ] Linux support
- [ ] macOS support
- [ ] Auto-update feature
- [ ] Custom themes

---

## ❓ FAQ

**Q: Is this safe to use?**
> Yes! This is just a wrapper around the official WhatsApp Web. Your messages are still end-to-end encrypted by WhatsApp.

**Q: Why does it use so much less memory?**
> The official WhatsApp uses Electron, which bundles an entire Chromium browser. This app uses Tauri with the system's built-in WebView2, which is much more efficient.

**Q: Will I get banned for using this?**
> No. This app uses the official WhatsApp Web interface, the same as opening web.whatsapp.com in your browser.

**Q: Why doesn't it appear in my Start Menu after running the exe?**
> You need to install the app using the installer (.exe or .msi), not just run the executable directly.

---

## ⚠️ Disclaimer

This is an **unofficial** wrapper for WhatsApp Web.

- WhatsApp® is a registered trademark of Meta Platforms, Inc.
- This project is **not affiliated** with, endorsed by, or sponsored by Meta Platforms, Inc.
- This app simply wraps the official WhatsApp Web interface in a native window
- All WhatsApp functionality is provided by WhatsApp's servers

---

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

You are free to:
- ✅ Use commercially
- ✅ Modify
- ✅ Distribute
- ✅ Use privately

---

## ⭐ Support

If you find this project useful, please consider:
- ⭐ **Starring** this repository
- 🐛 **Reporting** bugs or issues
- 💡 **Suggesting** new features
- 🤝 **Contributing** to the code

---

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - For the amazing lightweight framework
- [WhatsApp](https://www.whatsapp.com/) - For WhatsApp Web
- The open-source community - For inspiration and support

---

<p align="center">
  <b>Made with ❤️ by the Open Source Community</b>
</p>

<p align="center">
  <a href="https://github.com/user/whatsapp-desktop/stargazers">⭐ Star this repo if you find it useful!</a>
</p>
