use assert_matches::assert_matches;
use ntfy_desktop::config::{NotificationSound, PersistentNotificationMode};
use ntfy_desktop::notifications::{NotificationData, NotificationManager};

#[test]
fn test_notification_manager_creation() {
    let manager = NotificationManager::new();
    assert_matches!(manager, NotificationManager);

    let default_manager = NotificationManager::default();
    assert_matches!(default_manager, NotificationManager);
}

#[test]
fn test_notification_data_structure() {
    let notification = NotificationData {
        title: "Test Title".to_string(),
        subtitle: Some("Test Subtitle".to_string()),
        message: "Test Message".to_string(),
        topic: "test-topic".to_string(),
        timestamp: 1234567890,
        urgent: true,
        sound: NotificationSound::Alert,
        persistent: true,
        icon_url: Some("https://example.com/icon.png".to_string()),
    };

    assert_eq!(notification.title, "Test Title");
    assert_eq!(notification.subtitle, Some("Test Subtitle".to_string()));
    assert_eq!(notification.message, "Test Message");
    assert_eq!(notification.topic, "test-topic");
    assert_eq!(notification.timestamp, 1234567890);
    assert!(notification.urgent);
    assert_eq!(notification.sound, NotificationSound::Alert);
    assert!(notification.persistent);
    assert_eq!(
        notification.icon_url,
        Some("https://example.com/icon.png".to_string())
    );
}

#[test]
fn test_notification_data_without_subtitle() {
    let notification = NotificationData {
        title: "Title Only".to_string(),
        subtitle: None,
        message: "Message".to_string(),
        topic: "topic".to_string(),
        timestamp: 0,
        urgent: false,
        sound: NotificationSound::Default,
        persistent: false,
        icon_url: None,
    };

    assert_eq!(notification.title, "Title Only");
    assert_eq!(notification.subtitle, None);
    assert_eq!(notification.message, "Message");
    assert_eq!(notification.topic, "topic");
    assert_eq!(notification.timestamp, 0);
    assert!(!notification.urgent);
    assert_eq!(notification.sound, NotificationSound::Default);
    assert!(!notification.persistent);
    assert_eq!(notification.icon_url, None);
}

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
            title: "Test".to_string(),
            subtitle: None,
            message: "Message".to_string(),
            topic: "topic".to_string(),
            timestamp: 0,
            urgent: false,
            sound: sound.clone(),
            persistent: false,
            icon_url: None,
        };

        assert_eq!(notification.sound, sound);
    }
}

#[test]
fn test_persistent_notification_modes() {
    use ntfy_desktop::config::AppConfig;

    let test_cases = vec![
        (PersistentNotificationMode::Off, false, false), // Never persistent
        (PersistentNotificationMode::Off, true, false),
        (PersistentNotificationMode::All, false, true), // Always persistent
        (PersistentNotificationMode::All, true, true),
        (PersistentNotificationMode::UrgentOnly, false, false), // Only urgent
        (PersistentNotificationMode::UrgentOnly, true, true),
    ];

    for (mode, is_urgent, expected_persistent) in test_cases {
        let config = AppConfig {
            persistent_notifications_mode: mode,
            ..AppConfig::default()
        };

        assert_eq!(
            config.should_persist_notification(is_urgent),
            expected_persistent
        );
    }
}

#[test]
fn test_notification_sound_selection() {
    use ntfy_desktop::config::AppConfig;

    let config = AppConfig {
        notification_sound: NotificationSound::Chime,
        urgent_notification_sound: NotificationSound::Alert,
        ..AppConfig::default()
    };

    // Regular notification should use regular sound
    assert_eq!(
        config.notification_sound_for(false),
        &NotificationSound::Chime
    );

    // Urgent notification should use urgent sound
    assert_eq!(
        config.notification_sound_for(true),
        &NotificationSound::Alert
    );
}

#[test]
fn test_notification_data_equality() {
    let notification1 = NotificationData {
        title: "Title".to_string(),
        subtitle: Some("Subtitle".to_string()),
        message: "Message".to_string(),
        topic: "topic".to_string(),
        timestamp: 123,
        urgent: true,
        sound: NotificationSound::Bell,
        persistent: true,
        icon_url: Some("icon.png".to_string()),
    };

    let notification2 = NotificationData {
        title: "Title".to_string(),
        subtitle: Some("Subtitle".to_string()),
        message: "Message".to_string(),
        topic: "topic".to_string(),
        timestamp: 123,
        urgent: true,
        sound: NotificationSound::Bell,
        persistent: true,
        icon_url: Some("icon.png".to_string()),
    };

    // Same data should be equal
    assert_eq!(notification1.title, notification2.title);
    assert_eq!(notification1.subtitle, notification2.subtitle);
    assert_eq!(notification1.message, notification2.message);
    assert_eq!(notification1.topic, notification2.topic);
    assert_eq!(notification1.timestamp, notification2.timestamp);
    assert_eq!(notification1.urgent, notification2.urgent);
    assert_eq!(notification1.sound, notification2.sound);
    assert_eq!(notification1.persistent, notification2.persistent);
    assert_eq!(notification1.icon_url, notification2.icon_url);
}

#[test]
fn test_notification_data_differences() {
    let base_notification = NotificationData {
        title: "Title".to_string(),
        subtitle: Some("Subtitle".to_string()),
        message: "Message".to_string(),
        topic: "topic".to_string(),
        timestamp: 123,
        urgent: false,
        sound: NotificationSound::Default,
        persistent: false,
        icon_url: None,
    };

    // Different title
    let different_title = NotificationData {
        title: "Different Title".to_string(),
        ..base_notification.clone()
    };
    assert_ne!(base_notification.title, different_title.title);

    // Different urgency
    let different_urgency = NotificationData {
        urgent: true,
        ..base_notification.clone()
    };
    assert_ne!(base_notification.urgent, different_urgency.urgent);

    // Different sound
    let different_sound = NotificationData {
        sound: NotificationSound::Alert,
        ..base_notification.clone()
    };
    assert_ne!(base_notification.sound, different_sound.sound);

    // Different persistence
    let different_persistent = NotificationData {
        persistent: true,
        ..base_notification.clone()
    };
    assert_ne!(
        base_notification.persistent,
        different_persistent.persistent
    );
}

#[test]
fn test_notification_sound_default() {
    let default_sound = NotificationSound::default();
    assert_eq!(default_sound, NotificationSound::Default);
}

#[test]
fn test_persistent_notification_mode_default() {
    let default_mode = PersistentNotificationMode::default();
    assert_eq!(default_mode, PersistentNotificationMode::Off);
}

#[test]
fn test_notification_data_clone() {
    let original = NotificationData {
        title: "Original".to_string(),
        subtitle: Some("Subtitle".to_string()),
        message: "Message".to_string(),
        topic: "topic".to_string(),
        timestamp: 123,
        urgent: true,
        sound: NotificationSound::Chime,
        persistent: true,
        icon_url: Some("icon.png".to_string()),
    };

    let cloned = original.clone();

    assert_eq!(original.title, cloned.title);
    assert_eq!(original.subtitle, cloned.subtitle);
    assert_eq!(original.message, cloned.message);
    assert_eq!(original.topic, cloned.topic);
    assert_eq!(original.timestamp, cloned.timestamp);
    assert_eq!(original.urgent, cloned.urgent);
    assert_eq!(original.sound, cloned.sound);
    assert_eq!(original.persistent, cloned.persistent);
    assert_eq!(original.icon_url, cloned.icon_url);
}

#[test]
fn test_notification_debug_format() {
    let notification = NotificationData {
        title: "Test".to_string(),
        subtitle: Some("Sub".to_string()),
        message: "Msg".to_string(),
        topic: "topic".to_string(),
        timestamp: 123,
        urgent: true,
        sound: NotificationSound::Alert,
        persistent: true,
        icon_url: Some("icon.png".to_string()),
    };

    // Just ensure it doesn't panic
    let debug_output = format!("{:?}", notification);
    assert!(!debug_output.is_empty());
}

#[test]
fn test_notification_sound_debug() {
    let sound = NotificationSound::Bell;
    let debug_output = format!("{:?}", sound);
    assert_eq!(debug_output, "Bell");
}

#[test]
fn test_persistent_mode_debug() {
    let mode = PersistentNotificationMode::UrgentOnly;
    let debug_output = format!("{:?}", mode);
    assert_eq!(debug_output, "UrgentOnly");
}
