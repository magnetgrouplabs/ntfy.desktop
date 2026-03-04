# Testing Patterns

**Analysis Date:** 2026-03-04

## Test Framework

**Runner:**
- Rust built-in test framework via `cargo test`
- No external test runner configuration needed
- Uses `#[test]` and `#[tokio::test]` attributes

**Assertion Library:**
- Standard `assert_eq!`, `assert!`, `assert_ne!` macros
- `assert_matches` crate for pattern matching assertions: `assert_matches!(manager, NotificationManager)`

**Run Commands:**
```bash
cargo test                          # Run all tests
cargo test --lib                    # Run unit tests only
cargo test --test lib               # Run integration tests
cargo test config_tests             # Run specific test file
cargo test -- --nocapture           # Show println output
```

## Test File Organization

**Location:**
- Unit tests: Inline in source files under `#[cfg(test)] mod tests { }` blocks
- Integration tests: `src-tauri/tests/` directory
- Test modules: `src-tauri/tests/lib.rs` as entry point

**Naming:**
- Test files use `_tests.rs` suffix: `config_tests.rs`, `notification_tests.rs`
- Test functions use `test_` prefix: `test_default_config`, `test_config_serialization`

**Structure:**
```
src-tauri/
├── src/
│   ├── config.rs          # Contains inline #[cfg(test)] module
│   ├── notifications.rs   # Contains inline #[cfg(test)] module
│   └── performance.rs     # Contains inline #[cfg(test)] module
└── tests/
    ├── lib.rs             # Test module entry point
    ├── unit/
    │   ├── mod.rs
    │   ├── config_tests.rs
    │   ├── notification_tests.rs
    │   └── window_tests.rs
    └── integration/
        ├── mod.rs
        └── app_flow_tests.rs
```

## Test Structure

**Suite Organization:**
```rust
#[test]
fn test_config_default_values() {
    let config = AppConfig::default();

    assert_eq!(config.instance_url, "https://ntfy.sh/app");
    assert_eq!(config.api_token, "");
    assert_eq!(config.topics, "announcements,stats");
    assert_eq!(config.poll_rate, 60);
    assert!(!config.persistent_notifications);
    assert_eq!(config.persistent_notifications_mode, PersistentNotificationMode::Off);
}
```

**Patterns:**
- Test default values and initialization
- Test serialization/deserialization round-trips
- Test edge cases and boundary conditions
- Test error handling paths
- Group related tests in the same file by functionality

**Async Testing:**
```rust
#[tokio::test]
async fn test_file_operations() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("prefs.json");

    let config = AppConfig {
        instance_url: "https://test.example.com".to_string(),
        topics: "test-topic".to_string(),
        poll_rate: 120,
        ..Default::default()
    };

    let config_json = serde_json::to_string_pretty(&config)
        .expect("Failed to serialize config");
    fs::write(&config_path, config_json).await
        .expect("Failed to write config file");

    let config_data = fs::read(&config_path).await
        .expect("Failed to read config file");
    let loaded_config: AppConfig = serde_json::from_str(&config_str)
        .expect("Failed to deserialize config");

    assert_eq!(config.instance_url, loaded_config.instance_url);
}
```

## Mocking

**Framework:** No mocking framework used. Tests use real implementations with temporary resources.

**Patterns:**
- Use `tempfile::TempDir` for temporary file system operations
- Use real struct instances for testing (no mocks)
- Test in-memory operations when possible

**What to Mock:**
- Nothing currently mocked in this codebase
- Platform-specific operations are tested via actual execution

**What NOT to Mock:**
- Config serialization/deserialization - use real serde
- File operations - use temporary directories
- Struct initialization - use real instances

## Fixtures and Factories

**Test Data:**
```rust
// Create test instances inline
let config = AppConfig {
    instance_url: "https://test.example.com".to_string(),
    topics: "test-topic".to_string(),
    poll_rate: 120,
    datetime_format: "YYYY-MM-DD HH:mm".to_string(),
    persistent_notifications: true,
    ..Default::default()
};

// Use Default trait for minimal test config
let config = AppConfig::default();
```

**Location:**
- Test data created inline within test functions
- No separate fixture files or factory modules
- Use `..Default::default()` to fill unspecified fields

## Coverage

**Requirements:** No enforced coverage targets. Tests focus on core functionality.

**View Coverage:**
```bash
cargo tarpaulin --out Html    # Generate HTML coverage report (if installed)
```

**Coverage Areas:**
- Config serialization/deserialization
- URL parsing and normalization
- Topics parsing (comma-separated)
- Poll rate clamping (5-3600 range)
- Notification persistence logic
- Sound selection logic

## Test Types

**Unit Tests:**
- Located inline in source files under `#[cfg(test)]` modules
- Test individual functions, methods, and struct behaviors
- No external dependencies required
- Fast execution

**Integration Tests:**
- Located in `src-tauri/tests/` directory
- Test end-to-end flows and module interactions
- May use temporary files/directories
- Test config save/load workflows

**E2E Tests:**
- Not used in this project
- No automated UI testing
- Manual testing required for Tauri window operations

## Common Patterns

**Testing Default Values:**
```rust
#[test]
fn test_config_default_values() {
    let config = AppConfig::default();

    assert_eq!(config.instance_url, "https://ntfy.sh/app");
    assert_eq!(config.api_token, "");
    assert_eq!(config.topics, "announcements,stats");
    assert_eq!(config.poll_rate, 60);
}
```

**Testing Edge Cases:**
```rust
#[test]
fn test_poll_rate_clamping() {
    let mut config = AppConfig::default();

    // Below minimum
    config.poll_rate = 3;
    assert_eq!(config.effective_poll_rate(), 5);

    // Above maximum
    config.poll_rate = 5000;
    assert_eq!(config.effective_poll_rate(), 3600);

    // Valid range
    config.poll_rate = 60;
    assert_eq!(config.effective_poll_rate(), 60);
}
```

**Testing Serialization:**
```rust
#[test]
fn test_config_serialization() {
    let config = AppConfig {
        instance_url: "https://example.com/app".to_string(),
        topics: "topic1,topic2".to_string(),
        ..Default::default()
    };

    let serialized = serde_json::to_string(&config).expect("Failed to serialize");
    let deserialized: AppConfig = serde_json::from_str(&serialized).expect("Failed to deserialize");

    assert_eq!(config.instance_url, deserialized.instance_url);
    assert_eq!(config.topics, deserialized.topics);
}
```

**Testing Enum Variants:**
```rust
#[test]
fn test_notification_sound_variants() {
    let sounds = vec![
        NotificationSound::Default,
        NotificationSound::None,
        NotificationSound::Alert,
        NotificationSound::Bell,
        NotificationSound::Chime,
        NotificationSound::Pop,
    ];

    for sound in sounds {
        let notification = NotificationData {
            sound: sound.clone(),
            ..Default::default()
        };
        assert_eq!(notification.sound, sound);
    }
}
```

**Testing Async File Operations:**
```rust
#[tokio::test]
async fn test_config_save_load_flow() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("flow_test.json");

    let original_config = AppConfig {
        instance_url: "https://my-ntfy.example.com/app".to_string(),
        topics: "alerts,updates".to_string(),
        poll_rate: 30,
        ..AppConfig::default()
    };

    // Save
    let config_json = serde_json::to_string_pretty(&original_config).expect("...");
    fs::write(&config_path, config_json).await.expect("...");

    // Load
    let config_data = fs::read(&config_path).await.expect("...");
    let loaded_config: AppConfig = serde_json::from_str(&config_str).expect("...");

    // Verify
    assert_eq!(original_config.instance_url, loaded_config.instance_url);
}
```

**Testing Error Handling:**
```rust
#[tokio::test]
async fn test_error_recovery_flow() {
    let temp_dir = TempDir::new().expect("...");

    // Test invalid JSON
    let invalid_path = temp_dir.path().join("invalid.json");
    fs::write(&invalid_path, "{invalid json}").await.expect("...");

    let invalid_data = fs::read(&invalid_path).await.expect("...");
    let invalid_str = String::from_utf8(invalid_data).expect("...");
    let invalid_result: Result<AppConfig, _> = serde_json::from_str(&invalid_str);
    assert!(invalid_result.is_err(), "Invalid JSON should fail");
}
```

## Test Dependencies

**Cargo.toml dev-dependencies:**
```toml
[dev-dependencies]
tempfile = "3.8"
tokio-test = "0.4"
assert_matches = "1.5"
test-case = "3.3"
futures = "0.3"
```

## Test Organization Best Practices

1. **Group tests by functionality** - config tests in `config_tests.rs`, notification tests in `notification_tests.rs`
2. **Use descriptive names** - `test_poll_rate_clamping`, `test_config_serialization_roundtrip`
3. **Test both success and failure paths** - valid inputs and invalid inputs
4. **Use temporary directories** - `TempDir::new()` for file operations
5. **Clean separation** - unit tests inline, integration tests in separate directory
6. **Async tests use tokio** - `#[tokio::test]` attribute for async functions

---

*Testing analysis: 2026-03-04*