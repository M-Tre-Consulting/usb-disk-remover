# USB Disk Remover

A lightweight, portable Windows utility for safely ejecting removable USB and Firewire drives. Written in 100% pure Rust with a modern Windows 11 Fluent interface powered by [Slint](https://slint.dev/).

---

## ✨ Features

- **Accurate Detection**: Detects USB and Firewire drives, including external hard drives that Windows classifies as fixed disks.
- **Detailed Information**: Displays drive letters, volume labels, hardware vendor, and product names for all connected devices.
- **Safe Ejection**: Locks and dismounts file system volumes via `FSCTL_LOCK_VOLUME` and `FSCTL_DISMOUNT_VOLUME`, then cleanly ejects the physical device via the Windows PnP manager (`CM_Request_Device_Eject`).
- **Multi-Partition Support**: Handles multi-partition devices correctly by identifying and dismounting all sibling volumes before ejecting the parent device.
- **Windows 11 Fluent UI**: Native Windows 11 look and feel with automatic Dark/Light mode support, immersive title bar, and Fluent styling.
- **Ultra-Lightweight & Fast**: Zero Chromium/WebView2 overhead. Starts instantly (<50 ms) and uses ~15 MB of RAM.
- **System Tray Integration**: Minimizes to the notification area when closed with a quick context menu (Open, About, Quit).
- **Portable**: Single standalone executable with no runtime dependencies or installation required.

---

## 💻 Requirements

- **Operating System**: Windows 10 or Windows 11 (64-bit)
- **Privileges**: Standard user (no administrator rights required for normal removable drive operations)

---

## 🛠️ Building from Source

### Prerequisites

- [Rust Toolchain](https://rustup.rs/) (edition 2024, Rust 1.92+)
- Visual Studio C++ Build Tools (or `winget install Microsoft.VisualStudio.2022.BuildTools`)

> **Note:** Unlike previous versions, Node.js, npm, Vite, and Tauri are no longer required. The project builds entirely with `cargo`.

### Steps

```bash
git clone https://github.com/m4ce-w1ndu/usb-disk-remover
cd usb-disk-remover
cargo build --release
```

The compiled standalone executable will be located at:
```text
target/release/usb-disk-remover.exe
```

---

## 🚀 Usage

1. Launch `usb-disk-remover.exe`.
2. All connected removable drives will appear in the card list.
3. Select an entry and click **Rimuovi in sicurezza** (Safely Remove), or **double-click** any drive row to eject it immediately.
4. Closing the window with the **✕** button minimizes the application to the Windows System Tray. Click the tray icon to restore the window, or right-click for the context menu.

---

## ⚙️ How It Works

1. **Drive Enumeration**: Queries the Windows storage stack using `GetLogicalDrives`, `GetDriveTypeW`, and `DeviceIoControl` with `IOCTL_STORAGE_QUERY_PROPERTY`. This allows identifying the underlying bus type (USB, 1394) and extracting vendor and product strings even for drives that Windows reports as fixed disks.
2. **Two-Phase Safe Ejection**:
   - **Phase 1 (Lock & Dismount)**: Finds all partition volumes sharing the same physical device number (`IOCTL_STORAGE_GET_DEVICE_NUMBER`) and locks and dismounts them via the Windows file system layer (`FSCTL_LOCK_VOLUME` and `FSCTL_DISMOUNT_VOLUME`).
   - **Phase 2 (PnP Ejection)**: Locates the physical device instance node (`CM_Locate_DevNodeW`) and requests clean hardware ejection on its parent bus node via `CM_Request_Device_EjectW` (identical to the Windows "Safely Remove Hardware" mechanism).

---

## 📁 Project Structure

```text
usb-disk-remover/
├── Cargo.toml          # Rust package configuration & dependencies (Slint, Windows API)
├── build.rs            # Slint UI compiler configuration and Windows PE icon embedding
├── icons/              # Application & System Tray icons
├── src/
│   ├── main.rs         # Application entry point, window management, tray & async threads
│   ├── drives.rs       # Drive enumeration & bus property queries (Win32 IOCTL)
│   ├── eject.rs        # Two-phase volume locking, dismount & PnP device ejection
│   └── utils.rs        # String & bit manipulation helpers
└── ui/
    └── app.slint       # Windows 11 Fluent UI definition & System Tray component
```

---

## 📜 Acknowledgements

This project is inspired by and based upon the specification of [USB Disk Ejector](https://github.com/quickandeasysoftware/USB-Disk-Ejector) by QuickAndEasySoftware.

---

## 📄 License

[MIT](LICENSE)
