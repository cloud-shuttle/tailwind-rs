//! Caching components for performance optimization

pub mod rule_cache;

// Re-export color cache from top level
pub use crate::css_generator::color_cache::ColorCache;
pub use rule_cache::RuleCache;
