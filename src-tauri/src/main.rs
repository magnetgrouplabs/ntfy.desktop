// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod credentials;
mod notifications;
mod ntfy;
mod performance;

/// Initialize Windows notification registry for unpackaged apps.
/// This is required for toast notifications to work when running directly from exe.
#[cfg(target_os = "windows")]
fn init_windows_notification_registry() -> anyhow::Result<()> {
    use windows_registry::CURRENT_USER;

    const APP_ID: &str = "com.anthony.ntfy.desktop";
    const APP_NAME: &str = "ntfy.desktop";

    println!("Initializing Windows notification registry for AppUserModelID: {}", APP_ID);

    let key_path = format!(r"SOFTWARE\Classes\AppUserModelId\{APP_ID}");

    // Create the registry key (handles nested path creation)
    let key = CURRENT_USER.create(&key_path).map_err(|e| {
        eprintln!("ERROR: Failed to create registry key '{}': {}", key_path, e);
        e
    })?;

    // Set DisplayName
    key.set_string("DisplayName", APP_NAME).map_err(|e| {
        eprintln!("ERROR: Failed to set DisplayName in registry: {}", e);
        e
    })?;

    // Set IconBackgroundColor
    key.set_string("IconBackgroundColor", "0").map_err(|e| {
        eprintln!("ERROR: Failed to set IconBackgroundColor in registry: {}", e);
        e
    })?;

    // Try to set IconUri - this helps Windows find the app icon for notifications
    // Look for icon in common locations
    let icon_paths = [
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("icon.ico"))),
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("icons")).map(|d| d.join("icon.ico"))),
    ];

    for icon_path in icon_paths.iter().flatten() {
        if icon_path.exists() {
            if let Some(icon_str) = icon_path.to_str() {
                match key.set_string("IconUri", icon_str) {
                    Ok(_) => {
                        println!("Set notification icon to: {}", icon_str);
                        break;
                    }
                    Err(e) => {
                        eprintln!("WARNING: Failed to set IconUri in registry: {}", e);
                    }
                }
            }
        }
    }

    println!("Windows notification registry initialized successfully");
    Ok(())
}

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

/// Shared config state for runtime updates (accessible from both Tauri commands and polling)
struct SharedConfig(Arc<Mutex<config::AppConfig>>);

// ── Tauri Commands ──────────────────────────────────────────────────────────

#[tauri::command]
async fn save_config(
    config: config::AppConfig,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    // Save credentials to OS keychain (not to disk)
    let creds = credentials::Credentials {
        api_token: config.api_token.clone(),
        auth_user: config.auth_user.clone(),
        auth_pass: config.auth_pass.clone(),
    };
    credentials::save_credentials(&creds).map_err(|e| e.to_string())?;

    // Save non-sensitive config to disk (credentials are #[serde(skip)])
    config::save_config(&app_handle, config.clone())
        .await
        .map_err(|e| e.to_string())?;

    // Update shared runtime config so polling picks up changes immediately
    if let Some(shared) = app_handle.try_state::<SharedConfig>() {
        let mut locked = shared.0.lock().await;
        *locked = config;
    }

    Ok(())
}

#[tauri::command]
async fn load_config(app_handle: tauri::AppHandle) -> Result<config::AppConfig, String> {
    let mut config = config::load_config(&app_handle)
        .await
        .map_err(|e| e.to_string())?;

    // Merge credentials from OS keychain
    match credentials::load_credentials() {
        Ok(creds) => {
            if !creds.api_token.is_empty() {
                println!("load_config: Loaded API token from keychain ({} chars)", creds.api_token.len());
                config.api_token = creds.api_token;
            }
            if !creds.auth_user.is_empty() {
                println!("load_config: Loaded auth user from keychain: {}", creds.auth_user);
                config.auth_user = creds.auth_user;
            }
            if !creds.auth_pass.is_empty() {
                println!("load_config: Loaded auth password from keychain ({} chars)", creds.auth_pass.len());
                config.auth_pass = creds.auth_pass;
            }
        }
        Err(e) => {
            eprintln!("load_config: Failed to load credentials from keychain: {}", e);
        }
    }

    Ok(config)
}

#[tauri::command]
async fn show_notification(
    title: String,
    message: String,
    urgent: Option<bool>,
    sound: Option<String>,
    persistent: Option<bool>,
) -> Result<(), String> {
    use crate::config::NotificationSound;

    let sound = match sound.as_deref() {
        Some("none") => NotificationSound::None,
        Some("alert") => NotificationSound::Alert,
        Some("bell") => NotificationSound::Bell,
        Some("chime") => NotificationSound::Chime,
        Some("pop") => NotificationSound::Pop,
        _ => NotificationSound::Default,
    };

    notifications::show_notification(
        &title,
        &message,
        urgent.unwrap_or(false),
        &sound,
        persistent.unwrap_or(false),
    )
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn preview_notification_sound(sound: String, urgent: Option<bool>) -> Result<(), String> {
    println!("preview_notification_sound called: sound={}, urgent={:?}", sound, urgent);
    use crate::config::NotificationSound;

    let sound = match sound.as_str() {
        "none" => NotificationSound::None,
        "alert" => NotificationSound::Alert,
        "bell" => NotificationSound::Bell,
        "chime" => NotificationSound::Chime,
        "pop" => NotificationSound::Pop,
        _ => NotificationSound::Default,
    };

    let is_urgent = urgent.unwrap_or(false);
    let title = if is_urgent {
        "🔔 Urgent Notification"
    } else {
        "🔔 Test Notification"
    };

    println!("Showing notification with sound: {:?}", sound);

    match notifications::show_notification(
        title,
        "This is a preview of your selected sound",
        is_urgent,
        &sound,
        false,
    )
    .await
    {
        Ok(_) => {
            println!("Notification shown successfully");
            Ok(())
        }
        Err(e) => {
            println!("Failed to show notification: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
async fn get_memory_usage() -> Result<u64, String> {
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    {
        use sysinfo::System;
        let mut system = System::new_all();
        system.refresh_all();
        Ok(system.used_memory())
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Ok(0)
    }
}

#[tauri::command]
async fn get_performance_metrics() -> Result<serde_json::Value, String> {
    let mut monitor = performance::PerformanceMonitor::new();
    let metrics = monitor.get_metrics();
    serde_json::to_value(metrics).map_err(|e| e.to_string())
}

#[tauri::command]
async fn test_ntfy_connection(
    server_url: String,
    topic: String,
    api_token: Option<String>,
    auth_user: Option<String>,
    auth_pass: Option<String>,
) -> Result<bool, String> {
    let mut client = NtfyClient::new(&server_url);
    if let Some(token) = api_token.filter(|t| !t.is_empty()) {
        client = client.with_token(token);
    } else if let Some(user) = auth_user.filter(|u| !u.is_empty()) {
        client = client.with_basic_auth(user, auth_pass.unwrap_or_default());
    }
    client
        .test_connection(&topic)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn navigate_to(url: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    let window = app_handle
        .get_webview_window("main")
        .ok_or("Main window not found")?;

    let url = url
        .parse::<tauri::Url>()
        .map_err(|e| format!("Invalid URL: {}", e))?;

    window.navigate(url).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn complete_welcome(app_handle: tauri::AppHandle) -> Result<(), String> {
    let mut config = config::load_config(&app_handle)
        .await
        .map_err(|e| e.to_string())?;

    // Merge credentials from keychain (save_config already stored them there)
    if let Ok(creds) = credentials::load_credentials() {
        config.api_token = creds.api_token;
        config.auth_user = creds.auth_user;
        config.auth_pass = creds.auth_pass;
    }

    config.welcome_completed = true;

    config::save_config(&app_handle, config.clone())
        .await
        .map_err(|e| e.to_string())?;

    // Navigate main window to the configured instance URL
    if let Some(window) = app_handle.get_webview_window("main") {
        if let Ok(url) = config.instance_url.parse::<tauri::Url>() {
            let _ = window.navigate(url);
        }
        let _ = window.show();
        let _ = window.set_focus();
    }

    // Update shared runtime config so polling picks up the new settings
    if let Some(shared) = app_handle.try_state::<SharedConfig>() {
        let mut locked = shared.0.lock().await;
        *locked = config;
    }

    // Close the welcome window from Rust (window.close() is unreliable in Tauri webviews)
    if let Some(welcome_win) = app_handle.get_webview_window("welcome") {
        let _ = welcome_win.close();
    }

    Ok(())
}

#[tauri::command]
async fn close_window(label: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(win) = app_handle.get_webview_window(&label) {
        win.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn test_notification() -> Result<(), String> {
    println!("Test notification command called");

    use crate::config::NotificationSound;

    match notifications::show_notification(
        "Test Notification",
        "This is a test notification from ntfy.desktop",
        false,
        &NotificationSound::Default,
        false,
    )
    .await
    {
        Ok(_) => {
            println!("Test notification shown successfully");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to show test notification: {}", e);
            Err(e.to_string())
        }
    }
}

// ── Application Entry Point ─────────────────────────────────────────────────

fn main() -> anyhow::Result<()> {
    // Initialize Windows notification registry (required for unpackaged apps)
    // and set the AppUserModelID for the process
    #[cfg(target_os = "windows")]
    {
        if let Err(e) = init_windows_notification_registry() {
            eprintln!("Failed to init notification registry: {}", e);
        }

        use windows::Win32::UI::Shell::SetCurrentProcessExplicitAppUserModelID;
        unsafe {
            let result = SetCurrentProcessExplicitAppUserModelID(
                windows::core::w!("com.anthony.ntfy.desktop")
            );
            match result {
                Ok(_) => println!("AppUserModelID set successfully"),
                Err(e) => eprintln!("Failed to set AppUserModelID: {:?}", e),
            }
        }
    }

    // Parse CLI args
    let args: Vec<String> = std::env::args().collect();
    let start_hidden = args.contains(&"--hidden".to_string());
    let enable_devtools = args.contains(&"--devtools".to_string());

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("Another instance tried to run with args: {:?}, cwd: {:?}", argv, cwd);
            
            // Use a background thread to avoid blocking and allow window creation to complete
            let app_handle = app.clone();
            std::thread::spawn(move || {
                // Wait briefly for the main window to be created
                std::thread::sleep(std::time::Duration::from_millis(100));
                
                // Focus existing window - with error handling to prevent crashes
                match app_handle.get_webview_window("main") {
                    Some(window) => {
                        if let Err(e) = window.set_focus() {
                            eprintln!("Failed to focus window: {}", e);
                        }
                        if let Err(e) = window.show() {
                            eprintln!("Failed to show window: {}", e);
                        }
                        println!("Focused existing window");
                    }
                    None => {
                        eprintln!("Window 'main' not found when trying to focus existing instance - window may not be initialized yet");
                    }
                }
            });
        }))
        .invoke_handler(tauri::generate_handler![
            save_config,
            load_config,
            show_notification,
            preview_notification_sound,
            get_memory_usage,
            get_performance_metrics,
            test_ntfy_connection,
            navigate_to,
            complete_welcome,
            close_window,
            test_notification
        ])
        .setup(move |app| {
            let app_handle = app.handle().clone();

            // Load config synchronously (setup runs before async runtime is available)
            let config = load_config_sync(&app_handle);

            let should_hide = start_hidden || config.start_hidden;
            let quit_on_close = config.quit_on_close;
            let instance_url = config.instance_url.clone();
            let show_welcome = !config.welcome_completed;

            // Shared config registered as managed state so both polling
            // and save_config use the same instance
            let shared_config = Arc::new(Mutex::new(config.clone()));
            app.manage(SharedConfig(shared_config.clone()));
            let is_polling = Arc::new(AtomicBool::new(false));
            let badge_count = Arc::new(AtomicU32::new(0));

            // ── Create Main Window ──────────────────────────────────────

            let webview_url = WebviewUrl::External(
                instance_url
                    .parse()
                    .unwrap_or_else(|_| {
                        eprintln!("Failed to parse instance URL: {}, using fallback", instance_url);
                        "https://ntfy.sh/app".parse().unwrap_or_else(|_| {
                            eprintln!("Failed to parse fallback URL, using hardcoded URL");
                            tauri::Url::parse("https://ntfy.sh/app").unwrap_or_else(|_| {
                                eprintln!("All URL parsing attempts failed, using default URL");
                                // Use a known-good URL that should always work
                                tauri::Url::parse("https://ntfy.sh/app").expect("Hardcoded URL should always parse")
                            })
                        })
                    }),
            );

            let window = tauri::WebviewWindowBuilder::new(app, "main", webview_url)
                .title("ntfy.desktop")
                .inner_size(1280.0, 720.0)
                .min_inner_size(400.0, 300.0)
                .visible(!should_hide && !show_welcome)
                .center()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.0")
                .initialization_script(
                    // Stub browser Notification API as a no-op class.
                    // ntfy.desktop delivers notifications natively via the Rust backend,
                    // so we suppress web notifications to avoid duplicates while also
                    // preventing the "Notifications are blocked" banner in the ntfy web UI.
                    r#"
                    (function() {
                        function NoopNotification(title, options) {
                            this.title = title;
                            this.body = (options && options.body) || "";
                            this.onclick = null;
                            this.onclose = null;
                            this.onerror = null;
                            this.onshow = null;
                        }
                        NoopNotification.permission = "granted";
                        NoopNotification.requestPermission = function(cb) {
                            var p = Promise.resolve("granted");
                            if (cb) cb("granted");
                            return p;
                        };
                        NoopNotification.prototype.close = function() {};
                        NoopNotification.prototype.addEventListener = function() {};
                        NoopNotification.prototype.removeEventListener = function() {};
                        window.Notification = NoopNotification;
                        console.log("ntfy.desktop: Notification API stubbed");
                    })();
                    "#,
                )
                .build()?;

            // Open devtools if --devtools flag was passed (debug builds only)
            #[cfg(debug_assertions)]
            if enable_devtools {
                window.open_devtools();
            }

            // ── Create Welcome Window ───────────────────────────────────

            if show_welcome {
                let welcome_window = tauri::WebviewWindowBuilder::new(
                    app,
                    "welcome",
                    tauri::WebviewUrl::App("/welcome.html".into()),
                )
                .title("Welcome to ntfy.desktop")
                .inner_size(500.0, 600.0)
                .resizable(false)
                .decorations(true)
                .build()?;

                // Make welcome window modal-style
                welcome_window.set_always_on_top(true)?;
                welcome_window.center()?;

                // When welcome window closes and main window is hidden, show it
                let window_for_welcome = window.clone();
                welcome_window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { .. } = event {
                        if !window_for_welcome.is_visible().unwrap_or(false) {
                            let _ = window_for_welcome.show();
                            let _ = window_for_welcome.set_focus();
                        }
                    }
                });
            }

            // ── Close-to-tray behavior ──────────────────────────────────

            let window_for_close = window.clone();
            let quit_flag = Arc::new(AtomicBool::new(false));
            let quit_flag_close = quit_flag.clone();

            window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    if !quit_on_close && !quit_flag_close.load(Ordering::SeqCst) {
                        api.prevent_close();
                        let _ = window_for_close.hide();
                    }
                }
            });

            // ── System Tray ─────────────────────────────────────────────

            let show_item = MenuItemBuilder::with_id("show", "Show App").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let tray_menu = MenuBuilder::new(app)
                .item(&show_item)
                .separator()
                .item(&quit_item)
                .build()?;

            let icon = load_tray_icon();

            let window_for_tray = window.clone();
            let quit_flag_tray = quit_flag.clone();
            let badge_count_tray = badge_count.clone();

            let _tray = TrayIconBuilder::new()
                .icon(icon)
                .menu(&tray_menu)
                .tooltip("ntfy.desktop")
                .on_tray_icon_event(move |_tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        if window_for_tray.is_visible().unwrap_or(false) {
                            let _ = window_for_tray.hide();
                        } else {
                            let _ = window_for_tray.show();
                            let _ = window_for_tray.set_focus();
                            badge_count_tray.store(0, Ordering::SeqCst);
                        }
                    }
                })
                .on_menu_event(
                    move |app: &tauri::AppHandle, event: tauri::menu::MenuEvent| match event
                        .id()
                        .as_ref()
                    {
                        "show" => {
                            if let Some(win) = app.get_webview_window("main") {
                                let _ = win.show();
                                let _ = win.set_focus();
                            }
                        }
                        "quit" => {
                            quit_flag_tray.store(true, Ordering::SeqCst);
                            app.exit(0);
                        }
                        _ => {}
                    },
                )
                .build(app)?;

            // ── Application Menu Bar ────────────────────────────────────────

            let app_menu = create_app_menu(&app_handle)?;
            app_menu.set_as_app_menu()?;

            // Handle menu events
            app.on_menu_event(move |app, event| {
                handle_menu_event(app, event);
            });

            // ── Start Background Polling ─────────────────────────────────

            let base_url = config.api_base_url();
            let mut client = NtfyClient::new(&base_url);
            if !config.api_token.is_empty() {
                client = client.with_token(config.api_token.clone());
            } else if !config.auth_user.is_empty() {
                client = client.with_basic_auth(config.auth_user.clone(), config.auth_pass.clone());
            }
            let client = Arc::new(Mutex::new(client));

            let nm = Arc::new(Mutex::new(NotificationManager::new()));
            let config_for_poll = shared_config.clone();
            let polling_flag = is_polling.clone();
            let app_handle_poll = app_handle.clone();

            tauri::async_runtime::spawn(async move {
                ntfy::start_polling(app_handle_poll, client, nm, config_for_poll, polling_flag)
                    .await;
            });

            // ── Badge count listener ─────────────────────────────────────

            app.listen("badge-update", move |event| {
                if let Ok(count) = serde_json::from_str::<u32>(event.payload()) {
                    badge_count.fetch_add(count, Ordering::SeqCst);
                }
            });

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_app_handle, event| {
        if let RunEvent::ExitRequested { api, code, .. } = event {
            // Only prevent exit if code is None (window close button)
            // Allow exit if code is Some (explicit quit command)
            if code.is_none() {
                api.prevent_exit();
            }
        }
    });

    Ok(())
}

/// Load config synchronously from disk, merging credentials from OS keychain
fn load_config_sync(app_handle: &tauri::AppHandle) -> config::AppConfig {
    let app_dir = match app_handle.path().app_config_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to get app config directory: {}", e);
            return config::AppConfig::default();
        },
    };

    let config_path = app_dir.join("prefs.json");

    let mut config = if !config_path.exists() {
        println!("No prefs.json found, using default config");
        config::AppConfig::default()
    } else {
        match std::fs::read_to_string(&config_path) {
            Ok(data) => {
                println!("Loaded config from: {:?}", config_path);
                match serde_json::from_str(&data) {
                    Ok(config) => config,
                    Err(e) => {
                        eprintln!("Failed to parse config file {}, using default: {}", config_path.display(), e);
                        config::AppConfig::default()
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read config file {}: {}", config_path.display(), e);
                config::AppConfig::default()
            }
        }
    };

    // Merge credentials from OS keychain
    match credentials::load_credentials() {
        Ok(creds) => {
            if !creds.api_token.is_empty() {
                println!("Loaded API token from keychain ({} chars)", creds.api_token.len());
                config.api_token = creds.api_token;
            }
            if !creds.auth_user.is_empty() {
                println!("Loaded auth user from keychain: {}", creds.auth_user);
                config.auth_user = creds.auth_user;
            }
            if !creds.auth_pass.is_empty() {
                println!("Loaded auth password from keychain ({} chars)", creds.auth_pass.len());
                config.auth_pass = creds.auth_pass;
            }
        }
        Err(e) => {
            eprintln!("Failed to load credentials from keychain: {}", e);
        }
    }

    config
}

/// Create the application menu bar
fn create_app_menu(app: &tauri::AppHandle) -> Result<tauri::menu::Menu<tauri::Wry>, tauri::Error> {
    use tauri::menu::{MenuItemBuilder, SubmenuBuilder};

    // ── File Menu ──────────────────────────────────────────────────────────

    let test_notification_item =
        MenuItemBuilder::with_id("test-notification", "Test Notification").build(app)?;

    let quit_item = MenuItemBuilder::with_id("quit", "Quit")
        .accelerator(if cfg!(target_os = "macos") {
            "Cmd+Q"
        } else {
            "Ctrl+Q"
        })
        .build(app)?;

    let file_menu = SubmenuBuilder::new(app, "File")
        .item(&test_notification_item)
        .separator()
        .item(&quit_item)
        .build()?;

    // ── View Menu ──────────────────────────────────────────────────────────

    let back_item = MenuItemBuilder::with_id("back", "Back")
        .accelerator(if cfg!(target_os = "macos") {
            "Cmd+["
        } else {
            "Alt+Left"
        })
        .build(app)?;

    let forward_item = MenuItemBuilder::with_id("forward", "Forward")
        .accelerator(if cfg!(target_os = "macos") {
            "Cmd+]"
        } else {
            "Alt+Right"
        })
        .build(app)?;

    let reload_item = MenuItemBuilder::with_id("reload", "Reload")
        .accelerator(if cfg!(target_os = "macos") {
            "Cmd+R"
        } else {
            "Ctrl+R"
        })
        .build(app)?;

    let fullscreen_item = MenuItemBuilder::with_id("fullscreen", "Fullscreen")
        .accelerator(if cfg!(target_os = "macos") {
            "Cmd+Ctrl+F"
        } else {
            "F11"
        })
        .build(app)?;

    let view_menu = SubmenuBuilder::new(app, "View")
        .item(&back_item)
        .item(&forward_item)
        .item(&reload_item)
        .separator()
        .item(&fullscreen_item)
        .build()?;

    // ── Settings Menu ──────────────────────────────────────────────────────

    let instance_settings_item =
        MenuItemBuilder::with_id("settings-instance", "Instance URL").build(app)?;
    let auth_item = MenuItemBuilder::with_id("settings-token", "Authorization").build(app)?;
    let notification_settings_item =
        MenuItemBuilder::with_id("settings-notifications", "Notification Settings").build(app)?;
    let general_settings_item =
        MenuItemBuilder::with_id("settings-general", "General").build(app)?;

    let settings_menu = SubmenuBuilder::new(app, "Settings")
        .item(&instance_settings_item)
        .item(&auth_item)
        .item(&notification_settings_item)
        .item(&general_settings_item)
        .build()?;

    // ── Build Main Menu ────────────────────────────────────────────────────

    use tauri::menu::MenuBuilder;
    let app_menu = MenuBuilder::new(app)
        .item(&file_menu)
        .item(&view_menu)
        .item(&settings_menu)
        .build()?;

    Ok(app_menu)
}

/// Open settings window with specific page
fn open_settings_window(app: &tauri::AppHandle, page: &str) -> Result<(), tauri::Error> {
    let window_label = format!("settings-{}", page);

    // Check if window already exists
    if let Some(existing_window) = app.get_webview_window(&window_label) {
        let _ = existing_window.show();
        let _ = existing_window.set_focus();
        return Ok(());
    }

    let settings_url = format!("/settings.html?page={}", page);

    tauri::WebviewWindowBuilder::new(
        app,
        &window_label,
        tauri::WebviewUrl::App(settings_url.into()),
    )
    .title(format!("Settings - {}", page))
    .inner_size(500.0, 600.0)
    .resizable(false)
    .build()?;

    Ok(())
}

/// Handle menu events
fn handle_menu_event(app: &tauri::AppHandle, event: tauri::menu::MenuEvent) {
    match event.id().as_ref() {
        // File menu
        "test-notification" => {
            println!("Menu: Test notification clicked");
            tauri::async_runtime::spawn(async move {
                if let Err(e) = crate::notifications::show_notification(
                    "Test Notification",
                    "This is a test notification from ntfy.desktop",
                    false,
                    &crate::config::NotificationSound::Default,
                    false,
                )
                .await
                {
                    eprintln!("Failed to show test notification from menu: {}", e);
                }
            });
        }
        "quit" => {
            app.exit(0);
        }

        // View menu
        "back" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.eval("window.history.back()");
            }
        }
        "forward" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.eval("window.history.forward()");
            }
        }
        "reload" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.reload();
            }
        }
        "fullscreen" => {
            if let Some(window) = app.get_webview_window("main") {
                let is_fullscreen = window.is_fullscreen().unwrap_or(false);
                let _ = window.set_fullscreen(!is_fullscreen);
            }
        }

        // Settings menu
        "settings-instance" => {
            let _ = open_settings_window(app, "instance");
        }
        "settings-token" => {
            let _ = open_settings_window(app, "token");
        }
        "settings-notifications" => {
            let _ = open_settings_window(app, "notifications");
        }
        "settings-general" => {
            let _ = open_settings_window(app, "general");
        }

        _ => {}
    }
}

/// Load the tray icon from embedded bytes
fn load_tray_icon() -> Image<'static> {
    let ico_bytes = include_bytes!("../icons/icon.ico");
    Image::from_bytes(ico_bytes).unwrap_or_else(|e| {
        eprintln!("Failed to load tray icon: {}, using fallback", e);
        Image::new_owned(vec![0u8; 4], 1, 1)
    })
}
