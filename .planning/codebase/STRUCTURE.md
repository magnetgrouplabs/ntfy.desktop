# Codebase Structure

**Analysis Date:** 2026-03-04

## Directory Layout

```
ntfy.desktop/
├── src/                          # Frontend HTML (minimal - external UI)
│   ├── index.html                # Loading page shown while ntfy loads
│   ├── welcome.html              # First-run setup wizard
│   ├── settings.html             # Settings pages UI
│   └── assets/icon/              # Static assets
├── src-tauri/                    # Rust backend (primary codebase)
│   ├── src/
│   │   ├── main.rs               # Entry point, window creation, commands
│   │   ├── lib.rs                # Module exports
│   │   ├── config.rs             # AppConfig, load/save, validation
│   │   ├── credentials.rs        # OS keychain credential storage
│   │   ├── ntfy.rs               # NtfyClient, polling loop, NDJSON parsing
│   │   ├── notifications.rs      # Native OS notification handling
│   │   └── performance.rs        # System metrics via sysinfo
│   ├── tests/                    # Integration and unit tests
│   │   ├── lib.rs                # Test module root
│   │   ├── unit/                 # Unit tests per module
│   │   └── integration/          # Integration tests
│   ├── icons/                    # App icons for bundling
│   ├── resources/                # Bundled resources (ntfytoast.exe)
│   ├── capabilities/             # Tauri 2 permission grants
│   ├── Cargo.toml                # Rust dependencies
│   ├── tauri.conf.json           # Tauri configuration
│   └── build.rs                  # Build script
├── scripts/                      # Build and release scripts
│   ├── build-windows.ps1         # Windows build script
│   ├── build-linux.sh            # Linux build script
│   ├── build-macos.sh            # macOS build script
│   ├── release.sh                # Release automation
│   ├── bump-version.sh           # Version bumping
│   └── versioning.sh             # Version management
├── .github/workflows/            # GitHub Actions CI/CD
│   ├── ci.yml                    # Continuous integration
│   └── release.yml               # Release workflow
├── performance-tests/           # Performance benchmarking
│   ├── baseline/                 # Baseline metrics
│   ├── comparison/               # Comparison scripts
│   └── scripts/                  # Test automation
├── package.json                  # npm scripts (Tauri CLI only)
├── CLAUDE.md                     # Project documentation
└── .planning/codebase/           # Codebase analysis documents
```

## Directory Purposes

### `src/`
- Purpose: Frontend HTML pages for Tauri webviews
- Contains: Minimal HTML/CSS/JS for welcome, settings, and loading screens
- Key files: `welcome.html` (first-run), `settings.html` (configuration UI), `index.html` (loading state)
- Note: Main UI is external ntfy.sh web app loaded in webview

### `src-tauri/src/`
- Purpose: Core Rust backend
- Contains: All application logic, Tauri commands, platform integrations
- Key files: `main.rs` (entry point), `config.rs` (settings), `ntfy.rs` (polling), `notifications.rs` (platform notifications)

### `src-tauri/tests/`
- Purpose: Automated tests
- Contains: Unit tests for each module, integration tests for app flows
- Structure: `unit/` mirrors `src/` module structure, `integration/` for end-to-end

### `src-tauri/capabilities/`
- Purpose: Tauri 2 security permissions
- Contains: `default.json` defining allowed windows and permissions
- Key: Windows must be listed for IPC access

### `src-tauri/resources/`
- Purpose: Bundled resources for Windows notifications
- Contains: `ntfytoast.exe` (Windows Toast notification helper)
- Note: Bundled into app, extracted at runtime

### `scripts/`
- Purpose: Build automation and release tooling
- Contains: Platform-specific build scripts, version management
- Usage: `npm run build:windows`, `npm run release`

### `.github/workflows/`
- Purpose: CI/CD automation
- Contains: GitHub Actions for testing and releases
- Triggers: Push, PR, tag creation

## Key File Locations

### Entry Points
- `src-tauri/src/main.rs:371-724`: Application entry point
- `src-tauri/src/lib.rs:1-11`: Module exports for library usage
- `package.json:6-22`: npm scripts for development

### Configuration
- `src-tauri/tauri.conf.json`: Tauri app configuration (CSP, bundling, windows)
- `src-tauri/Cargo.toml`: Rust dependencies and build settings
- `src-tauri/capabilities/default.json`: Tauri 2 permission grants
- `src-tauri/src/config.rs:38-89`: AppConfig struct definition

### Core Logic
- `src-tauri/src/ntfy.rs:44-169`: NtfyClient HTTP client
- `src-tauri/src/ntfy.rs:172-346`: Background polling loop
- `src-tauri/src/notifications.rs:34-502`: Platform notification handling
- `src-tauri/src/credentials.rs:49-84`: Keychain credential load/save

### Frontend Pages
- `src/index.html`: Loading screen HTML
- `src/welcome.html:232-359`: Welcome wizard JavaScript
- `src/settings.html`: Settings page UI

### Testing
- `src-tauri/tests/unit/config_tests.rs`: Config unit tests
- `src-tauri/tests/integration/app_flow_tests.rs`: App flow tests
- `src-tauri/src/config.rs:192-506`: Inline unit tests for config

## Naming Conventions

### Files
- Rust modules: snake_case (e.g., `config.rs`, `ntfy.rs`, `notifications.rs`)
- Test files: `{module}_tests.rs` (e.g., `config_tests.rs`)
- HTML files: lowercase (e.g., `welcome.html`, `settings.html`)

### Rust Code
- Structs: PascalCase (e.g., `AppConfig`, `NtfyClient`, `NotificationManager`)
- Functions: snake_case (e.g., `load_config`, `save_config`, `start_polling`)
- Constants: SCREAMING_SNAKE_CASE (e.g., `SERVICE_NAME`, `KEY_API_TOKEN`)
- Modules: snake_case (e.g., `mod config`, `mod ntfy`)

### Tauri Commands
- snake_case command names (e.g., `save_config`, `load_config`, `show_notification`)
- snake_case rename for multi-word params (e.g., `#[tauri::command(rename_all = "snake_case")]`)

## Where to Add New Code

### New Tauri Command
1. Add function with `#[tauri::command]` attribute in `src-tauri/src/main.rs`
2. Register in `invoke_handler!` macro at `main.rs:425-438`
3. Add frontend JS call: `invoke('command_name', { params })`

### New Configuration Field
1. Add field to `AppConfig` struct in `src-tauri/src/config.rs:38-64`
2. Add default value in `Default` implementation
3. Add serialization test in `src-tauri/src/config.rs:192-506` or `tests/unit/config_tests.rs`
4. Update frontend to use new field

### New Settings Page
1. Add menu item in `create_app_menu()` at `src-tauri/src/main.rs:785-876`
2. Add handler in `handle_menu_event()` at `src-tauri/src/main.rs:904-967`
3. Create HTML page in `src/` or add section to `settings.html`

### New Notification Sound
1. Add variant to `NotificationSound` enum in `src-tauri/src/config.rs:21-36`
2. Add sound mapping in `src-tauri/src/notifications.rs:126-150`
3. Update frontend UI to include new option

### New Platform Support
1. Add platform-specific dependencies in `Cargo.toml` with `#[cfg(target_os = "...")]`
2. Implement notification handling in `src-tauri/src/notifications.rs`
3. Add platform-specific initialization in `main.rs` (similar to Windows registry setup)

### New Test
- Unit tests: Add to `src-tauri/tests/unit/` or inline in module files
- Integration tests: Add to `src-tauri/tests/integration/`

## Special Directories

### `.planning/`
- Purpose: Planning documents and codebase analysis
- Generated: By `/gsd:map-codebase` command
- Committed: Yes (part of repository)

### `src-tauri/target/`
- Purpose: Build artifacts (Rust compilation output)
- Generated: Yes (by `cargo build`)
- Committed: No (in `.gitignore`)

### `node_modules/`
- Purpose: npm dependencies (Tauri CLI only)
- Generated: Yes (by `npm install`)
- Committed: No (in `.gitignore`)

### `dist/`
- Purpose: Build output for frontend assets
- Generated: Yes (by Tauri build process)
- Note: This project uses `src/` directly as `frontendDist`, so `dist/` may be empty or legacy

### `performance-tests/`
- Purpose: Performance comparison with original Electron app
- Contains: Benchmark scripts and baseline data
- Usage: `npm run test:performance`

---

*Structure analysis: 2026-03-04*