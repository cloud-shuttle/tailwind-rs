//! CSS Generator Parser Methods
//!
//! This module contains all the parser delegation methods for CssGenerator.
//! The implementation has been modularized into separate files for better maintainability.

use super::types::CssProperty;
use crate::error::{Result, TailwindError};

// Import the modularized components
use super::parsers::generator_parsers::*;

/// Parser methods trait for CssGenerator
pub trait CssGeneratorParsers {
    /// Convert a class name to CSS properties
    fn class_to_properties(&self, class: &str) -> Result<Vec<CssProperty>>;

    /// Parse variants from a class name and return (variants, base_class)
    fn parse_variants(&self, class: &str) -> (Vec<String>, String);

    /// Convert a class name to a CSS rule
    fn class_to_css_rule(&self, class: &str) -> Result<super::types::CssRule>;
}

impl CssGeneratorParsers for super::CssGenerator {
    fn class_to_properties(&self, class: &str) -> Result<Vec<CssProperty>> {
        // Delegate to the modularized core parsing logic
        crate::css_generator::core::CssGenerator::class_to_properties(self, class)
    }

    fn parse_variants(&self, class: &str) -> (Vec<String>, String) {
        // Delegate to the modularized utility methods
        crate::css_generator::core::CssGenerator::parse_variants(self, class)
    }

    fn class_to_css_rule(&self, class: &str) -> Result<super::types::CssRule> {
        // Delegate to the modularized utility methods
        crate::css_generator::core::CssGenerator::class_to_css_rule(self, class)
    }
}

// All specialized parser methods have been moved to the modularized components
// in the generator_parsers module for better organization and maintainability.
