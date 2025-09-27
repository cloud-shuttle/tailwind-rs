//! Advanced performance optimization utilities for tailwind-rs
//!
//! This module provides advanced optimization strategies including:
//! - CSS minification and compression
//! - Critical CSS extraction
//! - Lazy loading optimization
//! - Bundle splitting strategies
//! - Memory usage optimization
//! - Runtime performance monitoring

pub mod css_minifier;
pub mod critical_css;
pub mod lazy_loading;
pub mod bundle_splitting;
pub mod memory_optimization;
pub mod performance_monitoring;
pub mod compression;
pub mod optimization_strategies;

pub use css_minifier::AdvancedCssMinifier;
pub use critical_css::CriticalCssExtractor;
pub use lazy_loading::LazyLoadingOptimizer;
pub use bundle_splitting::BundleSplitter;
pub use memory_optimization::MemoryOptimizer;
pub use performance_monitoring::PerformanceMonitor;
pub use compression::CssCompressor;
pub use optimization_strategies::OptimizationStrategy;
