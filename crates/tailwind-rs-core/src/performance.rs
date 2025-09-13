//! Performance optimization system for tailwind-rs
//!
//! This module provides caching and performance optimization functionality
//! for class generation and CSS optimization.

use crate::error::Result;
use lru::LruCache;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Performance metrics for tracking optimization effectiveness
#[derive(Debug, Clone)]
pub struct PerformanceMetric {
    pub operation: String,
    pub duration: Duration,
    pub timestamp: Instant,
    pub success: bool,
}

/// Error metrics for tracking validation and processing errors
#[derive(Debug, Clone)]
pub struct ErrorMetric {
    pub error_type: String,
    pub error_message: String,
    pub timestamp: Instant,
    pub count: u64,
}

/// Usage metrics for tracking class generation patterns
#[derive(Debug, Clone)]
pub struct UsageMetric {
    pub class_pattern: String,
    pub usage_count: u64,
    pub last_used: Instant,
    pub average_generation_time: Duration,
}

/// Caches generated classes for performance
#[derive(Debug)]
pub struct ClassCache {
    cache: Arc<tokio::sync::RwLock<LruCache<String, String>>>,
    hit_rate: AtomicU64,
    miss_rate: AtomicU64,
    total_requests: AtomicU64,
}

impl ClassCache {
    /// Create a new cache with specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Arc::new(tokio::sync::RwLock::new(LruCache::new(
                std::num::NonZeroUsize::new(capacity).unwrap(),
            ))),
            hit_rate: AtomicU64::new(0),
            miss_rate: AtomicU64::new(0),
            total_requests: AtomicU64::new(0),
        }
    }

    /// Retrieve a cached class string
    pub async fn get(&self, key: &str) -> Option<String> {
        self.total_requests.fetch_add(1, Ordering::Relaxed);

        let mut cache = self.cache.write().await;
        if let Some(value) = cache.get(key) {
            self.hit_rate.fetch_add(1, Ordering::Relaxed);
            Some(value.clone())
        } else {
            self.miss_rate.fetch_add(1, Ordering::Relaxed);
            None
        }
    }

    /// Store a class string in the cache
    pub async fn put(&self, key: String, value: String) {
        let mut cache = self.cache.write().await;
        cache.put(key, value);
    }

    /// Get cache hit rate (0.0 to 1.0)
    pub fn hit_rate(&self) -> f64 {
        let hits = self.hit_rate.load(Ordering::Relaxed) as f64;
        let total = self.total_requests.load(Ordering::Relaxed) as f64;

        if total == 0.0 { 0.0 } else { hits / total }
    }

    /// Get cache miss rate (0.0 to 1.0)
    pub fn miss_rate(&self) -> f64 {
        let misses = self.miss_rate.load(Ordering::Relaxed) as f64;
        let total = self.total_requests.load(Ordering::Relaxed) as f64;

        if total == 0.0 { 0.0 } else { misses / total }
    }

    /// Get total number of requests
    pub fn total_requests(&self) -> u64 {
        self.total_requests.load(Ordering::Relaxed)
    }

    /// Clear the cache
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }

    /// Get cache size
    pub async fn len(&self) -> usize {
        let cache = self.cache.read().await;
        cache.len()
    }

    /// Check if cache is empty
    pub async fn is_empty(&self) -> bool {
        let cache = self.cache.read().await;
        cache.is_empty()
    }
}

/// Performance optimization levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLevel {
    None,
    Low,
    Medium,
    High,
    Maximum,
}

impl OptimizationLevel {
    /// Get the cache capacity for this optimization level
    pub fn cache_capacity(&self) -> usize {
        match self {
            OptimizationLevel::None => 0,
            OptimizationLevel::Low => 100,
            OptimizationLevel::Medium => 500,
            OptimizationLevel::High => 1000,
            OptimizationLevel::Maximum => 5000,
        }
    }

    /// Get the optimization factor (1.0 = no optimization, higher = more optimization)
    pub fn optimization_factor(&self) -> f64 {
        match self {
            OptimizationLevel::None => 1.0,
            OptimizationLevel::Low => 1.2,
            OptimizationLevel::Medium => 1.5,
            OptimizationLevel::High => 2.0,
            OptimizationLevel::Maximum => 3.0,
        }
    }
}

/// Optimizes class generation performance
#[derive(Debug)]
pub struct PerformanceOptimizer {
    class_cache: ClassCache,
    css_cache: ClassCache,
    optimization_level: OptimizationLevel,
    performance_metrics: Arc<tokio::sync::RwLock<Vec<PerformanceMetric>>>,
    error_metrics: Arc<tokio::sync::RwLock<Vec<ErrorMetric>>>,
    usage_metrics: Arc<tokio::sync::RwLock<HashMap<String, UsageMetric>>>,
}

impl PerformanceOptimizer {
    /// Create a new optimizer instance
    pub fn new() -> Self {
        Self::with_optimization_level(OptimizationLevel::Medium)
    }

    /// Create a new optimizer with specific optimization level
    pub fn with_optimization_level(level: OptimizationLevel) -> Self {
        let capacity = level.cache_capacity();
        Self {
            class_cache: ClassCache::new(capacity),
            css_cache: ClassCache::new(capacity),
            optimization_level: level,
            performance_metrics: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            error_metrics: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            usage_metrics: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    /// Optimize class generation with caching
    pub async fn optimize_class_generation(&mut self, classes: &[String]) -> Result<String> {
        let start = Instant::now();
        let cache_key = self.generate_cache_key(classes);

        // Try to get from cache first
        if let Some(cached_result) = self.class_cache.get(&cache_key).await {
            self.record_performance("class_generation_cached", start.elapsed(), true)
                .await;
            return Ok(cached_result);
        }

        // Generate classes
        let result = self.generate_classes(classes).await?;

        // Cache the result
        self.class_cache.put(cache_key, result.clone()).await;

        self.record_performance("class_generation", start.elapsed(), true)
            .await;
        Ok(result)
    }

    /// Optimize CSS generation with caching
    pub async fn optimize_css_generation(&mut self, css: &str) -> Result<String> {
        let start = Instant::now();
        let cache_key = format!("css:{}", css);

        // Try to get from cache first
        if let Some(cached_result) = self.css_cache.get(&cache_key).await {
            self.record_performance("css_generation_cached", start.elapsed(), true)
                .await;
            return Ok(cached_result);
        }

        // Generate optimized CSS
        let result = self.optimize_css(css).await?;

        // Cache the result
        self.css_cache.put(cache_key, result.clone()).await;

        self.record_performance("css_generation", start.elapsed(), true)
            .await;
        Ok(result)
    }

    /// Generate cache key for classes
    fn generate_cache_key(&self, classes: &[String]) -> String {
        let mut key = String::new();
        for class in classes {
            key.push_str(class);
            key.push('|');
        }
        key
    }

    /// Generate classes with optimization
    async fn generate_classes(&self, classes: &[String]) -> Result<String> {
        // Apply optimization based on level
        let optimized_classes = match self.optimization_level {
            OptimizationLevel::None => classes.to_vec(),
            OptimizationLevel::Low => self.optimize_classes_low(classes).await,
            OptimizationLevel::Medium => self.optimize_classes_medium(classes).await,
            OptimizationLevel::High => self.optimize_classes_high(classes).await,
            OptimizationLevel::Maximum => self.optimize_classes_maximum(classes).await,
        };

        Ok(optimized_classes.join(" "))
    }

    /// Low-level optimization
    async fn optimize_classes_low(&self, classes: &[String]) -> Vec<String> {
        // Remove duplicates
        let mut unique_classes: Vec<String> = classes.to_vec();
        unique_classes.sort();
        unique_classes.dedup();
        unique_classes
    }

    /// Medium-level optimization
    async fn optimize_classes_medium(&self, classes: &[String]) -> Vec<String> {
        let mut optimized = self.optimize_classes_low(classes).await;

        // Remove conflicting classes
        optimized = self.remove_conflicting_classes(optimized).await;

        optimized
    }

    /// High-level optimization
    async fn optimize_classes_high(&self, classes: &[String]) -> Vec<String> {
        let mut optimized = self.optimize_classes_medium(classes).await;

        // Merge similar classes
        optimized = self.merge_similar_classes(optimized).await;

        optimized
    }

    /// Maximum-level optimization
    async fn optimize_classes_maximum(&self, classes: &[String]) -> Vec<String> {
        let mut optimized = self.optimize_classes_high(classes).await;

        // Apply advanced optimizations
        optimized = self.apply_advanced_optimizations(optimized).await;

        optimized
    }

    /// Remove conflicting classes
    async fn remove_conflicting_classes(&self, classes: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        let mut seen_groups: HashMap<String, String> = HashMap::new();

        for class in classes {
            let group = self.get_class_group(&class);
            if let Some(existing) = seen_groups.get(&group) {
                // Keep the more specific class
                if self.is_more_specific(&class, existing) {
                    if let Some(pos) = result.iter().position(|c| c == existing) {
                        result.remove(pos);
                    }
                    result.push(class.clone());
                    seen_groups.insert(group, class);
                }
            } else {
                result.push(class.clone());
                seen_groups.insert(group, class);
            }
        }

        result
    }

    /// Merge similar classes
    async fn merge_similar_classes(&self, classes: Vec<String>) -> Vec<String> {
        // This is a simplified implementation
        // In a real implementation, you would merge classes like:
        // "px-2", "px-4" -> "px-4" (keep the larger value)
        classes
    }

    /// Apply advanced optimizations
    async fn apply_advanced_optimizations(&self, classes: Vec<String>) -> Vec<String> {
        // This is a placeholder for advanced optimizations
        // In a real implementation, you might:
        // - Combine responsive classes
        // - Optimize color classes
        // - Merge spacing classes
        classes
    }

    /// Get class group for conflict detection
    fn get_class_group(&self, class: &str) -> String {
        if class.starts_with("bg-") {
            "background".to_string()
        } else if class.starts_with("text-") {
            "text".to_string()
        } else if class.starts_with("border-") {
            "border".to_string()
        } else if class.starts_with("p-") || class.starts_with("px-") || class.starts_with("py-") {
            "padding".to_string()
        } else if class.starts_with("m-") || class.starts_with("mx-") || class.starts_with("my-") {
            "margin".to_string()
        } else {
            "other".to_string()
        }
    }

    /// Check if one class is more specific than another
    fn is_more_specific(&self, class1: &str, class2: &str) -> bool {
        // Simple heuristic: longer class names are more specific
        class1.len() > class2.len()
    }

    /// Optimize CSS
    async fn optimize_css(&self, css: &str) -> Result<String> {
        let mut optimized = css.to_string();

        // Remove unnecessary whitespace
        optimized = optimized.replace("  ", " ");
        optimized = optimized.replace("\n", "");
        optimized = optimized.replace("\t", "");

        // Remove trailing semicolons
        optimized = optimized.replace(";}", "}");

        Ok(optimized)
    }

    /// Record performance metrics
    pub async fn record_performance(&self, operation: &str, duration: Duration, success: bool) {
        let metric = PerformanceMetric {
            operation: operation.to_string(),
            duration,
            timestamp: Instant::now(),
            success,
        };

        let mut metrics = self.performance_metrics.write().await;
        metrics.push(metric);

        // Keep only the last 1000 metrics
        let len = metrics.len();
        if len > 1000 {
            metrics.drain(0..len - 1000);
        }
    }

    /// Record error metrics
    pub async fn record_error(&self, error_type: &str, error: &dyn std::error::Error) {
        let metric = ErrorMetric {
            error_type: error_type.to_string(),
            error_message: error.to_string(),
            timestamp: Instant::now(),
            count: 1,
        };

        let mut metrics = self.error_metrics.write().await;
        metrics.push(metric);

        // Keep only the last 1000 error metrics
        let len = metrics.len();
        if len > 1000 {
            metrics.drain(0..len - 1000);
        }
    }

    /// Record usage metrics
    pub async fn record_usage(&self, class_pattern: &str, generation_time: Duration) {
        let mut metrics = self.usage_metrics.write().await;

        if let Some(usage) = metrics.get_mut(class_pattern) {
            usage.usage_count += 1;
            usage.last_used = Instant::now();
            usage.average_generation_time = Duration::from_nanos(
                ((usage.average_generation_time.as_nanos() + generation_time.as_nanos()) / 2)
                    as u64,
            );
        } else {
            metrics.insert(
                class_pattern.to_string(),
                UsageMetric {
                    class_pattern: class_pattern.to_string(),
                    usage_count: 1,
                    last_used: Instant::now(),
                    average_generation_time: generation_time,
                },
            );
        }
    }

    /// Get performance metrics
    pub async fn get_performance_metrics(&self) -> Vec<PerformanceMetric> {
        let metrics = self.performance_metrics.read().await;
        metrics.clone()
    }

    /// Get error metrics
    pub async fn get_error_metrics(&self) -> Vec<ErrorMetric> {
        let metrics = self.error_metrics.read().await;
        metrics.clone()
    }

    /// Get usage metrics
    pub async fn get_usage_metrics(&self) -> HashMap<String, UsageMetric> {
        let metrics = self.usage_metrics.read().await;
        metrics.clone()
    }

    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> CacheStats {
        CacheStats {
            class_cache_hit_rate: self.class_cache.hit_rate(),
            class_cache_miss_rate: self.class_cache.miss_rate(),
            class_cache_total_requests: self.class_cache.total_requests(),
            class_cache_size: self.class_cache.len().await,
            css_cache_hit_rate: self.css_cache.hit_rate(),
            css_cache_miss_rate: self.css_cache.miss_rate(),
            css_cache_total_requests: self.css_cache.total_requests(),
            css_cache_size: self.css_cache.len().await,
        }
    }

    /// Set optimization level
    pub fn set_optimization_level(&mut self, level: OptimizationLevel) {
        self.optimization_level = level;
    }

    /// Get optimization level
    pub fn optimization_level(&self) -> OptimizationLevel {
        self.optimization_level
    }
}

impl Default for PerformanceOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub class_cache_hit_rate: f64,
    pub class_cache_miss_rate: f64,
    pub class_cache_total_requests: u64,
    pub class_cache_size: usize,
    pub css_cache_hit_rate: f64,
    pub css_cache_miss_rate: f64,
    pub css_cache_total_requests: u64,
    pub css_cache_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimization_level() {
        assert_eq!(OptimizationLevel::None.cache_capacity(), 0);
        assert_eq!(OptimizationLevel::Low.cache_capacity(), 100);
        assert_eq!(OptimizationLevel::Medium.cache_capacity(), 500);
        assert_eq!(OptimizationLevel::High.cache_capacity(), 1000);
        assert_eq!(OptimizationLevel::Maximum.cache_capacity(), 5000);

        assert_eq!(OptimizationLevel::None.optimization_factor(), 1.0);
        assert_eq!(OptimizationLevel::Low.optimization_factor(), 1.2);
        assert_eq!(OptimizationLevel::Medium.optimization_factor(), 1.5);
        assert_eq!(OptimizationLevel::High.optimization_factor(), 2.0);
        assert_eq!(OptimizationLevel::Maximum.optimization_factor(), 3.0);
    }

    #[test]
    fn test_performance_optimizer_creation() {
        let optimizer = PerformanceOptimizer::new();
        assert_eq!(optimizer.optimization_level(), OptimizationLevel::Medium);
    }

    #[test]
    fn test_performance_optimizer_with_level() {
        let optimizer = PerformanceOptimizer::with_optimization_level(OptimizationLevel::High);
        assert_eq!(optimizer.optimization_level(), OptimizationLevel::High);
    }
}
