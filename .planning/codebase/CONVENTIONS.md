# Coding Conventions

**Analysis Date:** 2026-03-04

## Naming Patterns

**Files:**
- Rust source files use `snake_case`: `config.rs`, `ntfy.rs`, `notifications.rs`
- Test files use `snake_case` with `_tests.rs` suffix: `config_tests.rs`, `notification_tests.rs`
- Module organization files use `mod.rs` in subdirectories: `tests/unit/mod.rs`, `tests/integration/mod.rs`
- Frontend HTML files use `snake_case`: `index.html`, `settings.html`, `welcome.html`

**Functions:**
- Rust functions use `snake_case`: `save_config`, `load_config_sync`, `show_notification`
- Tauri commands use `snake_case` with `#[tauri::command]` attribute
- Async functions follow the same `snake_case` pattern: `start_polling`, `test_ntfy_connection`

**Variables:**
- Local variables use `snake_case`: `config_path`, `poll_rate`, `app_handle`
- Constants use `SCREAMING_SNAKE_CASE`: `SERVICE_NAME`, `KEY_API_TOKEN`, `KEY_AUTH_USER`

**Types:**
- Structs use `PascalCase`: `AppConfig`, `NtfyMessage`, `NotificationManager`, `PerformanceMetrics`
- Enums use `PascalCase` for enum name and variants: `NotificationSound::Default`, `PersistentNotificationMode::UrgentOnly`
- Traits use `PascalCase`: Standard Rust convention

## Code Style

**Formatting:**
- No explicit rustfmt.toml or .rustfmt.toml configuration - uses Rust defaults
- Standard Rust formatting with 4-space indentation
- Maximum line length follows Rust default (100 chars)
- Use `cargo fmt` to format code

**Linting:**
- No explicit clippy.toml or .clippy.toml configuration
- Use `cargo clippy` for linting
- Standard Rust linting rules apply

**Rust Edition:**
- Uses Rust 2021 edition (specified in `Cargo.toml`)
- Minimum Rust version: 1.60

## Import Organization

**Order:**
1. Standard library imports (`use std::...`)
2. External crate imports (`use anyhow::...`, `use serde::...`)
3. Tauri imports (`use tauri::...`)
4. Local module imports (`use super::...`, `use crate::...`)

**Example from `main.rs`:**
```rust
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

use notifications::NotificationManager;
use ntfy::NtfyClient;

use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Listener, Manager, RunEvent, WebviewUrl, WindowEvent,
};
```

**Path Aliases:**
- `super::` for parent module imports
- `crate::` for absolute imports from crate root
- No custom path aliases configured

## Error Handling

**Patterns:**
- Use `anyhow::Result<T>` for fallible operations
- Use `anyhow::anyhow!()` macro for error creation: `anyhow::anyhow!("Failed to parse config: {}", e)`
- Propagate errors with `?` operator
- Log errors to stderr with `eprintln!` for debugging
- Print informational messages with `println!`

**Error Examples:**
```rust
// From config.rs
pub async fn save_config(app_handle: &AppHandle, config: AppConfig) -> Result<()> {
    let app_dir = app_handle
        .path()
        .app_config_dir()
        .map_err(|e| anyhow::anyhow!("Could not get app config directory: {}", e))?;
    // ...
}
```

**Tauri Commands:**
- Return `Result<T, String>` for Tauri command functions
- Convert errors to strings with `.map_err(|e| e.to_string())`

## Logging

**Framework:** Standard `println!` and `eprintln!` macros (no external logging crate)

**Patterns:**
- Use `println!` for informational/debug messages
- Use `eprintln!` for warnings and errors
- Prefix debug output with `DEBUG:` for filtering
- Use `format!("...")` for complex string construction

**Example:**
```rust
println!("Config saved successfully to: {}", config_path.display());
eprintln!("Failed to load credentials from keychain: {}", e);
println!("DEBUG: Using ntfytoast.exe at: {}", exe_path);
```

## Comments

**When to Comment:**
- Document public APIs with `///` doc comments
- Use `// ── Section Name ─────────────` style for section headers
- Inline comments for complex logic or platform-specific code
- TODO-style comments for future work

**JSDoc/TSDoc:**
- Not used in this project (no TypeScript/JavaScript build step)

**Rust Doc Comments:**
```rust
/// Persistent notification mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PersistentNotificationMode {
    Off,
    All,
    UrgentOnly,
}

/// Get the base URL for API calls (strips /app suffix if present)
pub fn api_base_url(&self) -> String {
    self.instance_url
        .trim_end_matches('/')
        .trim_end_matches("/app")
        .to_string()
}
```

## Function Design

**Size:** Functions range from small helpers to moderately sized (up to ~100 lines). Complex functions like `main()` are larger but organized into clear sections.

**Parameters:**
- Prefer owned types for parameters that will be stored
- Use `&str` for string parameters that are read-only
- Use `Option<T>` for optional parameters
- Use `async fn` for async operations

**Return Values:**
- Return `Result<T>` for fallible operations
- Use `anyhow::Result<T>` as the standard result type
- Tauri commands return `Result<T, String>` for IPC compatibility

## Module Design

**Exports:**
- Use `pub mod` to expose submodules
- Re-export key types from `lib.rs`: `pub use config::{AppConfig, NotificationSound};`
- Keep internal implementation details private

**Barrel Files:**
- `lib.rs` re-exports public API items
- Test modules use `mod.rs` for organization

**Example lib.rs:**
```rust
pub mod config;
pub mod credentials;
pub mod notifications;
pub mod ntfy;
pub mod performance;

pub use config::{AppConfig, NotificationSound, PersistentNotificationMode};
pub use notifications::NotificationManager;
pub use ntfy::NtfyClient;
pub use performance::{PerformanceMetrics, PerformanceMonitor};
```

## Frontend Conventions

**HTML/CSS:**
- CSS variables for theming: `--primary-color`, `--bg-primary`, `--text-primary`
- Dark/light mode via `@media (prefers-color-scheme: dark/light)`
- Inline styles within HTML files (no separate CSS files)
- Semantic HTML structure with BEM-like class naming

**JavaScript:**
- Use `const` for constants, `let` for variables
- Async/await pattern for Tauri IPC: `await invoke("command_name", { args })`
- Access Tauri API via `window.__TAURI__.core.invoke`
- Error handling with try/catch around async operations

**Example:**
```javascript
async function saveSettings() {
  try {
    await invoke("save_config", { config: currentConfig });
    await closeThisWindow();
  } catch (e) {
    console.error("Failed to save config:", e);
    alert("Failed to save: " + e);
  }
}
```

## Platform-Specific Code

**Pattern:**
Use `#[cfg(target_os = "...")]` attributes for platform-specific implementations:

```rust
#[cfg(target_os = "windows")]
fn init_windows_notification_registry() -> anyhow::Result<()> {
    // Windows-specific code
}

#[cfg(target_os = "macos")]
{
    use std::process::Command;
    let _ = Command::new("osascript")
        .arg("-e")
        .arg(format!("display notification \"{}\" with title \"{}\"", msg_escaped, title_escaped))
        .output();
}
```

## Security Considerations

**Credential Handling:**
- API tokens and passwords stored in OS keychain (not in config file)
- Config file `prefs.json` stores non-sensitive settings only
- Credentials module uses `keyring` crate for secure storage

**Example:**
```rust
// Credentials are stripped before saving to disk
let mut disk_config = config.clone();
disk_config.api_token = String::new();
disk_config.auth_user = String::new();
disk_config.auth_pass = String::new();
```

---

*Convention analysis: 2026-03-04*