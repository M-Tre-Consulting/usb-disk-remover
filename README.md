# USB Disk Remover v1.0.0

A lightweight, portable Windows utility for safely ejecting removable USB and Firewire drives. Written in 100% pure Rust with a modern Windows 11 Fluent interface powered by [Slint](https://slint.dev/).

---

## ✨ Features

- **Accurate Detection**: Detects USB and Firewire drives, including external hard drives that Windows classifies as fixed disks.
- **Detailed Information**: Displays drive letters, volume labels, hardware vendor, and product names for all connected devices.
- **Safe Ejection**: Locks and dismounts file system volumes via `FSCTL_LOCK_VOLUME` and `FSCTL_DISMOUNT_VOLUME`, then cleanly ejects the physical device via the Windows PnP manager (`CM_Request_Device_Eject`).
- **Multi-Partition Support**: Handles multi-partition devices correctly by identifying and dismounting all sibling volumes before ejecting the parent device.
- **Adaptive Native UI (Windows 11 & Windows 10)**: Automatically adapts to your OS version at runtime: rounded Fluent design with Mica system backdrop on Windows 11 (Build >= 22000), and crisp flat design with sharp corners and classic Windows 10 blue accents on Windows 10. Automatic Dark/Light mode support on both.
- **Ultra-Lightweight & Fast**: Zero Chromium/WebView2 overhead. Starts in <40 ms and uses ~15 MB of RAM.
- **Internationalization (i18n)**: Fully translated into 6 languages with live switching:
  - 🇬🇧 English
  - 🇮🇹 Italiano
  - 🇩🇪 Deutsch
  - 🇫🇷 Français
  - 🇸🇦 العربية
  - 🇷🇺 Русский
- **Settings & Autostart**: In-app Settings page to toggle:
  - UI Language selection.
  - Automatic startup with Windows (`HKCU\...\Run`).
  - Start minimized directly to the System Tray (`--minimized`).
  - Minimize to tray when clicking the close button (**✕**).
- **System Tray Integration**: Background resident with an interactive context menu (Open, Settings, About, Quit).
- **Portable or System-Wide Installer**: Available both as a portable single `.exe` and as a clean 64-bit Windows Setup installer for `Program Files`.

---

## 💻 Requirements

- **Operating System**: Windows 10 or Windows 11 (64-bit)
- **Privileges**: Standard user for portable use; administrator rights required only when running the system-wide installer.

---

## 📦 Installation & Download

### Option 1: Windows Setup Installer (System-Wide)
Run or compile the Inno Setup installer:
```text
target/installer/USBDiskRemover-Setup-1.0.0.exe
```
Features of the installer:
- Installs system-wide to `C:\Program Files\USB Disk Remover`.
- Multi-language installer support (English, Italian, German, French, Arabic, Russian).
- Adds Start Menu and optional Desktop shortcuts.
- Clean uninstaller registered in Windows Settings (*Apps & features*).

### Option 2: Standalone Portable Binary
Simply copy `usb-disk-remover.exe` anywhere (even on a USB flash drive) and launch it directly.

---

## 🛠️ Building from Source

### Prerequisites

- [Rust Toolchain](https://rustup.rs/) (edition 2024, Rust 1.92+)
- Visual Studio C++ Build Tools (or `winget install Microsoft.VisualStudio.2022.BuildTools`)
- *(Optional, for building the installer)* [Inno Setup 6](https://jrsoftware.org/isinfo.php) (`winget install JRSoftware.InnoSetup`)

### Build Steps

1. **Compile Release Binary**:
   ```bash
   git clone https://github.com/m4ce-w1ndu/usb-disk-remover
   cd usb-disk-remover
   cargo build --release
   ```
   The compiled standalone executable will be at `target/release/usb-disk-remover.exe`.

2. **Compile Windows Installer (Optional)**:
   ```bash
   ISCC.exe installer/setup.iss
   ```
   The installer package will be output to `target/installer/USBDiskRemover-Setup-1.0.0.exe`.

---

## 🚀 Usage

1. Launch `usb-disk-remover.exe`.
2. All connected removable drives will appear in the card list.
3. Select an entry and click **Safely Remove**, or **double-click** any drive row to eject it immediately.
4. Click **Settings** in the top toolbar to change the language or configure automatic startup and tray behavior.
5. Closing the window with the **✕** button minimizes the application to the Windows System Tray (configurable in Settings). Click the tray icon to restore the window, or right-click for the context menu.

### Command-Line Arguments

- `--minimized`: Starts the application resident in the system tray without displaying the main window. Used by Windows automatic startup.

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
├── Cargo.toml          # Rust package configuration (v1.0.0, Slint, Windows API, Serde)
├── build.rs            # Slint UI compiler and Windows PE icon embedding
├── icons/              # Application & System Tray icons
├── installer/
│   └── setup.iss       # Inno Setup Windows installer script (6 languages, Program Files)
├── src/
│   ├── main.rs         # Application lifecycle, tray icon, language dispatch & worker threads
│   ├── drives.rs       # Drive enumeration & bus property queries (Win32 IOCTL)
│   ├── eject.rs        # Two-phase volume locking, dismount & PnP device ejection
│   ├── i18n.rs         # Internationalization dictionary (en, it, de, fr, ar, ru)
│   ├── settings.rs     # App settings persistence and Windows autostart Registry integration
│   └── utils.rs        # String & bit manipulation helpers
├── ui/
│   └── app.slint       # Windows 11 Fluent UI, Settings modal & System Tray component
└── LICENSE             # MIT License
```

---

## 📜 Acknowledgements

This project is inspired by and based upon the specification of [USB Disk Ejector](https://github.com/quickandeasysoftware/USB-Disk-Ejector) by QuickAndEasySoftware.

---

## 📄 License

[MIT](LICENSE)
