<div align="center">

# 🔔 ntfy.desktop

**A native desktop client for ntfy**  
*Built with Rust and Tauri for performance, security, and deep OS integration*

<!-- Badges -->
<div align="center">

[![Version](https://img.shields.io/github/v/release/magnetgrouplabs/ntfy.desktop?style=for-the-badge&color=blue&label=Version)](https://github.com/magnetgrouplabs/ntfy.desktop/releases)
[![Build](https://img.shields.io/github/actions/workflow/status/magnetgrouplabs/ntfy.desktop/release.yml?style=for-the-badge&label=Build)](https://github.com/magnetgrouplabs/ntfy.desktop/actions)
[![License](https://img.shields.io/github/license/magnetgrouplabs/ntfy.desktop?style=for-the-badge&color=green&label=License)](LICENSE)
[![Downloads](https://img.shields.io/github/downloads/magnetgrouplabs/ntfy.desktop/total?style=for-the-badge&color=orange&label=Downloads)](https://github.com/magnetgrouplabs/ntfy.desktop/releases)

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-support-yellow?style=for-the-badge&logo=buy-me-a-coffee)](https://www.buymeacoffee.com/anthonymichael)

</div>

</div>

---

## 📋 Table of Contents

- [✨ What's New](#-whats-new)
- [⚡ Performance & Testing](#-performance--testing)
- [🚀 Complete Feature Set](#-complete-feature-set)
  - [🔔 Smart Notifications](#-smart-notifications)
  - [🔐 Security & Authentication](#-security--authentication)
  - [💻 Native System Integration](#-native-system-integration)
  - [⚙️ Configuration & Control](#️-configuration--control)
- [🖥️ Platform Support](#️-platform-support)
- [🤝 Contributing](#-contributing)
- [📄 License](#-license)

---

## ⚙️ Settings Configuration

**Quick access to settings documentation:**

- [Instance URL Configuration](#instance-url-configuration)
- [API Token Setup](#api-token-setup)
- [Topics Management](#topics-management)
- [Notification Settings](#notification-settings)
- [General Preferences](#general-preferences)

---

## 📖 Overview

ntfy.desktop is a ground-up Rust rewrite of the original Electron-based ntfy desktop client.

The focus is:

- Native performance
- Reduced memory footprint
- Secure credential handling
- Clean OS-level integration

---

## ✨ What's New

This Rust rebuild takes everything great about the original Electron client and pushes it further with native performance, deeper OS integration, and modern security practices.

### 🆚 Head-to-Head Comparison

| Capability | Original Electron | This Rust Build |
|------------|-------------------|-----------------|
| **Memory Usage (Idle)** | ~383 MB | **~34 MB** (91% less) |
| **Binary Size** | ~196 MB | **~16 MB** (92% smaller) |
| **Credential Storage** | Plaintext JSON | **OS-native keychain** |
| **Authentication** | API Token only | **Token + HTTP Basic Auth** |
| **Notification Icons** | Not supported | **Auto-download & cache** |
| **Sound Options** | Default only | **5 distinct profiles** |
| **Persistence Modes** | On/Off | **Off / All / Urgent** |
| **Menu Integration** | None | **Full native menu bar** |
| **First-Run Setup** | Manual configuration | **Built-in welcome wizard** |
| **Icon Cache Management** | N/A | **7-day TTL with auto-expiry** |

---

## ⚡ Performance & Testing

Measured on Windows 11 Pro (Build 22631), February 2026.

### 📊 Methodology Summary

- **60-minute sampling window** per application
- **Idle state**, no user interaction
- **Same topics subscribed**
- **Connected to ntfy.sh**
- **Working Set memory** measured via Process Monitor
- **CPU tracked** via Windows Performance Monitor
- **Binary size** measured from clean builds
- **Startup time** averaged over 10 cold launches

### 📈 Results

| Metric | Electron | Rust | Improvement |
|--------|----------|------|-------------|
| **Idle Memory** | ~383 MB | **~34 MB** | **91% less** |
| **Peak Memory** | ~412 MB | **~38 MB** | **91% less** |
| **Idle CPU** | ~0.35% | **~0.09%** | **74% lower** |
| **Binary Size** | ~196 MB | **~16 MB** | **92% smaller** |
| **Startup Time** | ~484 ms | **~482 ms** | Comparable |

### 🎯 Performance Gains

Performance improvements are achieved through:

1. **System WebView2** instead of bundled Chromium
2. **Native Rust compilation** with optimized builds
3. **Tokio async runtime** for efficient I/O
4. **Optimized release builds** with minimal overhead

Testing included **burst notifications**, **single-message latency checks**, and **24-hour stability runs** with memory monitoring.

---

## 🚀 Complete Feature Set

### 🔔 Smart Notifications

- **Automatic icon download** and 128x128 optimization for Windows Toast
- **Intelligent 7-day icon cache** with expiration
- **Five sound profiles**: Alert, Bell, Chime, Pop, None
- **Separate urgent notification** sound configuration
- **Persistence modes**: Off, All, Urgent Only

### 🔐 Security & Authentication

- **Credentials stored in OS-native keychain**
  - Windows Credential Manager
  - macOS Keychain
  - Linux Secret Service
- **No plaintext credentials** on disk
- **Bearer token support**
- **HTTP Basic authentication** support

### 💻 Native System Integration

- **Full native menu bar** with platform-specific shortcuts
- **System tray integration** with close-to-tray behavior
- **Toggle visibility** without quitting application
- **Keyboard shortcuts** for all navigation actions

### ⚙️ Configuration & Control

- **First-run welcome wizard** for easy setup
- **Dedicated settings windows** with individual pages
- **Adjustable poll rate** (5-3600 seconds)
- **NDJSON streaming support** for real-time updates
- **CLI arguments**:
  - `--hidden` - Start with window hidden
  - `--devtools` - Open browser devtools on launch

---

## ⚙️ Settings Configuration Details

### Instance URL Configuration
Configure your ntfy server instance URL (default: `https://ntfy.sh/app`). Supports both official ntfy.sh and self-hosted instances.

### API Token Setup
Securely store API tokens for authenticated access to protected topics. Tokens are stored in your OS-native keychain.

### Topics Management
Manage your subscribed topics with comma-separated lists.

### Notification Settings
Customize notification behavior including sound profiles, persistence modes, and icon caching preferences.

### General Preferences
Configure application behavior including startup options, keyboard shortcuts, and developer tools access.

---

## 🖥️ Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| **Windows** | ✅ **Fully supported** | All features implemented and tested |
| **macOS** | 🔄 **Planned** | Contributions welcome |
| **Linux** | 🔄 **Planned** | Contributions welcome |

macOS and Linux support is planned but not currently built or tested. Contributions are welcome — if you'd like to help get builds working on these platforms, feel free to open a PR.

---

## 🤝 Contributing

Contributions are welcome! If you're interested in helping out — whether it's macOS/Linux support, bug fixes, or new features — feel free to open an issue or submit a pull request.

**Areas needing help:**
- macOS build support
- Linux build support
- Additional platform-specific features
- Performance optimizations

---

## 📄 License

MIT License Copyright (c) 2026 The Magnet Group LLC
