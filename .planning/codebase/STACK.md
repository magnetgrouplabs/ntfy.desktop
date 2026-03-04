# Technology Stack

**Analysis Date:** 2026-03-04

## Languages

**Primary:**
- Rust 1.60+ - Backend logic in `src-tauri/src/`
- JavaScript/HTML - Minimal frontend (loading page only)

**Secondary:**
- Shell scripts (Bash/PowerShell) - Build scripts in `scripts/`

## Runtime

**Environment:**
- Node.js 20 - Build tooling only (Tauri CLI)
- No JavaScript runtime at application level

**Package Manager:**
- npm - Node package management
- Cargo - Rust package management
- Lockfiles: `package-lock.json`, `src-tauri/Cargo.lock`

## Frameworks

**Core:**
- Tauri 2.0 - Desktop application framework (Rust + WebView)
  - Features: `tray-icon`, `image-ico`
  - Plugins: `tauri-plugin-shell`, `tauri-plugin-single-instance`

**Testing:**
- Rust built-in tests (`#[test]`, `#[tokio::test]`)
- Test frameworks: `tempfile`, `tokio-test`, `assert_matches`, `test-case`

**Build/Dev:**
- Cargo - Rust build system
- Tauri CLI (`@tauri-apps/cli ^2.0.0`) - Build orchestration

## Key Dependencies

**Critical:**
- `reqwest 0.13` - HTTP client for ntfy API polling (features: `json`)
- `tokio 1.0.0` - Async runtime (features: `full`)
- `serde 1.0` / `serde_json 1.0` - JSON serialization for config and API
- `anyhow 1.0` - Error handling

**Desktop Integration:**
- `keyring 3` - Secure credential storage (OS keychain)
  - Features: `sync-secret-service`, `windows-native`, `apple-native`
- `notify-rust 4` - Linux notification fallback (Windows)
- `windows 0.62` - Windows API bindings for toast notifications
  - Features: `Data_Xml_Dom`, `UI_Notifications`, `Foundation`, `Win32_*`
- `windows-registry 0.5` - Windows registry for AppUserModelID

**Utilities:**
- `chrono 0.4` - Date/time formatting
- `sha2 0.10` - Icon URL hashing for cache keys
- `image 0.25` - Image processing for notification icons (features: `png`)
- `dirs 5.0` - Cross-platform config/cache directories
- `fs2 0.4` - File system utilities
- `open 5.0` - Open URLs in default browser
- `sysinfo 0.30` - Performance monitoring

## Configuration

**Environment:**
- No `.env` files - Configuration stored in `prefs.json` and OS keychain
- Build configuration: `src-tauri/tauri.conf.json`
- App identifier: `com.anthony.ntfy.desktop`

**Build:**
- `src-tauri/Cargo.toml` - Rust dependencies
- `package.json` - npm scripts for Tauri CLI
- `src-tauri/capabilities/default.json` - Tauri 2 permission grants

**App Config (prefs.json):**
- Location: `{app_config_dir}/prefs.json`
- Credentials stored separately in OS keychain via `keyring` crate
- Schema defined in `src-tauri/src/config.rs:AppConfig`

## Platform Requirements

**Development:**
- Rust stable toolchain
- Node.js 20+
- Platform-specific:
  - **Windows**: Windows SDK, Visual Studio Build Tools
  - **macOS**: Xcode command line tools
  - **Linux**: libwebkit2gtk and dependencies

**Production:**
- Windows-only builds currently (CI/CD releases Windows installers)
- Outputs: NSIS installer, MSI, portable exe

---

*Stack analysis: 2026-03-04*