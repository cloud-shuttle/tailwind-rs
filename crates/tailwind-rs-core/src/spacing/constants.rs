//! Spacing Constants Module
//!
//! This module defines constants and patterns used throughout the spacing system.

/// Supported spacing patterns
pub const SPACING_PATTERNS: &[&str] = &[
    // Padding patterns
    "p-*", "px-*", "py-*", "pt-*", "pr-*", "pb-*", "pl-*",
    "ps-*", "pe-*", // logical properties

    // Margin patterns
    "m-*", "mx-*", "my-*", "mt-*", "mr-*", "mb-*", "ml-*",
    "ms-*", "me-*", // logical properties

    // Gap patterns
    "gap-*", "gap-x-*", "gap-y-*",
];

/// Spacing property names
pub const SPACING_PROPERTIES: &[&str] = &[
    "padding", "padding-top", "padding-right", "padding-bottom", "padding-left",
    "padding-inline-start", "padding-inline-end",
    "margin", "margin-top", "margin-right", "margin-bottom", "margin-left",
    "margin-inline-start", "margin-inline-end",
    "gap", "row-gap", "column-gap",
];

/// Spacing direction mappings
pub struct SpacingDirections;

impl SpacingDirections {
    /// Get CSS property for directional padding
    pub fn get_padding_property(direction: &str) -> Option<&'static str> {
        match direction {
            "t" => Some("padding-top"),
            "r" => Some("padding-right"),
            "b" => Some("padding-bottom"),
            "l" => Some("padding-left"),
            "s" => Some("padding-inline-start"),
            "e" => Some("padding-inline-end"),
            _ => None,
        }
    }

    /// Get CSS property for directional margin
    pub fn get_margin_property(direction: &str) -> Option<&'static str> {
        match direction {
            "t" => Some("margin-top"),
            "r" => Some("margin-right"),
            "b" => Some("margin-bottom"),
            "l" => Some("margin-left"),
            "s" => Some("margin-inline-start"),
            "e" => Some("margin-inline-end"),
            _ => None,
        }
    }

    /// Get all supported directions
    pub fn all_directions() -> &'static [&'static str] {
        &["t", "r", "b", "l", "s", "e"]
    }
}

/// Spacing axis mappings
pub struct SpacingAxes;

impl SpacingAxes {
    /// Get CSS properties for axis padding
    pub fn get_padding_properties(axis: &str) -> Option<Vec<&'static str>> {
        match axis {
            "x" => Some(vec!["padding-left", "padding-right"]),
            "y" => Some(vec!["padding-top", "padding-bottom"]),
            _ => None,
        }
    }

    /// Get CSS properties for axis margin
    pub fn get_margin_properties(axis: &str) -> Option<Vec<&'static str>> {
        match axis {
            "x" => Some(vec!["margin-left", "margin-right"]),
            "y" => Some(vec!["margin-top", "margin-bottom"]),
            _ => None,
        }
    }

    /// Get CSS properties for axis gap
    pub fn get_gap_properties(axis: &str) -> Option<Vec<&'static str>> {
        match axis {
            "x" => Some(vec!["column-gap"]),
            "y" => Some(vec!["row-gap"]),
            _ => None,
        }
    }

    /// Get all supported axes
    pub fn all_axes() -> &'static [&'static str] {
        &["x", "y"]
    }
}

/// Spacing prefixes for different utilities
pub struct SpacingPrefixes;

impl SpacingPrefixes {
    pub const PADDING_ALL: &str = "p-";
    pub const PADDING_X: &str = "px-";
    pub const PADDING_Y: &str = "py-";
    pub const PADDING_TOP: &str = "pt-";
    pub const PADDING_RIGHT: &str = "pr-";
    pub const PADDING_BOTTOM: &str = "pb-";
    pub const PADDING_LEFT: &str = "pl-";
    pub const PADDING_START: &str = "ps-";
    pub const PADDING_END: &str = "pe-";

    pub const MARGIN_ALL: &str = "m-";
    pub const MARGIN_X: &str = "mx-";
    pub const MARGIN_Y: &str = "my-";
    pub const MARGIN_TOP: &str = "mt-";
    pub const MARGIN_RIGHT: &str = "mr-";
    pub const MARGIN_BOTTOM: &str = "mb-";
    pub const MARGIN_LEFT: &str = "ml-";
    pub const MARGIN_START: &str = "ms-";
    pub const MARGIN_END: &str = "me-";

    pub const GAP_ALL: &str = "gap-";
    pub const GAP_X: &str = "gap-x-";
    pub const GAP_Y: &str = "gap-y-";
}

/// Spacing priority levels for CSS cascade
pub struct SpacingPriority {
    pub padding: u32,
    pub margin: u32,
    pub gap: u32,
}

impl Default for SpacingPriority {
    fn default() -> Self {
        Self {
            padding: 100,
            margin: 100,
            gap: 100,
        }
    }
}

/// Spacing validation rules
pub struct SpacingValidation;

impl SpacingValidation {
    /// Maximum spacing value to prevent performance issues
    pub const MAX_SPACING_VALUE: f32 = 1000.0;

    /// Maximum arbitrary value length
    pub const MAX_ARBITRARY_LENGTH: usize = 50;

    /// Validate spacing token is not too large
    pub fn validate_spacing_token(token: &str) -> Result<(), &'static str> {
        if let Ok(value) = token.parse::<f32>() {
            if value > Self::MAX_SPACING_VALUE {
                return Err("Spacing value too large");
            }
        }

        if token.len() > Self::MAX_ARBITRARY_LENGTH {
            return Err("Arbitrary value too long");
        }

        Ok(())
    }

    /// Validate arbitrary spacing value format
    pub fn validate_arbitrary_value(value: &str) -> Result<(), &'static str> {
        if value.is_empty() {
            return Err("Empty arbitrary value");
        }

        if value.len() > Self::MAX_ARBITRARY_LENGTH {
            return Err("Arbitrary value too long");
        }

        // Basic CSS value validation
        if !value.chars().all(|c| {
            c.is_alphanumeric() || c == '.' || c == '-' || c == '%' ||
            c == 'p' || c == 'x' || c == 'r' || c == 'e' || c == 'm' || c == 'v' || c == 'h' || c == 'i' || c == 'n'
        }) {
            return Err("Invalid characters in arbitrary value");
        }

        Ok(())
    }
}

/// Spacing parser configuration
#[derive(Debug, Clone)]
pub struct SpacingParserConfig {
    pub enable_arbitrary_values: bool,
    pub enable_logical_properties: bool,
    pub enable_axis_properties: bool,
    pub max_spacing_scale: f32,
}

impl Default for SpacingParserConfig {
    fn default() -> Self {
        Self {
            enable_arbitrary_values: true,
            enable_logical_properties: true,
            enable_axis_properties: true,
            max_spacing_scale: 96.0,
        }
    }
}

impl SpacingParserConfig {
    /// Check if a spacing value is allowed by configuration
    pub fn is_spacing_allowed(&self, value: f32) -> bool {
        value <= self.max_spacing_scale
    }

    /// Check if arbitrary values are enabled
    pub fn arbitrary_values_enabled(&self) -> bool {
        self.enable_arbitrary_values
    }

    /// Check if logical properties are enabled
    pub fn logical_properties_enabled(&self) -> bool {
        self.enable_logical_properties
    }

    /// Check if axis properties are enabled
    pub fn axis_properties_enabled(&self) -> bool {
        self.enable_axis_properties
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spacing_patterns_complete() {
        assert!(SPACING_PATTERNS.contains(&"p-*"));
        assert!(SPACING_PATTERNS.contains(&"m-*"));
        assert!(SPACING_PATTERNS.contains(&"gap-*"));
        assert!(SPACING_PATTERNS.contains(&"px-*"));
        assert!(SPACING_PATTERNS.contains(&"py-*"));
    }

    #[test]
    fn spacing_properties_complete() {
        assert!(SPACING_PROPERTIES.contains(&"padding"));
        assert!(SPACING_PROPERTIES.contains(&"margin"));
        assert!(SPACING_PROPERTIES.contains(&"gap"));
        assert!(SPACING_PROPERTIES.contains(&"padding-top"));
        assert!(SPACING_PROPERTIES.contains(&"margin-left"));
    }

    #[test]
    fn spacing_directions_mapping() {
        assert_eq!(SpacingDirections::get_padding_property("t"), Some("padding-top"));
        assert_eq!(SpacingDirections::get_padding_property("r"), Some("padding-right"));
        assert_eq!(SpacingDirections::get_padding_property("x"), None); // x is axis, not direction

        assert_eq!(SpacingDirections::get_margin_property("l"), Some("margin-left"));
        assert_eq!(SpacingDirections::get_margin_property("s"), Some("margin-inline-start"));
    }

    #[test]
    fn spacing_axes_mapping() {
        let x_padding = SpacingAxes::get_padding_properties("x");
        assert_eq!(x_padding, Some(vec!["padding-left", "padding-right"]));

        let y_margin = SpacingAxes::get_margin_properties("y");
        assert_eq!(y_margin, Some(vec!["margin-top", "margin-bottom"]));

        let x_gap = SpacingAxes::get_gap_properties("x");
        assert_eq!(x_gap, Some(vec!["column-gap"]));
    }

    #[test]
    fn spacing_prefixes_defined() {
        assert_eq!(SpacingPrefixes::PADDING_ALL, "p-");
        assert_eq!(SpacingPrefixes::MARGIN_X, "mx-");
        assert_eq!(SpacingPrefixes::GAP_Y, "gap-y-");
    }

    #[test]
    fn spacing_validation() {
        assert!(SpacingValidation::validate_spacing_token("4").is_ok());
        assert!(SpacingValidation::validate_spacing_token("9999").is_err());
        assert!(SpacingValidation::validate_spacing_token(&"a".repeat(60)).is_err());
    }

    #[test]
    fn arbitrary_value_validation() {
        assert!(SpacingValidation::validate_arbitrary_value("10px").is_ok());
        assert!(SpacingValidation::validate_arbitrary_value("2rem").is_ok());
        assert!(SpacingValidation::validate_arbitrary_value("").is_err());
        assert!(SpacingValidation::validate_arbitrary_value(&"a".repeat(60)).is_err());
    }

    #[test]
    fn parser_config_defaults() {
        let config = SpacingParserConfig::default();

        assert!(config.arbitrary_values_enabled());
        assert!(config.logical_properties_enabled());
        assert!(config.axis_properties_enabled());
        assert!(config.is_spacing_allowed(96.0));
        assert!(!config.is_spacing_allowed(100.0));
    }

    #[test]
    fn all_directions_covered() {
        let directions = SpacingDirections::all_directions();
        assert_eq!(directions.len(), 6);
        assert!(directions.contains(&"t"));
        assert!(directions.contains(&"r"));
        assert!(directions.contains(&"b"));
        assert!(directions.contains(&"l"));
        assert!(directions.contains(&"s"));
        assert!(directions.contains(&"e"));
    }

    #[test]
    fn all_axes_covered() {
        let axes = SpacingAxes::all_axes();
        assert_eq!(axes.len(), 2);
        assert!(axes.contains(&"x"));
        assert!(axes.contains(&"y"));
    }
}
