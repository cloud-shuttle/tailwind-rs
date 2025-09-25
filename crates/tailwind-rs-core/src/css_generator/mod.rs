//! CSS Generator Module
//!
//! This module provides the main CSS generation interface,
//! orchestrating the various parser modules to generate CSS.

pub mod types;
pub mod generator;
pub mod css_output;
pub mod core;
pub mod variants;
pub mod utils;
pub mod parsers;

// Re-export main types and functionality
pub use types::{CssRule, CssProperty, CssGenerationConfig};
pub use generator::CssGenerator;
pub use css_output::CssOutputGenerator;
pub use variants::VariantParser;
pub use parsers::*;
