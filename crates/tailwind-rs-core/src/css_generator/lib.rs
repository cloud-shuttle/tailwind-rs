//! CSS Generator - Main Interface
//! 
//! This module provides the main CSS generation interface,
//! orchestrating the various parser modules to generate CSS.

pub mod core;
pub mod variants;
pub mod utils;
pub mod parsers;

pub use core::CssGenerator;
pub use variants::VariantParser;
pub use parsers::*;

// Re-export main types for backward compatibility
pub use core::{CssRule, CssProperty};
