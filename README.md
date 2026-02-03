# LAN Chat

A lightweight, modern messaging app for communicating with PCs on the same local network.

## Download

### Windows

Download the latest version from the [Releases](https://github.com/YashasVM/LAN-Messanger/releases) page:

- **LAN-Chat_x.x.x_x64-setup.exe** - Windows installer (recommended)
- **LAN-Chat_x.x.x_x64_en-US.msi** - MSI installer

Simply download, run the installer, and you're ready to go!

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

## For Developers

If you want to build from source:

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

### Prerequisites for Building

- Node.js v18+
- Rust (latest stable)
- Windows 10/11 SDK

## License

MIT

---

Made by [@Yashas.VM](https://github.com/YashasVM)
