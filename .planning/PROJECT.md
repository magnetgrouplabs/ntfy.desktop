# ntfy.desktop - Settings UI Fixes

## What This Is

**ntfy.desktop** is a Tauri 2.0 desktop notification client for ntfy.sh. It loads the ntfy web UI in a webview and bridges web notifications to native OS notifications. This milestone focuses on fixing critical issues in the Settings UI before release.

## Core Value

A working settings experience where users can configure their ntfy instance, credentials, and notification preferences — and have those settings persist and take effect.

## Requirements

### Validated

- ✓ Tauri 2.0 app loads ntfy web UI successfully
- ✓ Background polling for new messages works
- ✓ Native OS notifications display correctly (Windows Toast, macOS osascript, Linux notify-send)
- ✓ System tray with close-to-tray behavior works
- ✓ Menu bar (File/View/Settings) implemented
- ✓ Welcome screen for first-time setup
- ✓ Build process produces MSI and NSIS installers

### Active

- [ ] **BUG-01**: Save button in Settings window should close the window after saving
  - Current: Settings save silently but window remains open
  - Expected: Window closes automatically after successful save

- [ ] **BUG-02**: Add "Test Credentials" button to validate ntfy instance connection
  - Location: Instance or Token settings page
  - Behavior: Show success (green checkmark) or error indicator
  - Uses existing `test_ntfy_connection` Tauri command

- [ ] **BUG-03**: Settings credentials should load when reopening Settings window
  - Current: Settings fields are empty when window is reopened
  - Expected: Previously saved values appear in all fields

- [ ] **BUG-04**: General and Notification settings pages should show different content
  - Current: Both pages show the same content (poll rate, sounds, etc.)
  - Expected: General = start_hidden, quit_on_close, dev_tools; Notifications = poll rate, sounds, persistence settings

### Out of Scope

- New features beyond bug fixes — this is a stabilization release
- Changes to notification handling or polling logic
- Changes to main window or tray functionality
- macOS or Linux-specific fixes (Windows-focused testing for now)

## Context

**Architecture:**
- Rust backend (`src-tauri/src/`) handles config persistence, ntfy polling, native notifications
- Frontend is external ntfy web UI loaded in Tauri webview
- Settings UI (`src/settings.html`) is a separate window with Tauri IPC

**Settings Pages:**
- `instance` — Instance URL, self-hosted toggle
- `token` — API token or username/password authentication
- `notifications` — Poll rate, datetime format, notification sounds, persistence
- `general` — Start hidden, quit on close, developer tools

**Key Files:**
- `src/settings.html` — Settings UI with page routing via query params
- `src-tauri/src/config.rs` — AppConfig struct and persistence
- `src-tauri/src/main.rs` — Tauri commands including `save_config`, `load_config`, `test_ntfy_connection`

## Constraints

- **Tech stack**: Rust (Tauri 2.0), HTML/CSS/JS frontend, no build step for frontend
- **Platform**: Windows is primary test platform for this release
- **Backward compat**: Settings must work with existing `prefs.json` format

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Fix before release | Old releases are buggy, this version works better | — Pending |

---
*Last updated: 2026-03-04 after initialization*