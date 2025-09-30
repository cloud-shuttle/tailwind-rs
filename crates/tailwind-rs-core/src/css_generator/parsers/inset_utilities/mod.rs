//! Inset Utilities Parser Module
//!
//! Comprehensive inset utilities parser that handles all inset-related Tailwind classes:
//! - inset-* (all sides), inset-x-* (horizontal), inset-y-* (vertical)
//! - top-*, right-*, bottom-*, left-* (individual positions)
//! - start-*, end-* (logical positions)
//! - Arbitrary values and custom properties (planned)

use crate::css_generator::types::CssProperty;
use super::super::ParserCategory;
use super::super::UtilityParser;

// Re-export all inset utility types and traits
pub mod arbitrary;
pub mod custom;
pub mod inset;
pub mod position;
pub mod values;

// Re-export all types for easy access
pub use arbitrary::ArbitraryInsetParser;
pub use custom::CustomInsetParser;
pub use inset::GeneralInsetParser;
pub use position::PositionInsetParser;
pub use values::InsetSpacingValues;

/// Main inset utilities parser that coordinates all inset parsing
#[derive(Debug, Clone)]
pub struct InsetParser {
    values: InsetSpacingValues,
    general_parser: GeneralInsetParser,
    position_parser: PositionInsetParser,
    arbitrary_parser: ArbitraryInsetParser,
    custom_parser: CustomInsetParser,
}

impl InsetParser {
    /// Create a new inset parser with all sub-parsers
    pub fn new() -> Self {
        let values = InsetSpacingValues::new();
        Self {
            general_parser: GeneralInsetParser::new(values.clone()),
            position_parser: PositionInsetParser::new(values.clone()),
            arbitrary_parser: ArbitraryInsetParser::new(values.clone()),
            custom_parser: CustomInsetParser::new(values.clone()),
            values,
        }
    }

    /// Parse inset class (all sides)
    pub fn parse_inset_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.general_parser.parse_inset_class(class)
    }

    /// Parse inset-x class (horizontal inset)
    pub fn parse_inset_x_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.general_parser.parse_inset_x_class(class)
    }

    /// Parse inset-y class (vertical inset)
    pub fn parse_inset_y_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.general_parser.parse_inset_y_class(class)
    }

    /// Parse top position class
    pub fn parse_top_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.position_parser.parse_top_class(class)
    }

    /// Parse right position class
    pub fn parse_right_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.position_parser.parse_right_class(class)
    }

    /// Parse bottom position class
    pub fn parse_bottom_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.position_parser.parse_bottom_class(class)
    }

    /// Parse left position class
    pub fn parse_left_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.position_parser.parse_left_class(class)
    }

    /// Parse start position class
    pub fn parse_start_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.position_parser.parse_start_class(class)
    }

    /// Parse end position class
    pub fn parse_end_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.position_parser.parse_end_class(class)
    }

    /// Parse arbitrary inset class
    pub fn parse_arbitrary_inset_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.arbitrary_parser.parse_arbitrary_inset_class(class)
    }

    /// Parse custom property inset class
    pub fn parse_custom_property_inset_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.custom_parser.parse_custom_property_inset_class(class)
    }

    /// Get access to the spacing values
    pub fn values(&self) -> &InsetSpacingValues {
        &self.values
    }
}

impl UtilityParser for InsetParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try parsing in order of specificity/precedence
        if let Some(properties) = self.parse_inset_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_inset_x_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_inset_y_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_top_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_right_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_bottom_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_left_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_start_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_end_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_arbitrary_inset_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_custom_property_inset_class(class) {
            return Some(properties);
        }

        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        // For now, return a static list of common inset patterns
        // TODO: Implement proper pattern collection when sub-parsers are fully implemented
        vec![
            "inset-", "inset-x-", "inset-y-", "top-", "right-", "bottom-", "left-", "start-", "end-",
        ]
    }

    fn get_priority(&self) -> u32 {
        80 // Inset utilities have high priority
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Spacing
    }
}

impl Default for InsetParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Inset utilities trait for extending ClassBuilder
pub trait InsetUtilities {
    /// Add inset class (all sides)
    fn inset(self, value: &str) -> Self;

    /// Add inset-x class (horizontal)
    fn inset_x(self, value: &str) -> Self;

    /// Add inset-y class (vertical)
    fn inset_y(self, value: &str) -> Self;

    /// Add top position class
    fn top(self, value: &str) -> Self;

    /// Add right position class
    fn right(self, value: &str) -> Self;

    /// Add bottom position class
    fn bottom(self, value: &str) -> Self;

    /// Add left position class
    fn left(self, value: &str) -> Self;

    /// Add start position class
    fn start(self, value: &str) -> Self;

    /// Add end position class
    fn end(self, value: &str) -> Self;
}

impl InsetUtilities for crate::classes::ClassBuilder {
    fn inset(self, value: &str) -> Self {
        self.class(&format!("inset-{}", value))
    }

    fn inset_x(self, value: &str) -> Self {
        self.class(&format!("inset-x-{}", value))
    }

    fn inset_y(self, value: &str) -> Self {
        self.class(&format!("inset-y-{}", value))
    }

    fn top(self, value: &str) -> Self {
        self.class(&format!("top-{}", value))
    }

    fn right(self, value: &str) -> Self {
        self.class(&format!("right-{}", value))
    }

    fn bottom(self, value: &str) -> Self {
        self.class(&format!("bottom-{}", value))
    }

    fn left(self, value: &str) -> Self {
        self.class(&format!("left-{}", value))
    }

    fn start(self, value: &str) -> Self {
        self.class(&format!("start-{}", value))
    }

    fn end(self, value: &str) -> Self {
        self.class(&format!("end-{}", value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inset_parser_creation() {
        let parser = InsetParser::new();
        // Parser should be created successfully
        assert!(true); // Placeholder - would test actual functionality
    }

    #[test]
    fn utility_parser_implementation() {
        let parser = InsetParser::new();

        assert_eq!(parser.get_priority(), 80);
        assert_eq!(parser.get_category(), ParserCategory::Spacing);

        let patterns = parser.get_supported_patterns();
        assert!(!patterns.is_empty());
        // Should contain various inset patterns
        assert!(patterns.iter().any(|p| p.contains("inset-")));
        assert!(patterns.iter().any(|p| p.contains("top-")));
    }

    #[test]
    fn general_parsing_delegation() {
        let parser = InsetParser::new();

        // Test that general parsing works
        let result = parser.parse_inset_class("inset-4");
        assert!(result.is_some());

        let result = parser.parse_inset_x_class("inset-x-2");
        assert!(result.is_some());

        let result = parser.parse_inset_y_class("inset-y-1");
        assert!(result.is_some());
    }

    #[test]
    fn position_parsing_delegation() {
        let parser = InsetParser::new();

        // Test that position parsing works
        let result = parser.parse_top_class("top-4");
        assert!(result.is_some());

        let result = parser.parse_right_class("right-2");
        assert!(result.is_some());

        let result = parser.parse_bottom_class("bottom-1");
        assert!(result.is_some());

        let result = parser.parse_left_class("left-3");
        assert!(result.is_some());

        let result = parser.parse_start_class("start-4");
        assert!(result.is_some());

        let result = parser.parse_end_class("end-2");
        assert!(result.is_some());
    }

    #[test]
    fn comprehensive_inset_parsing() {
        let parser = InsetParser::new();

        // Test that the main dispatcher works for implemented parsers
        let result = parser.parse_class("inset-4");
        assert!(result.is_some());

        let result = parser.parse_class("inset-x-2");
        assert!(result.is_some());

        let result = parser.parse_class("top-1");
        assert!(result.is_some());

        let result = parser.parse_class("right-3");
        assert!(result.is_some());

        let result = parser.parse_class("invalid-inset-class");
        assert!(result.is_none());
    }

    #[test]
    fn inset_utilities_trait() {
        use crate::classes::ClassBuilder;

        let builder = ClassBuilder::new();

        // Test trait methods (simplified - would need actual ClassBuilder implementation)
        let _result = builder.inset("4").top("2").left("1");
        // In a real implementation, this would add classes to the builder
    }
}
