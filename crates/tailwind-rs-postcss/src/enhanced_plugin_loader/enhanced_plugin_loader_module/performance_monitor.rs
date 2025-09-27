//! Plugin Performance Monitor
//!
//! This module provides plugin performance monitoring functionality.

use std::collections::HashMap;
use std::time::Duration;

/// Plugin performance monitor
pub struct PluginPerformanceMonitor {
    load_times: HashMap<String, Duration>,
    execution_times: HashMap<String, Duration>,
    metrics: HashMap<String, f64>,
}

impl PluginPerformanceMonitor {
    pub fn new() -> Self {
        Self {
            load_times: HashMap::new(),
            execution_times: HashMap::new(),
            metrics: HashMap::new(),
        }
    }

    pub fn record_load_time(&mut self, plugin_name: &str, duration: Duration) {
        self.load_times.insert(plugin_name.to_string(), duration);
    }

    pub fn record_execution_time(&mut self, plugin_name: &str, duration: Duration) {
        self.execution_times.insert(plugin_name.to_string(), duration);
    }

    pub fn get_load_time(&self, plugin_name: &str) -> Option<&Duration> {
        self.load_times.get(plugin_name)
    }

    pub fn get_execution_time(&self, plugin_name: &str) -> Option<&Duration> {
        self.execution_times.get(plugin_name)
    }
}
