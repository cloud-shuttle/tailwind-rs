//! Filter Utilities Helper Functions Module
//!
//! Utility functions and helpers for filter parsing:
//! - Value parsing and validation
//! - CSS property generation
//! - Filter-specific utilities

use super::types::*;
use crate::css_generator::types::CssProperty;

/// Helper functions for parsing filter values
pub struct FilterValueParser;

impl FilterValueParser {
    /// Parse a percentage value (0-100)
    pub fn parse_percentage(value: &str) -> Option<f32> {
        if let Some(num_str) = value.strip_suffix('%') {
            num_str.parse::<f32>().ok().filter(|&n| n >= 0.0 && n <= 100.0)
        } else {
            None
        }
    }

    /// Parse an angle value in degrees
    pub fn parse_angle(value: &str) -> Option<f32> {
        if let Some(num_str) = value.strip_suffix("deg") {
            num_str.parse::<f32>().ok()
        } else {
            value.parse::<f32>().ok()
        }
    }

    /// Parse a length value
    pub fn parse_length(value: &str) -> Option<String> {
        // Basic validation - could be more sophisticated
        if value.contains("px") || value.contains("em") || value.contains("rem") || value.contains("vh") || value.contains("vw") {
            Some(value.to_string())
        } else {
            None
        }
    }

    /// Parse a color value for drop-shadow
    pub fn parse_color(value: &str) -> Option<String> {
        // Basic color validation - could be more sophisticated
        if value.starts_with('#') || value.starts_with("rgb") || value.starts_with("hsl") || value == "transparent" || value == "currentColor" {
            Some(value.to_string())
        } else {
            None
        }
    }

    /// Extract value from custom property syntax: filter-(value)
    pub fn extract_custom_property(value: &str, prefix: &str) -> Option<String> {
        if let Some(stripped) = value.strip_prefix(&format!("{}-(", prefix)) {
            if let Some(inner) = stripped.strip_suffix(')') {
                return Some(inner.to_string());
            }
        }
        None
    }

    /// Extract value from arbitrary value syntax: filter-[value]
    pub fn extract_arbitrary_value(value: &str, prefix: &str) -> Option<String> {
        if let Some(stripped) = value.strip_prefix(&format!("{}-[", prefix)) {
            if let Some(inner) = stripped.strip_suffix(']') {
                return Some(inner.to_string());
            }
        }
        None
    }
}

/// CSS property generation utilities
pub struct FilterCssGenerator;

impl FilterCssGenerator {
    /// Generate CSS property for filter
    pub fn generate_filter_property(value: &str) -> CssProperty {
        CssProperty {
            name: "filter".to_string(),
            value: value.to_string(),
            important: false,
        }
    }

    /// Generate CSS property for backdrop-filter
    pub fn generate_backdrop_filter_property(value: &str) -> CssProperty {
        CssProperty {
            name: "backdrop-filter".to_string(),
            value: value.to_string(),
            important: false,
        }
    }

    /// Generate multiple filter properties (for complex filters)
    pub fn generate_filter_properties(values: Vec<String>) -> Vec<CssProperty> {
        values.into_iter().map(|value| Self::generate_filter_property(&value)).collect()
    }
}

/// Filter validation utilities
pub struct FilterValidator;

impl FilterValidator {
    /// Validate a percentage value
    pub fn is_valid_percentage(value: f32) -> bool {
        value >= 0.0 && value <= 100.0
    }

    /// Validate a blur size class
    pub fn is_valid_blur_size(class: &str) -> bool {
        matches!(class, "blur-none" | "blur-xs" | "blur-sm" | "blur-md" | "blur-lg" | "blur-xl" | "blur-2xl" | "blur-3xl")
    }

    /// Validate a filter class prefix
    pub fn is_valid_filter_prefix(class: &str, prefix: &str) -> bool {
        class.starts_with(&format!("{}-", prefix))
    }

    /// Check if a class represents a none filter
    pub fn is_none_filter(class: &str) -> bool {
        class == "filter-none" || class == "backdrop-filter-none"
    }
}

/// Filter class parsing utilities
pub struct FilterClassParser;

impl FilterClassParser {
    /// Parse blur size from class name
    pub fn parse_blur_size(class: &str) -> Option<BlurSize> {
        match class {
            "blur-none" => Some(BlurSize::None),
            "blur-xs" => Some(BlurSize::Xs),
            "blur-sm" => Some(BlurSize::Sm),
            "blur-md" => Some(BlurSize::Md),
            "blur-lg" => Some(BlurSize::Lg),
            "blur-xl" => Some(BlurSize::Xl),
            "blur-2xl" => Some(BlurSize::Xxl),
            "blur-3xl" => Some(BlurSize::Xxxl),
            _ => None,
        }
    }

    /// Parse filter type from class prefix
    pub fn parse_filter_type(class: &str) -> Option<FilterType> {
        if class.starts_with("blur-") {
            Some(FilterType::Blur)
        } else if class.starts_with("brightness-") {
            Some(FilterType::Brightness)
        } else if class.starts_with("contrast-") {
            Some(FilterType::Contrast)
        } else if class.starts_with("drop-shadow-") {
            Some(FilterType::DropShadow)
        } else if class.starts_with("grayscale-") {
            Some(FilterType::Grayscale)
        } else if class.starts_with("hue-rotate-") {
            Some(FilterType::HueRotate)
        } else if class.starts_with("invert-") {
            Some(FilterType::Invert)
        } else if class.starts_with("opacity-") {
            Some(FilterType::Opacity)
        } else if class.starts_with("saturate-") {
            Some(FilterType::Saturate)
        } else if class.starts_with("sepia-") {
            Some(FilterType::Sepia)
        } else if class.starts_with("backdrop-") {
            Some(FilterType::Backdrop)
        } else {
            None
        }
    }

    /// Extract filter value from class name
    pub fn extract_filter_value(class: &str, filter_type: &FilterType) -> Option<String> {
        let prefix = match filter_type {
            FilterType::Blur => "blur",
            FilterType::Brightness => "brightness",
            FilterType::Contrast => "contrast",
            FilterType::DropShadow => "drop-shadow",
            FilterType::Grayscale => "grayscale",
            FilterType::HueRotate => "hue-rotate",
            FilterType::Invert => "invert",
            FilterType::Opacity => "opacity",
            FilterType::Saturate => "saturate",
            FilterType::Sepia => "sepia",
            FilterType::Backdrop => "backdrop",
        };

        class.strip_prefix(&format!("{}-", prefix)).map(|s| s.to_string())
    }
}

/// Backdrop filter utilities
pub struct BackdropFilterUtils;

impl BackdropFilterUtils {
    /// Check if a class is a backdrop filter
    pub fn is_backdrop_filter(class: &str) -> bool {
        class.starts_with("backdrop-")
    }

    /// Convert backdrop filter class to regular filter class
    pub fn convert_to_regular_filter(class: &str) -> Option<String> {
        if let Some(stripped) = class.strip_prefix("backdrop-") {
            Some(format!("filter-{}", stripped))
        } else {
            None
        }
    }

    /// Parse backdrop filter type from class
    pub fn parse_backdrop_type(class: &str) -> Option<BackdropFilterType> {
        if class.starts_with("backdrop-blur-") {
            Some(BackdropFilterType::Blur)
        } else if class.starts_with("backdrop-brightness-") {
            Some(BackdropFilterType::Brightness)
        } else if class.starts_with("backdrop-contrast-") {
            Some(BackdropFilterType::Contrast)
        } else if class.starts_with("backdrop-grayscale-") {
            Some(BackdropFilterType::Grayscale)
        } else if class.starts_with("backdrop-hue-rotate-") {
            Some(BackdropFilterType::HueRotate)
        } else if class.starts_with("backdrop-invert-") {
            Some(BackdropFilterType::Invert)
        } else if class.starts_with("backdrop-opacity-") {
            Some(BackdropFilterType::Opacity)
        } else if class.starts_with("backdrop-saturate-") {
            Some(BackdropFilterType::Saturate)
        } else if class.starts_with("backdrop-sepia-") {
            Some(BackdropFilterType::Sepia)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_value_parser_percentage() {
        assert_eq!(FilterValueParser::parse_percentage("50%"), Some(50.0));
        assert_eq!(FilterValueParser::parse_percentage("150%"), None); // Invalid range
        assert_eq!(FilterValueParser::parse_percentage("abc"), None); // Invalid format
    }

    #[test]
    fn filter_value_parser_angle() {
        assert_eq!(FilterValueParser::parse_angle("45deg"), Some(45.0));
        assert_eq!(FilterValueParser::parse_angle("90"), Some(90.0));
        assert_eq!(FilterValueParser::parse_angle("abc"), None);
    }

    #[test]
    fn filter_value_parser_length() {
        assert_eq!(FilterValueParser::parse_length("5px"), Some("5px".to_string()));
        assert_eq!(FilterValueParser::parse_length("2em"), Some("2em".to_string()));
        assert_eq!(FilterValueParser::parse_length("invalid"), None);
    }

    #[test]
    fn filter_css_generator() {
        let prop = FilterCssGenerator::generate_filter_property("blur(5px)");
        assert_eq!(prop.name, "filter");
        assert_eq!(prop.value, "blur(5px)");
        assert!(!prop.important);
    }

    #[test]
    fn filter_validator() {
        assert!(FilterValidator::is_valid_percentage(50.0));
        assert!(!FilterValidator::is_valid_percentage(150.0));

        assert!(FilterValidator::is_valid_blur_size("blur-md"));
        assert!(!FilterValidator::is_valid_blur_size("blur-invalid"));

        assert!(FilterValidator::is_valid_filter_prefix("blur-5px", "blur"));
        assert!(!FilterValidator::is_valid_filter_prefix("contrast-5px", "blur"));
    }

    #[test]
    fn filter_class_parser() {
        assert_eq!(FilterClassParser::parse_blur_size("blur-lg"), Some(BlurSize::Lg));
        assert_eq!(FilterClassParser::parse_blur_size("blur-invalid"), None);

        assert_eq!(FilterClassParser::parse_filter_type("brightness-50"), Some(FilterType::Brightness));
        assert_eq!(FilterClassParser::parse_filter_type("invalid-class"), None);

        assert_eq!(FilterClassParser::extract_filter_value("brightness-50", &FilterType::Brightness), Some("50".to_string()));
    }

    #[test]
    fn backdrop_filter_utils() {
        assert!(BackdropFilterUtils::is_backdrop_filter("backdrop-blur-md"));
        assert!(!BackdropFilterUtils::is_backdrop_filter("blur-md"));

        assert_eq!(BackdropFilterUtils::convert_to_regular_filter("backdrop-blur-md"), Some("filter-blur-md".to_string()));
        assert_eq!(BackdropFilterUtils::parse_backdrop_type("backdrop-sepia-50"), Some(BackdropFilterType::Sepia));
    }
}
