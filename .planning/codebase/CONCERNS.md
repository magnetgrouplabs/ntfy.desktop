# Codebase Concerns

**Analysis Date:** 2026-03-04

## Tech Debt

### Large main.rs File
- Issue: `src-tauri/src/main.rs` is 976 lines with multiple responsibilities
- Files: `src-tauri/src/main.rs`
- Impact: Difficult to navigate, test, and maintain; mixes Tauri commands, window management, menu handling, polling setup
- Fix approach: Extract into modules:
  - `commands.rs` for Tauri command handlers (save_config, load_config, show_notification, etc.)
  - `menu.rs` for menu creation and event handling
  - `tray.rs` for system tray setup
  - `window.rs` for window creation and lifecycle

### Deprecated Field Migration
- Issue: `persistent_notifications` field kept in `AppConfig` solely for backward compatibility
- Files: `src-tauri/src/config.rs:51-53`
- Impact: Confusion for developers, unused field in config struct
- Fix approach: Remove deprecated field after confirming no legacy prefs.json files need migration

### Inline JavaScript in Rust
- Issue: Large initialization script (80+ lines) embedded as raw string in `main.rs:482-558`
- Files: `src-tauri/src/main.rs:472-559`
- Impact: No syntax highlighting, difficult to debug, hard to modify
- Fix approach: Move to separate `.js` file loaded at build time with `include_str!`

### Hardcoded Development Paths
- Issue: Absolute development paths embedded for local testing
- Files: `src-tauri/src/notifications.rs:281`, `src-tauri/src/notifications.rs:311-312`
- Impact: Will fail silently on other machines
- Fix approach: Remove hardcoded paths or wrap in `#[cfg(debug_assertions)]`

## Known Bugs

### ntfytoast.exe Fallback Path Detection
- Symptoms: Notifications may silently fail on some systems
- Files: `src-tauri/src/notifications.rs:268-291`
- Trigger: When ntfytoast.exe is not found in expected locations
- Workaround: Falls back to notify-rust which has different capabilities

### Polling Race Condition
- Symptoms: Occasional duplicate notifications
- Files: `src-tauri/src/ntfy.rs:187-190`
- Trigger: When polling completes quickly and swap returns true
- Cause: `is_polling` flag check at line 187 allows concurrent polling to skip

## Security Considerations

### Credential Logging
- Risk: Log messages reveal credential existence and length
- Files: `src-tauri/src/credentials.rs:50-60`, `src-tauri/src/main.rs:764-774`
- Current mitigation: Logs presence/length, not values
- Recommendations: Consider removing credential length from logs in production

### HTTP Client Security
- Risk: No TLS certificate pinning for ntfy API connections
- Files: `src-tauri/src/ntfy.rs:54-60`
- Current mitigation: Uses reqwest with default TLS
- Recommendations: Consider certificate pinning for self-hosted instances

### Icon Download from Remote URLs
- Risk: Downloads icons from arbitrary URLs for notifications
- Files: `src-tauri/src/notifications.rs:327-454`
- Current mitigation: 200KB size check, image format validation
- Recommendations: Consider URL validation, domain allowlisting for icon sources

### Windows Registry Modifications
- Risk: App writes to Windows registry for notification setup
- Files: `src-tauri/src/main.rs:13-70`
- Current mitigation: Error handling for failures
- Recommendations: None - standard practice for Windows apps

## Performance Bottlenecks

### HashSet Memory Growth
- Problem: `seen_ids` HashSet grows unbounded during polling session
- Files: `src-tauri/src/ntfy.rs:179`
- Cause: Message IDs accumulated for deduplication
- Improvement path: Current cleanup every hour (line 182-183, 334-337) helps but could grow large on high-traffic topics
- Recommendation: Consider LRU cache or max size limit

### Image Processing Blocking
- Problem: Image resize uses `spawn_blocking` but could block tokio runtime on slow machines
- Files: `src-tauri/src/notifications.rs:461-485`
- Cause: Lanczos3 resize is CPU-intensive
- Improvement path: Consider async image crate or offloading to thread pool

### Synchronous Config Loading in Setup
- Problem: `load_config_sync` blocks the main thread during app startup
- Files: `src-tauri/src/main.rs:727-782`
- Cause: Tauri setup runs synchronously
- Improvement path: Acceptable for current config size, but could cause delays if config grows

## Fragile Areas

### Platform-Specific Notification Code
- Files: `src-tauri/src/notifications.rs:59-87`
- Why fragile: Three different notification implementations (Windows, macOS, Linux)
- Safe modification: Test all platforms after any notification changes
- Test coverage: None for macOS and Linux notification paths

### Single Instance Window Focus
- Files: `src-tauri/src/main.rs:399-424`
- Why fragile: Uses thread spawn with 100ms sleep to wait for window creation
- Safe modification: The sleep is a race condition workaround
- Test coverage: Manual testing required

### Tauri Window Event Handlers
- Files: `src-tauri/src/main.rs:588-596`, `src-tauri/src/main.rs:604-611`
- Why fragile: Closure captures and event timing depend on Tauri internals
- Safe modification: Changes to window lifecycle need testing across platforms
- Test coverage: Integration tests exist but minimal

### Icon Path Detection
- Files: `src-tauri/src/notifications.rs:294-322`
- Why fragile: Multiple fallback paths with different logic for development vs production
- Safe modification: Test both dev and release builds after changes
- Test coverage: No automated tests

## Scaling Limits

### seen_ids Memory Usage
- Current capacity: Unbounded (hourly cleanup)
- Limit: Long-running sessions with high message volume
- Scaling path: Implement bounded cache or TTL-based cleanup

### Icon Cache Directory
- Current capacity: Unlimited cache size
- Files: `src-tauri/src/notifications.rs:329-332`
- Limit: No max cache size enforcement
- Scaling path: Add periodic cleanup or size limit for icon cache

### Concurrent Polling Lock
- Current capacity: Single polling loop
- Files: `src-tauri/src/ntfy.rs:186-190`
- Limit: Cannot parallelize topic subscriptions
- Scaling path: Consider per-topic polling for large topic counts

## Dependencies at Risk

### keyring Crate
- Risk: Platform-specific behavior for credential storage
- Files: `src-tauri/src/credentials.rs`
- Impact: Keychain access failures on some Linux desktop environments
- Migration plan: Consider fallback to config file encryption if keyring fails

### notify-rust Crate
- Risk: Limited Windows support, fallback for ntfytoast
- Files: `src-tauri/src/notifications.rs:213-264`
- Impact: Notifications may not work correctly on all platforms
- Migration plan: Windows uses ntfytoast.exe; notify-rust is fallback only

### reqwest Crate
- Risk: Default TLS implementation may not work in restricted environments
- Files: `src-tauri/src/ntfy.rs:54-60`
- Impact: Connection failures in corporate networks with TLS proxies
- Migration plan: Consider native-tls feature flag for Windows

## Missing Critical Features

### Error Recovery for Polling
- Problem: Polling errors are logged but don't trigger reconnection logic
- Files: `src-tauri/src/ntfy.rs:326-328`
- Blocks: Automatic recovery from network issues

### Settings UI Validation
- Problem: No frontend validation for settings inputs
- Files: `src/settings.html`
- Blocks: Users can enter invalid URLs/topic names

### Connection Status Indicator
- Problem: No visual indication when polling fails
- Blocks: Users unaware of connectivity issues

## Test Coverage Gaps

### Untested ntfy.rs Polling Logic
- What's not tested: HTTP requests, NDJSON parsing, message deduplication, error handling
- Files: `src-tauri/src/ntfy.rs`
- Risk: Network failures could cause silent polling failures
- Priority: High - Core functionality

### Untested notifications.rs Platform Code
- What's not tested: Windows toast, macOS osascript, Linux notify-send
- Files: `src-tauri/src/notifications.rs`
- Risk: Platform-specific regressions go undetected
- Priority: Medium - Requires platform-specific test infrastructure

### Untested main.rs Tauri Commands
- What's not tested: save_config, load_config, show_notification, navigate_to
- Files: `src-tauri/src/main.rs:91-366`
- Risk: Command handler bugs affect frontend-backend communication
- Priority: Medium

### Untested credentials.rs Keyring Operations
- What's not tested: Load, save, delete operations with OS keychain
- Files: `src-tauri/src/credentials.rs`
- Risk: Keychain failures on different platforms
- Priority: Medium - Requires OS environment

### Untested Error Paths
- What's not tested: Config file corruption, network timeouts, keychain failures
- Files: Multiple
- Risk: Edge cases cause crashes instead of graceful degradation
- Priority: Medium

---

*Concerns audit: 2026-03-04*