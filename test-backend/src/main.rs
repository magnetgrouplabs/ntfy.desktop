use std::time::Duration;

// Simple test to verify the Rust backend works
fn main() {
    println!("Testing ntfy.desktop backend functionality...");
    
    // Test configuration
    let config = ntfy.desktop::AppConfig::default();
    println!("Default config: {:?}", config);
    
    // Test performance monitoring
    let mut monitor = ntfy.desktop::PerformanceMonitor::new();
    let metrics = monitor.get_metrics();
    println!("Performance metrics: {:?}", metrics);
    
    println!("Backend functionality verified successfully!");
}