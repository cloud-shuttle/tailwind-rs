//! Optimization Strategies
//!
//! This module provides optimization strategy definitions.

use serde::{Deserialize, Serialize};

/// Optimization strategy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OptimizationStrategy {
    /// Minify CSS
    Minification,
    /// Compress CSS
    Compression,
    /// Extract critical CSS
    CriticalCss,
    /// Optimize lazy loading
    LazyLoading,
    /// Split bundles
    BundleSplitting,
    /// Optimize memory usage
    MemoryOptimization,
    /// Monitor performance
    PerformanceMonitoring,
}
