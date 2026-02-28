use ntfy_desktop::config::AppConfig;
use tempfile::TempDir;
use tokio::fs;

#[tokio::test]
async fn test_window_settings_persistence() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("window_settings.json");
    
    // Create config with various window settings
    let original_config = AppConfig {
        start_hidden: true,
        quit_on_close: false,
        hotkeys_enabled: true,
        dev_tools: false,
        welcome_completed: true,
        ..AppConfig::default()
    };
    
    // Save config
    let config_json = serde_json::to_string_pretty(&original_config)
        .expect("Failed to serialize config");
    
    fs::write(&config_path, config_json)
        .await
        .expect("Failed to write config file");
    
    // Load config
    let config_data = fs::read(&config_path)
        .await
        .expect("Failed to read config file");
    
    let config_str = String::from_utf8(config_data)
        .expect("Invalid UTF-8");
    
    let loaded_config: AppConfig = serde_json::from_str(&config_str)
        .expect("Failed to deserialize config");
    
    // Verify window settings persisted correctly
    assert_eq!(original_config.start_hidden, loaded_config.start_hidden);
    assert_eq!(original_config.quit_on_close, loaded_config.quit_on_close);
    assert_eq!(original_config.hotkeys_enabled, loaded_config.hotkeys_enabled);
    assert_eq!(original_config.dev_tools, loaded_config.dev_tools);
    assert_eq!(original_config.welcome_completed, loaded_config.welcome_completed);
}

#[tokio::test]
async fn test_multiple_window_configurations() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Test different window configuration combinations
    let configurations = vec![
        (true, false, true, false),   // Start hidden + hotkeys
        (false, true, false, true),   // Quit on close + dev tools
        (true, true, true, true),     // All enabled
        (false, false, false, false),  // All disabled
    ];
    
    for (i, (start_hidden, quit_on_close, hotkeys_enabled, dev_tools)) in configurations.iter().enumerate() {
        let config_path = temp_dir.path().join(format!("config_{}.json", i));
        
        let config = AppConfig {
            start_hidden: *start_hidden,
            quit_on_close: *quit_on_close,
            hotkeys_enabled: *hotkeys_enabled,
            dev_tools: *dev_tools,
            ..AppConfig::default()
        };
        
        // Save
        let config_json = serde_json::to_string_pretty(&config)
            .expect("Failed to serialize config");
        
        fs::write(&config_path, config_json)
            .await
            .expect("Failed to write config file");
        
        // Load
        let config_data = fs::read(&config_path)
            .await
            .expect("Failed to read config file");
        
        let config_str = String::from_utf8(config_data)
            .expect("Invalid UTF-8");
        
        let loaded_config: AppConfig = serde_json::from_str(&config_str)
            .expect("Failed to deserialize config");
        
        // Verify
        assert_eq!(config.start_hidden, loaded_config.start_hidden);
        assert_eq!(config.quit_on_close, loaded_config.quit_on_close);
        assert_eq!(config.hotkeys_enabled, loaded_config.hotkeys_enabled);
        assert_eq!(config.dev_tools, loaded_config.dev_tools);
    }
}

#[tokio::test]
async fn test_window_configuration_recovery() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("recovery_test.json");
    
    // Create a partial config (simulating corrupted/missing fields)
    let partial_config_json = r#"
    {
        "instance_url": "https://ntfy.sh/app",
        "topics": "test-topic",
        "poll_rate": 60,
        "datetime_format": "YYYY-MM-DD hh:mm a",
        "start_hidden": true,
        "quit_on_close": false
    }
    "#;
    
    fs::write(&config_path, partial_config_json)
        .await
        .expect("Failed to write partial config");
    
    // Load should succeed with default values for missing fields
    let config_data = fs::read(&config_path)
        .await
        .expect("Failed to read config file");
    
    let config_str = String::from_utf8(config_data)
        .expect("Invalid UTF-8");
    
    let loaded_config: AppConfig = serde_json::from_str(&config_str)
        .expect("Failed to deserialize partial config");
    
    // Verify explicit values
    assert_eq!(loaded_config.instance_url, "https://ntfy.sh/app");
    assert_eq!(loaded_config.topics, "test-topic");
    assert_eq!(loaded_config.poll_rate, 60);
    assert_eq!(loaded_config.datetime_format, "YYYY-MM-DD hh:mm a");
    assert!(loaded_config.start_hidden);
    assert!(!loaded_config.quit_on_close);
    
    // Verify default values for missing fields
    assert!(!loaded_config.hotkeys_enabled);
    assert!(!loaded_config.dev_tools);
    assert!(!loaded_config.welcome_completed);
}

#[tokio::test]
async fn test_window_configuration_migration() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("migration_test.json");
    
    // Simulate old config format with deprecated fields
    let old_config_json = r#"
    {
        "instance_url": "https://ntfy.sh/app",
        "topics": "test-topic",
        "poll_rate": 60,
        "datetime_format": "YYYY-MM-DD hh:mm a",
        "persistent_notifications": true,
        "start_hidden": false,
        "quit_on_close": true,
        "hotkeys_enabled": false,
        "dev_tools": true,
        "welcome_completed": false
    }
    "#;
    
    fs::write(&config_path, old_config_json)
        .await
        .expect("Failed to write old config");
    
    // Load should handle deprecated fields gracefully
    let config_data = fs::read(&config_path)
        .await
        .expect("Failed to read config file");
    
    let config_str = String::from_utf8(config_data)
        .expect("Invalid UTF-8");
    
    let loaded_config: AppConfig = serde_json::from_str(&config_str)
        .expect("Failed to deserialize old config");
    
    // Verify values are loaded correctly
    assert_eq!(loaded_config.instance_url, "https://ntfy.sh/app");
    assert_eq!(loaded_config.topics, "test-topic");
    assert_eq!(loaded_config.poll_rate, 60);
    assert_eq!(loaded_config.datetime_format, "YYYY-MM-DD hh:mm a");
    assert!(loaded_config.persistent_notifications); // Deprecated but should load
    assert!(!loaded_config.start_hidden);
    assert!(loaded_config.quit_on_close);
    assert!(!loaded_config.hotkeys_enabled);
    assert!(loaded_config.dev_tools);
    assert!(!loaded_config.welcome_completed);
}

#[tokio::test]
async fn test_window_settings_concurrent_access() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("concurrent_test.json");
    
    // Create initial config
    let initial_config = AppConfig {
        start_hidden: false,
        quit_on_close: false,
        hotkeys_enabled: true,
        dev_tools: false,
        ..AppConfig::default()
    };
    
    let initial_json = serde_json::to_string_pretty(&initial_config)
        .expect("Failed to serialize initial config");
    
    fs::write(&config_path, initial_json)
        .await
        .expect("Failed to write initial config");
    
    // Simulate multiple reads
    let read_tasks: Vec<_> = (0..5)
        .map(|_| {
            let path = config_path.clone();
            tokio::spawn(async move {
                let data = fs::read(&path).await.expect("Failed to read config");
                let config_str = String::from_utf8(data).expect("Invalid UTF-8");
                serde_json::from_str::<AppConfig>(&config_str).expect("Failed to deserialize")
            })
        })
        .collect();
    
    let results: Vec<AppConfig> = futures::future::join_all(read_tasks)
        .await
        .into_iter()
        .map(|result| result.expect("Task failed"))
        .collect();
    
    // All reads should return the same config
    for config in &results {
        assert_eq!(config.start_hidden, initial_config.start_hidden);
        assert_eq!(config.quit_on_close, initial_config.quit_on_close);
        assert_eq!(config.hotkeys_enabled, initial_config.hotkeys_enabled);
        assert_eq!(config.dev_tools, initial_config.dev_tools);
    }
    
    // Update config
    let updated_config = AppConfig {
        start_hidden: true,
        quit_on_close: true,
        hotkeys_enabled: false,
        dev_tools: true,
        ..AppConfig::default()
    };
    
    let updated_json = serde_json::to_string_pretty(&updated_config)
        .expect("Failed to serialize updated config");
    
    fs::write(&config_path, updated_json)
        .await
        .expect("Failed to write updated config");
    
    // Read updated config
    let final_data = fs::read(&config_path)
        .await
        .expect("Failed to read updated config");
    
    let final_str = String::from_utf8(final_data)
        .expect("Invalid UTF-8");
    
    let final_config: AppConfig = serde_json::from_str(&final_str)
        .expect("Failed to deserialize updated config");
    
    // Verify update
    assert!(final_config.start_hidden);
    assert!(final_config.quit_on_close);
    assert!(!final_config.hotkeys_enabled);
    assert!(final_config.dev_tools);
}

#[tokio::test]
async fn test_window_settings_error_handling() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Test with invalid JSON
    let invalid_path = temp_dir.path().join("invalid.json");
    fs::write(&invalid_path, "invalid json content")
        .await
        .expect("Failed to write invalid JSON");
    
    let invalid_data = fs::read(&invalid_path)
        .await
        .expect("Failed to read invalid file");
    
    let invalid_str = String::from_utf8(invalid_data)
        .expect("Invalid UTF-8");
    
    let result: Result<AppConfig, _> = serde_json::from_str(&invalid_str);
    assert!(result.is_err(), "Invalid JSON should fail to parse");
    
    // Test with empty file
    let empty_path = temp_dir.path().join("empty.json");
    fs::write(&empty_path, "")
        .await
        .expect("Failed to write empty file");
    
    let empty_data = fs::read(&empty_path)
        .await
        .expect("Failed to read empty file");
    
    let empty_str = String::from_utf8(empty_data)
        .expect("Invalid UTF-8");
    
    let empty_result: Result<AppConfig, _> = serde_json::from_str(&empty_str);
    assert!(empty_result.is_err(), "Empty file should fail to parse");
    
    // Test with missing file (should use defaults)
    let missing_path = temp_dir.path().join("nonexistent.json");
    assert!(!missing_path.exists(), "File should not exist");
}

#[tokio::test]
async fn test_window_settings_performance() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("performance_test.json");
    
    let config = AppConfig::default();
    
    // Measure serialization time
    let start = std::time::Instant::now();
    let json = serde_json::to_string_pretty(&config)
        .expect("Serialization failed");
    let serialize_time = start.elapsed();
    
    // Measure deserialization time
    let start = std::time::Instant::now();
    let _: AppConfig = serde_json::from_str(&json)
        .expect("Deserialization failed");
    let deserialize_time = start.elapsed();
    
    // Basic performance checks (these are just sanity checks, not strict performance tests)
    assert!(serialize_time < std::time::Duration::from_millis(100),
        "Serialization should be fast: {:?}", serialize_time);
    assert!(deserialize_time < std::time::Duration::from_millis(100),
        "Deserialization should be fast: {:?}", deserialize_time);
    
    // Write performance
    let start = std::time::Instant::now();
    fs::write(&config_path, &json)
        .await
        .expect("Write failed");
    let write_time = start.elapsed();
    
    // Read performance
    let start = std::time::Instant::now();
    let _ = fs::read(&config_path)
        .await
        .expect("Read failed");
    let read_time = start.elapsed();
    
    assert!(write_time < std::time::Duration::from_millis(100),
        "Write should be fast: {:?}", write_time);
    assert!(read_time < std::time::Duration::from_millis(100),
        "Read should be fast: {:?}", read_time);
}