# LAN Chat

A lightweight, modern messaging app for communicating with PCs on the same local network.

## Features

- **LAN Peer Discovery**: Automatically discovers other LAN Chat users on your network using UDP broadcast
- **Real-time Messaging**: Send and receive messages instantly over TCP
- **File Sharing**: Share files of any type with other users
- **Emoji Support**: Built-in emoji picker for expressive messages
- **Native Notifications**: Get notified when new messages arrive (Windows)
- **Fully Resizable**: Resize from a small widget to full screen
- **Lightweight**: Built with Tauri for minimal resource usage (~10MB)

## Screenshots

The app features a clean, modern UI with red/black/white color scheme inspired by contemporary design principles.

## Building for Windows

### Prerequisites

- [Node.js](https://nodejs.org/) v18 or later
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- Windows 10/11

### Build Steps

1. Clone the repository:
```bash
git clone https://github.com/YashasVM/LAN-Messanger.git
cd LAN-Messanger
```

2. Install dependencies:
```bash
npm install
```

3. Build the app:
```bash
npm run tauri build
```

The Windows executable will be generated in:
- `src-tauri/target/release/bundle/nsis/` (NSIS installer)
- `src-tauri/target/release/bundle/msi/` (MSI installer)

### Development

Run in development mode:
```bash
npm run tauri dev
```

## How It Works

1. **Peer Discovery**: The app broadcasts its presence on the local network every 3 seconds using UDP
2. **Connection**: When you select a peer, messages are sent via TCP for reliable delivery
3. **File Transfer**: Files are base64-encoded and sent through the same TCP channel
4. **Notifications**: Windows native notifications alert you to new messages

## Tech Stack

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri
- **Styling**: Custom CSS with CSS variables
- **Networking**: UDP for discovery, TCP for messaging

## Requirements

- Both devices must be on the same local network
- Firewall must allow UDP port 45677 (discovery) and TCP port 45678 (messaging)

## License

MIT
