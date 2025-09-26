//! CSS Generator Module
//!
//! This module provides the main CSS generation interface,
//! orchestrating the various parser modules to generate CSS.

pub mod core;
pub mod css_output;
pub mod generator;
pub mod generator_builders;
pub mod generator_operations;
pub mod generator_parsers;
pub mod parsers;
pub mod types;
pub mod utils;
pub mod variants;

// Re-export main types and functionality
pub use css_output::CssOutputGenerator;
pub use generator::CssGenerator;
pub use parsers::*;
pub use types::{CssGenerationConfig, CssProperty, CssRule};
pub use variants::VariantParser;
