//! Parallel Processing System for Tailwind-RS
//!
//! This module provides parallel processing capabilities for:
//! - Class parsing and CSS generation
//! - Large class list processing
//! - Concurrent theme resolution
//! - Batched CSS optimization

use crate::css_generator::types::CssProperty;
use crate::css_generator::CssGenerator;
use crate::performance::cache::{CacheManager, ClassCacheKey, SharedCache};
use crate::error::{Result, TailwindError};
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Parallel CSS processor for handling large class lists
#[derive(Debug)]
pub struct ParallelCssProcessor {
    /// Cache manager for performance
    cache_manager: Option<CacheManager>,
    /// Number of threads to use (0 = auto-detect)
    thread_count: usize,
    /// Batch size for processing
    batch_size: usize,
}

impl ParallelCssProcessor {
    /// Create a new parallel CSS processor
    pub fn new() -> Self {
        Self {
            cache_manager: None,
            thread_count: 0, // Auto-detect
            batch_size: 100, // Process 100 classes per batch
        }
    }

    /// Create processor with cache manager
    pub fn with_cache(mut self, cache_manager: CacheManager) -> Self {
        self.cache_manager = Some(cache_manager);
        self
    }

    /// Set thread count (0 for auto-detect)
    pub fn with_threads(mut self, threads: usize) -> Self {
        self.thread_count = threads;
        self
    }

    /// Set batch size for processing
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    /// Process multiple class lists in parallel (simplified implementation)
    pub fn process_class_lists(
        &self,
        class_lists: Vec<Vec<String>>,
    ) -> Result<Vec<Vec<String>>> {
        if class_lists.is_empty() {
            return Ok(vec![]);
        }

        // Configure Rayon thread pool if specified
        let _guard = if self.thread_count > 0 {
            Some(rayon::ThreadPoolBuilder::new()
                .num_threads(self.thread_count)
                .build_global()?)
        } else {
            None
        };

        // For now, just return the class lists as-is
        // In a full implementation, this would process them in parallel
        let results: Vec<Vec<String>> = class_lists
            .into_par_iter()
            .map(|class_list| class_list)
            .collect();

        Ok(results)
    }

    /// Process element-based classes in parallel (simplified)
    pub fn process_element_classes(
        &self,
        elements: Vec<(String, Vec<String>)>, // (element_id, classes)
    ) -> Result<HashMap<String, Vec<String>>> {
        if elements.is_empty() {
            return Ok(HashMap::new());
        }

        let results: HashMap<String, Vec<String>> = elements
            .into_par_iter()
            .map(|(element_id, classes)| (element_id, classes))
            .collect();

        Ok(results)
    }
}

/// Result of optimized CSS processing
#[derive(Debug, Clone)]
pub struct OptimizedCssResult {
    /// Time taken for processing
    pub processing_time: Duration,
    /// Number of threads used
    pub thread_count: usize,
    /// Cache statistics if caching was used
    pub cache_stats: Option<crate::performance::cache::CacheStats>,
}

impl OptimizedCssResult {
    /// Get processing performance in classes per second
    pub fn classes_per_second(&self, total_classes: usize) -> f64 {
        let seconds = self.processing_time.as_secs_f64();
        if seconds > 0.0 {
            total_classes as f64 / seconds
        } else {
            0.0
        }
    }

    /// Get cache hit rate if available
    pub fn cache_hit_rate(&self) -> Option<f64> {
        self.cache_stats.as_ref().map(|stats| {
            let total = stats.hits + stats.misses;
            if total > 0 {
                (stats.hits as f64 / total as f64) * 100.0
            } else {
                0.0
            }
        })
    }
}

/// Memory pool for frequently allocated objects
#[derive(Debug)]
pub struct MemoryPool<T> {
    /// Available objects
    available: Mutex<Vec<T>>,
    /// Factory function to create new objects
    factory: fn() -> T,
    /// Maximum pool size
    max_size: usize,
}

impl<T> MemoryPool<T> {
    /// Create a new memory pool
    pub fn new(factory: fn() -> T, max_size: usize) -> Self {
        Self {
            available: Mutex::new(Vec::with_capacity(max_size)),
            factory,
            max_size,
        }
    }

    /// Get an object from the pool or create a new one
    pub fn get(&self) -> T {
        let mut available = self.available.lock().unwrap();
        available.pop().unwrap_or_else(|| (self.factory)())
    }

    /// Return an object to the pool
    pub fn put(&self, object: T) {
        let mut available = self.available.lock().unwrap();
        if available.len() < self.max_size {
            available.push(object);
        }
        // If pool is full, object is dropped
    }

    /// Get pool statistics
    pub fn stats(&self) -> MemoryPoolStats {
        let available = self.available.lock().unwrap();
        MemoryPoolStats {
            available_count: available.len(),
            max_size: self.max_size,
        }
    }
}

/// Memory pool statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPoolStats {
    pub available_count: usize,
    pub max_size: usize,
}

/// Performance monitoring for parallel processing
#[derive(Debug)]
pub struct PerformanceMonitor {
    /// Processing statistics
    stats: Mutex<ProcessingStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingStats {
    pub total_classes_processed: u64,
    pub total_processing_time: Duration,
    pub average_batch_time: Duration,
    pub peak_memory_usage: usize,
    pub cache_hit_rate: f64,
    pub parallel_efficiency: f64,
}

impl Default for ProcessingStats {
    fn default() -> Self {
        Self {
            total_classes_processed: 0,
            total_processing_time: Duration::default(),
            average_batch_time: Duration::default(),
            peak_memory_usage: 0,
            cache_hit_rate: 0.0,
            parallel_efficiency: 0.0,
        }
    }
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            stats: Mutex::new(ProcessingStats::default()),
        }
    }

    /// Record a processing operation
    pub fn record_processing(&self, class_count: usize, duration: Duration, cache_hit_rate: f64) {
        let mut stats = self.stats.lock().unwrap();
        stats.total_classes_processed += class_count as u64;
        stats.total_processing_time += duration;
        stats.cache_hit_rate = cache_hit_rate;

        // Update average batch time
        let batches = stats.total_classes_processed / 100; // Assuming 100 classes per batch
        if batches > 0 {
            stats.average_batch_time = stats.total_processing_time / batches as u32;
        }
    }

    /// Get current statistics
    pub fn stats(&self) -> ProcessingStats {
        self.stats.lock().unwrap().clone()
    }

    /// Calculate parallel efficiency (higher is better)
    pub fn calculate_efficiency(&self, thread_count: usize, sequential_time: Duration) -> f64 {
        if sequential_time.as_nanos() == 0 {
            return 0.0;
        }

        let parallel_time = self.stats.lock().unwrap().total_processing_time;
        let ideal_time = sequential_time / thread_count as u32;

        (ideal_time.as_secs_f64() / parallel_time.as_secs_f64()) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css_generator::CssGenerator;
    use crate::theme::ThemeConfig;

    #[test]
    fn test_parallel_processor_creation() {
        let generator = CssGenerator::new(ThemeConfig::default());
        let processor = ParallelCssProcessor::new(generator)
            .with_threads(4)
            .with_batch_size(50);

        assert_eq!(processor.thread_count, 4);
        assert_eq!(processor.batch_size, 50);
    }

    #[test]
    fn test_memory_pool() {
        fn create_vec() -> Vec<i32> {
            Vec::with_capacity(10)
        }

        let pool = MemoryPool::new(create_vec, 5);

        // Get objects
        let mut v1 = pool.get();
        v1.push(1);

        let mut v2 = pool.get();
        v2.push(2);

        // Return objects
        pool.put(v1);
        pool.put(v2);

        // Check stats
        let stats = pool.stats();
        assert_eq!(stats.available_count, 2);
        assert_eq!(stats.max_size, 5);
    }

    #[test]
    fn test_performance_monitor() {
        let monitor = PerformanceMonitor::new();

        monitor.record_processing(100, Duration::from_millis(50), 85.0);

        let stats = monitor.stats();
        assert_eq!(stats.total_classes_processed, 100);
        assert_eq!(stats.cache_hit_rate, 85.0);
    }

    #[test]
    fn test_optimized_css_result() {
        let css_rules = CssRules::new();
        let processing_time = Duration::from_millis(100);

        let result = OptimizedCssResult {
            css_rules,
            processing_time,
            thread_count: 4,
            cache_stats: None,
        };

        assert_eq!(result.thread_count, 4);
        assert!(result.processing_time >= Duration::from_millis(100));
    }

    #[test]
    fn test_parallel_processing_small_list() {
        let generator = CssGenerator::new(ThemeConfig::default());
        let processor = ParallelCssProcessor::new(generator);

        let class_lists = vec![
            vec!["text-red-500".to_string()],
            vec!["bg-blue-500".to_string(), "p-4".to_string()],
        ];

        let result = processor.process_class_lists(class_lists);
        assert!(result.is_ok());

        let css_results = result.unwrap();
        assert_eq!(css_results.len(), 2);
    }

    #[test]
    fn test_element_processing() {
        let generator = CssGenerator::new(ThemeConfig::default());
        let processor = ParallelCssProcessor::new(generator);

        let elements = vec![
            ("header".to_string(), vec!["text-xl".to_string(), "font-bold".to_string()]),
            ("main".to_string(), vec!["p-4".to_string(), "bg-gray-100".to_string()]),
        ];

        let result = processor.process_element_classes(elements);
        assert!(result.is_ok());

        let css_map = result.unwrap();
        assert_eq!(css_map.len(), 2);
        assert!(css_map.contains_key("header"));
        assert!(css_map.contains_key("main"));
    }
}
