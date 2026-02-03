# LAN Chat

A lightweight, modern messaging app for communicating with PCs on the same local network.

<p align="center">
  <img src="https://img.shields.io/badge/Platform-Windows-blue?style=for-the-badge&logo=windows" alt="Windows">
  <img src="https://img.shields.io/badge/Size-~10MB-green?style=for-the-badge" alt="Size">
  <img src="https://img.shields.io/github/v/release/YashasVM/LAN-Messanger?style=for-the-badge&label=Version" alt="Version">
</p>

## Download

<p align="center">
  <a href="https://github.com/YashasVM/LAN-Messanger/releases/latest">
    <img src="https://img.shields.io/badge/Download-Windows%20Installer-E53935?style=for-the-badge&logo=windows&logoColor=white" alt="Download for Windows">
  </a>
</p>

**[Click here to download the latest version](https://github.com/YashasVM/LAN-Messanger/releases/latest)**

Simply download the `.exe` installer, run it, and you're ready to go!

## Features

- **Auto-Discovery** - Automatically finds other LAN Chat users on your network
- **Instant Messaging** - Send and receive messages in real-time
- **File Sharing** - Share any file with other users
- **Emoji Support** - Built-in emoji picker
- **Notifications** - Get notified when new messages arrive
- **Fully Resizable** - From a small widget to full screen
- **Lightweight** - Only ~10MB, uses minimal resources

## How to Use

1. Download and install LAN Chat on all PCs you want to connect
2. Make sure all PCs are on the same local network (WiFi or Ethernet)
3. Open LAN Chat - it will automatically find other users
4. Click on a contact to start chatting!

## Requirements

- Windows 10 or Windows 11
- All devices must be on the same local network
- Firewall must allow:
  - UDP port 45677 (for discovery)
  - TCP port 45678 (for messaging)

## Screenshots

The app features a clean, modern UI with red/black/white color scheme.

---

## For Developers

<details>
<summary>Click to expand build instructions</summary>

### Building from Source

```bash
# Clone the repository
git clone https://github.com/YashasVM/LAN-Messanger.git
cd LAN-Messanger

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Prerequisites

- Node.js v18+
- Rust (latest stable)
- Windows 10/11 SDK

</details>

---

## License

MIT

---

<p align="center">
  Made with ❤️ by <a href="https://github.com/YashasVM">@Yashas.VM</a>
</p>
