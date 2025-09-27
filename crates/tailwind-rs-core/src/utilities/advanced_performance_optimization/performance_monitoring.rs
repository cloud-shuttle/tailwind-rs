//! Performance Monitor
//!
//! This module provides performance monitoring functionality.

use std::time::Duration;

/// Performance monitor
pub struct PerformanceMonitor {
    pub metrics: std::collections::HashMap<String, f64>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics: std::collections::HashMap::new(),
        }
    }

    pub fn record_metric(&mut self, name: String, value: f64) {
        self.metrics.insert(name, value);
    }

    pub fn get_metric(&self, name: &str) -> Option<&f64> {
        self.metrics.get(name)
    }
}
