use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use super::config::AppConfig;
use super::notifications::NotificationManager;

/// Raw message from ntfy NDJSON response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NtfyMessage {
    pub id: Option<String>,
    #[serde(default)]
    pub time: u64,
    #[serde(default)]
    pub event: Option<String>,
    #[serde(default)]
    pub topic: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    pub priority: Option<u8>,
    pub tags: Option<Vec<String>>,
    pub click: Option<String>,
    pub actions: Option<Vec<NtfyAction>>,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NtfyAction {
    pub action: String,
    pub label: String,
    pub url: Option<String>,
    pub clear: Option<bool>,
}

pub struct NtfyClient {
    client: Client,
    pub(crate) base_url: String,
    pub(crate) api_token: Option<String>,
    pub(crate) auth_user: Option<String>,
    pub(crate) auth_pass: Option<String>,
}

impl NtfyClient {
    pub fn new(base_url: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_else(|e| {
                eprintln!("Failed to build HTTP client with timeout, using default: {}", e);
                Client::new()
            });

        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            api_token: None,
            auth_user: None,
            auth_pass: None,
        }
    }

    pub fn with_token(mut self, token: String) -> Self {
        if !token.is_empty() {
            self.api_token = Some(token);
        }
        self
    }

    pub fn with_basic_auth(mut self, user: String, pass: String) -> Self {
        if !user.is_empty() {
            self.auth_user = Some(user);
            self.auth_pass = Some(pass);
        }
        self
    }

    /// Apply authentication to a request builder
    fn apply_auth(&self, mut request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(token) = &self.api_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        } else if let Some(user) = &self.auth_user {
            let pass = self.auth_pass.as_deref().unwrap_or("");
            request = request.basic_auth(user, Some(pass));
        }
        request
    }

    /// Poll for messages using the original ntfy-desktop URL format:
    /// GET {base_url}/{topics}/json?since={poll_rate}s&poll=1
    pub async fn poll_messages(&self, topics: &str, poll_rate: u64) -> Result<Vec<NtfyMessage>> {
        let url = format!(
            "{}/{}/json?since={}s&poll=1",
            self.base_url, topics, poll_rate
        );

        let request = self.apply_auth(self.client.get(&url));

        let response = request.send().await?;
        let status = response.status();

        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(anyhow::anyhow!("Unauthorized (401) - check your credentials"));
        }

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(anyhow::anyhow!("Rate limited (429) - polling too fast"));
        }

        if !status.is_success() {
            return Err(anyhow::anyhow!("HTTP error: {}", status));
        }

        // ntfy returns newline-delimited JSON (NDJSON), not a JSON array
        let body = response.text().await?;
        let mut messages = Vec::new();

        for line in body.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match serde_json::from_str::<NtfyMessage>(line) {
                Ok(msg) => {
                    // Only include actual messages, skip keepalive/open events
                    if msg.event.as_deref() == Some("message") {
                        messages.push(msg);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse ntfy message: {} (line: {})", e, line);
                }
            }
        }

        Ok(messages)
    }

    /// Test connection to the ntfy instance
    pub async fn test_connection(&self, topic: &str) -> Result<bool> {
        let url = format!("{}/{}/json?poll=1&since=0", self.base_url, topic);
        let request = self.apply_auth(self.client.get(&url));

        match request.send().await {
            Ok(response) => {
                let status = response.status();
                if status.is_success() {
                    Ok(true)
                } else {
                    Err(anyhow::anyhow!(
                        "HTTP {}: {}",
                        status,
                        response.text().await?
                    ))
                }
            }
            Err(e) => Err(anyhow::anyhow!("Connection failed: {}", e)),
        }
    }
}

/// Main polling loop that runs in the background
pub async fn start_polling(
    app_handle: AppHandle,
    client: Arc<Mutex<NtfyClient>>,
    notification_manager: Arc<Mutex<NotificationManager>>,
    config: Arc<Mutex<AppConfig>>,
    is_polling: Arc<AtomicBool>,
) {
    let mut seen_ids: HashSet<String> = HashSet::new();

    // Cleanup interval - clear seen_ids every hour to prevent memory leak
    let cleanup_interval = Duration::from_secs(3600);
    let mut last_cleanup = tokio::time::Instant::now();

    loop {
        // Guard against concurrent polling
        if is_polling.swap(true, Ordering::SeqCst) {
            sleep(Duration::from_secs(1)).await;
            continue;
        }

        let (
            topics_path,
            poll_rate,
            api_token,
            auth_user,
            auth_pass,
            base_url,
            urgent_threshold,
            notification_sound,
            urgent_notification_sound,
            persistent_notifications_mode,
        ) = {
            let cfg = config.lock().await;
            (
                cfg.topics_path(),
                cfg.effective_poll_rate(),
                cfg.api_token.clone(),
                cfg.auth_user.clone(),
                cfg.auth_pass.clone(),
                cfg.api_base_url(),
                cfg.urgent_priority_threshold,
                cfg.notification_sound.clone(),
                cfg.urgent_notification_sound.clone(),
                cfg.persistent_notifications_mode.clone(),
            )
        };

        if topics_path.is_empty() {
            is_polling.store(false, Ordering::SeqCst);
            sleep(Duration::from_secs(5)).await;
            continue;
        }

        // Update client auth and base_url if changed
        {
            let mut client_lock = client.lock().await;
            client_lock.base_url = base_url;
            client_lock.api_token = if !api_token.is_empty() { Some(api_token) } else { None };
            client_lock.auth_user = if !auth_user.is_empty() { Some(auth_user) } else { None };
            client_lock.auth_pass = if !auth_pass.is_empty() { Some(auth_pass) } else { None };
        }

        let result = {
            let client_lock = client.lock().await;
            client_lock.poll_messages(&topics_path, poll_rate).await
        };

        match result {
            Ok(messages) => {
                let mut new_count = 0u32;
                for msg in messages {
                    let msg_id = match &msg.id {
                        Some(id) => id.clone(),
                        None => continue,
                    };

                    if seen_ids.contains(&msg_id) {
                        continue;
                    }

                    seen_ids.insert(msg_id);
                    new_count += 1;

                    // Format notification like Electron app:
                    // title: "{topic} - {date}" (or "{topic} - {msg_title}" if available)
                    // message: the actual message
                    let topic = msg.topic.clone().unwrap_or_else(|| "ntfy".to_string());
                    let message_body = msg.message.clone().unwrap_or_default();
                    let msg_title = msg.title.clone();

                    // Format date like Electron app: "YYYY-MM-DD hh:mm a"
                    let datetime = chrono::DateTime::from_timestamp(msg.time as i64, 0)
                        .map(|dt| dt.format("%Y-%m-%d %I:%M %p").to_string())
                        .unwrap_or_else(|| "now".to_string());

                    // Title: use message title if available, otherwise "New Notification"
                    // Topic is shown separately in the message body
                    let notification_title = if let Some(ref title) = msg_title {
                        if !title.is_empty() {
                            title.clone()
                        } else {
                            "New Notification".to_string()
                        }
                    } else {
                        "New Notification".to_string()
                    };

                    // Show native OS notification
                    let priority = msg.priority.unwrap_or(3);
                    let urgent = priority >= urgent_threshold;
                    let sound = if urgent {
                        &urgent_notification_sound
                    } else {
                        &notification_sound
                    };
                    let persistent = match persistent_notifications_mode {
                        crate::config::PersistentNotificationMode::Off => false,
                        crate::config::PersistentNotificationMode::All => true,
                        crate::config::PersistentNotificationMode::UrgentOnly => urgent,
                    };

                    // Format message with topic at the end
                    let formatted_message = if message_body.is_empty() {
                        format!("Topic: {}", topic)
                    } else {
                        format!("{}\n\nTopic: {}", message_body, topic)
                    };

                    // Use full notification data for better formatting
                    use crate::notifications::NotificationData;
                    let notification_data = NotificationData {
                        title: notification_title,
                        subtitle: Some(datetime),
                        message: formatted_message,
                        topic,
                        timestamp: msg.time,
                        urgent,
                        sound: sound.clone(),
                        persistent,
                        icon_url: msg.icon.clone(),
                    };

                    let nm = notification_manager.lock().await;
                    if let Err(e) = nm.show_notification_full(&notification_data).await {
                        eprintln!("Failed to show notification: {}", e);
                    }

                    // Emit event for badge count tracking
                    let _ = app_handle.emit("new-notification", &msg);
                }

                if new_count > 0 {
                    let _ = app_handle.emit("badge-update", new_count);
                }
            }
            Err(e) => {
                eprintln!("Polling error: {}", e);
            }
        }

        is_polling.store(false, Ordering::SeqCst);

        // Periodic cleanup of seen_ids
        if last_cleanup.elapsed() > cleanup_interval {
            seen_ids.clear();
            last_cleanup = tokio::time::Instant::now();
        }

        let poll_rate = {
            let cfg = config.lock().await;
            cfg.effective_poll_rate()
        };

        sleep(Duration::from_secs(poll_rate)).await;
    }
}
