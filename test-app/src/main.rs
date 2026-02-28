use std::time::Duration;

mod config;
mod ntfy;
mod notifications;
mod performance;

#[tokio::main]
async fn main() {
    println!("Testing ntfy.desktop core functionality...");
    
    // Test configuration
    let config = config::AppConfig::default();
    println!("Default config: {:?}", config);
    
    // Test notification
    if let Err(e) = notifications::show_notification("Test", "Notification test").await {
        println!("Notification error: {}", e);
    } else {
        println!("Notification sent successfully");
    }
    
    // Test performance monitoring
    let mut monitor = performance::PerformanceMonitor::new();
    let metrics = monitor.get_metrics();
    println!("Performance metrics: {:?}", metrics);
    
    // Test ntfy client
    let client = ntfy::NtfyClient::new("https://ntfy.sh");
    match client.test_connection("test-topic").await {
        Ok(success) => println!("Ntfy connection test: {}", success),
        Err(e) => println!("Ntfy connection error: {}", e),
    }
    
    println!("All tests completed!");
}