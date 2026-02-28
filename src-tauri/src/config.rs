use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

/// Persistent notification mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PersistentNotificationMode {
    Off,
    All,
    UrgentOnly,
}

impl Default for PersistentNotificationMode {
    fn default() -> Self {
        PersistentNotificationMode::Off
    }
}

/// Notification sound options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NotificationSound {
    Default,
    None,
    Alert,
    Bell,
    Chime,
    Pop,
}

impl Default for NotificationSound {
    fn default() -> Self {
        NotificationSound::Default
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub instance_url: String,
    #[serde(skip_serializing, default)]
    pub api_token: String,
    #[serde(skip_serializing, default)]
    pub auth_user: String,
    #[serde(skip_serializing, default)]
    pub auth_pass: String,
    pub topics: String,
    pub poll_rate: u64,
    pub datetime_format: String,
    // Deprecated: kept for migration, use persistent_notifications_mode instead
    #[serde(default)]
    pub persistent_notifications: bool,
    pub persistent_notifications_mode: PersistentNotificationMode,
    pub notification_sound: NotificationSound,
    pub urgent_notification_sound: NotificationSound,
    pub self_hosted_instance: bool,
    pub start_hidden: bool,
    pub quit_on_close: bool,
    pub hotkeys_enabled: bool,
    pub dev_tools: bool,
    pub welcome_completed: bool,
    pub urgent_priority_threshold: u8,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            instance_url: "https://ntfy.sh/app".to_string(),
            api_token: String::new(),
            auth_user: String::new(),
            auth_pass: String::new(),
            topics: "announcements,stats".to_string(),
            poll_rate: 60,
            datetime_format: "YYYY-MM-DD hh:mm a".to_string(),
            persistent_notifications: false,
            persistent_notifications_mode: PersistentNotificationMode::Off,
            notification_sound: NotificationSound::Default,
            urgent_notification_sound: NotificationSound::Default,
            self_hosted_instance: false,
            start_hidden: false,
            quit_on_close: false,
            hotkeys_enabled: false,
            dev_tools: false,
            welcome_completed: false,
            urgent_priority_threshold: 4,
        }
    }
}

impl AppConfig {
    /// Get the base URL for API calls (strips /app suffix if present)
    pub fn api_base_url(&self) -> String {
        self.instance_url
            .trim_end_matches('/')
            .trim_end_matches("/app")
            .to_string()
    }

    /// Get the topics as a vector
    pub fn topics_list(&self) -> Vec<String> {
        self.topics
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Get the topics as a comma-separated path for the ntfy API
    pub fn topics_path(&self) -> String {
        self.topics_list().join(",")
    }

    /// Clamp poll_rate to valid range (5-3600 seconds)
    pub fn effective_poll_rate(&self) -> u64 {
        self.poll_rate.clamp(5, 3600)
    }

    /// Check if notifications should be persistent based on urgency
    pub fn should_persist_notification(&self, is_urgent: bool) -> bool {
        match self.persistent_notifications_mode {
            PersistentNotificationMode::Off => false,
            PersistentNotificationMode::All => true,
            PersistentNotificationMode::UrgentOnly => is_urgent,
        }
    }

    /// Get the sound to use for a notification based on urgency
    pub fn notification_sound_for(&self, is_urgent: bool) -> &NotificationSound {
        if is_urgent {
            &self.urgent_notification_sound
        } else {
            &self.notification_sound
        }
    }
}

pub async fn save_config(app_handle: &AppHandle, config: AppConfig) -> Result<()> {
    let app_dir = app_handle
        .path()
        .app_config_dir()
        .map_err(|_| anyhow::anyhow!("Could not get app config directory"))?;

    std::fs::create_dir_all(&app_dir)?;

    let config_path = app_dir.join("prefs.json");
    let config_json = serde_json::to_string_pretty(&config)?;

    tokio::fs::write(config_path, config_json).await?;

    Ok(())
}

pub async fn load_config(app_handle: &AppHandle) -> Result<AppConfig> {
    let app_dir = app_handle
        .path()
        .app_config_dir()
        .map_err(|_| anyhow::anyhow!("Could not get app config directory"))?;

    let config_path = app_dir.join("prefs.json");

    if !config_path.exists() {
        return Ok(AppConfig::default());
    }

    let config_data = tokio::fs::read(config_path).await?;
    let config_str = String::from_utf8(config_data)?;

    let config: AppConfig = serde_json::from_str(&config_str)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::fs;

    #[test]
    fn test_default_config() {
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
    fn test_config_serialization() {
        let config = AppConfig {
            instance_url: "https://example.com/app".to_string(),
            api_token: "test-token".to_string(),
            auth_user: "testuser".to_string(),
            auth_pass: "testpass".to_string(),
            topics: "topic1,topic2,topic3".to_string(),
            poll_rate: 30,
            datetime_format: "MM/DD/YYYY HH:mm".to_string(),
            persistent_notifications: true,
            persistent_notifications_mode: PersistentNotificationMode::UrgentOnly,
            notification_sound: NotificationSound::Chime,
            urgent_notification_sound: NotificationSound::Alert,
            self_hosted_instance: true,
            start_hidden: true,
            quit_on_close: true,
            hotkeys_enabled: true,
            dev_tools: true,
            welcome_completed: true,
            urgent_priority_threshold: 4,
        };

        let serialized = serde_json::to_string(&config).unwrap();

        // Credentials should NOT appear in serialized JSON (stored in OS keychain)
        assert!(!serialized.contains("test-token"));
        assert!(!serialized.contains("testuser"));
        assert!(!serialized.contains("testpass"));

        let deserialized: AppConfig = serde_json::from_str(&serialized).unwrap();

        assert_eq!(config.instance_url, deserialized.instance_url);
        // Credentials are skip_serializing so they round-trip as empty
        assert_eq!(deserialized.api_token, "");
        assert_eq!(deserialized.auth_user, "");
        assert_eq!(deserialized.auth_pass, "");
        assert_eq!(config.topics, deserialized.topics);
        assert_eq!(config.poll_rate, deserialized.poll_rate);
        assert_eq!(config.datetime_format, deserialized.datetime_format);
        assert_eq!(
            config.persistent_notifications,
            deserialized.persistent_notifications
        );
        assert_eq!(
            config.persistent_notifications_mode,
            deserialized.persistent_notifications_mode
        );
        assert_eq!(config.notification_sound, deserialized.notification_sound);
        assert_eq!(
            config.urgent_notification_sound,
            deserialized.urgent_notification_sound
        );
        assert_eq!(
            config.self_hosted_instance,
            deserialized.self_hosted_instance
        );
        assert_eq!(config.start_hidden, deserialized.start_hidden);
        assert_eq!(config.quit_on_close, deserialized.quit_on_close);
        assert_eq!(config.hotkeys_enabled, deserialized.hotkeys_enabled);
        assert_eq!(config.dev_tools, deserialized.dev_tools);
        assert_eq!(config.welcome_completed, deserialized.welcome_completed);
    }

    #[test]
    fn test_api_base_url() {
        let mut config = AppConfig::default();

        // Test normal URL
        config.instance_url = "https://ntfy.sh/app".to_string();
        assert_eq!(config.api_base_url(), "https://ntfy.sh");

        // Test URL without /app suffix
        config.instance_url = "https://ntfy.sh".to_string();
        assert_eq!(config.api_base_url(), "https://ntfy.sh");

        // Test URL with trailing slash
        config.instance_url = "https://ntfy.sh/app/".to_string();
        assert_eq!(config.api_base_url(), "https://ntfy.sh");

        // Test custom instance
        config.instance_url = "https://my-ntfy.example.com/app".to_string();
        assert_eq!(config.api_base_url(), "https://my-ntfy.example.com");

        // Test self-hosted instance without /app suffix
        config.instance_url = "http://localhost:8080".to_string();
        assert_eq!(config.api_base_url(), "http://localhost:8080");

        // Test self-hosted instance with /app suffix
        config.instance_url = "http://192.168.1.100:8080/app".to_string();
        assert_eq!(config.api_base_url(), "http://192.168.1.100:8080");
    }

    #[test]
    fn test_topics_parsing() {
        let mut config = AppConfig::default();

        // Test normal comma-separated topics
        config.topics = "topic1,topic2,topic3".to_string();
        assert_eq!(config.topics_list(), vec!["topic1", "topic2", "topic3"]);
        assert_eq!(config.topics_path(), "topic1,topic2,topic3");

        // Test with spaces
        config.topics = "topic1, topic2, topic3".to_string();
        assert_eq!(config.topics_list(), vec!["topic1", "topic2", "topic3"]);
        assert_eq!(config.topics_path(), "topic1,topic2,topic3");

        // Test with empty topics
        config.topics = "".to_string();
        assert_eq!(config.topics_list(), Vec::<String>::new());
        assert_eq!(config.topics_path(), "");

        // Test with extra commas
        config.topics = "topic1,,topic2,".to_string();
        assert_eq!(config.topics_list(), vec!["topic1", "topic2"]);
        assert_eq!(config.topics_path(), "topic1,topic2");
    }

    #[test]
    fn test_poll_rate_clamping() {
        let mut config = AppConfig::default();

        // Test normal value
        config.poll_rate = 60;
        assert_eq!(config.effective_poll_rate(), 60);

        // Test lower bound
        config.poll_rate = 3;
        assert_eq!(config.effective_poll_rate(), 5);

        // Test upper bound
        config.poll_rate = 5000;
        assert_eq!(config.effective_poll_rate(), 3600);

        // Test minimum valid
        config.poll_rate = 5;
        assert_eq!(config.effective_poll_rate(), 5);

        // Test maximum valid
        config.poll_rate = 3600;
        assert_eq!(config.effective_poll_rate(), 3600);
    }

    #[tokio::test]
    async fn test_file_operations() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("prefs.json");

        // Test saving config (credentials stored in keychain, not on disk)
        let config = AppConfig {
            instance_url: "https://test.example.com".to_string(),
            topics: "test-topic".to_string(),
            poll_rate: 120,
            datetime_format: "YYYY-MM-DD HH:mm".to_string(),
            persistent_notifications: true,
            self_hosted_instance: false,
            start_hidden: true,
            quit_on_close: false,
            hotkeys_enabled: true,
            dev_tools: false,
            welcome_completed: true,
            ..Default::default()
        };

        // Save config manually
        let config_json = serde_json::to_string_pretty(&config).unwrap();
        fs::write(&config_path, config_json).await.unwrap();

        // Load config manually
        let config_data = fs::read(&config_path).await.unwrap();
        let config_str = String::from_utf8(config_data).unwrap();
        let loaded_config: AppConfig = serde_json::from_str(&config_str).unwrap();

        // Verify loaded config matches saved config (non-credential fields)
        assert_eq!(config.instance_url, loaded_config.instance_url);
        assert_eq!(config.topics, loaded_config.topics);
        assert_eq!(config.poll_rate, loaded_config.poll_rate);
        assert_eq!(config.datetime_format, loaded_config.datetime_format);
        assert_eq!(
            config.persistent_notifications,
            loaded_config.persistent_notifications
        );
        assert_eq!(
            config.self_hosted_instance,
            loaded_config.self_hosted_instance
        );
        assert_eq!(config.start_hidden, loaded_config.start_hidden);
        assert_eq!(config.quit_on_close, loaded_config.quit_on_close);
        assert_eq!(config.hotkeys_enabled, loaded_config.hotkeys_enabled);
        assert_eq!(config.dev_tools, loaded_config.dev_tools);
    }

    #[tokio::test]
    async fn test_file_operations_with_directory_creation() {
        let temp_dir = TempDir::new().unwrap();
        let sub_dir = temp_dir.path().join("config");
        let config_path = sub_dir.join("prefs.json");

        // Create directory and save config
        fs::create_dir_all(&sub_dir).await.unwrap();

        let config = AppConfig::default();
        let config_json = serde_json::to_string_pretty(&config).unwrap();
        fs::write(&config_path, config_json).await.unwrap();

        // Verify file exists
        assert!(config_path.exists());

        // Load and verify config
        let config_data = fs::read(&config_path).await.unwrap();
        let config_str = String::from_utf8(config_data).unwrap();
        let loaded_config: AppConfig = serde_json::from_str(&config_str).unwrap();

        assert_eq!(config.instance_url, loaded_config.instance_url);
    }

    #[tokio::test]
    async fn test_invalid_json_handling() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("prefs.json");

        // Write invalid JSON
        fs::write(&config_path, "invalid json content")
            .await
            .unwrap();

        // Attempt to load - this should fail with serde error
        let config_data = fs::read(&config_path).await.unwrap();
        let config_str = String::from_utf8(config_data).unwrap();
        let result: Result<AppConfig, _> = serde_json::from_str(&config_str);

        assert!(result.is_err());
    }

    #[test]
    fn test_serialization_edge_cases() {
        // Test with empty strings
        let empty_config = AppConfig {
            instance_url: "".to_string(),
            topics: "".to_string(),
            poll_rate: 0,
            datetime_format: "".to_string(),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&empty_config).unwrap();
        let deserialized: AppConfig = serde_json::from_str(&serialized).unwrap();

        assert_eq!(empty_config.instance_url, deserialized.instance_url);
        assert_eq!(empty_config.topics, deserialized.topics);
        assert_eq!(empty_config.poll_rate, deserialized.poll_rate);

        // Credentials should never appear in serialized output
        assert!(!serialized.contains("api_token"));
        assert!(!serialized.contains("auth_user"));
        assert!(!serialized.contains("auth_pass"));

        // Test with special characters
        let special_config = AppConfig {
            instance_url: "https://example.com/path?query=test&param=value".to_string(),
            topics: "topic-with-ünicode,another-topic".to_string(),
            poll_rate: 123,
            datetime_format: "YYYY年MM月DD日 HH時mm分".to_string(),
            persistent_notifications: true,
            start_hidden: true,
            hotkeys_enabled: true,
            dev_tools: true,
            welcome_completed: true,
            ..Default::default()
        };

        let serialized = serde_json::to_string(&special_config).unwrap();
        let deserialized: AppConfig = serde_json::from_str(&serialized).unwrap();

        assert_eq!(special_config.instance_url, deserialized.instance_url);
        assert_eq!(special_config.topics, deserialized.topics);
        assert_eq!(special_config.poll_rate, deserialized.poll_rate);
        assert_eq!(special_config.datetime_format, deserialized.datetime_format);
    }
}
