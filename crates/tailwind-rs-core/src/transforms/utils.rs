//! Transform Utilities Module
//!
//! Utility functions, constants, and helpers for transform parsing,
//! including value validation, conversion, and common operations.

use std::collections::HashMap;

/// Transform value validation utilities
pub struct TransformValidation;

impl TransformValidation {
    /// Validate scale values (0.5, 0.75, 1, 1.25, 1.5, etc.)
    pub fn is_valid_scale_value(value: &str) -> bool {
        matches!(value,
            "0" | "50" | "75" | "90" | "95" | "100" | "105" | "110" | "125" | "150" |
            "0.5" | "0.75" | "0.9" | "0.95" | "1" | "1.05" | "1.1" | "1.25" | "1.5"
        )
    }

    /// Validate rotation degrees
    pub fn is_valid_rotation_degree(value: &str) -> bool {
        matches!(value,
            "0" | "1" | "2" | "3" | "6" | "12" | "45" | "90" | "180" |
            "-1" | "-2" | "-3" | "-6" | "-12" | "-45" | "-90" | "-180"
        )
    }

    /// Validate skew degrees
    pub fn is_valid_skew_degree(value: &str) -> bool {
        matches!(value,
            "0" | "1" | "2" | "3" | "6" | "12" |
            "-1" | "-2" | "-3" | "-6" | "-12"
        )
    }

    /// Validate perspective values
    pub fn is_valid_perspective_value(value: &str) -> bool {
        matches!(value,
            "dramatic" | "near" | "normal" | "midrange" | "distant" |
            "none" | "250px" | "500px" | "750px" | "1000px"
        )
    }

    /// Validate transform origin values
    pub fn is_valid_origin_value(value: &str) -> bool {
        matches!(value,
            "center" | "top" | "top-right" | "right" | "bottom-right" |
            "bottom" | "bottom-left" | "left" | "top-left"
        )
    }
}

/// Transform value conversion utilities
pub struct TransformConversion;

impl TransformConversion {
    /// Convert scale percentage to decimal (e.g., "50" -> "0.5")
    pub fn scale_percentage_to_decimal(percentage: &str) -> String {
        match percentage {
            "0" => "0".to_string(),
            "50" => "0.5".to_string(),
            "75" => "0.75".to_string(),
            "90" => "0.9".to_string(),
            "95" => "0.95".to_string(),
            "100" => "1".to_string(),
            "105" => "1.05".to_string(),
            "110" => "1.1".to_string(),
            "125" => "1.25".to_string(),
            "150" => "1.5".to_string(),
            _ => percentage.to_string(),
        }
    }

    /// Convert rotation degree string to CSS value with unit
    pub fn rotation_degree_to_css(degree: &str) -> String {
        if degree.starts_with('-') {
            format!("{}deg", degree)
        } else {
            format!("{}deg", degree)
        }
    }

    /// Convert skew degree string to CSS value with unit
    pub fn skew_degree_to_css(degree: &str) -> String {
        if degree.starts_with('-') {
            format!("{}deg", degree)
        } else {
            format!("{}deg", degree)
        }
    }

    /// Convert perspective value to CSS value
    pub fn perspective_value_to_css(value: &str) -> String {
        match value {
            "dramatic" => "100px".to_string(),
            "near" => "300px".to_string(),
            "normal" => "500px".to_string(),
            "midrange" => "800px".to_string(),
            "distant" => "1200px".to_string(),
            "none" => "none".to_string(),
            _ => value.to_string(),
        }
    }

    /// Convert transform origin value to CSS value
    pub fn origin_value_to_css(value: &str) -> String {
        match value {
            "top" => "top".to_string(),
            "top-right" => "top right".to_string(),
            "right" => "right".to_string(),
            "bottom-right" => "bottom right".to_string(),
            "bottom" => "bottom".to_string(),
            "bottom-left" => "bottom left".to_string(),
            "left" => "left".to_string(),
            "top-left" => "top left".to_string(),
            "center" => "center".to_string(),
            _ => value.to_string(),
        }
    }
}

/// Transform pattern constants
pub struct TransformPatterns;

impl TransformPatterns {
    // Basic transform patterns
    pub const TRANSFORM: &str = "transform";
    pub const TRANSFORM_GPU: &str = "transform-gpu";
    pub const TRANSFORM_CPU: &str = "transform-cpu";
    pub const TRANSFORM_NONE: &str = "transform-none";

    // Backface visibility patterns
    pub const BACKFACE_VISIBLE: &str = "backface-visible";
    pub const BACKFACE_HIDDEN: &str = "backface-hidden";

    // Transform style patterns
    pub const TRANSFORM_STYLE_FLAT: &str = "transform-flat";
    pub const TRANSFORM_STYLE_PRESERVE_3D: &str = "transform-style-3d";
    pub const TRANSFORM_STYLE_PRESERVE: &str = "transform-style-preserve-3d";

    // Perspective patterns
    pub const PERSPECTIVE_NONE: &str = "perspective-none";
    pub const PERSPECTIVE_DRAMATIC: &str = "perspective-dramatic";
    pub const PERSPECTIVE_NEAR: &str = "perspective-near";
    pub const PERSPECTIVE_NORMAL: &str = "perspective-normal";
    pub const PERSPECTIVE_MIDRANGE: &str = "perspective-midrange";
    pub const PERSPECTIVE_DISTANT: &str = "perspective-distant";

    // Origin patterns
    pub const ORIGIN_CENTER: &str = "origin-center";
    pub const ORIGIN_TOP: &str = "origin-top";
    pub const ORIGIN_TOP_RIGHT: &str = "origin-top-right";
    pub const ORIGIN_RIGHT: &str = "origin-right";
    pub const ORIGIN_BOTTOM_RIGHT: &str = "origin-bottom-right";
    pub const ORIGIN_BOTTOM: &str = "origin-bottom";
    pub const ORIGIN_BOTTOM_LEFT: &str = "origin-bottom-left";
    pub const ORIGIN_LEFT: &str = "origin-left";
    pub const ORIGIN_TOP_LEFT: &str = "origin-top-left";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_validation() {
        assert!(TransformValidation::is_valid_scale_value("50"));
        assert!(TransformValidation::is_valid_scale_value("100"));
        assert!(TransformValidation::is_valid_scale_value("150"));
        assert!(TransformValidation::is_valid_scale_value("0.5"));
        assert!(TransformValidation::is_valid_scale_value("1.25"));
        assert!(!TransformValidation::is_valid_scale_value("200"));
        assert!(!TransformValidation::is_valid_scale_value("invalid"));
    }

    #[test]
    fn rotation_validation() {
        assert!(TransformValidation::is_valid_rotation_degree("45"));
        assert!(TransformValidation::is_valid_rotation_degree("-90"));
        assert!(TransformValidation::is_valid_rotation_degree("180"));
        assert!(TransformValidation::is_valid_rotation_degree("0"));
        assert!(!TransformValidation::is_valid_rotation_degree("360"));
        assert!(!TransformValidation::is_valid_rotation_degree("invalid"));
    }

    #[test]
    fn skew_validation() {
        assert!(TransformValidation::is_valid_skew_degree("12"));
        assert!(TransformValidation::is_valid_skew_degree("-6"));
        assert!(TransformValidation::is_valid_skew_degree("0"));
        assert!(!TransformValidation::is_valid_skew_degree("45"));
        assert!(!TransformValidation::is_valid_skew_degree("invalid"));
    }

    #[test]
    fn perspective_validation() {
        assert!(TransformValidation::is_valid_perspective_value("dramatic"));
        assert!(TransformValidation::is_valid_perspective_value("none"));
        assert!(TransformValidation::is_valid_perspective_value("500px"));
        assert!(!TransformValidation::is_valid_perspective_value("invalid"));
    }

    #[test]
    fn origin_validation() {
        assert!(TransformValidation::is_valid_origin_value("center"));
        assert!(TransformValidation::is_valid_origin_value("top-left"));
        assert!(TransformValidation::is_valid_origin_value("bottom-right"));
        assert!(!TransformValidation::is_valid_origin_value("middle"));
    }

    #[test]
    fn scale_conversion() {
        assert_eq!(TransformConversion::scale_percentage_to_decimal("50"), "0.5");
        assert_eq!(TransformConversion::scale_percentage_to_decimal("100"), "1");
        assert_eq!(TransformConversion::scale_percentage_to_decimal("150"), "1.5");
        assert_eq!(TransformConversion::scale_percentage_to_decimal("75"), "0.75");
    }

    #[test]
    fn rotation_conversion() {
        assert_eq!(TransformConversion::rotation_degree_to_css("45"), "45deg");
        assert_eq!(TransformConversion::rotation_degree_to_css("-90"), "-90deg");
        assert_eq!(TransformConversion::rotation_degree_to_css("180"), "180deg");
    }

    #[test]
    fn skew_conversion() {
        assert_eq!(TransformConversion::skew_degree_to_css("12"), "12deg");
        assert_eq!(TransformConversion::skew_degree_to_css("-6"), "-6deg");
        assert_eq!(TransformConversion::skew_degree_to_css("0"), "0deg");
    }

    #[test]
    fn perspective_conversion() {
        assert_eq!(TransformConversion::perspective_value_to_css("dramatic"), "100px");
        assert_eq!(TransformConversion::perspective_value_to_css("normal"), "500px");
        assert_eq!(TransformConversion::perspective_value_to_css("none"), "none");
        assert_eq!(TransformConversion::perspective_value_to_css("1000px"), "1000px");
    }

    #[test]
    fn origin_conversion() {
        assert_eq!(TransformConversion::origin_value_to_css("center"), "center");
        assert_eq!(TransformConversion::origin_value_to_css("top-left"), "top left");
        assert_eq!(TransformConversion::origin_value_to_css("bottom-right"), "bottom right");
    }

    #[test]
    fn pattern_constants() {
        assert_eq!(TransformPatterns::TRANSFORM, "transform");
        assert_eq!(TransformPatterns::TRANSFORM_GPU, "transform-gpu");
        assert_eq!(TransformPatterns::BACKFACE_VISIBLE, "backface-visible");
        assert_eq!(TransformPatterns::ORIGIN_CENTER, "origin-center");
        assert_eq!(TransformPatterns::PERSPECTIVE_NONE, "perspective-none");
    }
}
