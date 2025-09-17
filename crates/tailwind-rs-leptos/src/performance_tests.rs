//! # Performance Tests for Leptos Integration
//!
//! This module provides performance tests that measure the efficiency of the
//! Leptos integration with tailwind-rs.

use std::time::{Duration, Instant};
use leptos::prelude::{Get, Set};
use tailwind_rs_core::ClassBuilder;
use crate::{DynamicClassBuilder, TailwindSignalManager};

/// Performance test utilities
pub struct PerformanceTestUtils;

impl PerformanceTestUtils {
    /// Measure the time it takes to execute a function
    pub fn measure_time<F, R>(f: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }
    
    /// Measure the time it takes to execute a function multiple times
    pub fn measure_time_iterations<F, R>(f: F, iterations: usize) -> (Vec<R>, Duration)
    where
        F: Fn() -> R,
    {
        let start = Instant::now();
        let mut results = Vec::with_capacity(iterations);
        
        for _ in 0..iterations {
            results.push(f());
        }
        
        let duration = start.elapsed();
        (results, duration)
    }
    
    /// Measure the time it takes to execute a function and return average time
    pub fn measure_average_time<F, R>(f: F, iterations: usize) -> (Vec<R>, Duration, Duration)
    where
        F: Fn() -> R,
    {
        let (results, total_duration) = Self::measure_time_iterations(f, iterations);
        let average_duration = total_duration / iterations as u32;
        (results, total_duration, average_duration)
    }
    
    /// Benchmark class generation performance
    pub fn benchmark_class_generation(iterations: usize) -> (Duration, Duration) {
        let (_, total_duration, average_duration) = Self::measure_average_time(|| {
            let mut builder = ClassBuilder::new();
            builder = builder
                .class("px-4 py-2")
                .class("bg-blue-600 text-white")
                .class("rounded-lg")
                .class("hover:bg-blue-700")
                .class("focus:ring-2")
                .class("focus:ring-blue-500")
                .class("transition-colors")
                .class("duration-200");
            
            builder.build().to_css_classes()
        }, iterations);
        
        (total_duration, average_duration)
    }
    
    /// Benchmark dynamic class builder performance
    pub fn benchmark_dynamic_class_builder(iterations: usize) -> (Duration, Duration) {
        let (_, total_duration, average_duration) = Self::measure_average_time(|| {
            let builder = DynamicClassBuilder::new()
                .base("px-4 py-2")
                .variant("bg-blue-600 text-white")
                .responsive("sm:text-sm md:text-base")
                .state("hover:bg-blue-700 focus:ring-2")
                .custom("transition-colors duration-200");
            
            builder.classes()
        }, iterations);
        
        (total_duration, average_duration)
    }
    
    /// Benchmark signal manager performance
    pub fn benchmark_signal_manager(iterations: usize) -> (Duration, Duration) {
        let (_, total_duration, average_duration) = Self::measure_average_time(|| {
            let manager = TailwindSignalManager::new();
            
            // Test signal creation and updates
            let theme_signal = manager.theme();
            let variant_signal = manager.variant();
            let size_signal = manager.size();
            let disabled_signal = manager.disabled();
            let loading_signal = manager.loading();
            
            // Simulate signal updates
            use tailwind_rs_core::Theme;
            theme_signal.set(Theme::new("dark".to_string()));
            variant_signal.set("secondary".to_string());
            size_signal.set("large".to_string());
            disabled_signal.set(true);
            loading_signal.set(false);
            
            // Return some result to prevent optimization
            format!("{}-{}-{}-{}-{}", 
                theme_signal.get().name,
                variant_signal.get(),
                size_signal.get(),
                disabled_signal.get(),
                loading_signal.get()
            )
        }, iterations);
        
        (total_duration, average_duration)
    }
    
    /// Benchmark complex class generation performance
    pub fn benchmark_complex_class_generation(iterations: usize) -> (Duration, Duration) {
        let (_, total_duration, average_duration) = Self::measure_average_time(|| {
            let mut builder = ClassBuilder::new();
            
            // Add many classes to test performance with large class sets
            for i in 0..50 {
                builder = builder.class(format!("class-{}", i));
            }
            
            // Add responsive classes
            builder = builder
                .class("sm:px-2 sm:py-1")
                .class("md:px-4 md:py-2")
                .class("lg:px-6 lg:py-3")
                .class("xl:px-8 xl:py-4");
            
            // Add state classes
            builder = builder
                .class("hover:bg-blue-700")
                .class("focus:ring-2")
                .class("active:bg-blue-800")
                .class("disabled:opacity-50");
            
            // Add custom variant classes
            builder = builder
                .aria("expanded", "bg-green-500")
                .data("loading", Some("true".to_string()), "animate-spin")
                .supports("backdrop-filter", "backdrop-blur-sm");
            
            builder.build().to_css_classes()
        }, iterations);
        
        (total_duration, average_duration)
    }
    
    /// Benchmark memory usage for class generation
    pub fn benchmark_memory_usage(iterations: usize) -> (Duration, Duration) {
        let (_, total_duration, average_duration) = Self::measure_average_time(|| {
            let mut builders = Vec::with_capacity(100);
            
            // Create many builders to test memory usage
            for i in 0..100 {
                let mut builder = ClassBuilder::new();
                builder = builder
                    .class(format!("px-{}", i))
                    .class(format!("py-{}", i))
                    .class(format!("bg-color-{}", i % 10))
                    .class(format!("text-size-{}", i % 5));
                
                builders.push(builder.build().to_css_classes());
            }
            
            // Return total length to prevent optimization
            builders.iter().map(|s| s.len()).sum::<usize>()
        }, iterations);
        
        (total_duration, average_duration)
    }
}

/// Performance benchmarks for Leptos integration
pub struct LeptosPerformanceBenchmarks;

impl LeptosPerformanceBenchmarks {
    /// Run all performance benchmarks
    pub fn run_all_benchmarks() -> BenchmarkResults {
        let iterations = 1000;
        
        let (class_gen_total, class_gen_avg) = PerformanceTestUtils::benchmark_class_generation(iterations);
        let (dynamic_total, dynamic_avg) = PerformanceTestUtils::benchmark_dynamic_class_builder(iterations);
        let (signal_total, signal_avg) = PerformanceTestUtils::benchmark_signal_manager(iterations);
        let (complex_total, complex_avg) = PerformanceTestUtils::benchmark_complex_class_generation(iterations);
        let (memory_total, memory_avg) = PerformanceTestUtils::benchmark_memory_usage(iterations);
        
        BenchmarkResults {
            class_generation: BenchmarkResult {
                total_time: class_gen_total,
                average_time: class_gen_avg,
                iterations,
            },
            dynamic_class_builder: BenchmarkResult {
                total_time: dynamic_total,
                average_time: dynamic_avg,
                iterations,
            },
            signal_manager: BenchmarkResult {
                total_time: signal_total,
                average_time: signal_avg,
                iterations,
            },
            complex_class_generation: BenchmarkResult {
                total_time: complex_total,
                average_time: complex_avg,
                iterations,
            },
            memory_usage: BenchmarkResult {
                total_time: memory_total,
                average_time: memory_avg,
                iterations,
            },
        }
    }
    
    /// Run performance regression tests
    pub fn run_regression_tests() -> RegressionTestResults {
        let iterations = 100;
        
        // Test that performance is within acceptable bounds
        let (_class_gen_total, class_gen_avg) = PerformanceTestUtils::benchmark_class_generation(iterations);
        let (_dynamic_total, dynamic_avg) = PerformanceTestUtils::benchmark_dynamic_class_builder(iterations);
        let (_signal_total, signal_avg) = PerformanceTestUtils::benchmark_signal_manager(iterations);
        
        // Define performance thresholds (in microseconds)
        let class_gen_threshold = Duration::from_micros(100); // 100μs per operation
        let dynamic_threshold = Duration::from_micros(150); // 150μs per operation
        let signal_threshold = Duration::from_micros(200); // 200μs per operation
        
        RegressionTestResults {
            class_generation: class_gen_avg <= class_gen_threshold,
            dynamic_class_builder: dynamic_avg <= dynamic_threshold,
            signal_manager: signal_avg <= signal_threshold,
            class_generation_time: class_gen_avg,
            dynamic_class_builder_time: dynamic_avg,
            signal_manager_time: signal_avg,
            thresholds: PerformanceThresholds {
                class_generation: class_gen_threshold,
                dynamic_class_builder: dynamic_threshold,
                signal_manager: signal_threshold,
            },
        }
    }
}

/// Results from performance benchmarks
#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    pub class_generation: BenchmarkResult,
    pub dynamic_class_builder: BenchmarkResult,
    pub signal_manager: BenchmarkResult,
    pub complex_class_generation: BenchmarkResult,
    pub memory_usage: BenchmarkResult,
}

/// Individual benchmark result
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub total_time: Duration,
    pub average_time: Duration,
    pub iterations: usize,
}

/// Results from regression tests
#[derive(Debug, Clone)]
pub struct RegressionTestResults {
    pub class_generation: bool,
    pub dynamic_class_builder: bool,
    pub signal_manager: bool,
    pub class_generation_time: Duration,
    pub dynamic_class_builder_time: Duration,
    pub signal_manager_time: Duration,
    pub thresholds: PerformanceThresholds,
}

/// Performance thresholds for regression tests
#[derive(Debug, Clone)]
pub struct PerformanceThresholds {
    pub class_generation: Duration,
    pub dynamic_class_builder: Duration,
    pub signal_manager: Duration,
}

impl BenchmarkResults {
    /// Print benchmark results in a formatted way
    pub fn print_results(&self) {
        println!("🚀 Leptos Integration Performance Benchmarks");
        println!("=============================================");
        
        self.print_benchmark_result("Class Generation", &self.class_generation);
        self.print_benchmark_result("Dynamic Class Builder", &self.dynamic_class_builder);
        self.print_benchmark_result("Signal Manager", &self.signal_manager);
        self.print_benchmark_result("Complex Class Generation", &self.complex_class_generation);
        self.print_benchmark_result("Memory Usage", &self.memory_usage);
    }
    
    fn print_benchmark_result(&self, name: &str, result: &BenchmarkResult) {
        println!("\n📊 {}", name);
        println!("  Total time: {:?}", result.total_time);
        println!("  Average time: {:?}", result.average_time);
        println!("  Iterations: {}", result.iterations);
        println!("  Operations per second: {:.0}", 
            result.iterations as f64 / result.total_time.as_secs_f64());
    }
}

impl RegressionTestResults {
    /// Print regression test results
    pub fn print_results(&self) {
        println!("🔍 Performance Regression Tests");
        println!("===============================");
        
        self.print_regression_result("Class Generation", 
            self.class_generation, 
            self.class_generation_time, 
            self.thresholds.class_generation);
        
        self.print_regression_result("Dynamic Class Builder", 
            self.dynamic_class_builder, 
            self.dynamic_class_builder_time, 
            self.thresholds.dynamic_class_builder);
        
        self.print_regression_result("Signal Manager", 
            self.signal_manager, 
            self.signal_manager_time, 
            self.thresholds.signal_manager);
        
        let all_passed = self.class_generation && self.dynamic_class_builder && self.signal_manager;
        println!("\n🎯 Overall Result: {}", if all_passed { "✅ PASSED" } else { "❌ FAILED" });
    }
    
    fn print_regression_result(&self, name: &str, passed: bool, actual: Duration, threshold: Duration) {
        let status = if passed { "✅ PASS" } else { "❌ FAIL" };
        println!("  {} {}: {:?} (threshold: {:?})", status, name, actual, threshold);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_utils_measurement() {
        let (result, duration) = PerformanceTestUtils::measure_time(|| {
            // Simple operation
            let mut sum = 0;
            for i in 0..1000 {
                sum += i;
            }
            sum
        });
        
        assert_eq!(result, 499500);
        assert!(duration.as_millis() < 100); // Should be very fast
    }
    
    #[test]
    fn test_performance_utils_iterations() {
        let iterations = 10;
        let (results, duration) = PerformanceTestUtils::measure_time_iterations(|| {
            // Simple operation
            let mut sum = 0;
            for i in 0..100 {
                sum += i;
            }
            sum
        }, iterations);
        
        assert_eq!(results.len(), iterations);
        assert!(results.iter().all(|&r| r == 4950));
        assert!(duration.as_millis() < 100); // Should be very fast
    }
    
    #[test]
    fn test_performance_utils_average() {
        let iterations = 5;
        let (results, total_duration, average_duration) = PerformanceTestUtils::measure_average_time(|| {
            // Simple operation
            let mut sum = 0;
            for i in 0..100 {
                sum += i;
            }
            sum
        }, iterations);
        
        assert_eq!(results.len(), iterations);
        assert!(results.iter().all(|&r| r == 4950));
        assert!(total_duration >= average_duration);
        assert!(average_duration.as_millis() < 50); // Should be very fast
    }
    
    #[test]
    fn test_class_generation_benchmark() {
        let iterations = 100;
        let (total_duration, average_duration) = PerformanceTestUtils::benchmark_class_generation(iterations);
        
        assert!(total_duration.as_millis() < 1000); // Should complete in under 1 second
        assert!(average_duration.as_micros() < 10000); // Average should be under 10ms
    }
    
    #[test]
    fn test_dynamic_class_builder_benchmark() {
        let iterations = 100;
        let (total_duration, average_duration) = PerformanceTestUtils::benchmark_dynamic_class_builder(iterations);
        
        assert!(total_duration.as_millis() < 1000); // Should complete in under 1 second
        assert!(average_duration.as_micros() < 10000); // Average should be under 10ms
    }
    
    #[test]
    fn test_complex_class_generation_benchmark() {
        let iterations = 50; // Fewer iterations for complex test
        let (total_duration, average_duration) = PerformanceTestUtils::benchmark_complex_class_generation(iterations);
        
        assert!(total_duration.as_millis() < 2000); // Should complete in under 2 seconds
        assert!(average_duration.as_micros() < 50000); // Average should be under 50ms
    }
    
    #[test]
    fn test_memory_usage_benchmark() {
        let iterations = 10; // Fewer iterations for memory test
        let (total_duration, average_duration) = PerformanceTestUtils::benchmark_memory_usage(iterations);
        
        assert!(total_duration.as_millis() < 1000); // Should complete in under 1 second
        assert!(average_duration.as_micros() < 100000); // Average should be under 100ms
    }
    
    #[test]
    fn test_regression_tests() {
        let results = LeptosPerformanceBenchmarks::run_regression_tests();
        
        // These tests should pass on most systems
        // If they fail, it might indicate a performance regression
        assert!(results.class_generation, 
            "Class generation performance regression: {:?} > {:?}", 
            results.class_generation_time, 
            results.thresholds.class_generation);
        
        assert!(results.dynamic_class_builder, 
            "Dynamic class builder performance regression: {:?} > {:?}", 
            results.dynamic_class_builder_time, 
            results.thresholds.dynamic_class_builder);
        
        assert!(results.signal_manager, 
            "Signal manager performance regression: {:?} > {:?}", 
            results.signal_manager_time, 
            results.thresholds.signal_manager);
    }
    
    #[test]
    fn test_benchmark_results_formatting() {
        let results = BenchmarkResults {
            class_generation: BenchmarkResult {
                total_time: Duration::from_millis(100),
                average_time: Duration::from_micros(100),
                iterations: 1000,
            },
            dynamic_class_builder: BenchmarkResult {
                total_time: Duration::from_millis(150),
                average_time: Duration::from_micros(150),
                iterations: 1000,
            },
            signal_manager: BenchmarkResult {
                total_time: Duration::from_millis(200),
                average_time: Duration::from_micros(200),
                iterations: 1000,
            },
            complex_class_generation: BenchmarkResult {
                total_time: Duration::from_millis(500),
                average_time: Duration::from_micros(500),
                iterations: 1000,
            },
            memory_usage: BenchmarkResult {
                total_time: Duration::from_millis(300),
                average_time: Duration::from_micros(300),
                iterations: 1000,
            },
        };
        
        // Test that formatting doesn't panic
        results.print_results();
    }
    
    #[test]
    fn test_regression_results_formatting() {
        let results = RegressionTestResults {
            class_generation: true,
            dynamic_class_builder: true,
            signal_manager: false,
            class_generation_time: Duration::from_micros(50),
            dynamic_class_builder_time: Duration::from_micros(100),
            signal_manager_time: Duration::from_micros(250),
            thresholds: PerformanceThresholds {
                class_generation: Duration::from_micros(100),
                dynamic_class_builder: Duration::from_micros(150),
                signal_manager: Duration::from_micros(200),
            },
        };
        
        // Test that formatting doesn't panic
        results.print_results();
    }
}
