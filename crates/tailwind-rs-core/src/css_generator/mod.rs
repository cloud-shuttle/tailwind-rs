//! CSS Generator Module
//!
//! This module provides the main CSS generation interface,
//! orchestrating the various parser modules to generate CSS.
//!
//! ## Example
//!
//! ```rust
//! use tailwind_rs_core::CssGenerator;
//!
//! // Create a CSS generator
//! let mut generator = CssGenerator::new();
//!
//! // Add classes (returns Result<()>)
//! generator.add_class("bg-blue-500").unwrap();
//! generator.add_class("text-white").unwrap();
//! generator.add_class("hover:bg-blue-600").unwrap();
//!
//! // Generate CSS
//! let css = generator.generate_css();
//! // Returns actual CSS with properties like:
//! // .bg-blue-500 { background-color: #3b82f6; }
//! // .text-white { color: #ffffff; }
//! // .hover\:bg-blue-600:hover { background-color: #2563eb; }
//! */

// CSS functions for @apply, @layer, @import
// Note: css_functions is declared in the parent lib.rs

// Core modules (refactored)
pub mod bridge;
pub mod caching;
pub mod core;
pub mod processing;

// Legacy modules (to be gradually migrated)
// Re-enabled essential modules, disabled only problematic ones
pub mod color_cache;
pub mod core_parsers;
pub mod css_output;
pub mod element_context;
#[cfg(test)]
pub mod element_context_test;
pub mod flexbox_parsers;
pub mod generator;
pub mod generator_operations;
// pub mod generator_parsers;  // Disabled - has compilation issues
pub mod grid_parsers;
pub mod plugin_system;
pub mod trie;
pub mod trie_benchmark;
pub mod layout_parsers;
pub mod parsers;
pub mod types;
pub mod utils;
pub mod variants;

// Re-export main types and functionality (new refactored structure)
pub use core::CssGenerator;
pub use processing::CssOutputGenerator;
pub use parsers::*;
pub use types::{CssGenerationConfig, CssProperty, CssRule};
pub use variants::VariantParser;

// Re-export legacy types for backward compatibility
pub use generator::CssGenerator as LegacyCssGenerator;
