//! Utility class implementations for tailwind-rs
//!
//! This module contains all the utility class implementations organized by category.
//! Each utility category follows a consistent pattern with traits and implementations.

pub mod advanced_animation_utilities;
pub mod advanced_performance_optimization;
pub mod advanced_plugin_system_modules;
pub mod animations_modules;
pub mod background_utilities;
pub mod borders;
pub mod color_functions;
pub mod colors;
pub mod container_queries;
pub mod css_nesting;
pub mod device_variants;
pub mod effects;
pub mod enhanced_backdrop_filters;
pub mod enhanced_validation_module;
pub mod filters_modules;
pub mod flexbox;
pub mod grid;
pub mod interactivity;
pub mod layout;
pub mod logical_properties;
pub mod mask;
pub mod modern_css_features;
pub mod performance_optimization;
pub mod performance_optimization_module;
pub mod sizing;
pub mod spacing_module;
pub mod text_shadow;
pub mod transforms;
pub mod transitions;
pub mod typography;

// Re-export all utility traits for easy access
// Note: Avoiding glob re-exports to prevent naming conflicts
pub use advanced_animation_utilities::AdvancedAnimationUtilities;
pub use advanced_performance_optimization::*;
pub use advanced_plugin_system_modules::*;
pub use animations_modules::AnimationUtilities;
pub use color_functions::*;
pub use container_queries::*;
pub use css_nesting::*;
pub use device_variants::*;
pub use enhanced_backdrop_filters::*;
pub use enhanced_validation_module::*;
pub use flexbox::*;
pub use grid::*;
pub use interactivity::*;
pub use layout::*;
pub use logical_properties::*;
pub use mask::*;
pub use modern_css_features::*;
pub use sizing::*;
pub use spacing_module::*;
pub use text_shadow::*;
pub use transforms::*;
pub use transitions::*;
pub use typography::*;
