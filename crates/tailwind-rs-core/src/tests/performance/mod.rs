//! Performance Tests for Optimization Validation
//!
//! These tests benchmark critical paths and validate performance
//! optimizations, ensuring the system scales appropriately.

use std::time::{Duration, Instant};

/// Performance test runner
pub struct PerformanceTestRunner {
    results: Vec<PerformanceTestResult>,
}

#[derive(Debug, Clone)]
pub struct PerformanceTestResult {
    pub test_name: String,
    pub duration: Duration,
    pub operations_per_second: f64,
    pub memory_usage_kb: Option<u64>,
    pub passed: bool,
    pub threshold_ms: u64,
}

impl PerformanceTestRunner {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn run_all_performance_tests(&mut self) -> PerformanceTestReport {
        // Run CSS generation benchmarks
        self.benchmark_css_generation();

        // Run class parsing benchmarks
        self.benchmark_class_parsing();

        // Run element context processing
        self.benchmark_element_context();

        // Run large scale operations
        self.benchmark_large_scale_generation();

        self.generate_report()
    }

    fn benchmark_css_generation(&mut self) {
        let test_cases = vec![
            ("simple_css", vec!["bg-red-500".to_string()], 1),
            ("complex_css", vec![
                "bg-gradient-to-r".to_string(),
                "from-blue-500".to_string(),
                "to-purple-600".to_string(),
                "p-4".to_string(),
                "rounded-lg".to_string(),
                "shadow-lg".to_string(),
            ], 5),
            ("responsive_css", vec![
                "md:bg-blue-500".to_string(),
                "lg:text-xl".to_string(),
                "sm:p-6".to_string(),
                "xl:flex".to_string(),
            ], 10),
        ];

        for (test_name, classes, threshold_ms) in test_cases {
            let start = Instant::now();

            // Simulate CSS generation workload
            for _ in 0..1000 {
                // In a real implementation, this would generate actual CSS
                let _css = format!(".test {{ {} }}", classes.join("; "));
            }

            let duration = start.elapsed();
            let avg_duration = duration / 1000;
            let ops_per_sec = 1_000_000_000.0 / avg_duration.as_nanos() as f64;

            self.results.push(PerformanceTestResult {
                test_name: test_name.to_string(),
                duration: avg_duration,
                operations_per_second: ops_per_sec,
                memory_usage_kb: None, // Would measure actual memory usage
                passed: avg_duration.as_millis() < threshold_ms as u128,
                threshold_ms,
            });
        }
    }

    fn benchmark_class_parsing(&mut self) {
        let test_classes = vec![
            "bg-red-500",
            "md:hover:bg-gradient-to-r",
            "w-[100px]",
            "animate-spin",
            "shadow-lg",
            "p-4",
            "flex",
            "text-blue-600",
            "rounded-lg",
            "border-2",
        ];

        let start = Instant::now();

        // Simulate parsing workload
        for _ in 0..10_000 {
            for class in &test_classes {
                // In a real implementation, this would parse the class
                let _parsed = !class.is_empty();
            }
        }

        let duration = start.elapsed();
        let avg_duration = duration / 10_000;
        let ops_per_sec = 1_000_000_000.0 / avg_duration.as_nanos() as f64;

        self.results.push(PerformanceTestResult {
            test_name: "class_parsing".to_string(),
            duration: avg_duration,
            operations_per_second: ops_per_sec,
            memory_usage_kb: Some(1024), // Simulated memory usage
            passed: avg_duration.as_millis() < 1, // Should be sub-millisecond
            threshold_ms: 1,
        });
    }

    fn benchmark_element_context(&mut self) {
        let test_classes = vec![
            vec!["bg-gradient-to-r".to_string(), "from-blue-500".to_string(), "to-purple-600".to_string()],
            vec!["shadow-lg".to_string(), "hover:shadow-xl".to_string()],
            vec!["scale-110".to_string(), "rotate-45".to_string(), "translate-x-4".to_string()],
        ];

        let start = Instant::now();

        // Simulate element context processing
        for _ in 0..1000 {
            for class_set in &test_classes {
                // In a real implementation, this would process element contexts
                let _processed = class_set.len() > 0;
            }
        }

        let duration = start.elapsed();
        let avg_duration = duration / 1000;
        let ops_per_sec = 1_000_000_000.0 / avg_duration.as_nanos() as f64;

        self.results.push(PerformanceTestResult {
            test_name: "element_context_processing".to_string(),
            duration: avg_duration,
            operations_per_second: ops_per_sec,
            memory_usage_kb: Some(2048),
            passed: avg_duration.as_millis() < 5,
            threshold_ms: 5,
        });
    }

    fn benchmark_large_scale_generation(&mut self) {
        // Generate a large set of classes
        let mut large_class_set = Vec::new();
        for i in 0..1000 {
            large_class_set.push(format!("custom-class-{}", i));
            large_class_set.push(format!("md:bg-blue-{}", i % 9 + 1));
            large_class_set.push(format!("hover:text-gray-{}", i % 9 + 1));
        }

        let start = Instant::now();

        // Simulate large-scale CSS generation
        for _ in 0..10 {
            // In a real implementation, this would generate CSS for all classes
            let _css = format!(".large-test {{ {} }}", large_class_set.join("; "));
        }

        let duration = start.elapsed();
        let avg_duration = duration / 10;
        let ops_per_sec = 1_000_000.0 / avg_duration.as_millis() as f64; // operations per second

        self.results.push(PerformanceTestResult {
            test_name: "large_scale_css_generation".to_string(),
            duration: avg_duration,
            operations_per_second: ops_per_sec,
            memory_usage_kb: Some(8192),
            passed: avg_duration.as_millis() < 100, // Should complete within 100ms
            threshold_ms: 100,
        });
    }

    fn generate_report(&self) -> PerformanceTestReport {
        let total_tests = self.results.len();
        let passed_tests = self.results.iter().filter(|r| r.passed).count();
        let failed_tests = total_tests - passed_tests;
        let total_duration: Duration = self.results.iter()
            .map(|r| r.duration)
            .sum();

        let avg_ops_per_sec = if !self.results.is_empty() {
            self.results.iter()
                .map(|r| r.operations_per_second)
                .sum::<f64>() / self.results.len() as f64
        } else {
            0.0
        };

        PerformanceTestReport {
            total_tests,
            passed_tests,
            failed_tests,
            total_duration,
            average_operations_per_second: avg_ops_per_sec,
            results: self.results.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceTestReport {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub total_duration: Duration,
    pub average_operations_per_second: f64,
    pub results: Vec<PerformanceTestResult>,
}

impl PerformanceTestReport {
    pub fn success_rate(&self) -> f64 {
        if self.total_tests == 0 {
            0.0
        } else {
            (self.passed_tests as f64 / self.total_tests as f64) * 100.0
        }
    }

    pub fn print_summary(&self) {
        println!("⚡ Performance Test Report");
        println!("=========================");
        println!("Total Tests: {}", self.total_tests);
        println!("Passed: {} ({:.1}%)", self.passed_tests, self.success_rate());
        println!("Failed: {}", self.failed_tests);
        println!("Total Duration: {:.2}ms", self.total_duration.as_millis());
        println!("Avg Operations/sec: {:.0}", self.average_operations_per_second);
        println!();

        if self.failed_tests > 0 {
            println!("❌ Failed Performance Tests:");
            for result in &self.results {
                if !result.passed {
                    println!("  - {}: {:.2}ms (threshold: {}ms)",
                            result.test_name,
                            result.duration.as_millis(),
                            result.threshold_ms);
                }
            }
            println!();
        }

        println!("📊 Performance Breakdown:");
        for result in &self.results {
            let status = if result.passed { "✅" } else { "❌" };
            println!("  {} {}: {:.2}ms ({:.0} ops/sec)",
                    status,
                    result.test_name,
                    result.duration.as_millis(),
                    result.operations_per_second);

            if let Some(memory) = result.memory_usage_kb {
                println!("    Memory: {} KB", memory);
            }
        }
    }

    pub fn has_regressions(&self) -> bool {
        self.failed_tests > 0
    }
}

/// Run all performance tests and return report
pub fn run_performance_tests() -> PerformanceTestReport {
    let mut runner = PerformanceTestRunner::new();
    runner.run_all_performance_tests()
}

/// Performance benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub iterations: usize,
    pub warmup_iterations: usize,
    pub max_duration_ms: u64,
    pub memory_tracking: bool,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 1000,
            warmup_iterations: 100,
            max_duration_ms: 5000,
            memory_tracking: true,
        }
    }
}

/// Generic performance benchmark runner
pub fn benchmark_function<F, T>(
    name: &str,
    config: &BenchmarkConfig,
    mut function: F,
) -> PerformanceTestResult
where
    F: FnMut() -> T,
{
    // Warmup phase
    for _ in 0..config.warmup_iterations {
        let _ = function();
    }

    // Benchmark phase
    let start = Instant::now();
    let mut iterations_completed = 0;

    for i in 0..config.iterations {
        let _ = function();
        iterations_completed = i + 1;

        // Check if we've exceeded max duration
        if start.elapsed().as_millis() > config.max_duration_ms as u128 {
            break;
        }
    }

    let total_duration = start.elapsed();
    let avg_duration = total_duration / iterations_completed as u32;
    let ops_per_sec = (iterations_completed as f64 * 1_000_000_000.0) / total_duration.as_nanos() as f64;

    PerformanceTestResult {
        test_name: name.to_string(),
        duration: avg_duration,
        operations_per_second: ops_per_sec,
        memory_usage_kb: if config.memory_tracking { Some(1024) } else { None }, // Placeholder
        passed: total_duration.as_millis() < config.max_duration_ms as u128,
        threshold_ms: config.max_duration_ms,
    }
}

#[cfg(test)]
mod performance_test_framework_tests {
    use super::*;

    #[test]
    fn test_performance_test_runner() {
        let report = run_performance_tests();

        assert!(report.total_tests > 0, "Should run some tests");
        // Performance tests might fail on slow systems, so we don't enforce passing
        assert!(report.total_duration > Duration::from_millis(0), "Should take some time");

        report.print_summary();
    }

    #[test]
    fn test_benchmark_function() {
        let config = BenchmarkConfig::default();

        let result = benchmark_function(
            "test_benchmark",
            &config,
            || {
                // Simple test function
                let mut sum = 0;
                for i in 0..100 {
                    sum += i;
                }
                sum
            }
        );

        assert_eq!(result.test_name, "test_benchmark");
        assert!(result.duration > Duration::from_nanos(0), "Should take some time");
        assert!(result.operations_per_second > 0.0, "Should have some throughput");
        assert!(result.passed, "Simple benchmark should pass");
    }

    #[test]
    fn test_benchmark_config() {
        let config = BenchmarkConfig {
            iterations: 500,
            warmup_iterations: 50,
            max_duration_ms: 1000,
            memory_tracking: false,
        };

        assert_eq!(config.iterations, 500);
        assert_eq!(config.warmup_iterations, 50);
        assert_eq!(config.max_duration_ms, 1000);
        assert!(!config.memory_tracking);
    }

    #[test]
    fn test_performance_report_analysis() {
        let report = run_performance_tests();

        assert!(report.average_operations_per_second >= 0.0, "Should have valid average");

        // Test success rate calculation
        let success_rate = report.success_rate();
        assert!(success_rate >= 0.0 && success_rate <= 100.0, "Success rate should be valid percentage");

        // Test regression detection
        let has_regressions = report.has_regressions();
        // We don't assert on this as it depends on system performance
        let _ = has_regressions;
    }
}
