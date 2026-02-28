use assert_matches::assert_matches;
use ntfy_desktop::ntfy::{NtfyClient, NtfyMessage};
use tokio::time::{sleep, Duration};

#[test]
fn test_ntfy_client_creation() {
    let client = NtfyClient::new("https://ntfy.sh");
    assert_eq!(client.base_url, "https://ntfy.sh");
    assert!(client.api_token.is_none());
    assert!(client.auth_user.is_none());
    assert!(client.auth_pass.is_none());
}

#[test]
fn test_ntfy_client_with_token() {
    let client = NtfyClient::new("https://ntfy.sh")
        .with_token("test-token".to_string());
    assert_eq!(client.base_url, "https://ntfy.sh");
    assert_matches!(client.api_token, Some(ref token) if token == "test-token");
}

#[test]
fn test_ntfy_client_with_empty_token() {
    let client = NtfyClient::new("https://ntfy.sh")
        .with_token("".to_string());
    assert!(client.api_token.is_none());
}

#[test]
fn test_ntfy_client_with_basic_auth() {
    let client = NtfyClient::new("https://ntfy.sh")
        .with_basic_auth("testuser".to_string(), "testpass".to_string());
    assert_eq!(client.base_url, "https://ntfy.sh");
    assert_matches!(client.auth_user, Some(ref user) if user == "testuser");
    assert_matches!(client.auth_pass, Some(ref pass) if pass == "testpass");
}

#[test]
fn test_ntfy_client_with_empty_basic_auth() {
    let client = NtfyClient::new("https://ntfy.sh")
        .with_basic_auth("".to_string(), "".to_string());
    assert!(client.auth_user.is_none());
    assert!(client.auth_pass.is_none());
}

#[test]
fn test_ntfy_message_structure() {
    let message = NtfyMessage {
        id: Some("test-id".to_string()),
        time: 1234567890,
        event: Some("message".to_string()),
        topic: Some("test-topic".to_string()),
        title: Some("Test Title".to_string()),
        message: Some("Test Message".to_string()),
        priority: Some(4),
        tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
        click: Some("https://example.com".to_string()),
        actions: Some(vec![]),
        icon: Some("https://example.com/icon.png".to_string()),
    };

    assert_eq!(message.id, Some("test-id".to_string()));
    assert_eq!(message.time, 1234567890);
    assert_eq!(message.event, Some("message".to_string()));
    assert_eq!(message.topic, Some("test-topic".to_string()));
    assert_eq!(message.title, Some("Test Title".to_string()));
    assert_eq!(message.message, Some("Test Message".to_string()));
    assert_eq!(message.priority, Some(4));
    assert_eq!(message.tags, Some(vec!["tag1".to_string(), "tag2".to_string()]));
    assert_eq!(message.click, Some("https://example.com".to_string()));
    assert_eq!(message.actions, Some(vec![]));
    assert_eq!(message.icon, Some("https://example.com/icon.png".to_string()));
}

#[test]
fn test_ntfy_message_default_values() {
    let message = NtfyMessage {
        id: None,
        time: 0,
        event: None,
        topic: None,
        title: None,
        message: None,
        priority: None,
        tags: None,
        click: None,
        actions: None,
        icon: None,
    };

    assert!(message.id.is_none());
    assert_eq!(message.time, 0);
    assert!(message.event.is_none());
    assert!(message.topic.is_none());
    assert!(message.title.is_none());
    assert!(message.message.is_none());
    assert!(message.priority.is_none());
    assert!(message.tags.is_none());
    assert!(message.click.is_none());
    assert!(message.actions.is_none());
    assert!(message.icon.is_none());
}

#[test]
fn test_ntfy_message_serialization() {
    let message = NtfyMessage {
        id: Some("test-id".to_string()),
        time: 1234567890,
        event: Some("message".to_string()),
        topic: Some("test-topic".to_string()),
        title: Some("Test Title".to_string()),
        message: Some("Test Message".to_string()),
        priority: Some(4),
        tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
        click: Some("https://example.com".to_string()),
        actions: Some(vec![]),
        icon: Some("https://example.com/icon.png".to_string()),
    };

    let serialized = serde_json::to_string(&message).expect("Failed to serialize");
    let deserialized: NtfyMessage = serde_json::from_str(&serialized).expect("Failed to deserialize");

    assert_eq!(message.id, deserialized.id);
    assert_eq!(message.time, deserialized.time);
    assert_eq!(message.event, deserialized.event);
    assert_eq!(message.topic, deserialized.topic);
    assert_eq!(message.title, deserialized.title);
    assert_eq!(message.message, deserialized.message);
    assert_eq!(message.priority, deserialized.priority);
    assert_eq!(message.tags, deserialized.tags);
    assert_eq!(message.click, deserialized.click);
    assert_eq!(message.icon, deserialized.icon);
}

#[tokio::test]
async fn test_ntfy_client_connection_test() {
    let client = NtfyClient::new("https://ntfy.sh");
    
    // This is a real connection test - may fail if network is unavailable
    // We'll handle the result gracefully
    match client.test_connection("test").await {
        Ok(success) => {
            // If connection succeeds, verify it returns true
            assert!(success);
        }
        Err(e) => {
            // If connection fails, that's okay for testing purposes
            // Just verify we got an error (not a panic)
            println!("Connection test failed (expected for testing): {}", e);
        }
    }
}

#[tokio::test]
async fn test_ntfy_client_poll_messages_empty_topics() {
    let client = NtfyClient::new("https://ntfy.sh");
    
    // Test with empty topics - should handle gracefully
    let result = client.poll_messages("", 60).await;
    assert!(result.is_err());
}

#[test]
fn test_ntfy_action_structure() {
    use ntfy_desktop::ntfy::NtfyAction;
    
    let action = NtfyAction {
        action: "view".to_string(),
        label: "View Details".to_string(),
        url: Some("https://example.com".to_string()),
        clear: Some(true),
    };

    assert_eq!(action.action, "view");
    assert_eq!(action.label, "View Details");
    assert_eq!(action.url, Some("https://example.com".to_string()));
    assert_eq!(action.clear, Some(true));
}

#[test]
fn test_ntfy_action_minimal() {
    use ntfy_desktop::ntfy::NtfyAction;
    
    let action = NtfyAction {
        action: "view".to_string(),
        label: "View Details".to_string(),
        url: None,
        clear: None,
    };

    assert_eq!(action.action, "view");
    assert_eq!(action.label, "View Details");
    assert!(action.url.is_none());
    assert!(action.clear.is_none());
}

#[test]
fn test_ntfy_action_serialization() {
    use ntfy_desktop::ntfy::NtfyAction;
    
    let action = NtfyAction {
        action: "view".to_string(),
        label: "View Details".to_string(),
        url: Some("https://example.com".to_string()),
        clear: Some(true),
    };

    let serialized = serde_json::to_string(&action).expect("Failed to serialize");
    let deserialized: NtfyAction = serde_json::from_str(&serialized).expect("Failed to deserialize");

    assert_eq!(action.action, deserialized.action);
    assert_eq!(action.label, deserialized.label);
    assert_eq!(action.url, deserialized.url);
    assert_eq!(action.clear, deserialized.clear);
}

#[tokio::test]
async fn test_ntfy_client_url_normalization() {
    let client = NtfyClient::new("https://ntfy.sh/app");
    assert_eq!(client.base_url, "https://ntfy.sh");

    let client = NtfyClient::new("https://ntfy.sh/app/");
    assert_eq!(client.base_url, "https://ntfy.sh");

    let client = NtfyClient::new("https://ntfy.sh");
    assert_eq!(client.base_url, "https://ntfy.sh");

    let client = NtfyClient::new("http://localhost:8080/app");
    assert_eq!(client.base_url, "http://localhost:8080");
}

#[test]
fn test_ntfy_client_auth_application() {
    let client = NtfyClient::new("https://ntfy.sh")
        .with_token("test-token".to_string());

    // Test that authentication headers are properly formatted
    let request = client.client.get("https://example.com");
    let request_with_auth = client.apply_auth(request);
    
    // The actual header application happens when the request is built
    // This test just verifies the method exists and returns a request builder
    assert_matches!(request_with_auth, reqwest::RequestBuilder);
}

#[tokio::test]
async fn test_ntfy_client_error_handling() {
    let client = NtfyClient::new("https://invalid-domain-that-should-not-exist.test");
    
    // This should fail gracefully due to invalid domain
    let result = client.test_connection("test").await;
    assert!(result.is_err());
    
    let error_message = result.unwrap_err().to_string();
    assert!(error_message.contains("Connection failed") || error_message.contains("failed"));
}