# External Integrations

**Analysis Date:** 2026-03-04

## APIs & External Services

**ntfy.sh API:**
- Purpose: Push notification service - app polls for messages
- SDK/Client: Custom `NtfyClient` in `src-tauri/src/ntfy.rs`
- Connection: Configurable `instance_url` (default: `https://ntfy.sh/app`)
- Authentication:
  - Bearer token authentication via `Authorization: Bearer {token}`
  - Basic auth via `auth_user`/`auth_pass`
- Polling: `GET {base_url}/{topics}/json?since={poll_rate}s&poll=1`
- Response: NDJSON (newline-delimited JSON) format
- Deduplication: Message IDs tracked in memory, cleared hourly

**ntfy Web UI:**
- Purpose: Primary user interface (loaded in WebView)
- URL: `https://ntfy.sh/app` or self-hosted instance
- Integration: Direct WebView load via `WebviewUrl::External`
- CSP configured to allow external content from ntfy instances

## Data Storage

**Databases:**
- None - No database dependencies

**File Storage:**
- Local filesystem only
- Config: `{app_config_dir}/prefs.json` (JSON serialization)
- Icon cache: `{config_dir}/com.anthony.ntfy.desktop/icons/` (downloaded notification icons, 7-day TTL)
- Icon size limit: 128x128 PNG, <= 200KB

**Caching:**
- Icon cache for remote notification icons
- SHA256 hash of URL used as cache filename
- 7-day TTL before re-download

## Authentication & Identity

**Auth Provider:**
- ntfy.sh authentication (API tokens or basic auth)
- Implementation: Credentials stored in OS keychain

**Credential Storage:**
- Service: OS-native keychain
  - Windows: Windows Credential Manager
  - macOS: Keychain Services
  - Linux: Secret Service API (GNOME Keyring/KWallet)
- Library: `keyring` crate v3
- Keys stored:
  - `api_token` - Bearer token for ntfy
  - `auth_user` - Basic auth username
  - `auth_pass` - Basic auth password
- Service name: `ntfy-desktop`

## Monitoring & Observability

**Error Tracking:**
- None - Console output only

**Logs:**
- Console printing via `println!` and `eprintln!`
- Debug output for notification flow, icon caching, credential operations

**Performance Monitoring:**
- Built-in via `sysinfo` crate
- Exposes `get_memory_usage` and `get_performance_metrics` commands
- Metrics: memory usage, CPU usage, system uptime

## CI/CD & Deployment

**Hosting:**
- GitHub Releases - Distributable download
- No server-side hosting (desktop app)

**CI Pipeline:**
- GitHub Actions (`.github/workflows/`)
  - `ci.yml` - Build & test on push/PR to master
  - `release.yml` - Build and publish on version tags
- Security: `cargo audit` and `npm audit` in CI

**Build Outputs:**
- Windows NSIS installer: `ntfy.desktop_*_x64-setup.exe`
- Windows MSI: `ntfy.desktop_*_x64_en-US.msi`
- Portable exe: `ntfy.desktop_*_portable.exe`

## Environment Configuration

**Required env vars:**
- None at runtime
- Build-time: Standard Rust/Node toolchain env vars

**Secrets location:**
- User credentials: OS keychain (not in files)
- No server-side secrets

## Webhooks & Callbacks

**Incoming:**
- None - Polling only, no webhook server

**Outgoing:**
- HTTP GET to ntfy instance for polling
- HTTP GET for downloading notification icons

## Notification Bridge Architecture

**Web UI to Native Notifications:**
- Bridge: JavaScript injection in `main.rs` initialization script
- Intercepts:
  - `window.Notification` constructor
  - `ServiceWorkerRegistration.showNotification()`
  - `ServiceWorkerRegistration.getNotifications()`
- Forwarding: Tauri IPC `invoke('show_notification', args)`
- Native handlers: Platform-specific notification APIs
  - Windows: `ntfytoast.exe` (BurntToast wrapper) or `notify-rust` fallback
  - macOS: `osascript` for `display notification`
  - Linux: `notify-send` command

## External Tool Dependencies

**Windows:**
- `ntfytoast.exe` - BurntToast-based PowerShell toast tool
  - Located: `{exe_dir}/ntfytoast.exe` or `{exe_dir}/resources/ntfytoast.exe`
  - Args: `-t` (title), `-m` (message), `-p` (icon PNG), `-s` (sound), `-appID`, `-persistent`, `-d` (duration), `-silent`

**macOS:**
- `osascript` - AppleScript for notifications

**Linux:**
- `notify-send` - libnotify CLI

---

*Integration audit: 2026-03-04*