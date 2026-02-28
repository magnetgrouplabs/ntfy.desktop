# ntfy.desktop

**A native desktop client for ntfy.**
Built with Rust and Tauri for performance, security, and deep OS integration.

[![Version](https://img.shields.io/github/v/release/magnetgrouplabs/ntfy.desktop?style=flat-square&color=blue)](https://github.com/magnetgrouplabs/ntfy.desktop/releases)
[![Build](https://img.shields.io/github/actions/workflow/status/magnetgrouplabs/ntfy.desktop/release.yml?style=flat-square)](https://github.com/magnetgrouplabs/ntfy.desktop/actions)
[![License](https://img.shields.io/github/license/magnetgrouplabs/ntfy.desktop?style=flat-square&color=green)](LICENSE)
[![Downloads](https://img.shields.io/github/downloads/magnetgrouplabs/ntfy.desktop/total?style=flat-square&color=orange)](https://github.com/magnetgrouplabs/ntfy.desktop/releases)

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-support-yellow?style=flat-square&logo=buy-me-a-coffee)](https://www.buymeacoffee.com/anthonymichael)

---

## Overview

ntfy.desktop is a ground-up Rust rewrite of the original Electron-based ntfy desktop client.

The focus is:

- Native performance
- Reduced memory footprint
- Secure credential handling
- Clean OS-level integration

Designed for power users and self-hosters who want a lightweight native ntfy experience.

---

## Performance Summary

Measured on Windows 11 Pro (Build 22631), February 2026.

| Metric | Electron Client | Rust Build |
|--------|-----------------|------------|
| Idle Memory (avg) | ~383 MB | ~34 MB |
| Binary Size | ~196 MB | ~16 MB |
| Idle CPU (avg) | ~0.35% | ~0.09% |
| Startup Time | ~484 ms | ~482 ms |

Improvements:

- 91% less memory usage
- 92% smaller binary
- 73% lower idle CPU
- Comparable startup performance

---

## Feature Set

### Smart Notifications

- Dynamic icon download and caching
- 128x128 image optimization for Windows Toast
- 7-day cache expiration
- Five sound profiles: Alert, Bell, Chime, Pop, None
- Separate urgent sound configuration
- Three persistence modes: Off, All, Urgent Only

### Security & Authentication

- OS-native credential storage
- No plaintext credentials on disk
- API Token (Bearer)
- HTTP Basic Authentication
- Automatic migration from legacy storage

### Native OS Integration

- Native menu bar
- System tray support
- Close-to-tray behavior
- Platform keyboard shortcuts
- Window visibility toggle

---

## Platform Support

| Platform | Status |
|----------|--------|
| Windows | Fully supported |
| macOS | Not yet available |
| Linux | Not yet available |

macOS and Linux support is planned but not currently built or tested. Contributions are welcome — if you'd like to help get builds working on these platforms, feel free to open a PR.

---

## Contributing

Contributions are welcome. If you're interested in helping out — whether it's macOS/Linux support, bug fixes, or new features — feel free to open an issue or submit a pull request.

---

## License

MIT License Copyright (c) 2026 The Magnet Group LLC
