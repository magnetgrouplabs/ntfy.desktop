use ntfy_desktop::config::AppConfig;
use ntfy_desktop::notifications::NotificationManager;
use tempfile::TempDir;
use tokio::fs;

#[test]
fn test_app_config_default_flow() {
    // Test that default config provides sensible values
    let config = AppConfig::default();
    
    // Verify default instance URL
    assert_eq!(config.instance_url, "https://ntfy.sh/app");
    assert_eq!(config.api_base_url(), "https://ntfy.sh");
    
    // Verify default topics
    assert_eq!(config.topics, "announcements,stats");
    assert_eq!(config.topics_list(), vec!["announcements", "stats"]);
    assert_eq!(config.topics_path(), "announcements,stats");
    
    // Verify default poll rate
    assert_eq!(config.poll_rate, 60);
    assert_eq!(config.effective_poll_rate(), 60);
    
    // Verify default window behavior
    assert!(!config.start_hidden);
    assert!(!config.quit_on_close);
    assert!(!config.hotkeys_enabled);
    assert!(!config.dev_tools);
    
    // Verify notification defaults
    assert_eq!(config.persistent_notifications_mode, ntfy_desktop::config::PersistentNotificationMode::Off);
    assert_eq!(config.notification_sound, ntfy_desktop::config::NotificationSound::Default);
    assert_eq!(config.urgent_notification_sound, ntfy_desktop::config::NotificationSound::Default);
}

#[test]
fn test_notification_manager_flow() {
    // Test notification manager initialization and basic operations
    let manager = NotificationManager::new();
    
    // Manager should be created successfully
    assert!(matches!(manager, NotificationManager));
    
    // Default manager should also work
    let default_manager = NotificationManager::default();
    assert!(matches!(default_manager, NotificationManager));
}

#[tokio::test]
async fn test_config_save_load_flow() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("flow_test.json");
    
    // Create a comprehensive config
    let original_config = AppConfig {
        instance_url: "https://my-ntfy.example.com/app".to_string(),
        topics: "alerts,updates,monitoring".to_string(),
        poll_rate: 30,
        datetime_format: "MM/DD/YYYY HH:mm".to_string(),
        start_hidden: true,
        quit_on_close: false,
        hotkeys_enabled: true,
        dev_tools: false,
        welcome_completed: true,
        persistent_notifications_mode: ntfy_desktop::config::PersistentNotificationMode::UrgentOnly,
        notification_sound: ntfy_desktop::config::NotificationSound::Chime,
        urgent_notification_sound: ntfy_desktop::config::NotificationSound::Alert,
        ..AppConfig::default()
    };
    
    // Save config
    let config_json = serde_json::to_string_pretty(&original_config)
        .expect("Failed to serialize config");
    
    fs::write(&config_path, config_json)
        .await
        .expect("Failed to write config");
    
    // Verify file exists
    assert!(config_path.exists(), "Config file should exist");
    
    // Load config
    let config_data = fs::read(&config_path)
        .await
        .expect("Failed to read config");
    
    let config_str = String::from_utf8(config_data)
        .expect("Invalid UTF-8");
    
    let loaded_config: AppConfig = serde_json::from_str(&config_str)
        .expect("Failed to deserialize config");
    
    // Verify complete round-trip
    assert_eq!(original_config.instance_url, loaded_config.instance_url);
    assert_eq!(original_config.topics, loaded_config.topics);
    assert_eq!(original_config.poll_rate, loaded_config.poll_rate);
    assert_eq!(original_config.datetime_format, loaded_config.datetime_format);
    assert_eq!(original_config.start_hidden, loaded_config.start_hidden);
    assert_eq!(original_config.quit_on_close, loaded_config.quit_on_close);
    assert_eq!(original_config.hotkeys_enabled, loaded_config.hotkeys_enabled);
    assert_eq!(original_config.dev_tools, loaded_config.dev_tools);
    assert_eq!(original_config.welcome_completed, loaded_config.welcome_completed);
    assert_eq!(original_config.persistent_notifications_mode, loaded_config.persistent_notifications_mode);
    assert_eq!(original_config.notification_sound, loaded_config.notification_sound);
    assert_eq!(original_config.urgent_notification_sound, loaded_config.urgent_notification_sound);
    
    // Test config methods
    assert_eq!(loaded_config.api_base_url(), "https://my-ntfy.example.com");
    assert_eq!(loaded_config.topics_list(), vec!["alerts", "updates", "monitoring"]);
    assert_eq!(loaded_config.topics_path(), "alerts,updates,monitoring");
    assert_eq!(loaded_config.effective_poll_rate(), 30);
}

#[test]
fn test_notification_logic_flow() {
    let config = AppConfig {
        persistent_notifications_mode: ntfy_desktop::config::PersistentNotificationMode::UrgentOnly,
        notification_sound: ntfy_desktop::config::NotificationSound::Bell,
        urgent_notification_sound: ntfy_desktop::config::NotificationSound::Alert,
        ..AppConfig::default()
    };
    
    // Test persistent notification logic
    assert!(!config.should_persist_notification(false)); // Regular notification
    assert!(config.should_persist_notification(true));   // Urgent notification
    
    // Test sound selection
    assert_eq!(config.notification_sound_for(false), &ntfy_desktop::config::NotificationSound::Bell);
    assert_eq!(config.notification_sound_for(true), &ntfy_desktop::config::NotificationSound::Alert);
}

#[tokio::test]
async fn test_multiple_config_flow() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Test creating and managing multiple configs
    let configs = vec![
        ("config1", AppConfig {
            instance_url: "https://ntfy1.example.com/app".to_string(),
            topics: "topic1".to_string(),
            start_hidden: true,
            ..AppConfig::default()
        }),
        ("config2", AppConfig {
            instance_url: "https://ntfy2.example.com/app".to_string(),
            topics: "topic2,topic3".to_string(),
            start_hidden: false,
            quit_on_close: true,
            ..AppConfig::default()
        }),
        ("config3", AppConfig {
            instance_url: "https://ntfy3.example.com".to_string(), // No /app suffix
            topics: "".to_string(), // Empty topics
            poll_rate: 5, // Minimum poll rate
            ..AppConfig::default()
        }),
    ];
    
    for (name, config) in &configs {
        let config_path = temp_dir.path().join(format!("{}.json", name));
        
        // Save
        let config_json = serde_json::to_string_pretty(config)
            .expect("Failed to serialize config");
        
        fs::write(&config_path, config_json)
            .await
            .expect("Failed to write config");
        
        // Load and verify
        let config_data = fs::read(&config_path)
            .await
            .expect("Failed to read config");
        
        let config_str = String::from_utf8(config_data)
            .expect("Invalid UTF-8");
        
        let loaded_config: AppConfig = serde_json::from_str(&config_str)
            .expect("Failed to deserialize config");
        
        assert_eq!(config.instance_url, loaded_config.instance_url);
        assert_eq!(config.topics, loaded_config.topics);
        assert_eq!(config.poll_rate, loaded_config.poll_rate);
        
        // Test config methods
        assert_eq!(config.api_base_url(), loaded_config.api_base_url());
        assert_eq!(config.topics_list(), loaded_config.topics_list());
        assert_eq!(config.topics_path(), loaded_config.topics_path());
        assert_eq!(config.effective_poll_rate(), loaded_config.effective_poll_rate());
    }
    
    // Verify all files exist
    for (name, _) in &configs {
        let config_path = temp_dir.path().join(format!("{}.json", name));
        assert!(config_path.exists(), "Config file {} should exist", name);
    }
}

#[test]
fn test_config_methods_flow() {
    // Test all config methods with various inputs
    let test_cases = vec![
        ("https://ntfy.sh/app", "https://ntfy.sh"),
        ("https://ntfy.sh", "https://ntfy.sh"),
        ("https://my-ntfy.example.com/app", "https://my-ntfy.example.com"),
        ("http://localhost:8080", "http://localhost:8080"),
    ];
    
    for (input_url, expected_base_url) in test_cases {
        let config = AppConfig {
            instance_url: input_url.to_string(),
            ..AppConfig::default()
        };
        
        assert_eq!(config.api_base_url(), expected_base_url);
    }
    
    // Test topics parsing
    let topics_cases = vec![
        ("topic1", vec!["topic1"], "topic1"),
        ("topic1,topic2", vec!["topic1", "topic2"], "topic1,topic2"),
        ("topic1, topic2", vec!["topic1", "topic2"], "topic1,topic2"),
        ("", Vec::<&str>::new(), ""),
        ("topic1,,topic2", vec!["topic1", "topic2"], "topic1,topic2"),
    ];
    
    for (input_topics, expected_list, expected_path) in topics_cases {
        let config = AppConfig {
            topics: input_topics.to_string(),
            ..AppConfig::default()
        };
        
        assert_eq!(config.topics_list(), expected_list);
        assert_eq!(config.topics_path(), expected_path);
    }
    
    // Test poll rate clamping
    let poll_cases = vec![
        (5, 5),   // Minimum
        (60, 60), // Normal
        (3600, 3600), // Maximum
        (3, 5),   // Below minimum -> clamped
        (5000, 3600), // Above maximum -> clamped
    ];
    
    for (input_rate, expected_rate) in poll_cases {
        let config = AppConfig {
            poll_rate: input_rate,
            ..AppConfig::default()
        };
        
        assert_eq!(config.effective_poll_rate(), expected_rate);
    }
}

#[test]
fn test_notification_flow_scenarios() {
    // Test different notification scenarios
    let scenarios = vec![
        // (persistent_mode, is_urgent, expected_persistent)
        (ntfy_desktop::config::PersistentNotificationMode::Off, false, false),
        (ntfy_desktop::config::PersistentNotificationMode::Off, true, false),
        (ntfy_desktop::config::PersistentNotificationMode::All, false, true),
        (ntfy_desktop::config::PersistentNotificationMode::All, true, true),
        (ntfy_desktop::config::PersistentNotificationMode::UrgentOnly, false, false),
        (ntfy_desktop::config::PersistentNotificationMode::UrgentOnly, true, true),
    ];
    
    for (mode, is_urgent, expected_persistent) in scenarios {
        let config = AppConfig {
            persistent_notifications_mode: mode,
            ..AppConfig::default()
        };
        
        assert_eq!(config.should_persist_notification(is_urgent), expected_persistent);
    }
    
    // Test sound selection scenarios
    let sound_scenarios = vec![
        // (regular_sound, urgent_sound, is_urgent, expected_sound)
        (ntfy_desktop::config::NotificationSound::Bell, ntfy_desktop::config::NotificationSound::Alert, false, ntfy_desktop::config::NotificationSound::Bell),
        (ntfy_desktop::config::NotificationSound::Bell, ntfy_desktop::config::NotificationSound::Alert, true, ntfy_desktop::config::NotificationSound::Alert),
        (ntfy_desktop::config::NotificationSound::Chime, ntfy_desktop::config::NotificationSound::Pop, false, ntfy_desktop::config::NotificationSound::Chime),
        (ntfy_desktop::config::NotificationSound::Chime, ntfy_desktop::config::NotificationSound::Pop, true, ntfy_desktop::config::NotificationSound::Pop),
    ];
    
    for (regular_sound, urgent_sound, is_urgent, expected_sound) in sound_scenarios {
        let config = AppConfig {
            notification_sound: regular_sound,
            urgent_notification_sound: urgent_sound,
            ..AppConfig::default()
        };
        
        assert_eq!(config.notification_sound_for(is_urgent), &expected_sound);
    }
}

#[tokio::test]
async fn test_error_recovery_flow() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Test various error scenarios and recovery
    
    // Scenario 1: Invalid JSON
    let invalid_path = temp_dir.path().join("invalid.json");
    fs::write(&invalid_path, "{invalid json}")
        .await
        .expect("Failed to write invalid JSON");
    
    let invalid_data = fs::read(&invalid_path).await.expect("Failed to read");
    let invalid_str = String::from_utf8(invalid_data).expect("Invalid UTF-8");
    let invalid_result: Result<AppConfig, _> = serde_json::from_str(&invalid_str);
    assert!(invalid_result.is_err(), "Invalid JSON should fail");
    
    // Scenario 2: Missing file (should be handled gracefully by application)
    let missing_path = temp_dir.path().join("missing.json");
    assert!(!missing_path.exists(), "File should not exist");
    
    // Scenario 3: Empty file
    let empty_path = temp_dir.path().join("empty.json");
    fs::write(&empty_path, "")
        .await
        .expect("Failed to write empty file");
    
    let empty_data = fs::read(&empty_path).await.expect("Failed to read");
    let empty_str = String::from_utf8(empty_data).expect("Invalid UTF-8");
    let empty_result: Result<AppConfig, _> = serde_json::from_str(&empty_str);
    assert!(empty_result.is_err(), "Empty file should fail");
    
    // Scenario 4: Partial config (missing required fields)
    let partial_path = temp_dir.path().join("partial.json");
    let partial_json = r#"{"instance_url": "https://example.com"}"#; // Missing other fields
    fs::write(&partial_path, partial_json)
        .await
        .expect("Failed to write partial config");
    
    let partial_data = fs::read(&partial_path).await.expect("Failed to read");
    let partial_str = String::from_utf8(partial_data).expect("Invalid UTF-8");
    let partial_result: Result<AppConfig, _> = serde_json::from_str(&partial_str);
    
    // This should succeed with default values for missing fields
    assert!(partial_result.is_ok(), "Partial config should load with defaults");
    
    if let Ok(partial_config) = partial_result {
        assert_eq!(partial_config.instance_url, "https://example.com");
        // Other fields should have default values
        assert_eq!(partial_config.topics, "announcements,stats");
        assert_eq!(partial_config.poll_rate, 60);
    }
}

#[test]
fn test_config_validation_flow() {
    // Test that config validation works correctly
    
    // Valid config
    let valid_config = AppConfig::default();
    assert_eq!(valid_config.instance_url, "https://ntfy.sh/app");
    assert_eq!(valid_config.poll_rate, 60);
    assert_eq!(valid_config.effective_poll_rate(), 60);
    
    // Test with extreme values
    let extreme_config = AppConfig {
        poll_rate: 1, // Will be clamped to 5
        ..AppConfig::default()
    };
    assert_eq!(extreme_config.effective_poll_rate(), 5);
    
    let extreme_config2 = AppConfig {
        poll_rate: 10000, // Will be clamped to 3600
        ..AppConfig::default()
    };
    assert_eq!(extreme_config2.effective_poll_rate(), 3600);
    
    // Test URL normalization
    let url_cases = vec![
        ("https://ntfy.sh/app", "https://ntfy.sh"),
        ("https://ntfy.sh/app/", "https://ntfy.sh"),
        ("https://ntfy.sh", "https://ntfy.sh"),
        ("http://localhost:8080/app", "http://localhost:8080"),
    ];
    
    for (input_url, expected_url) in url_cases {
        let config = AppConfig {
            instance_url: input_url.to_string(),
            ..AppConfig::default()
        };
        assert_eq!(config.api_base_url(), expected_url);
    }
}