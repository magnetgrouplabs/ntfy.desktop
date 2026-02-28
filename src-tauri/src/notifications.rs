use anyhow::Result;
use crate::config::NotificationSound;
use std::process::Command;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
pub struct NotificationManager;

/// Full notification data matching Electron app format
#[derive(Debug, Clone)]
pub struct NotificationData {
    pub title: String,
    pub subtitle: Option<String>,
    pub message: String,
    pub topic: String,
    pub timestamp: u64,
    pub urgent: bool,
    pub sound: NotificationSound,
    pub persistent: bool,
    pub icon_url: Option<String>,
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn show_notification(
        &self,
        title: &str,
        message: &str,
        urgent: bool,
        sound: &NotificationSound,
        persistent: bool,
    ) -> Result<()> {
        let data = NotificationData {
            title: title.to_string(),
            subtitle: None,
            message: message.to_string(),
            topic: title.to_string(),
            timestamp: 0,
            urgent,
            sound: sound.clone(),
            persistent,
            icon_url: None,
        };
        self.show_notification_full(&data).await
    }

    pub async fn show_notification_full(&self, data: &NotificationData) -> Result<()> {
        println!("DEBUG: show_notification called - title: {}", data.title);

        #[cfg(target_os = "windows")]
        {
            self.show_notification_windows(data).await?;
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let msg_escaped = data.message.replace('\\', "\\\\").replace('"', "\\\"");
            let title_escaped = data.title.replace('\\', "\\\\").replace('"', "\\\"");
            let _ = Command::new("osascript")
                .arg("-e")
                .arg(format!("display notification \"{}\" with title \"{}\"", msg_escaped, title_escaped))
                .output();
        }

        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            let mut cmd = Command::new("notify-send");
            cmd.arg(&data.title).arg(&data.message);
            if data.urgent {
                cmd.arg("-u").arg("critical");
            }
            let _ = cmd.output();
        }

        Ok(())
    }

    /// Show notification on Windows using ntfytoast.exe
    #[cfg(target_os = "windows")]
    async fn show_notification_windows(&self, data: &NotificationData) -> Result<()> {
        // Find the ntfytoast.exe path
        let ntfytoast_path = self.find_ntfytoast_path();

        let Some(exe_path) = ntfytoast_path else {
            eprintln!("ntfytoast.exe not found, falling back to notify-rust");
            return self.show_notification_fallback(data).await;
        };
        println!("DEBUG: Using ntfytoast.exe at: {}", exe_path);

        // Build command arguments
        let mut args = Vec::new();

        // Title: message title (or "New Notification"), with urgent indicator
        let display_title = if data.urgent {
            format!("🔔 {}", data.title)
        } else {
            data.title.clone()
        };
        args.push("-t".to_string());
        args.push(display_title);

        // Message body (includes topic info)
        args.push("-m".to_string());
        args.push(data.message.clone());

        // Icon (must be PNG, max 1024x1024, <= 200KB)
        // Try cached remote icon first, then fall back to local app icon
        if let Some(icon_path) = self.get_notification_icon_path(&data.icon_url).await {
            println!("DEBUG: Using notification icon: {}", icon_path);
            args.push("-p".to_string());
            args.push(icon_path);
        }

        // Sound
        match &data.sound {
            NotificationSound::None => {
                args.push("-silent".to_string());
            }
            NotificationSound::Alert => {
                args.push("-s".to_string());
                args.push("Notification.Default".to_string());
            }
            NotificationSound::Bell => {
                args.push("-s".to_string());
                args.push("Notification.IM".to_string());
            }
            NotificationSound::Chime => {
                args.push("-s".to_string());
                args.push("Notification.SMS".to_string());
            }
            NotificationSound::Pop => {
                args.push("-s".to_string());
                args.push("Notification.Looping.Alarm".to_string());
            }
            NotificationSound::Default => {
                args.push("-s".to_string());
                args.push("Notification.Default".to_string());
            }
        }

        // Persistent notifications
        if data.persistent {
            args.push("-persistent".to_string());
            args.push("true".to_string());
        }

        // Duration - use "long" for urgent notifications
        if data.urgent {
            args.push("-d".to_string());
            args.push("long".to_string());
        }

        // App ID for proper grouping
        args.push("-appID".to_string());
        args.push("com.anthony.ntfy.desktop".to_string());

        // Execute ntfytoast.exe
        println!("Executing ntfytoast.exe with args: {:?}", args);

        let output = match Command::new(&exe_path)
            .args(&args)
            .output() 
        {
            Ok(output) => output,
            Err(e) => {
                eprintln!("Failed to execute ntfytoast.exe: {}", e);
                return self.show_notification_fallback(data).await;
            }
        };

        let exit_code = output.status.code().unwrap_or(-1);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stdout.is_empty() {
            println!("ntfytoast.exe stdout: {}", stdout);
        }
        if !stderr.is_empty() {
            eprintln!("ntfytoast.exe stderr: {}", stderr);
        }

        // Exit codes:
        // 0 = Success, 1 = Hidden, 2 = Dismissed, 3 = TimedOut, 4 = ButtonPressed
        // 5 = TextEntered, -1 = Failed
        match exit_code {
            0 | 1 | 2 | 3 | 4 | 5 => {
                println!("Windows toast notification shown successfully (exit code: {})", exit_code);
            }
            _ => {
                eprintln!("ntfytoast.exe failed with exit code: {}", exit_code);
                // Fall back to notify-rust
                return self.show_notification_fallback(data).await;
            }
        }

        Ok(())
    }

    /// Fallback to notify-rust if ntfytoast.exe fails
    #[cfg(target_os = "windows")]
    async fn show_notification_fallback(&self, data: &NotificationData) -> Result<()> {
        use notify_rust::Notification;

        let timeout = if data.persistent {
            notify_rust::Timeout::Never
        } else {
            notify_rust::Timeout::Default
        };

        let display_title = if data.urgent {
            format!("🔔 {}", data.title)
        } else {
            data.title.clone()
        };

        let sound_name: Option<&str> = match &data.sound {
            NotificationSound::None => None,
            NotificationSound::Alert => Some("Notification.Default"),
            NotificationSound::Bell => Some("Notification.IM"),
            NotificationSound::Chime => Some("Notification.SMS"),
            NotificationSound::Pop => Some("Notification.Looping.Alarm"),
            NotificationSound::Default => Some("Notification.Default"),
        };

        let icon_path = if let Some(ref url) = data.icon_url {
            self.get_cached_icon_path(url).await
        } else {
            self.find_icon_path()
        };

        // Build notification using chained builder pattern
        let mut notification_builder = Notification::new();
        notification_builder
            .appname("com.anthony.ntfy.desktop")
            .summary(&display_title)
            .body(&data.message)
            .timeout(timeout);

        if let Some(ref icon) = icon_path {
            notification_builder.icon(icon);
        }

        if let Some(sound) = sound_name {
            notification_builder.sound_name(sound);
        }

        match notification_builder.show() {
            Ok(id) => println!("DEBUG: Fallback Windows toast notification shown (id: {:?})", id),
            Err(e) => eprintln!("DEBUG: Failed to show fallback notification: {:?}", e),
        }

        Ok(())
    }

    /// Find the ntfytoast.exe executable
    #[cfg(target_os = "windows")]
    fn find_ntfytoast_path(&self) -> Option<String> {
        // Check common locations for ntfytoast.exe
        let possible_paths = [
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|d| d.join("ntfytoast.exe"))),
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|d| d.join("resources").join("ntfytoast.exe"))),
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|d| d.join("..").join("resources").join("ntfytoast.exe"))),
            // Development path
            Some(std::path::PathBuf::from("C:/Users/anthony/ntfy.desktop/src-tauri/resources/ntfytoast.exe")),
        ];

        for path in possible_paths.iter().flatten() {
            if path.exists() {
                return path.to_str().map(|s| s.to_string());
            }
        }

        None
    }

    /// Find the app icon for notifications (must be PNG for ntfytoast)
    fn find_icon_path(&self) -> Option<String> {
        // ntfytoast requires PNG images (max 1024x1024, <= 200KB)
        // Prefer 128x128.png for better quality, fallback to 32x32.png
        let possible_paths = [
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|d| d.join("icons")).map(|d| d.join("128x128.png"))),
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|d| d.join("icons")).map(|d| d.join("32x32.png"))),
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|d| d.join("..").join("icons").join("128x128.png"))),
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|d| d.join("..").join("icons").join("32x32.png"))),
            // Development paths - prefer 128x128 first
            Some(std::path::PathBuf::from("C:/Users/anthony/ntfy.desktop/src-tauri/icons/128x128.png")),
            Some(std::path::PathBuf::from("C:/Users/anthony/ntfy.desktop/src-tauri/icons/32x32.png")),
        ];

        for path in possible_paths.iter().flatten() {
            if path.exists() {
                return path.to_str().map(|s| s.to_string());
            }
        }

        None
    }

    /// Download and cache remote icon for notifications
    /// Returns local path to cached icon file (resized to 128x128 PNG)
    #[cfg(target_os = "windows")]
    async fn get_cached_icon_path(&self, icon_url: &str) -> Option<String> {
        // Get cache directory
        let cache_dir = dirs::config_dir()
            .unwrap_or_else(|| std::env::temp_dir())
            .join("com.anthony.ntfy.desktop")
            .join("icons");

        // Create cache directory if it doesn't exist
        if let Err(e) = tokio::fs::create_dir_all(&cache_dir).await {
            eprintln!("DEBUG: Failed to create icon cache directory: {}", e);
            return None;
        }

        // Generate cache filename from URL hash
        let url_hash = format!("{:x}", Sha256::digest(icon_url.as_bytes()));
        let cache_path = cache_dir.join(format!("{}.png", &url_hash[..16]));

        // Check if already cached AND not expired (7 days TTL)
        if cache_path.exists() {
            let use_cached = if let Ok(metadata) = tokio::fs::metadata(&cache_path).await {
                if let Ok(modified) = metadata.modified() {
                    let age = std::time::SystemTime::now()
                        .duration_since(modified)
                        .unwrap_or_default();
                    if age.as_secs() < 7 * 24 * 60 * 60 { // 7 days
                        true
                    } else {
                        println!("DEBUG: Cached icon expired (age: {} days), re-downloading", age.as_secs() / 86400);
                        false
                    }
                } else {
                    true // Can't get modified time, assume valid
                }
            } else {
                true // Can't get metadata, assume valid
            };

            if use_cached {
                println!("DEBUG: Using cached icon: {}", cache_path.display());
                return cache_path.to_str().map(|s| s.to_string());
            }
        }

        // Download the icon
        println!("DEBUG: Downloading icon from: {}", icon_url);
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                
                // Safe header parsing with fallbacks
                if let Ok(accept_header) = "image/avif,image/webp,image/apng,image/svg+xml,image/*,*/*;q=0.8".parse() {
                    headers.insert(reqwest::header::ACCEPT, accept_header);
                } else {
                    eprintln!("WARNING: Failed to parse Accept header, using default");
                }
                
                if let Ok(referer_header) = "https://ntfy.sh/".parse() {
                    headers.insert(reqwest::header::REFERER, referer_header);
                } else {
                    eprintln!("WARNING: Failed to parse Referer header, using default");
                }
                
                if let Ok(language_header) = "en-US,en;q=0.9".parse() {
                    headers.insert(reqwest::header::ACCEPT_LANGUAGE, language_header);
                } else {
                    eprintln!("WARNING: Failed to parse Accept-Language header, using default");
                }
                
                headers
            })
            .build()
            .ok()?;

        let response = match client.get(icon_url).send().await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("DEBUG: Failed to download icon from {}: {}", icon_url, e);
                // Check if it's a 403 (blocked) vs network error vs timeout
                if e.is_status() {
                    if let Some(status) = e.status() {
                        eprintln!("DEBUG: HTTP status: {}", status);
                        if status == 403 {
                            eprintln!("DEBUG: Server may be blocking requests without proper User-Agent or referrer");
                        }
                    }
                }
                return None;
            }
        };

        if !response.status().is_success() {
            eprintln!("DEBUG: Icon download failed with status: {}", response.status());
            return None;
        }

        // Read response bytes
        let bytes = match response.bytes().await {
            Ok(b) => b,
            Err(e) => {
                eprintln!("DEBUG: Failed to read icon bytes: {}", e);
                return None;
            }
        };

        // Process and resize image to 128x128 PNG
        let processed = match self.process_icon_image(&bytes).await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("DEBUG: Failed to process icon image: {}", e);
                return None;
            }
        };

        // Save to cache
        if let Err(e) = tokio::fs::write(&cache_path, processed).await {
            eprintln!("DEBUG: Failed to write cached icon: {}", e);
            return None;
        }

        println!("DEBUG: Cached icon saved to: {}", cache_path.display());
        cache_path.to_str().map(|s| s.to_string())
    }

    /// Process icon image: resize to 128x128, convert to PNG
    /// Returns processed image bytes
    async fn process_icon_image(&self, image_bytes: &[u8]) -> Result<Vec<u8>> {
        // Use blocking task for image processing
        let image_bytes = image_bytes.to_vec();
        let processed = tokio::task::spawn_blocking(move || -> Result<Vec<u8>> {
            // Load image using image crate
            let img = image::load_from_memory(&image_bytes)
                .map_err(|e| anyhow::anyhow!("Failed to load image from memory: {}", e))?;

            // Resize to 128x128 using Lanczos3 for quality
            let resized = img.resize(128, 128, image::imageops::FilterType::Lanczos3);

            // Encode as PNG
            let mut png_bytes: Vec<u8> = Vec::new();
            let mut cursor = std::io::Cursor::new(&mut png_bytes);
            resized.write_to(&mut cursor, image::ImageFormat::Png)
                .map_err(|e| anyhow::anyhow!("Failed to encode image as PNG: {}", e))?;

            // Check size (ntfytoast requires <= 200KB)
            if png_bytes.len() > 200 * 1024 {
                eprintln!("Warning - icon size {} bytes exceeds 200KB limit", png_bytes.len());
            }

            Ok(png_bytes)
        }).await
        .map_err(|e| anyhow::anyhow!("Image processing task failed: {}", e))??;

        Ok(processed)
    }

    /// Get the best available icon for notification
    /// 1. Try cached remote icon if icon_url provided
    /// 2. Fall back to local app icon
    #[cfg(target_os = "windows")]
    async fn get_notification_icon_path(&self, icon_url: &Option<String>) -> Option<String> {
        // Try remote icon if provided
        if let Some(url) = icon_url {
            if let Some(cached) = self.get_cached_icon_path(url).await {
                return Some(cached);
            }
        }

        // Fall back to local app icon
        self.find_icon_path()
    }
}

pub async fn show_notification(
    title: &str,
    message: &str,
    urgent: bool,
    sound: &NotificationSound,
    persistent: bool,
) -> Result<()> {
    let manager = NotificationManager::new();
    manager.show_notification(title, message, urgent, sound, persistent).await
}
