//! Position Inset Utilities Module
//!
//! Handles parsing of individual position inset utilities:
//! - top-*, right-*, bottom-*, left-*
//! - start-*, end-*

use crate::css_generator::types::CssProperty;
use super::values::InsetSpacingValues;

/// Position inset utilities parser
#[derive(Debug, Clone)]
pub struct PositionInsetParser {
    values: InsetSpacingValues,
}

impl PositionInsetParser {
    /// Create a new position inset parser
    pub fn new(values: InsetSpacingValues) -> Self {
        Self { values }
    }

    /// Parse top position class
    pub fn parse_top_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_position_class(class, "top-", "top")
    }

    /// Parse right position class
    pub fn parse_right_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_position_class(class, "right-", "right")
    }

    /// Parse bottom position class
    pub fn parse_bottom_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_position_class(class, "bottom-", "bottom")
    }

    /// Parse left position class
    pub fn parse_left_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_position_class(class, "left-", "left")
    }

    /// Parse start position class
    pub fn parse_start_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_position_class(class, "start-", "inset-inline-start")
    }

    /// Parse end position class
    pub fn parse_end_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_position_class(class, "end-", "inset-inline-end")
    }

    /// Generic position parsing method
    fn parse_position_class(&self, class: &str, prefix: &str, property_name: &str) -> Option<Vec<CssProperty>> {
        // Positive values
        if let Some(value) = class.strip_prefix(prefix) {
            if let Some(spacing_value) = self.values.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: property_name.to_string(),
                    value: spacing_value.clone(),
                    important: false,
                }]);
            }
            if let Some(fraction_value) = self.values.get_fraction_value(value) {
                return Some(vec![CssProperty {
                    name: property_name.to_string(),
                    value: fraction_value.clone(),
                    important: false,
                }]);
            }
        }

        // Negative values
        let negative_prefix = format!("-{}", prefix);
        if let Some(value) = class.strip_prefix(&negative_prefix) {
            if let Some(spacing_value) = self.values.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: property_name.to_string(),
                    value: format!("-{}", spacing_value),
                    important: false,
                }]);
            }
            if let Some(fraction_value) = self.values.get_fraction_value(value) {
                return Some(vec![CssProperty {
                    name: property_name.to_string(),
                    value: format!("-{}", fraction_value),
                    important: false,
                }]);
            }
        }

        // Auto values
        let auto_class = format!("{}auto", prefix.trim_end_matches('-'));
        if class == auto_class {
            return Some(vec![CssProperty {
                name: property_name.to_string(),
                value: "auto".to_string(),
                important: false,
            }]);
        }

        None
    }

    /// Get supported position inset patterns
    pub fn supported_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();
        let positions = ["top", "right", "bottom", "left", "start", "end"];

        for position in &positions {
            let prefix = format!("{}-", position);

            // Positive patterns
            for key in self.values.spacing_keys() {
                patterns.push(format!("{}{}", prefix, key));
            }
            for key in self.values.fraction_keys() {
                patterns.push(format!("{}{}", prefix, key));
            }

            // Negative patterns
            for key in self.values.spacing_keys() {
                patterns.push(format!("-{}{}", prefix, key));
            }
            for key in self.values.fraction_keys() {
                patterns.push(format!("-{}{}", prefix, key));
            }

            // Auto patterns
            patterns.push(format!("{}auto", position));
        }

        patterns
    }
}

impl Default for PositionInsetParser {
    fn default() -> Self {
        Self::new(InsetSpacingValues::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_parsing_top() {
        let parser = PositionInsetParser::new(InsetSpacingValues::new());

        // Test positive top
        let result = parser.parse_top_class("top-4");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "top");
        assert_eq!(properties[0].value, "1rem");

        // Test negative top
        let result = parser.parse_top_class("-top-2");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "-0.5rem");

        // Test top-auto
        let result = parser.parse_top_class("top-auto");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "auto");
    }

    #[test]
    fn position_parsing_right() {
        let parser = PositionInsetParser::new(InsetSpacingValues::new());

        let result = parser.parse_right_class("right-8");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "right");
        assert_eq!(properties[0].value, "2rem");
    }

    #[test]
    fn position_parsing_bottom() {
        let parser = PositionInsetParser::new(InsetSpacingValues::new());

        let result = parser.parse_bottom_class("bottom-1/2");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "bottom");
        assert_eq!(properties[0].value, "50%");
    }

    #[test]
    fn position_parsing_left() {
        let parser = PositionInsetParser::new(InsetSpacingValues::new());

        let result = parser.parse_left_class("left-full");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "left");
        assert_eq!(properties[0].value, "100%");
    }

    #[test]
    fn position_parsing_start_end() {
        let parser = PositionInsetParser::new(InsetSpacingValues::new());

        let result = parser.parse_start_class("start-4");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "inset-inline-start");
        assert_eq!(properties[0].value, "1rem");

        let result = parser.parse_end_class("end-2");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "inset-inline-end");
        assert_eq!(properties[0].value, "0.5rem");
    }

    #[test]
    fn invalid_position_classes() {
        let parser = PositionInsetParser::new(InsetSpacingValues::new());

        assert!(parser.parse_top_class("top-invalid").is_none());
        assert!(parser.parse_right_class("right-invalid").is_none());
        assert!(parser.parse_bottom_class("bottom-invalid").is_none());
        assert!(parser.parse_left_class("left-invalid").is_none());
        assert!(parser.parse_start_class("start-invalid").is_none());
        assert!(parser.parse_end_class("end-invalid").is_none());
    }

    #[test]
    fn supported_patterns_includes_all_positions() {
        let parser = PositionInsetParser::new(InsetSpacingValues::new());
        let patterns = parser.supported_patterns();

        // Should include patterns for all positions
        assert!(patterns.iter().any(|p| p.starts_with("top-")));
        assert!(patterns.iter().any(|p| p.starts_with("right-")));
        assert!(patterns.iter().any(|p| p.starts_with("bottom-")));
        assert!(patterns.iter().any(|p| p.starts_with("left-")));
        assert!(patterns.iter().any(|p| p.starts_with("start-")));
        assert!(patterns.iter().any(|p| p.starts_with("end-")));

        // Should include auto variants
        assert!(patterns.contains(&"top-auto".to_string()));
        assert!(patterns.contains(&"right-auto".to_string()));
        assert!(patterns.contains(&"bottom-auto".to_string()));
        assert!(patterns.contains(&"left-auto".to_string()));
        assert!(patterns.contains(&"start-auto".to_string()));
        assert!(patterns.contains(&"end-auto".to_string()));
    }
}
