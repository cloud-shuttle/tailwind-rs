//! Performance Monitor
//!
//! This module provides comprehensive performance monitoring for
//! PostCSS operations with metrics collection and analysis.

use super::types::*;
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime};

/// PostCSS performance monitor
pub struct PostCSSPerformanceMonitor {
    metrics: PerformanceMetrics,
    profiler: PerformanceProfiler,
    reporter: PerformanceReporter,
    alerts: Vec<PerformanceAlert>,
    operation_timers: HashMap<String, Instant>,
}

impl PostCSSPerformanceMonitor {
    /// Create new performance monitor
    pub fn new() -> Self {
        Self {
            metrics: PerformanceMetrics {
                total_time: Duration::from_secs(0),
                parsing_time: Duration::from_secs(0),
                transformation_time: Duration::from_secs(0),
                generation_time: Duration::from_secs(0),
                memory_usage: 0,
                cpu_usage: 0.0,
            },
            profiler: PerformanceProfiler::new(),
            reporter: PerformanceReporter::new(),
            alerts: Vec::new(),
            operation_timers: HashMap::new(),
        }
    }

    /// Start performance monitoring
    pub fn start_monitoring(&mut self, operation: &str) -> PerformanceTimer {
        let timer = PerformanceTimer::new(operation.to_string());
        self.operation_timers
            .insert(operation.to_string(), Instant::now());
        timer
    }

    /// Record performance metrics
    pub fn record_metrics(&mut self, operation: &str, metrics: OperationMetrics) {
        // Update overall metrics
        self.metrics.total_time += metrics.duration;
        self.metrics.memory_usage = self.metrics.memory_usage.max(metrics.memory_delta as usize);
        self.metrics.cpu_usage = self.metrics.cpu_usage.max(metrics.cpu_usage);

        // Record operation metrics
        self.profiler.record_operation(operation, metrics.clone());

        // Check for performance alerts
        self.check_performance_alerts(operation, &metrics);
    }

    /// Generate performance report
    pub fn generate_report(&self) -> PerformanceReport {
        let operations = self.profiler.get_operation_metrics();
        let recommendations = self.generate_recommendations(&operations);

        PerformanceReport {
            total_time: self.metrics.total_time,
            operations,
            memory_usage: self.metrics.memory_usage,
            cpu_usage: self.metrics.cpu_usage,
            alerts: self.alerts.clone(),
            recommendations,
        }
    }

    /// Monitor processing pipeline
    pub fn monitor_processing_pipeline(
        &mut self,
        css: &str,
        pipeline: &ProcessingPipeline,
    ) -> Result<String, AdvancedFeatureError> {
        let start_time = Instant::now();
        let start_memory = self.get_memory_usage();

        let mut processed_css = css.to_string();

        for step in &pipeline.steps {
            let step_timer = self.start_monitoring(&step.name);

            // Execute step
            processed_css = (step.execute)(&processed_css)?;

            // Record metrics
            let step_metrics = OperationMetrics {
                operation: step.name.clone(),
                duration: step_timer.elapsed(),
                memory_delta: self.get_memory_usage() as i64 - start_memory as i64,
                cpu_usage: self.get_cpu_usage(),
                input_size: step.input_size,
                output_size: processed_css.len(),
            };

            self.record_metrics(&step.name, step_metrics);
        }

        let total_time = start_time.elapsed();
        let total_memory = self.get_memory_usage();

        // Record overall metrics
        self.metrics.total_time = total_time;
        self.metrics.memory_usage = total_memory;

        // Check for performance alerts
        self.check_performance_alerts(
            "pipeline",
            &OperationMetrics {
                operation: "pipeline".to_string(),
                duration: total_time,
                memory_delta: total_memory as i64 - start_memory as i64,
                cpu_usage: self.get_cpu_usage(),
                input_size: css.len(),
                output_size: processed_css.len(),
            },
        );

        Ok(processed_css)
    }

    /// Check for performance alerts
    fn check_performance_alerts(&mut self, operation: &str, metrics: &OperationMetrics) {
        // Check for slow operations
        if metrics.duration > Duration::from_millis(1000) {
            self.alerts.push(PerformanceAlert {
                operation: operation.to_string(),
                issue: "Slow execution".to_string(),
                severity: AlertSeverity::Medium,
                timestamp: SystemTime::now(),
                metrics: metrics.clone(),
            });
        }

        // Check for high memory usage
        if metrics.memory_delta > 100 * 1024 * 1024 {
            // 100MB
            self.alerts.push(PerformanceAlert {
                operation: operation.to_string(),
                issue: "High memory usage".to_string(),
                severity: AlertSeverity::High,
                timestamp: SystemTime::now(),
                metrics: metrics.clone(),
            });
        }

        // Check for high CPU usage
        if metrics.cpu_usage > 0.8 {
            self.alerts.push(PerformanceAlert {
                operation: operation.to_string(),
                issue: "High CPU usage".to_string(),
                severity: AlertSeverity::High,
                timestamp: SystemTime::now(),
                metrics: metrics.clone(),
            });
        }
    }

    /// Generate performance recommendations
    fn generate_recommendations(
        &self,
        operations: &HashMap<String, OperationMetrics>,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Find slowest operations
        let mut sorted_operations: Vec<_> = operations.values().collect();
        sorted_operations.sort_by(|a, b| b.duration.cmp(&a.duration));

        if let Some(slowest) = sorted_operations.first() {
            if slowest.duration > Duration::from_millis(500) {
                recommendations.push(format!(
                    "Consider optimizing '{}' operation (took {:?})",
                    slowest.operation, slowest.duration
                ));
            }
        }

        // Check memory usage
        let total_memory: usize = operations
            .values()
            .map(|m| m.memory_delta.max(0) as usize)
            .sum();
        if total_memory > 50 * 1024 * 1024 {
            // 50MB
            recommendations
                .push("Consider implementing memory optimization strategies".to_string());
        }

        // Check CPU usage
        let avg_cpu: f64 =
            operations.values().map(|m| m.cpu_usage).sum::<f64>() / operations.len() as f64;
        if avg_cpu > 0.7 {
            recommendations.push("Consider implementing CPU optimization strategies".to_string());
        }

        recommendations
    }

    /// Get current memory usage
    fn get_memory_usage(&self) -> usize {
        // Simplified implementation - would use actual memory monitoring
        0
    }

    /// Get current CPU usage
    fn get_cpu_usage(&self) -> f64 {
        // Simplified implementation - would use actual CPU monitoring
        0.0
    }
}

/// Performance timer
pub struct PerformanceTimer {
    operation: String,
    start_time: Instant,
}

impl PerformanceTimer {
    /// Create new performance timer
    pub fn new(operation: String) -> Self {
        Self {
            operation,
            start_time: Instant::now(),
        }
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get operation name
    pub fn operation(&self) -> &str {
        &self.operation
    }
}

/// Performance profiler
pub struct PerformanceProfiler {
    operation_metrics: HashMap<String, OperationMetrics>,
    operation_counts: HashMap<String, usize>,
}

impl PerformanceProfiler {
    /// Create new performance profiler
    pub fn new() -> Self {
        Self {
            operation_metrics: HashMap::new(),
            operation_counts: HashMap::new(),
        }
    }

    /// Record operation metrics
    pub fn record_operation(&mut self, operation: &str, metrics: OperationMetrics) {
        self.operation_metrics
            .insert(operation.to_string(), metrics);
        *self
            .operation_counts
            .entry(operation.to_string())
            .or_insert(0) += 1;
    }

    /// Get operation metrics
    pub fn get_operation_metrics(&self) -> HashMap<String, OperationMetrics> {
        self.operation_metrics.clone()
    }

    /// Get operation counts
    pub fn get_operation_counts(&self) -> HashMap<String, usize> {
        self.operation_counts.clone()
    }
}

/// Performance reporter
pub struct PerformanceReporter;

impl PerformanceReporter {
    /// Create new performance reporter
    pub fn new() -> Self {
        Self
    }
}

/// Processing pipeline
pub struct ProcessingPipeline {
    pub steps: Vec<ProcessingStep>,
}

impl ProcessingPipeline {
    /// Create new processing pipeline
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    /// Add processing step
    pub fn add_step(&mut self, step: ProcessingStep) {
        self.steps.push(step);
    }
}

/// Processing step
pub struct ProcessingStep {
    pub name: String,
    pub input_size: usize,
    pub execute: fn(&str) -> Result<String, AdvancedFeatureError>,
}

impl ProcessingStep {
    /// Create new processing step
    pub fn new(
        name: String,
        input_size: usize,
        execute: fn(&str) -> Result<String, AdvancedFeatureError>,
    ) -> Self {
        Self {
            name,
            input_size,
            execute,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_monitoring() {
        let mut monitor = PostCSSPerformanceMonitor::new();
        let timer = monitor.start_monitoring("test_operation");

        // Simulate work
        std::thread::sleep(Duration::from_millis(100));

        let metrics = OperationMetrics {
            operation: "test_operation".to_string(),
            duration: timer.elapsed(),
            memory_delta: 1024,
            cpu_usage: 0.5,
            input_size: 100,
            output_size: 200,
        };

        monitor.record_metrics("test_operation", metrics);
        let report = monitor.generate_report();
        assert!(report.operations.contains_key("test_operation"));
    }

    #[test]
    fn test_performance_timer() {
        let timer = PerformanceTimer::new("test".to_string());
        std::thread::sleep(Duration::from_millis(10));
        assert!(timer.elapsed() >= Duration::from_millis(10));
        assert_eq!(timer.operation(), "test");
    }

    #[test]
    fn test_performance_profiler() {
        let mut profiler = PerformanceProfiler::new();
        let metrics = OperationMetrics {
            operation: "test".to_string(),
            duration: Duration::from_millis(100),
            memory_delta: 1024,
            cpu_usage: 0.5,
            input_size: 100,
            output_size: 200,
        };

        profiler.record_operation("test", metrics);
        let operation_metrics = profiler.get_operation_metrics();
        assert!(operation_metrics.contains_key("test"));
    }

    #[test]
    fn test_processing_pipeline() {
        let mut pipeline = ProcessingPipeline::new();
        let step = ProcessingStep::new("test_step".to_string(), 100, |input| Ok(input.to_string()));

        pipeline.add_step(step);
        assert_eq!(pipeline.steps.len(), 1);
    }

    #[test]
    fn test_performance_alerts() {
        let mut monitor = PostCSSPerformanceMonitor::new();
        let slow_metrics = OperationMetrics {
            operation: "slow_operation".to_string(),
            duration: Duration::from_millis(2000), // 2 seconds
            memory_delta: 1024,
            cpu_usage: 0.5,
            input_size: 100,
            output_size: 200,
        };

        monitor.record_metrics("slow_operation", slow_metrics);
        let report = monitor.generate_report();
        assert!(!report.alerts.is_empty());
    }
}
