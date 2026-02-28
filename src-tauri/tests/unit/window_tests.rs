use ntfy_desktop::config::AppConfig;

#[test]
fn test_window_settings_validation() {
    let config = AppConfig::default();

    // Test default window settings
    assert!(!config.start_hidden);
    assert!(!config.quit_on_close);
    assert!(!config.hotkeys_enabled);
    assert!(!config.dev_tools);

    // Test with custom window settings
    let custom_config = AppConfig {
        start_hidden: true,
        quit_on_close: true,
        hotkeys_enabled: true,
        dev_tools: true,
        ..AppConfig::default()
    };

    assert!(custom_config.start_hidden);
    assert!(custom_config.quit_on_close);
    assert!(custom_config.hotkeys_enabled);
    assert!(custom_config.dev_tools);
}

#[test]
fn test_window_configuration_combinations() {
    // Test various combinations of window settings
    let test_cases = vec![
        (false, false, false, false), // All disabled
        (true, false, false, false),  // Start hidden only
        (false, true, false, false),  // Quit on close only
        (false, false, true, false),  // Hotkeys enabled only
        (false, false, false, true),  // Dev tools only
        (true, true, true, true),     // All enabled
    ];

    for (start_hidden, quit_on_close, hotkeys_enabled, dev_tools) in test_cases {
        let config = AppConfig {
            start_hidden,
            quit_on_close,
            hotkeys_enabled,
            dev_tools,
            ..AppConfig::default()
        };

        assert_eq!(config.start_hidden, start_hidden);
        assert_eq!(config.quit_on_close, quit_on_close);
        assert_eq!(config.hotkeys_enabled, hotkeys_enabled);
        assert_eq!(config.dev_tools, dev_tools);
    }
}

#[test]
fn test_window_settings_serialization() {
    let config = AppConfig {
        start_hidden: true,
        quit_on_close: true,
        hotkeys_enabled: true,
        dev_tools: true,
        ..AppConfig::default()
    };

    let serialized = serde_json::to_string(&config).expect("Failed to serialize");
    let deserialized: AppConfig = serde_json::from_str(&serialized).expect("Failed to deserialize");

    assert_eq!(config.start_hidden, deserialized.start_hidden);
    assert_eq!(config.quit_on_close, deserialized.quit_on_close);
    assert_eq!(config.hotkeys_enabled, deserialized.hotkeys_enabled);
    assert_eq!(config.dev_tools, deserialized.dev_tools);
}

#[test]
fn test_window_behavior_flags() {
    // Test that window behavior flags are properly set
    let config = AppConfig::default();

    // Default behavior: window visible, close minimizes to tray
    assert!(!config.start_hidden);
    assert!(!config.quit_on_close);

    // Test quit_on_close behavior
    let quit_config = AppConfig {
        quit_on_close: true,
        ..AppConfig::default()
    };
    assert!(quit_config.quit_on_close);

    // Test start_hidden behavior
    let hidden_config = AppConfig {
        start_hidden: true,
        ..AppConfig::default()
    };
    assert!(hidden_config.start_hidden);
}

#[test]
fn test_hotkey_configuration() {
    let config = AppConfig::default();

    // Hotkeys disabled by default
    assert!(!config.hotkeys_enabled);

    // Test enabling hotkeys
    let hotkey_config = AppConfig {
        hotkeys_enabled: true,
        ..AppConfig::default()
    };
    assert!(hotkey_config.hotkeys_enabled);
}

#[test]
fn test_dev_tools_configuration() {
    let config = AppConfig::default();

    // Dev tools disabled by default
    assert!(!config.dev_tools);

    // Test enabling dev tools
    let dev_config = AppConfig {
        dev_tools: true,
        ..AppConfig::default()
    };
    assert!(dev_config.dev_tools);
}

#[test]
fn test_window_settings_edge_cases() {
    // Test with minimum valid values
    let min_config = AppConfig {
        instance_url: "".to_string(),
        topics: "".to_string(),
        poll_rate: 5, // Minimum poll rate
        datetime_format: "".to_string(),
        start_hidden: false,
        quit_on_close: false,
        hotkeys_enabled: false,
        dev_tools: false,
        ..AppConfig::default()
    };

    assert!(!min_config.start_hidden);
    assert!(!min_config.quit_on_close);
    assert!(!min_config.hotkeys_enabled);
    assert!(!min_config.dev_tools);

    // Test with maximum valid values
    let max_config = AppConfig {
        poll_rate: 3600, // Maximum poll rate
        start_hidden: true,
        quit_on_close: true,
        hotkeys_enabled: true,
        dev_tools: true,
        ..AppConfig::default()
    };

    assert!(max_config.start_hidden);
    assert!(max_config.quit_on_close);
    assert!(max_config.hotkeys_enabled);
    assert!(max_config.dev_tools);
}

#[test]
fn test_window_settings_interaction() {
    // Test that window settings don't interfere with each other
    let config = AppConfig {
        start_hidden: true,
        quit_on_close: false, // Can start hidden but not quit on close
        hotkeys_enabled: true,
        dev_tools: false,
        ..AppConfig::default()
    };

    assert!(config.start_hidden);
    assert!(!config.quit_on_close);
    assert!(config.hotkeys_enabled);
    assert!(!config.dev_tools);

    // Test opposite combination
    let opposite_config = AppConfig {
        start_hidden: false,
        quit_on_close: true,
        hotkeys_enabled: false,
        dev_tools: true,
        ..AppConfig::default()
    };

    assert!(!opposite_config.start_hidden);
    assert!(opposite_config.quit_on_close);
    assert!(!opposite_config.hotkeys_enabled);
    assert!(opposite_config.dev_tools);
}

#[test]
fn test_window_configuration_json_stability() {
    // Test that window settings survive JSON round-trip
    let original = AppConfig {
        start_hidden: true,
        quit_on_close: false,
        hotkeys_enabled: true,
        dev_tools: false,
        welcome_completed: true,
        ..AppConfig::default()
    };

    let json = serde_json::to_string(&original).expect("Serialization failed");
    let restored: AppConfig = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(original.start_hidden, restored.start_hidden);
    assert_eq!(original.quit_on_close, restored.quit_on_close);
    assert_eq!(original.hotkeys_enabled, restored.hotkeys_enabled);
    assert_eq!(original.dev_tools, restored.dev_tools);
    assert_eq!(original.welcome_completed, restored.welcome_completed);
}

#[test]
fn test_window_settings_default_consistency() {
    // Ensure default values are consistent
    let default1 = AppConfig::default();
    let default2 = AppConfig::default();

    assert_eq!(default1.start_hidden, default2.start_hidden);
    assert_eq!(default1.quit_on_close, default2.quit_on_close);
    assert_eq!(default1.hotkeys_enabled, default2.hotkeys_enabled);
    assert_eq!(default1.dev_tools, default2.dev_tools);
    assert_eq!(default1.welcome_completed, default2.welcome_completed);
}
