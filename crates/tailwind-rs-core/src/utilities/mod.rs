//! Utility class implementations for tailwind-rs
//!
//! This module contains all the utility class implementations organized by category.
//! Each utility category follows a consistent pattern with traits and implementations.

pub mod advanced_animations;
pub mod advanced_performance_optimization;
pub mod advanced_plugin_system;
pub mod animations;
pub mod backgrounds;
pub mod borders;
pub mod color_functions;
pub mod colors;
pub mod container_queries;
pub mod css_nesting;
pub mod device_variants;
pub mod effects;
pub mod enhanced_backdrop_filters;
pub mod enhanced_validation;
pub mod filters;
pub mod flexbox;
pub mod grid;
pub mod interactivity;
pub mod layout;
pub mod logical_properties;
pub mod mask;
pub mod modern_css_features;
pub mod performance_optimization;
pub mod sizing;
pub mod spacing;
pub mod text_shadow;
pub mod transforms;
pub mod transitions;
pub mod typography;

// Re-export all utility traits for easy access
pub use advanced_animations::*;
pub use advanced_performance_optimization::*;
pub use advanced_plugin_system::*;
pub use animations::*;
pub use backgrounds::*;
pub use borders::*;
pub use color_functions::*;
pub use colors::*;
pub use container_queries::*;
pub use css_nesting::*;
pub use device_variants::*;
pub use effects::*;
pub use enhanced_backdrop_filters::*;
pub use enhanced_validation::*;
pub use filters::*;
pub use flexbox::*;
pub use grid::*;
pub use interactivity::*;
pub use layout::*;
pub use logical_properties::*;
pub use mask::*;
pub use modern_css_features::*;
pub use performance_optimization::*;
pub use sizing::*;
pub use spacing::*;
pub use text_shadow::*;
pub use transforms::*;
pub use transitions::*;
pub use typography::*;
