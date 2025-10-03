//! TDD Tests for Synchronous API Migration
//!
//! These tests define the expected behavior of the synchronous API
//! before we implement the migration from async to sync.

use crate::performance::{ClassCache, OptimizationLevel, PerformanceOptimizer};
use crate::{css_optimizer_modules::CssOptimizer, TailwindBuilder};
use std::sync::Arc;
use std::thread;

#[cfg(test)]
mod sync_class_cache_tests {
    use super::*;

    #[test]
    fn test_class_cache_new() {
        // Test: ClassCache should be created synchronously
        let cache = ClassCache::new(100);
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_class_cache_get_put() {
        // Test: ClassCache should support synchronous get/put operations
        let cache = ClassCache::new(100);

        // Initially empty
        assert!(cache.get("key1").is_none());

        // Put a value
        cache.put("key1".to_string(), "value1".to_string());

        // Should be able to get it back
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        assert_eq!(cache.len(), 1);
        assert!(!cache.is_empty());
    }

    #[test]
    fn test_class_cache_clear() {
        // Test: ClassCache should support synchronous clear operation
        let cache = ClassCache::new(100);

        cache.put("key1".to_string(), "value1".to_string());
        cache.put("key2".to_string(), "value2".to_string());
        assert_eq!(cache.len(), 2);

        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_class_cache_thread_safety() {
        // Test: ClassCache should be thread-safe with parking_lot::RwLock
        let cache = Arc::new(ClassCache::new(100));
        let mut handles = vec![];

        // Spawn multiple threads to test thread safety
        for i in 0..10 {
            let cache_clone = Arc::clone(&cache);
            let handle = thread::spawn(move || {
                for j in 0..10 {
                    let key = format!("key_{}_{}", i, j);
                    let value = format!("value_{}_{}", i, j);
                    cache_clone.put(key.clone(), value.clone());
                    assert_eq!(cache_clone.get(&key), Some(value));
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Should have 100 items total
        assert_eq!(cache.len(), 100);
    }

    #[test]
    fn test_class_cache_lru_eviction() {
        // Test: ClassCache should respect LRU eviction policy
        let cache = ClassCache::new(3);

        // Fill cache to capacity
        cache.put("key1".to_string(), "value1".to_string());
        cache.put("key2".to_string(), "value2".to_string());
        cache.put("key3".to_string(), "value3".to_string());
        assert_eq!(cache.len(), 3);

        // Access key1 to make it recently used
        assert_eq!(cache.get("key1"), Some("value1".to_string()));

        // Add a new key, should evict key2 (least recently used)
        cache.put("key4".to_string(), "value4".to_string());
        assert_eq!(cache.len(), 3);

        // key2 should be evicted
        assert!(cache.get("key2").is_none());
        // key1, key3, key4 should still be present
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        assert_eq!(cache.get("key3"), Some("value3".to_string()));
        assert_eq!(cache.get("key4"), Some("value4".to_string()));
    }
}

#[cfg(test)]
mod sync_performance_optimizer_tests {
    use super::*;

    #[test]
    fn test_performance_optimizer_new() {
        // Test: PerformanceOptimizer should be created synchronously
        let optimizer = PerformanceOptimizer::new();
        assert_eq!(optimizer.optimization_level(), OptimizationLevel::Medium);
    }

    #[test]
    fn test_performance_optimizer_with_level() {
        // Test: PerformanceOptimizer should support different optimization levels
        let optimizer = PerformanceOptimizer::with_optimization_level(OptimizationLevel::High);
        assert_eq!(optimizer.optimization_level(), OptimizationLevel::High);
    }

    #[test]
    fn test_optimize_class_generation_sync() {
        // Test: optimize_class_generation should work synchronously
        let mut optimizer = PerformanceOptimizer::new();
        let classes = vec![
            "px-4".to_string(),
            "py-2".to_string(),
            "bg-blue-500".to_string(),
        ];

        // First call should generate and cache
        let result1 = optimizer.optimize_class_generation(&classes).unwrap();
        assert!(!result1.is_empty());

        // Second call should use cache
        let result2 = optimizer.optimize_class_generation(&classes).unwrap();
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_optimize_css_generation_sync() {
        // Test: optimize_css_generation should work synchronously
        let mut optimizer = PerformanceOptimizer::new();
        let css = ".px-4 { padding-left: 1rem; padding-right: 1rem; }";

        // First call should generate and cache
        let result1 = optimizer.optimize_css_generation(css).unwrap();
        assert!(!result1.is_empty());

        // Second call should use cache
        let result2 = optimizer.optimize_css_generation(css).unwrap();
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_optimization_levels() {
        // Test: Different optimization levels should produce different results
        let classes = vec!["px-2".to_string(), "px-4".to_string(), "py-2".to_string()];

        let mut optimizer_none =
            PerformanceOptimizer::with_optimization_level(OptimizationLevel::None);
        let mut optimizer_low =
            PerformanceOptimizer::with_optimization_level(OptimizationLevel::Low);
        let mut optimizer_high =
            PerformanceOptimizer::with_optimization_level(OptimizationLevel::High);

        let result_none = optimizer_none.optimize_class_generation(&classes).unwrap();
        let result_low = optimizer_low.optimize_class_generation(&classes).unwrap();
        let result_high = optimizer_high.optimize_class_generation(&classes).unwrap();

        // All should produce valid results
        assert!(!result_none.is_empty());
        assert!(!result_low.is_empty());
        assert!(!result_high.is_empty());

        // High optimization should be different from none (conflict resolution)
        assert_ne!(result_none, result_high);
    }

    #[test]
    fn test_performance_metrics_sync() {
        // Test: Performance metrics should work synchronously
        let optimizer = PerformanceOptimizer::new();

        // Record some metrics
        optimizer.record_performance(
            "test_operation",
            std::time::Duration::from_millis(100),
            true,
        );
        optimizer.record_error(
            "test_error",
            &std::io::Error::new(std::io::ErrorKind::Other, "test"),
        );
        optimizer.record_usage("test_pattern", std::time::Duration::from_millis(50));

        // Should be able to retrieve metrics
        let perf_metrics = optimizer.get_performance_metrics();
        let error_metrics = optimizer.get_error_metrics();
        let usage_metrics = optimizer.get_usage_metrics();
        let cache_stats = optimizer.get_cache_stats();

        assert_eq!(perf_metrics.len(), 1);
        assert_eq!(error_metrics.len(), 1);
        assert_eq!(usage_metrics.len(), 1);
        assert_eq!(cache_stats.class_cache_size, 0); // No classes cached yet
    }

    #[test]
    fn test_performance_optimizer_thread_safety() {
        // Test: PerformanceOptimizer should be thread-safe
        use std::sync::Mutex;
        let optimizer = Arc::new(Mutex::new(PerformanceOptimizer::new()));
        let mut handles = vec![];

        // Spawn multiple threads to test thread safety
        for i in 0..5 {
            let optimizer_clone = Arc::clone(&optimizer);
            let handle = thread::spawn(move || {
                let classes = vec![format!("px-{}", i), format!("py-{}", i)];
                let mut optimizer_guard = optimizer_clone.lock().unwrap();
                let result = optimizer_guard.optimize_class_generation(&classes).unwrap();
                assert!(!result.is_empty());
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
    }
}

#[cfg(test)]
mod sync_builder_tests {
    use super::*;

    #[test]
    fn test_tailwind_builder_sync() {
        // Test: TailwindBuilder should work synchronously
        let builder = TailwindBuilder::new()
            .scan_source(std::path::Path::new("src"))
            .output_css(std::path::Path::new("output.css"))
            .enable_minification()
            .enable_source_maps();

        // Build should complete synchronously
        let result = builder.build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_css_optimizer_sync() {
        // Test: CssOptimizer should work synchronously
        let optimizer = CssOptimizer::new();

        // Test CSS optimization
        let css = ".test { padding: 1rem; margin: 0px; }";
        let result = optimizer.optimize_css(css);
        assert!(result.is_ok());

        let result_data = result.unwrap();
        assert!(result_data.len() <= css.len());
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_full_workflow_sync() {
        // Test: Complete workflow should work synchronously
        let mut optimizer = PerformanceOptimizer::with_optimization_level(OptimizationLevel::High);

        // Generate classes
        let classes = vec![
            "px-4".to_string(),
            "py-2".to_string(),
            "bg-blue-500".to_string(),
            "text-white".to_string(),
            "rounded-lg".to_string(),
        ];

        let optimized_classes = optimizer.optimize_class_generation(&classes).unwrap();
        assert!(!optimized_classes.is_empty());

        // Generate CSS
        let css = format!(".{} {{ /* styles */ }}", optimized_classes);
        let optimized_css = optimizer.optimize_css_generation(&css).unwrap();
        assert!(!optimized_css.is_empty());

        // Check cache stats
        let stats = optimizer.get_cache_stats();
        assert!(stats.class_cache_size > 0);
        assert!(stats.css_cache_size > 0);
    }

    #[test]
    fn test_performance_under_load() {
        // Test: Performance should be good under load
        let mut optimizer =
            PerformanceOptimizer::with_optimization_level(OptimizationLevel::Maximum);

        let start = std::time::Instant::now();

        // Generate many class combinations
        for i in 0..1000 {
            let classes = vec![
                format!("px-{}", i % 10),
                format!("py-{}", i % 10),
                format!("bg-{}-500", ["blue", "red", "green", "yellow"][i % 4]),
            ];

            let result = optimizer.optimize_class_generation(&classes).unwrap();
            assert!(!result.is_empty());
        }

        let duration = start.elapsed();

        // Should complete in reasonable time (less than 1 second)
        assert!(duration.as_millis() < 1000);

        // Check that caching is working
        let stats = optimizer.get_cache_stats();
        assert!(stats.class_cache_hit_rate > 0.0);
    }
}
