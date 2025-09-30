//! Spacing Utilities Module
//!
//! This module contains the core utility functions for parsing and generating
//! spacing-related CSS properties.

use crate::css_generator::types::CssProperty;
use super::{values::*, constants::*};

/// Core spacing parsing utilities
#[derive(Debug, Clone)]
pub struct SpacingUtilities {
    values: SpacingValues,
    config: SpacingParserConfig,
}

impl SpacingUtilities {
    /// Create new spacing utilities with default configuration
    pub fn new() -> Self {
        Self {
            values: SpacingValues::new(),
            config: SpacingParserConfig::default(),
        }
    }

    /// Create spacing utilities with custom configuration
    pub fn with_config(config: SpacingParserConfig) -> Self {
        Self {
            values: SpacingValues::new(),
            config,
        }
    }

    /// Get spacing value for a token
    pub fn get_spacing_value(&self, token: &str) -> Option<String> {
        // First check standard spacing values
        if let Some(value) = self.values.get_value(token) {
            return Some(value);
        }

        // Check for arbitrary values if enabled
        if self.config.arbitrary_values_enabled() {
            if let Some(arbitrary) = SpacingValueUtils::parse_arbitrary_value(token) {
                return Some(arbitrary);
            }
        }

        None
    }

    /// Get the parser configuration
    pub fn config(&self) -> &SpacingParserConfig {
        &self.config
    }

    /// Parse padding class and return CSS properties
    pub fn parse_padding_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try different padding patterns in order of specificity
        self.parse_all_padding(class)
            .or_else(|| self.parse_axis_padding(class))
            .or_else(|| self.parse_directional_padding(class))
    }

    /// Parse margin class and return CSS properties
    pub fn parse_margin_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try different margin patterns in order of specificity
        self.parse_all_margin(class)
            .or_else(|| self.parse_axis_margin(class))
            .or_else(|| self.parse_directional_margin(class))
    }

    /// Parse gap class and return CSS properties
    pub fn parse_gap_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try gap patterns
        self.parse_all_gap(class)
            .or_else(|| self.parse_axis_gap(class))
    }

    /// Parse all-direction padding (p-*)
    fn parse_all_padding(&self, class: &str) -> Option<Vec<CssProperty>> {
        class.strip_prefix(SpacingPrefixes::PADDING_ALL)
            .and_then(|value| self.get_spacing_value(value))
            .map(|spacing| vec![CssProperty {
                name: "padding".to_string(),
                value: spacing,
                important: false,
            }])
    }

    /// Parse axis padding (px-*, py-*)
    fn parse_axis_padding(&self, class: &str) -> Option<Vec<CssProperty>> {
        if !self.config.axis_properties_enabled() {
            return None;
        }

        if let Some(value) = class.strip_prefix(SpacingPrefixes::PADDING_X) {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![
                    CssProperty {
                        name: "padding-left".to_string(),
                        value: spacing.clone(),
                        important: false,
                    },
                    CssProperty {
                        name: "padding-right".to_string(),
                        value: spacing,
                        important: false,
                    },
                ]);
            }
        }

        if let Some(value) = class.strip_prefix(SpacingPrefixes::PADDING_Y) {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![
                    CssProperty {
                        name: "padding-top".to_string(),
                        value: spacing.clone(),
                        important: false,
                    },
                    CssProperty {
                        name: "padding-bottom".to_string(),
                        value: spacing,
                        important: false,
                    },
                ]);
            }
        }

        None
    }

    /// Parse directional padding (pt-*, pr-*, pb-*, pl-*, ps-*, pe-*)
    fn parse_directional_padding(&self, class: &str) -> Option<Vec<CssProperty>> {
        let patterns = [
            (SpacingPrefixes::PADDING_TOP, "t"),
            (SpacingPrefixes::PADDING_RIGHT, "r"),
            (SpacingPrefixes::PADDING_BOTTOM, "b"),
            (SpacingPrefixes::PADDING_LEFT, "l"),
        ];

        for (prefix, direction) in &patterns {
            if let Some(value) = class.strip_prefix(prefix) {
                if let Some(spacing) = self.get_spacing_value(value) {
                    if let Some(property) = SpacingDirections::get_padding_property(direction) {
                        return Some(vec![CssProperty {
                            name: property.to_string(),
                            value: spacing,
                            important: false,
                        }]);
                    }
                }
            }
        }

        // Logical properties if enabled
        if self.config.logical_properties_enabled() {
            let logical_patterns = [
                (SpacingPrefixes::PADDING_START, "s"),
                (SpacingPrefixes::PADDING_END, "e"),
            ];

            for (prefix, direction) in &logical_patterns {
                if let Some(value) = class.strip_prefix(prefix) {
                    if let Some(spacing) = self.get_spacing_value(value) {
                        if let Some(property) = SpacingDirections::get_padding_property(direction) {
                            return Some(vec![CssProperty {
                                name: property.to_string(),
                                value: spacing,
                                important: false,
                            }]);
                        }
                    }
                }
            }
        }

        None
    }

    /// Parse all-direction margin (m-*)
    fn parse_all_margin(&self, class: &str) -> Option<Vec<CssProperty>> {
        class.strip_prefix(SpacingPrefixes::MARGIN_ALL)
            .and_then(|value| self.get_spacing_value(value))
            .map(|spacing| vec![CssProperty {
                name: "margin".to_string(),
                value: spacing,
                important: false,
            }])
    }

    /// Parse axis margin (mx-*, my-*)
    fn parse_axis_margin(&self, class: &str) -> Option<Vec<CssProperty>> {
        if !self.config.axis_properties_enabled() {
            return None;
        }

        if let Some(value) = class.strip_prefix(SpacingPrefixes::MARGIN_X) {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![
                    CssProperty {
                        name: "margin-left".to_string(),
                        value: spacing.clone(),
                        important: false,
                    },
                    CssProperty {
                        name: "margin-right".to_string(),
                        value: spacing,
                        important: false,
                    },
                ]);
            }
        }

        if let Some(value) = class.strip_prefix(SpacingPrefixes::MARGIN_Y) {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![
                    CssProperty {
                        name: "margin-top".to_string(),
                        value: spacing.clone(),
                        important: false,
                    },
                    CssProperty {
                        name: "margin-bottom".to_string(),
                        value: spacing,
                        important: false,
                    },
                ]);
            }
        }

        None
    }

    /// Parse directional margin (mt-*, mr-*, mb-*, ml-*, ms-*, me-*)
    fn parse_directional_margin(&self, class: &str) -> Option<Vec<CssProperty>> {
        let patterns = [
            (SpacingPrefixes::MARGIN_TOP, "t"),
            (SpacingPrefixes::MARGIN_RIGHT, "r"),
            (SpacingPrefixes::MARGIN_BOTTOM, "b"),
            (SpacingPrefixes::MARGIN_LEFT, "l"),
        ];

        for (prefix, direction) in &patterns {
            if let Some(value) = class.strip_prefix(prefix) {
                if let Some(spacing) = self.get_spacing_value(value) {
                    if let Some(property) = SpacingDirections::get_margin_property(direction) {
                        return Some(vec![CssProperty {
                            name: property.to_string(),
                            value: spacing,
                            important: false,
                        }]);
                    }
                }
            }
        }

        // Logical properties if enabled
        if self.config.logical_properties_enabled() {
            let logical_patterns = [
                (SpacingPrefixes::MARGIN_START, "s"),
                (SpacingPrefixes::MARGIN_END, "e"),
            ];

            for (prefix, direction) in &logical_patterns {
                if let Some(value) = class.strip_prefix(prefix) {
                    if let Some(spacing) = self.get_spacing_value(value) {
                        if let Some(property) = SpacingDirections::get_margin_property(direction) {
                            return Some(vec![CssProperty {
                                name: property.to_string(),
                                value: spacing,
                                important: false,
                            }]);
                        }
                    }
                }
            }
        }

        None
    }

    /// Parse all-direction gap (gap-*)
    fn parse_all_gap(&self, class: &str) -> Option<Vec<CssProperty>> {
        class.strip_prefix(SpacingPrefixes::GAP_ALL)
            .and_then(|value| self.get_spacing_value(value))
            .map(|spacing| vec![CssProperty {
                name: "gap".to_string(),
                value: spacing,
                important: false,
            }])
    }

    /// Parse axis gap (gap-x-*, gap-y-*)
    fn parse_axis_gap(&self, class: &str) -> Option<Vec<CssProperty>> {
        if !self.config.axis_properties_enabled() {
            return None;
        }

        if let Some(value) = class.strip_prefix(SpacingPrefixes::GAP_X) {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "column-gap".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }

        if let Some(value) = class.strip_prefix(SpacingPrefixes::GAP_Y) {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "row-gap".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }

        None
    }

    /// Get supported patterns for this utility
    pub fn get_supported_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();

        // Padding patterns
        patterns.push(SpacingPrefixes::PADDING_ALL.to_string() + "*");
        if self.config.axis_properties_enabled() {
            patterns.push(SpacingPrefixes::PADDING_X.to_string() + "*");
            patterns.push(SpacingPrefixes::PADDING_Y.to_string() + "*");
        }
        patterns.push(SpacingPrefixes::PADDING_TOP.to_string() + "*");
        patterns.push(SpacingPrefixes::PADDING_RIGHT.to_string() + "*");
        patterns.push(SpacingPrefixes::PADDING_BOTTOM.to_string() + "*");
        patterns.push(SpacingPrefixes::PADDING_LEFT.to_string() + "*");
        if self.config.logical_properties_enabled() {
            patterns.push(SpacingPrefixes::PADDING_START.to_string() + "*");
            patterns.push(SpacingPrefixes::PADDING_END.to_string() + "*");
        }

        // Margin patterns
        patterns.push(SpacingPrefixes::MARGIN_ALL.to_string() + "*");
        if self.config.axis_properties_enabled() {
            patterns.push(SpacingPrefixes::MARGIN_X.to_string() + "*");
            patterns.push(SpacingPrefixes::MARGIN_Y.to_string() + "*");
        }
        patterns.push(SpacingPrefixes::MARGIN_TOP.to_string() + "*");
        patterns.push(SpacingPrefixes::MARGIN_RIGHT.to_string() + "*");
        patterns.push(SpacingPrefixes::MARGIN_BOTTOM.to_string() + "*");
        patterns.push(SpacingPrefixes::MARGIN_LEFT.to_string() + "*");
        if self.config.logical_properties_enabled() {
            patterns.push(SpacingPrefixes::MARGIN_START.to_string() + "*");
            patterns.push(SpacingPrefixes::MARGIN_END.to_string() + "*");
        }

        // Gap patterns
        patterns.push(SpacingPrefixes::GAP_ALL.to_string() + "*");
        if self.config.axis_properties_enabled() {
            patterns.push(SpacingPrefixes::GAP_X.to_string() + "*");
            patterns.push(SpacingPrefixes::GAP_Y.to_string() + "*");
        }

        patterns
    }
}

impl Default for SpacingUtilities {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spacing_utilities_creation() {
        let utils = SpacingUtilities::new();
        assert_eq!(utils.get_spacing_value("4"), Some("1rem".to_string()));
        assert_eq!(utils.get_spacing_value("invalid"), None);
    }

    #[test]
    fn padding_parsing() {
        let utils = SpacingUtilities::new();

        // All padding
        let result = utils.parse_all_padding("p-4");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "1rem");

        // Axis padding
        let result = utils.parse_axis_padding("px-2");
        assert!(result.is_some());
        let props = result.unwrap();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].name, "padding-left");
        assert_eq!(props[1].name, "padding-right");

        // Directional padding
        let result = utils.parse_directional_padding("pt-1");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].name, "padding-top");
    }

    #[test]
    fn margin_parsing() {
        let utils = SpacingUtilities::new();

        // All margin
        let result = utils.parse_all_margin("m-3");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "0.75rem");

        // Axis margin
        let result = utils.parse_axis_margin("my-2");
        assert!(result.is_some());
        let props = result.unwrap();
        assert_eq!(props.len(), 2);

        // Directional margin
        let result = utils.parse_directional_margin("ml-4");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].name, "margin-left");
    }

    #[test]
    fn gap_parsing() {
        let utils = SpacingUtilities::new();

        // All gap
        let result = utils.parse_all_gap("gap-4");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].name, "gap");

        // Axis gap
        let result = utils.parse_axis_gap("gap-x-2");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].name, "column-gap");
    }

    #[test]
    fn arbitrary_values() {
        let utils = SpacingUtilities::new();

        // Test arbitrary value parsing
        assert_eq!(utils.get_spacing_value("[10px]"), Some("10px".to_string()));
        assert_eq!(utils.get_spacing_value("[2rem]"), Some("2rem".to_string()));
        assert_eq!(utils.get_spacing_value("invalid[arbitrary]"), None);
    }

    #[test]
    fn configuration_respected() {
        let config = SpacingParserConfig {
            enable_arbitrary_values: false,
            enable_logical_properties: false,
            enable_axis_properties: false,
            max_spacing_scale: 96.0,
        };
        let utils = SpacingUtilities::with_config(config);

        // Should not parse arbitrary values
        assert_eq!(utils.get_spacing_value("[10px]"), None);

        // Should not parse axis properties
        assert!(utils.parse_axis_padding("px-4").is_none());

        // Should not parse logical properties
        assert!(utils.parse_directional_padding("ps-4").is_none());
    }

    #[test]
    fn supported_patterns() {
        let utils = SpacingUtilities::new();
        let patterns = utils.get_supported_patterns();

        assert!(patterns.contains(&"p-*".to_string()));
        assert!(patterns.contains(&"m-*".to_string()));
        assert!(patterns.contains(&"gap-*".to_string()));
        assert!(patterns.contains(&"pt-*".to_string()));
        assert!(patterns.contains(&"px-*".to_string()));
        assert!(patterns.contains(&"py-*".to_string()));
    }
}
