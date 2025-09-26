//! Arbitrary values support for tailwind-rs
//!
//! This module provides support for Tailwind CSS arbitrary values,
//! allowing users to specify custom values using square bracket notation.
//! Examples: w-[123px], bg-[#ff0000], text-[14px], etc.

use crate::classes::ClassBuilder;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents an arbitrary value in Tailwind CSS
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArbitraryValue {
    /// The property prefix (e.g., "w", "bg", "text")
    pub property: String,
    /// The arbitrary value content (e.g., "123px", "#ff0000", "14px")
    pub value: String,
}

impl ArbitraryValue {
    /// Create a new arbitrary value
    pub fn new(property: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            property: property.into(),
            value: value.into(),
        }
    }

    /// Convert to Tailwind CSS class name
    pub fn to_class_name(&self) -> String {
        format!("{}-[{}]", self.property, self.value)
    }

    /// Validate the arbitrary value
    pub fn validate(&self) -> Result<(), ArbitraryValueError> {
        // Validate property name
        if !is_valid_property(&self.property) {
            return Err(ArbitraryValueError::InvalidProperty(self.property.clone()));
        }

        // Validate value format
        if !is_valid_arbitrary_value(&self.value) {
            return Err(ArbitraryValueError::InvalidValue(self.value.clone()));
        }

        Ok(())
    }
}

/// Errors that can occur when working with arbitrary values
#[derive(Debug, thiserror::Error)]
pub enum ArbitraryValueError {
    #[error("Invalid property: {0}")]
    InvalidProperty(String),

    #[error("Invalid arbitrary value: {0}")]
    InvalidValue(String),

    #[error("Unsupported property for arbitrary values: {0}")]
    UnsupportedProperty(String),
}

impl fmt::Display for ArbitraryValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Check if a property name is valid for arbitrary values
fn is_valid_property(property: &str) -> bool {
    // Common Tailwind CSS properties that support arbitrary values
    let valid_properties = [
        // Spacing
        "p",
        "pt",
        "pr",
        "pb",
        "pl",
        "px",
        "py",
        "m",
        "mt",
        "mr",
        "mb",
        "ml",
        "mx",
        "my",
        "space-x",
        "space-y",
        "gap",
        "gap-x",
        "gap-y",
        // Sizing
        "w",
        "h",
        "min-w",
        "max-w",
        "min-h",
        "max-h",
        // Typography
        "text",
        "font",
        "leading",
        "tracking",
        "indent",
        "text-indent",
        // Colors
        "bg",
        "text",
        "border",
        "ring",
        "accent",
        "caret",
        "fill",
        "stroke",
        // Borders
        "border",
        "border-t",
        "border-r",
        "border-b",
        "border-l",
        "border-x",
        "border-y",
        "rounded",
        "rounded-t",
        "rounded-r",
        "rounded-b",
        "rounded-l",
        "rounded-tl",
        "rounded-tr",
        "rounded-br",
        "rounded-bl",
        // Effects
        "shadow",
        "opacity",
        "backdrop-blur",
        "backdrop-brightness",
        "backdrop-contrast",
        "backdrop-grayscale",
        "backdrop-hue-rotate",
        "backdrop-invert",
        "backdrop-opacity",
        "backdrop-saturate",
        "backdrop-sepia",
        "blur",
        "brightness",
        "contrast",
        "drop-shadow",
        "grayscale",
        "hue-rotate",
        "invert",
        "saturate",
        "sepia",
        // Transforms
        "scale",
        "scale-x",
        "scale-y",
        "rotate",
        "translate-x",
        "translate-y",
        "skew-x",
        "skew-y",
        // Positioning
        "top",
        "right",
        "bottom",
        "left",
        "inset",
        "inset-x",
        "inset-y",
        // Z-index
        "z",
        // Grid
        "grid-cols",
        "grid-rows",
        "col-start",
        "col-end",
        "row-start",
        "row-end",
        "col-span",
        "row-span",
        "auto-cols",
        "auto-rows",
        // Flexbox
        "flex",
        "flex-grow",
        "flex-shrink",
        "flex-basis",
        "grow",
        "shrink",
        "basis",
        "order",
        "self",
        "place-self",
        "justify-self",
        "justify-items",
        // Animation
        "animate",
        "transition",
        "duration",
        "delay",
        "ease",
        // Interactivity
        "cursor",
        "select",
        "resize",
        "scroll",
        "touch",
        "will-change",
        "overscroll",
        // Accessibility
        "sr",
        "motion",
        "forced-color-adjust",
    ];

    valid_properties.contains(&property)
}

/// Check if an arbitrary value is valid
fn is_valid_arbitrary_value(value: &str) -> bool {
    // Common patterns for arbitrary values
    let patterns = [
        // Length values (px, rem, em, %, vw, vh, etc.)
        r"^-?\d+(\.\d+)?(px|rem|em|%|vw|vh|vmin|vmax|ch|ex|cm|mm|in|pt|pc)$",
        // Color values (hex, rgb, rgba, hsl, hsla, named colors)
        r"^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$",
        r"^rgb\(\s*\d+\s*,\s*\d+\s*,\s*\d+\s*\)$",
        r"^rgba\(\s*\d+\s*,\s*\d+\s*,\s*\d+\s*,\s*[\d.]+\s*\)$",
        r"^hsl\(\s*\d+\s*,\s*\d+%\s*,\s*\d+%\s*\)$",
        r"^hsla\(\s*\d+\s*,\s*\d+%\s*,\s*\d+%\s*,\s*[\d.]+\s*\)$",
        // Fractional values
        r"^-?\d+(\.\d+)?$",
        // CSS custom properties
        r"^var\(--[a-zA-Z0-9-_]+\)$",
        // CSS functions
        r"^(calc|min|max|clamp)\(.+\)$",
        // Keywords
        r"^(auto|none|inherit|initial|unset|revert|currentColor|transparent)$",
        // Viewport units
        r"^\d+(\.\d+)?(dvw|dvh|dvi|dvb|svw|svh|svi|svb|lvw|lvh|lvi|lvb)$",
        // Container units
        r"^\d+(\.\d+)?(cqw|cqh|cqi|cqb|cqmin|cqmax)$",
    ];

    for pattern in &patterns {
        if let Ok(regex) = Regex::new(pattern) {
            if regex.is_match(value) {
                return true;
            }
        }
    }

    false
}

/// Trait for adding arbitrary value utilities to a class builder
pub trait ArbitraryValueUtilities {
    /// Add an arbitrary value class
    fn arbitrary_value(self, property: impl Into<String>, value: impl Into<String>) -> Self;

    /// Add arbitrary width
    fn w_arbitrary(self, value: impl Into<String>) -> Self;

    /// Add arbitrary height
    fn h_arbitrary(self, value: impl Into<String>) -> Self;

    /// Add arbitrary padding
    fn p_arbitrary(self, value: impl Into<String>) -> Self;

    /// Add arbitrary margin
    fn m_arbitrary(self, value: impl Into<String>) -> Self;

    /// Add arbitrary background color
    fn bg_arbitrary(self, value: impl Into<String>) -> Self;

    /// Add arbitrary text color
    fn text_arbitrary(self, value: impl Into<String>) -> Self;

    /// Add arbitrary border color
    fn border_arbitrary(self, value: impl Into<String>) -> Self;

    /// Add arbitrary font size
    fn text_size_arbitrary(self, value: impl Into<String>) -> Self;

    /// Add arbitrary line height
    fn leading_arbitrary(self, value: impl Into<String>) -> Self;

    /// Add arbitrary letter spacing
    fn tracking_arbitrary(self, value: impl Into<String>) -> Self;

    /// Add arbitrary border radius
    fn rounded_arbitrary(self, value: impl Into<String>) -> Self;

    /// Add arbitrary shadow
    fn shadow_arbitrary(self, value: impl Into<String>) -> Self;

    /// Add arbitrary opacity
    fn opacity_arbitrary(self, value: impl Into<String>) -> Self;

    /// Add arbitrary z-index
    fn z_arbitrary(self, value: impl Into<String>) -> Self;

    /// Add arbitrary top position
    fn top_arbitrary(self, value: impl Into<String>) -> Self;

    /// Add arbitrary right position
    fn right_arbitrary(self, value: impl Into<String>) -> Self;

    /// Add arbitrary bottom position
    fn bottom_arbitrary(self, value: impl Into<String>) -> Self;

    /// Add arbitrary left position
    fn left_arbitrary(self, value: impl Into<String>) -> Self;
}

impl ArbitraryValueUtilities for ClassBuilder {
    fn arbitrary_value(self, property: impl Into<String>, value: impl Into<String>) -> Self {
        let arbitrary = ArbitraryValue::new(property, value);
        self.class(arbitrary.to_class_name())
    }

    fn w_arbitrary(self, value: impl Into<String>) -> Self {
        self.arbitrary_value("w", value)
    }

    fn h_arbitrary(self, value: impl Into<String>) -> Self {
        self.arbitrary_value("h", value)
    }

    fn p_arbitrary(self, value: impl Into<String>) -> Self {
        self.arbitrary_value("p", value)
    }

    fn m_arbitrary(self, value: impl Into<String>) -> Self {
        self.arbitrary_value("m", value)
    }

    fn bg_arbitrary(self, value: impl Into<String>) -> Self {
        self.arbitrary_value("bg", value)
    }

    fn text_arbitrary(self, value: impl Into<String>) -> Self {
        self.arbitrary_value("text", value)
    }

    fn border_arbitrary(self, value: impl Into<String>) -> Self {
        self.arbitrary_value("border", value)
    }

    fn text_size_arbitrary(self, value: impl Into<String>) -> Self {
        self.arbitrary_value("text", value)
    }

    fn leading_arbitrary(self, value: impl Into<String>) -> Self {
        self.arbitrary_value("leading", value)
    }

    fn tracking_arbitrary(self, value: impl Into<String>) -> Self {
        self.arbitrary_value("tracking", value)
    }

    fn rounded_arbitrary(self, value: impl Into<String>) -> Self {
        self.arbitrary_value("rounded", value)
    }

    fn shadow_arbitrary(self, value: impl Into<String>) -> Self {
        self.arbitrary_value("shadow", value)
    }

    fn opacity_arbitrary(self, value: impl Into<String>) -> Self {
        self.arbitrary_value("opacity", value)
    }

    fn z_arbitrary(self, value: impl Into<String>) -> Self {
        self.arbitrary_value("z", value)
    }

    fn top_arbitrary(self, value: impl Into<String>) -> Self {
        self.arbitrary_value("top", value)
    }

    fn right_arbitrary(self, value: impl Into<String>) -> Self {
        self.arbitrary_value("right", value)
    }

    fn bottom_arbitrary(self, value: impl Into<String>) -> Self {
        self.arbitrary_value("bottom", value)
    }

    fn left_arbitrary(self, value: impl Into<String>) -> Self {
        self.arbitrary_value("left", value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arbitrary_value_creation() {
        let arbitrary = ArbitraryValue::new("w", "123px");
        assert_eq!(arbitrary.to_class_name(), "w-[123px]");
    }

    #[test]
    fn test_arbitrary_value_validation() {
        // Valid values
        assert!(ArbitraryValue::new("w", "123px").validate().is_ok());
        assert!(ArbitraryValue::new("bg", "#ff0000").validate().is_ok());
        assert!(ArbitraryValue::new("text", "14px").validate().is_ok());
        assert!(ArbitraryValue::new("p", "1.5rem").validate().is_ok());
        assert!(ArbitraryValue::new("opacity", "0.5").validate().is_ok());

        // Invalid values
        assert!(ArbitraryValue::new("invalid", "123px").validate().is_err());
        assert!(ArbitraryValue::new("w", "invalid-value")
            .validate()
            .is_err());
    }

    #[test]
    fn test_arbitrary_value_utilities() {
        let classes = ClassBuilder::new()
            .w_arbitrary("123px")
            .h_arbitrary("456px")
            .bg_arbitrary("#ff0000")
            .text_arbitrary("#ffffff")
            .p_arbitrary("1.5rem")
            .m_arbitrary("2rem")
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("w-[123px]"));
        assert!(css_classes.contains("h-[456px]"));
        assert!(css_classes.contains("bg-[#ff0000]"));
        assert!(css_classes.contains("text-[#ffffff]"));
        assert!(css_classes.contains("p-[1.5rem]"));
        assert!(css_classes.contains("m-[2rem]"));
    }

    #[test]
    fn test_arbitrary_value_display() {
        let arbitrary = ArbitraryValue::new("w", "123px");
        assert_eq!(format!("{}", arbitrary), "w-[123px]");
    }

    #[test]
    fn test_valid_properties() {
        assert!(is_valid_property("w"));
        assert!(is_valid_property("h"));
        assert!(is_valid_property("bg"));
        assert!(is_valid_property("text"));
        assert!(is_valid_property("p"));
        assert!(is_valid_property("m"));
        assert!(is_valid_property("rounded"));
        assert!(is_valid_property("shadow"));
        assert!(is_valid_property("opacity"));
        assert!(is_valid_property("z"));
        assert!(is_valid_property("top"));
        assert!(is_valid_property("right"));
        assert!(is_valid_property("bottom"));
        assert!(is_valid_property("left"));

        assert!(!is_valid_property("invalid"));
        assert!(!is_valid_property(""));
    }

    #[test]
    fn test_valid_arbitrary_values() {
        // Length values
        assert!(is_valid_arbitrary_value("123px"));
        assert!(is_valid_arbitrary_value("1.5rem"));
        assert!(is_valid_arbitrary_value("50%"));
        assert!(is_valid_arbitrary_value("100vw"));
        assert!(is_valid_arbitrary_value("100vh"));

        // Color values
        assert!(is_valid_arbitrary_value("#ff0000"));
        assert!(is_valid_arbitrary_value("#f00"));
        assert!(is_valid_arbitrary_value("rgb(255, 0, 0)"));
        assert!(is_valid_arbitrary_value("rgba(255, 0, 0, 0.5)"));
        assert!(is_valid_arbitrary_value("hsl(0, 100%, 50%)"));
        assert!(is_valid_arbitrary_value("hsla(0, 100%, 50%, 0.5)"));

        // Fractional values
        assert!(is_valid_arbitrary_value("0.5"));
        assert!(is_valid_arbitrary_value("1.25"));
        assert!(is_valid_arbitrary_value("-0.5"));

        // CSS custom properties
        assert!(is_valid_arbitrary_value("var(--my-color)"));
        assert!(is_valid_arbitrary_value("var(--spacing-lg)"));

        // CSS functions
        assert!(is_valid_arbitrary_value("calc(100% - 20px)"));
        assert!(is_valid_arbitrary_value("min(100px, 50%)"));
        assert!(is_valid_arbitrary_value("max(100px, 50%)"));
        assert!(is_valid_arbitrary_value("clamp(100px, 50%, 200px)"));

        // Keywords
        assert!(is_valid_arbitrary_value("auto"));
        assert!(is_valid_arbitrary_value("none"));
        assert!(is_valid_arbitrary_value("inherit"));
        assert!(is_valid_arbitrary_value("currentColor"));
        assert!(is_valid_arbitrary_value("transparent"));

        // Viewport units
        assert!(is_valid_arbitrary_value("100dvw"));
        assert!(is_valid_arbitrary_value("100dvh"));
        assert!(is_valid_arbitrary_value("100svw"));
        assert!(is_valid_arbitrary_value("100svh"));
        assert!(is_valid_arbitrary_value("100lvw"));
        assert!(is_valid_arbitrary_value("100lvh"));

        // Container units
        assert!(is_valid_arbitrary_value("100cqw"));
        assert!(is_valid_arbitrary_value("100cqh"));
        assert!(is_valid_arbitrary_value("100cqi"));
        assert!(is_valid_arbitrary_value("100cqb"));
        assert!(is_valid_arbitrary_value("100cqmin"));
        assert!(is_valid_arbitrary_value("100cqmax"));

        // Invalid values
        assert!(!is_valid_arbitrary_value(""));
        assert!(!is_valid_arbitrary_value("invalid"));
        assert!(!is_valid_arbitrary_value("abc"));
    }
}
