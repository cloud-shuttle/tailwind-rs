//! Filter Utilities Module
//!
//! Comprehensive filter utilities for Tailwind-RS:
//! - Blur, brightness, contrast, grayscale, invert, opacity, saturate, sepia filters
//! - Hue-rotate and drop-shadow filters
//! - Backdrop filters for background elements
//! - Custom and arbitrary filter values

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

// Re-export all filter utility components
pub mod types;
pub mod utilities;
pub mod parser;

// Re-export all types and utilities for easy access
pub use types::*;
pub use utilities::*;
pub use parser::*;

/// Main filter utilities parser that implements UtilityParser trait
#[derive(Debug, Clone)]
pub struct FilterUtilitiesParser;

impl FilterUtilitiesParser {
    pub fn new() -> Self {
        Self
    }
}

impl UtilityParser for FilterUtilitiesParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Use the parser module's implementation
        parser::FilterUtilitiesParser::new().parse_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "filter-", "blur-", "brightness-", "contrast-", "grayscale-", "invert-",
            "opacity-", "saturate-", "sepia-", "hue-rotate-", "drop-shadow-",
            "backdrop-blur-", "backdrop-brightness-", "backdrop-contrast-",
            "backdrop-grayscale-", "backdrop-invert-", "backdrop-opacity-",
            "backdrop-saturate-", "backdrop-sepia-", "backdrop-hue-rotate-",
        ]
    }

    fn get_priority(&self) -> u32 {
        30 // Higher priority than basic utilities
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Effects
    }
}

impl Default for FilterUtilitiesParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for complex filter configurations
#[derive(Debug, Clone)]
pub struct FilterBuilder {
    filters: Vec<String>,
}

impl FilterBuilder {
    /// Create a new filter builder
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    /// Add a blur filter
    pub fn blur(mut self, value: &str) -> Self {
        self.filters.push(format!("blur({})", value));
        self
    }

    /// Add a brightness filter
    pub fn brightness(mut self, value: &str) -> Self {
        self.filters.push(format!("brightness({})", value));
        self
    }

    /// Add a contrast filter
    pub fn contrast(mut self, value: &str) -> Self {
        self.filters.push(format!("contrast({})", value));
        self
    }

    /// Add a grayscale filter
    pub fn grayscale(mut self, value: &str) -> Self {
        self.filters.push(format!("grayscale({})", value));
        self
    }

    /// Add a hue-rotate filter
    pub fn hue_rotate(mut self, value: &str) -> Self {
        self.filters.push(format!("hue-rotate({})", value));
        self
    }

    /// Add a drop-shadow filter
    pub fn drop_shadow(mut self, value: &str) -> Self {
        self.filters.push(format!("drop-shadow({})", value));
        self
    }

    /// Add a sepia filter
    pub fn sepia(mut self, value: &str) -> Self {
        self.filters.push(format!("sepia({})", value));
        self
    }

    /// Build the CSS filter property
    pub fn build(&self) -> Option<CssProperty> {
        if self.filters.is_empty() {
            None
        } else {
            let value = self.filters.join(" ");
            Some(FilterCssGenerator::generate_filter_property(&value))
        }
    }

    /// Clear all filters
    pub fn clear(mut self) -> Self {
        self.filters.clear();
        self
    }
}

impl Default for FilterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for filter combinations
pub mod filter_combinations {
    use super::*;

    /// Create a standard blur + drop-shadow combination
    pub fn soft_shadow() -> FilterBuilder {
        FilterBuilder::new()
            .blur("2px")
            .drop_shadow("0 2px 4px rgba(0,0,0,0.1)")
    }

    /// Create a dramatic blur effect
    pub fn dramatic_blur() -> FilterBuilder {
        FilterBuilder::new()
            .blur("10px")
            .brightness("120%")
            .contrast("110%")
    }

    /// Create a vintage sepia effect
    pub fn vintage_sepia() -> FilterBuilder {
        FilterBuilder::new()
            .sepia("70%")
            .contrast("110%")
            .brightness("110%")
    }

    /// Create a high contrast effect
    pub fn high_contrast() -> FilterBuilder {
        FilterBuilder::new()
            .contrast("200%")
            .brightness("90%")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_utilities_parser_trait() {
        let parser = FilterUtilitiesParser::new();

        // Test supported patterns
        let patterns = parser.get_supported_patterns();
        assert!(patterns.contains(&"blur-"));
        assert!(patterns.contains(&"backdrop-blur-"));
        assert!(patterns.contains(&"filter-"));

        // Test priority
        assert_eq!(parser.get_priority(), 30);

        // Test category
        assert!(matches!(parser.get_category(), ParserCategory::Filter));
    }

    #[test]
    fn filter_builder_operations() {
        let builder = FilterBuilder::new()
            .blur("5px")
            .brightness("120%")
            .drop_shadow("0 2px 4px rgba(0,0,0,0.1)");

        let property = builder.build().unwrap();
        assert_eq!(property.name, "filter");
        assert!(property.value.contains("blur(5px)"));
        assert!(property.value.contains("brightness(120%)"));
        assert!(property.value.contains("drop-shadow(0 2px 4px rgba(0,0,0,0.1))"));
    }

    #[test]
    fn filter_combinations() {
        let soft_shadow = filter_combinations::soft_shadow();
        let property = soft_shadow.build().unwrap();
        assert!(property.value.contains("blur(2px)"));
        assert!(property.value.contains("drop-shadow"));

        let vintage = filter_combinations::vintage_sepia();
        let property = vintage.build().unwrap();
        assert!(property.value.contains("sepia(70%)"));
        assert!(property.value.contains("contrast(110%)"));
    }

    #[test]
    fn filter_builder_clear() {
        let builder = FilterBuilder::new()
            .blur("5px")
            .brightness("120%");

        let property1 = builder.build().unwrap();
        assert!(property1.value.contains("blur(5px)"));

        let cleared_builder = FilterBuilder::new().clear();
        let property2 = cleared_builder.build();
        assert!(property2.is_none());
    }
}
