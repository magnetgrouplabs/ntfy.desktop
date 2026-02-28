use std::time::Instant;
use sysinfo::System;

#[derive(Debug, Clone, serde::Serialize)]
pub struct PerformanceMetrics {
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f32,
    pub network_bytes_sent: u64,
    pub network_bytes_received: u64,
    pub uptime_seconds: u64,
}

pub struct PerformanceMonitor {
    system: System,
    last_measurement: Instant,
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self {
            system,
            last_measurement: Instant::now(),
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_all();
        self.last_measurement = Instant::now();
    }

    pub fn get_metrics(&mut self) -> PerformanceMetrics {
        self.refresh();

        PerformanceMetrics {
            memory_usage_mb: self.system.used_memory() as f64 / 1024.0 / 1024.0,
            cpu_usage_percent: self.system.global_cpu_info().cpu_usage(),
            network_bytes_sent: 0,     // sysinfo 0.30 removed networks() API
            network_bytes_received: 0, // sysinfo 0.30 removed networks() API
            uptime_seconds: System::uptime(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_monitor() {
        let mut monitor = PerformanceMonitor::new();
        let metrics = monitor.get_metrics();

        assert!(metrics.memory_usage_mb >= 0.0);
        assert!(metrics.cpu_usage_percent >= 0.0);
        assert!(metrics.cpu_usage_percent <= 100.0);
    }
}
