# Architecture

**Analysis Date:** 2026-03-04

## Pattern Overview

**Overall:** Tauri 2.0 Desktop Application with External Webview UI

**Key Characteristics:**
- **Hybrid architecture**: Rust backend + external web UI (ntfy.sh web app loaded in webview)
- **Notification bridge pattern**: Browser Notification API intercepted and forwarded to native OS notifications
- **Background polling**: Async Rust polling loop for ntfy message retrieval
- **System tray integration**: Close-to-tray behavior with context menu
- **Secure credential storage**: OS keychain integration for API tokens and credentials

## Layers

### Rust Backend Layer
- Purpose: Core application logic, native integrations, polling
- Location: `src-tauri/src/`
- Contains: Business logic, Tauri commands, platform-specific notification handling
- Depends on: Tauri framework, reqwest, tokio, platform-specific APIs
- Used by: Frontend via Tauri IPC commands

### Frontend Layer (External)
- Purpose: User interface rendered from external ntfy web UI
- Location: `src/` (minimal HTML/CSS for loading states)
- Contains: Welcome screen (`welcome.html`), settings UI (`settings.html`), loading page (`index.html`)
- Depends on: Tauri IPC bridge (`window.__TAURI__.core.invoke`)
- Used by: User interaction

### Configuration Layer
- Purpose: Persistent settings and secure credential storage
- Location: `src-tauri/src/config.rs`, `src-tauri/src/credentials.rs`
- Contains: `AppConfig` struct, JSON persistence, OS keychain integration
- Depends on: `dirs` crate for app config directory, `keyring` crate for secure storage
- Used by: All layers for configuration access

## Data Flow

### Application Startup Flow

1. Parse CLI args (`--hidden`, `--devtools`)
2. Initialize Windows notification registry (Windows only)
3. Load config from `prefs.json`, merge with credentials from OS keychain
4. Create main window (webview) with external ntfy URL
5. Inject JavaScript bridge for Notification API interception
6. Create system tray with context menu
7. Create application menu bar (File, View, Settings)
8. Start background polling task
9. Show welcome window if first launch

### Notification Flow

1. ntfy web UI calls `ServiceWorkerRegistration.showNotification()` or `new Notification()`
2. Injected JavaScript bridge intercepts call
3. Bridge invokes Rust `show_notification` command via Tauri IPC
4. Rust `NotificationManager` displays native OS notification
5. Platform-specific handling: Windows Toast via `ntfytoast.exe`, macOS osascript, Linux notify-send

### Configuration Save Flow

1. Frontend calls `save_config` Tauri command with `AppConfig`
2. Rust merges incoming credentials with existing keychain values
3. Credentials saved to OS keychain via `keyring` crate
4. Non-sensitive config saved to `prefs.json` (credentials stripped)
5. Shared runtime config updated via `Arc<Mutex<AppConfig>>`

**State Management:**
- Runtime state: `Arc<Mutex<AppConfig>>` shared between polling loop and commands
- Window state: `Arc<AtomicBool>` for quit flag, `Arc<AtomicU32>` for badge count
- Polling state: `Arc<AtomicBool>` prevents concurrent polling

## Key Abstractions

### NtfyClient
- Purpose: HTTP client for ntfy API communication
- Examples: `src-tauri/src/ntfy.rs:44-169`
- Pattern: Builder pattern with `with_token()` and `with_basic_auth()` methods

### AppConfig
- Purpose: Application configuration with validation
- Examples: `src-tauri/src/config.rs:38-136`
- Pattern: Serde-serialized struct with default values, helper methods for URL/topic parsing

### NotificationManager
- Purpose: Cross-platform native notification handling
- Examples: `src-tauri/src/notifications.rs:6-502`
- Pattern: Platform-specific implementation with fallback (ntfytoast.exe -> notify-rust on Windows)

### Credentials
- Purpose: Secure credential storage abstraction
- Examples: `src-tauri/src/credentials.rs:41-46`
- Pattern: Simple struct with keyring-based load/save functions

## Entry Points

### Main Entry Point
- Location: `src-tauri/src/main.rs:371-724`
- Triggers: Application executable start
- Responsibilities: Window creation, tray setup, menu creation, polling initialization

### Tauri Commands (IPC Entry Points)
- `save_config`: Save configuration and credentials
- `load_config`: Load configuration with merged credentials
- `show_notification`: Display native notification
- `test_ntfy_connection`: Test connection to ntfy server
- `navigate_to`: Navigate webview to URL
- `complete_welcome`: Complete first-run setup
- `get_version`: Return app version
- `get_performance_metrics`: Return system metrics

### Settings Windows
- Location: `src-tauri/src/main.rs:879-902`
- Triggers: Menu events (Settings menu items)
- Responsibilities: Create modal settings windows with query parameter for page

## Error Handling

**Strategy:** Result-based error propagation with `anyhow`

**Patterns:**
- Errors logged to stderr with `eprintln!` or `println!` for debugging
- User-facing errors returned as `Result<T, String>` to frontend
- Configuration errors fall back to defaults gracefully
- Network errors in polling loop logged but do not crash app

**Credential Error Handling:**
- Missing credentials return empty strings (not errors)
- Keychain access failures logged but allow app to continue

## Cross-Cutting Concerns

**Logging:** Console output via `println!` and `eprintln!` (no structured logging framework)

**Validation:**
- Poll rate clamped to 5-3600 seconds
- URL parsing with fallback to default
- Topics parsing with empty string handling

**Authentication:**
- Three modes: No auth, API token, or username/password
- Credentials stored in OS keychain (Windows Credential Manager, macOS Keychain, Linux Secret Service)
- Token used in `Authorization: Bearer` header, user/pass as HTTP Basic Auth

**Platform-Specific Code:**
- Windows: `notify-rust` crate + `ntfytoast.exe` for Toast notifications, registry setup for AppUserModelID
- macOS: `osascript` for notifications
- Linux: `notify-send` for notifications

---

*Architecture analysis: 2026-03-04*