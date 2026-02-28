use ntfy_desktop::config::{AppConfig, PersistentNotificationMode, NotificationSound};
use tempfile::TempDir;
use tokio::fs;

#[test]
fn test_config_default_values() {
    let config = AppConfig::default();
    
    assert_eq!(config.instance_url, "https://ntfy.sh/app");
    assert_eq!(config.api_token, "");
    assert_eq!(config.auth_user, "");
    assert_eq!(config.auth_pass, "");
    assert_eq!(config.topics, "announcements,stats");
    assert_eq!(config.poll_rate, 60);
    assert_eq!(config.datetime_format, "YYYY-MM-DD hh:mm a");
    assert!(!config.persistent_notifications);
    assert_eq!(config.persistent_notifications_mode, PersistentNotificationMode::Off);
    assert_eq!(config.notification_sound, NotificationSound::Default);
    assert_eq!(config.urgent_notification_sound, NotificationSound::Default);
    assert!(!config.self_hosted_instance);
    assert!(!config.start_hidden);
    assert!(!config.quit_on_close);
    assert!(!config.hotkeys_enabled);
    assert!(!config.dev_tools);
    assert!(!config.welcome_completed);
    assert_eq!(config.urgent_priority_threshold, 4);
}

#[test]
fn test_api_base_url_parsing() {
    let mut config = AppConfig::default();
    
    // Test various URL formats
    config.instance_url = "https://ntfy.sh/app".to_string();
    assert_eq!(config.api_base_url(), "https://ntfy.sh");
    
    config.instance_url = "https://ntfy.sh".to_string();
    assert_eq!(config.api_base_url(), "https://ntfy.sh");
    
    config.instance_url = "https://ntfy.sh/app/".to_string();
    assert_eq!(config.api_base_url(), "https://ntfy.sh");
    
    config.instance_url = "https://my-ntfy.example.com/app".to_string();
    assert_eq!(config.api_base_url(), "https://my-ntfy.example.com");
    
    config.instance_url = "http://localhost:8080".to_string();
    assert_eq!(config.api_base_url(), "http://localhost:8080");
}

#[test]
fn test_topics_parsing() {
    let mut config = AppConfig::default();
    
    // Normal comma-separated topics
    config.topics = "topic1,topic2,topic3".to_string();
    assert_eq!(config.topics_list(), vec!["topic1", "topic2", "topic3"]);
    assert_eq!(config.topics_path(), "topic1,topic2,topic3");
    
    // Topics with spaces
    config.topics = "topic1, topic2, topic3".to_string();
    assert_eq!(config.topics_list(), vec!["topic1", "topic2", "topic3"]);
    assert_eq!(config.topics_path(), "topic1,topic2,topic3");
    
    // Empty topics
    config.topics = "".to_string();
    assert_eq!(config.topics_list(), Vec::<String>::new());
    assert_eq!(config.topics_path(), "");
    
    // Topics with extra commas
    config.topics = "topic1,,topic2,".to_string();
    assert_eq!(config.topics_list(), vec!["topic1", "topic2"]);
    assert_eq!(config.topics_path(), "topic1,topic2");
}

#[test]
fn test_poll_rate_clamping() {
    let mut config = AppConfig::default();
    
    // Normal value
    config.poll_rate = 60;
    assert_eq!(config.effective_poll_rate(), 60);
    
    // Below minimum
    config.poll_rate = 3;
    assert_eq!(config.effective_poll_rate(), 5);
    
    // Above maximum
    config.poll_rate = 5000;
    assert_eq!(config.effective_poll_rate(), 3600);
    
    // At minimum
    config.poll_rate = 5;
    assert_eq!(config.effective_poll_rate(), 5);
    
    // At maximum
    config.poll_rate = 3600;
    assert_eq!(config.effective_poll_rate(), 3600);
}

#[test]
fn test_persistent_notification_logic() {
    let mut config = AppConfig::default();
    
    // Off mode
    config.persistent_notifications_mode = PersistentNotificationMode::Off;
    assert!(!config.should_persist_notification(false));
    assert!(!config.should_persist_notification(true));
    
    // All mode
    config.persistent_notifications_mode = PersistentNotificationMode::All;
    assert!(config.should_persist_notification(false));
    assert!(config.should_persist_notification(true));
    
    // UrgentOnly mode
    config.persistent_notifications_mode = PersistentNotificationMode::UrgentOnly;
    assert!(!config.should_persist_notification(false));
    assert!(config.should_persist_notification(true));
}

#[test]
fn test_notification_sound_selection() {
    let mut config = AppConfig::default();
    
    config.notification_sound = NotificationSound::Chime;
    config.urgent_notification_sound = NotificationSound::Alert;
    
    assert_eq!(config.notification_sound_for(false), &NotificationSound::Chime);
    assert_eq!(config.notification_sound_for(true), &NotificationSound::Alert);
}

#[tokio::test]
async fn test_config_serialization_roundtrip() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("prefs.json");
    
    let original_config = AppConfig {
        instance_url: "https://test.example.com".to_string(),
        topics: "test-topic".to_string(),
        poll_rate: 120,
        datetime_format: "YYYY-MM-DD HH:mm".to_string(),
        persistent_notifications: true,
        persistent_notifications_mode: PersistentNotificationMode::UrgentOnly,
        notification_sound: NotificationSound::Chime,
        urgent_notification_sound: NotificationSound::Alert,
        self_hosted_instance: false,
        start_hidden: true,
        quit_on_close: false,
        hotkeys_enabled: true,
        dev_tools: false,
        welcome_completed: true,
        ..AppConfig::default()
    };
    
    // Serialize
    let config_json = serde_json::to_string_pretty(&original_config)
        .expect("Failed to serialize config");
    
    // Write to file
    fs::write(&config_path, config_json)
        .await
        .expect("Failed to write config file");
    
    // Read back
    let config_data = fs::read(&config_path)
        .await
        .expect("Failed to read config file");
    let config_str = String::from_utf8(config_data).expect("Invalid UTF-8");
    
    let loaded_config: AppConfig = serde_json::from_str(&config_str)
        .expect("Failed to deserialize config");
    
    // Verify non-sensitive fields match
    assert_eq!(original_config.instance_url, loaded_config.instance_url);
    assert_eq!(original_config.topics, loaded_config.topics);
    assert_eq!(original_config.poll_rate, loaded_config.poll_rate);
    assert_eq!(original_config.datetime_format, loaded_config.datetime_format);
    assert_eq!(original_config.persistent_notifications, loaded_config.persistent_notifications);
    assert_eq!(original_config.persistent_notifications_mode, loaded_config.persistent_notifications_mode);
    assert_eq!(original_config.notification_sound, loaded_config.notification_sound);
    assert_eq!(original_config.urgent_notification_sound, loaded_config.urgent_notification_sound);
    assert_eq!(original_config.self_hosted_instance, loaded_config.self_hosted_instance);
    assert_eq!(original_config.start_hidden, loaded_config.start_hidden);
    assert_eq!(original_config.quit_on_close, loaded_config.quit_on_close);
    assert_eq!(original_config.hotkeys_enabled, loaded_config.hotkeys_enabled);
    assert_eq!(original_config.dev_tools, loaded_config.dev_tools);
    assert_eq!(original_config.welcome_completed, loaded_config.welcome_completed);
    
    // Sensitive fields should be empty (not serialized)
    assert_eq!(loaded_config.api_token, "");
    assert_eq!(loaded_config.auth_user, "");
    assert_eq!(loaded_config.auth_pass, "");
}

#[tokio::test]
async fn test_config_with_special_characters() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("prefs.json");
    
    let config = AppConfig {
        instance_url: "https://example.com/path?query=test&param=value".to_string(),
        topics: "topic-with-ünicode,another-topic".to_string(),
        poll_rate: 123,
        datetime_format: "YYYY年MM月DD日 HH時mm分".to_string(),
        ..AppConfig::default()
    };
    
    let config_json = serde_json::to_string_pretty(&config)
        .expect("Failed to serialize config");
    
    fs::write(&config_path, config_json)
        .await
        .expect("Failed to write config file");
    
    let config_data = fs::read(&config_path)
        .await
        .expect("Failed to read config file");
    let config_str = String::from_utf8(config_data).expect("Invalid UTF-8");
    
    let loaded_config: AppConfig = serde_json::from_str(&config_str)
        .expect("Failed to deserialize config");
    
    assert_eq!(config.instance_url, loaded_config.instance_url);
    assert_eq!(config.topics, loaded_config.topics);
    assert_eq!(config.poll_rate, loaded_config.poll_rate);
    assert_eq!(config.datetime_format, loaded_config.datetime_format);
}

#[test]
fn test_config_edge_cases() {
    // Test with empty strings
    let empty_config = AppConfig {
        instance_url: "".to_string(),
        topics: "".to_string(),
        poll_rate: 0,
        datetime_format: "".to_string(),
        ..AppConfig::default()
    };
    
    let serialized = serde_json::to_string(&empty_config).expect("Failed to serialize");
    let deserialized: AppConfig = serde_json::from_str(&serialized).expect("Failed to deserialize");
    
    assert_eq!(empty_config.instance_url, deserialized.instance_url);
    assert_eq!(empty_config.topics, deserialized.topics);
    assert_eq!(empty_config.poll_rate, deserialized.poll_rate);
    
    // Verify credentials are never serialized
    assert!(!serialized.contains("api_token"));
    assert!(!serialized.contains("auth_user"));
    assert!(!serialized.contains("auth_pass"));
}