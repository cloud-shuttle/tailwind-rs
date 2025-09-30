//! Background Utilities Module
//!
//! Utility functions and helpers for background parsing,
//! including color mapping, validation, and common operations.

use std::collections::HashMap;

/// Color mapping utilities for background colors
pub struct BackgroundColorUtils;

impl BackgroundColorUtils {
    /// Get color value for background color classes
    pub fn get_color_value(class: &str) -> Option<String> {
        // This is a simplified color mapping - in a real implementation,
        // you'd want a comprehensive color system
        match class {
            // Basic colors
            "bg-white" => Some("#ffffff".to_string()),
            "bg-black" => Some("#000000".to_string()),
            "bg-transparent" => Some("transparent".to_string()),
            "bg-current" => Some("currentColor".to_string()),
            "bg-inherit" => Some("inherit".to_string()),

            // Red scale
            "bg-red-50" => Some("#fef2f2".to_string()),
            "bg-red-100" => Some("#fee2e2".to_string()),
            "bg-red-200" => Some("#fecaca".to_string()),
            "bg-red-300" => Some("#fca5a5".to_string()),
            "bg-red-400" => Some("#f87171".to_string()),
            "bg-red-500" => Some("#ef4444".to_string()),
            "bg-red-600" => Some("#dc2626".to_string()),
            "bg-red-700" => Some("#b91c1c".to_string()),
            "bg-red-800" => Some("#991b1b".to_string()),
            "bg-red-900" => Some("#7f1d1d".to_string()),
            "bg-red-950" => Some("#450a0a".to_string()),

            // Blue scale
            "bg-blue-50" => Some("#eff6ff".to_string()),
            "bg-blue-100" => Some("#dbeafe".to_string()),
            "bg-blue-200" => Some("#bfdbfe".to_string()),
            "bg-blue-300" => Some("#93c5fd".to_string()),
            "bg-blue-400" => Some("#60a5fa".to_string()),
            "bg-blue-500" => Some("#3b82f6".to_string()),
            "bg-blue-600" => Some("#2563eb".to_string()),
            "bg-blue-700" => Some("#1d4ed8".to_string()),
            "bg-blue-800" => Some("#1e40af".to_string()),
            "bg-blue-900" => Some("#1e3a8a".to_string()),
            "bg-blue-950" => Some("#172554".to_string()),

            // Green scale
            "bg-green-50" => Some("#f0fdf4".to_string()),
            "bg-green-100" => Some("#dcfce7".to_string()),
            "bg-green-200" => Some("#bbf7d0".to_string()),
            "bg-green-300" => Some("#86efac".to_string()),
            "bg-green-400" => Some("#4ade80".to_string()),
            "bg-green-500" => Some("#22c55e".to_string()),
            "bg-green-600" => Some("#16a34a".to_string()),
            "bg-green-700" => Some("#15803d".to_string()),
            "bg-green-800" => Some("#166534".to_string()),
            "bg-green-900" => Some("#14532d".to_string()),
            "bg-green-950" => Some("#052e16".to_string()),

            // Gray scale
            "bg-gray-50" => Some("#f9fafb".to_string()),
            "bg-gray-100" => Some("#f3f4f6".to_string()),
            "bg-gray-200" => Some("#e5e7eb".to_string()),
            "bg-gray-300" => Some("#d1d5db".to_string()),
            "bg-gray-400" => Some("#9ca3af".to_string()),
            "bg-gray-500" => Some("#6b7280".to_string()),
            "bg-gray-600" => Some("#4b5563".to_string()),
            "bg-gray-700" => Some("#374151".to_string()),
            "bg-gray-800" => Some("#1f2937".to_string()),
            "bg-gray-900" => Some("#111827".to_string()),
            "bg-gray-950" => Some("#030712".to_string()),

            _ => None,
        }
    }

    /// Check if a class is a background color class
    pub fn is_color_class(class: &str) -> bool {
        class.starts_with("bg-") &&
        (class == "bg-white" || class == "bg-black" || class == "bg-transparent" ||
         class == "bg-current" || class == "bg-inherit" ||
         class.contains("-red-") || class.contains("-blue-") ||
         class.contains("-green-") || class.contains("-gray-"))
    }

    /// Extract color intensity from color class (e.g., "bg-red-500" -> "500")
    pub fn extract_color_intensity(class: &str) -> Option<&str> {
        if let Some(dash_pos) = class.rfind('-') {
            if let Some(prev_dash) = class[..dash_pos].rfind('-') {
                return Some(&class[prev_dash + 1..]);
            }
        }
        None
    }

    /// Get color family from class (e.g., "bg-red-500" -> "red")
    pub fn extract_color_family(class: &str) -> Option<&str> {
        if class.starts_with("bg-") {
            let after_bg = &class[3..];
            if let Some(dash_pos) = after_bg.find('-') {
                return Some(&after_bg[..dash_pos]);
            }
        }
        None
    }
}

/// Background value validation utilities
pub struct BackgroundValidation;

impl BackgroundValidation {
    /// Validate background attachment value
    pub fn is_valid_attachment(value: &str) -> bool {
        matches!(value, "fixed" | "local" | "scroll")
    }

    /// Validate background clip value
    pub fn is_valid_clip(value: &str) -> bool {
        matches!(value, "border-box" | "padding-box" | "content-box" | "text")
    }

    /// Validate background origin value
    pub fn is_valid_origin(value: &str) -> bool {
        matches!(value, "border-box" | "padding-box" | "content-box")
    }

    /// Validate background repeat value
    pub fn is_valid_repeat(value: &str) -> bool {
        matches!(value,
            "repeat" | "no-repeat" | "repeat-x" | "repeat-y" |
            "round" | "space" | "repeat-round" | "repeat-space")
    }

    /// Validate background size value
    pub fn is_valid_size(value: &str) -> bool {
        matches!(value, "auto" | "cover" | "contain") ||
        value.contains("px") || value.contains("%") ||
        value.contains("rem") || value.contains("em")
    }

    /// Validate background position value
    pub fn is_valid_position(value: &str) -> bool {
        let valid_positions = [
            "center", "top", "bottom", "left", "right",
            "top-left", "top-right", "bottom-left", "bottom-right"
        ];
        valid_positions.contains(&value) ||
        value.contains("px") || value.contains("%") ||
        value.contains("rem") || value.contains("em")
    }
}

/// Background pattern constants
pub struct BackgroundPatterns;

impl BackgroundPatterns {
    // Attachment patterns
    pub const ATTACHMENT_FIXED: &str = "bg-fixed";
    pub const ATTACHMENT_LOCAL: &str = "bg-local";
    pub const ATTACHMENT_SCROLL: &str = "bg-scroll";

    // Clip patterns
    pub const CLIP_BORDER: &str = "bg-clip-border";
    pub const CLIP_PADDING: &str = "bg-clip-padding";
    pub const CLIP_CONTENT: &str = "bg-clip-content";
    pub const CLIP_TEXT: &str = "bg-clip-text";

    // Origin patterns
    pub const ORIGIN_BORDER: &str = "bg-origin-border";
    pub const ORIGIN_PADDING: &str = "bg-origin-padding";
    pub const ORIGIN_CONTENT: &str = "bg-origin-content";

    // Repeat patterns
    pub const REPEAT: &str = "bg-repeat";
    pub const NO_REPEAT: &str = "bg-no-repeat";
    pub const REPEAT_X: &str = "bg-repeat-x";
    pub const REPEAT_Y: &str = "bg-repeat-y";
    pub const REPEAT_ROUND: &str = "bg-repeat-round";
    pub const REPEAT_SPACE: &str = "bg-repeat-space";

    // Size patterns
    pub const AUTO: &str = "bg-auto";
    pub const COVER: &str = "bg-cover";
    pub const CONTAIN: &str = "bg-contain";

    // Position patterns
    pub const BOTTOM: &str = "bg-bottom";
    pub const CENTER: &str = "bg-center";
    pub const LEFT: &str = "bg-left";
    pub const LEFT_BOTTOM: &str = "bg-left-bottom";
    pub const LEFT_TOP: &str = "bg-left-top";
    pub const RIGHT: &str = "bg-right";
    pub const RIGHT_BOTTOM: &str = "bg-right-bottom";
    pub const RIGHT_TOP: &str = "bg-right-top";
    pub const TOP: &str = "bg-top";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_value_mapping() {
        assert_eq!(BackgroundColorUtils::get_color_value("bg-white"), Some("#ffffff".to_string()));
        assert_eq!(BackgroundColorUtils::get_color_value("bg-black"), Some("#000000".to_string()));
        assert_eq!(BackgroundColorUtils::get_color_value("bg-red-500"), Some("#ef4444".to_string()));
        assert_eq!(BackgroundColorUtils::get_color_value("bg-blue-500"), Some("#3b82f6".to_string()));
        assert_eq!(BackgroundColorUtils::get_color_value("bg-invalid"), None);
    }

    #[test]
    fn color_class_detection() {
        assert!(BackgroundColorUtils::is_color_class("bg-red-500"));
        assert!(BackgroundColorUtils::is_color_class("bg-blue-200"));
        assert!(BackgroundColorUtils::is_color_class("bg-white"));
        assert!(BackgroundColorUtils::is_color_class("bg-transparent"));
        assert!(!BackgroundColorUtils::is_color_class("bg-fixed"));
        assert!(!BackgroundColorUtils::is_color_class("bg-cover"));
    }

    #[test]
    fn color_intensity_extraction() {
        assert_eq!(BackgroundColorUtils::extract_color_intensity("bg-red-500"), Some("500"));
        assert_eq!(BackgroundColorUtils::extract_color_intensity("bg-blue-200"), Some("200"));
        assert_eq!(BackgroundColorUtils::extract_color_intensity("bg-white"), None);
        assert_eq!(BackgroundColorUtils::extract_color_intensity("bg-gray-50"), Some("50"));
    }

    #[test]
    fn color_family_extraction() {
        assert_eq!(BackgroundColorUtils::extract_color_family("bg-red-500"), Some("red"));
        assert_eq!(BackgroundColorUtils::extract_color_family("bg-blue-200"), Some("blue"));
        assert_eq!(BackgroundColorUtils::extract_color_family("bg-gray-50"), Some("gray"));
        assert_eq!(BackgroundColorUtils::extract_color_family("bg-white"), None);
    }

    #[test]
    fn validation_functions() {
        // Attachment validation
        assert!(BackgroundValidation::is_valid_attachment("fixed"));
        assert!(BackgroundValidation::is_valid_attachment("scroll"));
        assert!(!BackgroundValidation::is_valid_attachment("invalid"));

        // Clip validation
        assert!(BackgroundValidation::is_valid_clip("border-box"));
        assert!(BackgroundValidation::is_valid_clip("text"));
        assert!(!BackgroundValidation::is_valid_clip("invalid"));

        // Repeat validation
        assert!(BackgroundValidation::is_valid_repeat("no-repeat"));
        assert!(BackgroundValidation::is_valid_repeat("repeat-x"));
        assert!(!BackgroundValidation::is_valid_repeat("invalid"));

        // Size validation
        assert!(BackgroundValidation::is_valid_size("cover"));
        assert!(BackgroundValidation::is_valid_size("50%"));
        assert!(BackgroundValidation::is_valid_size("100px"));
        assert!(!BackgroundValidation::is_valid_size("invalid"));

        // Position validation
        assert!(BackgroundValidation::is_valid_position("center"));
        assert!(BackgroundValidation::is_valid_position("top-left"));
        assert!(BackgroundValidation::is_valid_position("50% 25%"));
        assert!(!BackgroundValidation::is_valid_position("invalid"));
    }

    #[test]
    fn pattern_constants() {
        assert_eq!(BackgroundPatterns::ATTACHMENT_FIXED, "bg-fixed");
        assert_eq!(BackgroundPatterns::CLIP_TEXT, "bg-clip-text");
        assert_eq!(BackgroundPatterns::REPEAT, "bg-repeat");
        assert_eq!(BackgroundPatterns::COVER, "bg-cover");
        assert_eq!(BackgroundPatterns::CENTER, "bg-center");
    }
}
