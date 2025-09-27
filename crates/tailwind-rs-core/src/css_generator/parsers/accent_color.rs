//! Accent Color Parser for Tailwind CSS
//!
//! This module handles parsing of accent color utilities:
//! - Accent color (accent-*)
//! - Arbitrary accent colors (accent-[...], accent-(...))

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

/// Parser for accent color utilities
#[derive(Debug, Clone)]
pub struct AccentColorParser;

impl Default for AccentColorParser {
    fn default() -> Self {
        Self::new()
    }
}

impl AccentColorParser {
    /// Create a new accent color parser
    pub fn new() -> Self {
        Self
    }

    /// Parse accent color classes
    fn parse_accent_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "accent-auto" => Some(vec![CssProperty {
                name: "accent-color".to_string(),
                value: "auto".to_string(),
                important: false,
            }]),
            "accent-inherit" => Some(vec![CssProperty {
                name: "accent-color".to_string(),
                value: "inherit".to_string(),
                important: false,
            }]),
            "accent-current" => Some(vec![CssProperty {
                name: "accent-color".to_string(),
                value: "currentColor".to_string(),
                important: false,
            }]),
            "accent-transparent" => Some(vec![CssProperty {
                name: "accent-color".to_string(),
                value: "transparent".to_string(),
                important: false,
            }]),
            "accent-black" => Some(vec![CssProperty {
                name: "accent-color".to_string(),
                value: "#000000".to_string(),
                important: false,
            }]),
            "accent-white" => Some(vec![CssProperty {
                name: "accent-color".to_string(),
                value: "#ffffff".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for accent color
                if let Some(value) = class.strip_prefix("accent-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "accent-color".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }

                // Arbitrary values for accent color
                if let Some(value) = class.strip_prefix("accent-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "accent-color".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }

                // Standard color classes (accent-red-500, accent-blue-600, etc.)
                if let Some(color_value) = self.get_color_value(class) {
                    return Some(vec![CssProperty {
                        name: "accent-color".to_string(),
                        value: color_value,
                        important: false,
                    }]);
                }

                None
            }
        }
    }

    /// Get color value from class suffix
    fn get_color_value(&self, color: &str) -> Option<String> {
        // Handle Tailwind color scale (e.g., "red-500", "blue-300")
        if let Some(tailwind_color) = self.parse_tailwind_color(color) {
            return Some(tailwind_color);
        }

        // Handle hex colors
        if color.starts_with('#') {
            return Some(color.to_string());
        }

        // Handle rgb/rgba
        if color.starts_with("rgb") {
            return Some(color.to_string());
        }

        // Handle hsl/hsla
        if color.starts_with("hsl") {
            return Some(color.to_string());
        }

        None
    }

    /// Parse Tailwind color scale
    fn parse_tailwind_color(&self, color: &str) -> Option<String> {
        // Remove accent- prefix if present
        let color_name = if color.starts_with("accent-") {
            &color[7..]
        } else {
            color
        };

        match color_name {
            // Red colors
            "red-50" => Some("#fef2f2".to_string()),
            "red-100" => Some("#fee2e2".to_string()),
            "red-200" => Some("#fecaca".to_string()),
            "red-300" => Some("#fca5a5".to_string()),
            "red-400" => Some("#f87171".to_string()),
            "red-500" => Some("#ef4444".to_string()),
            "red-600" => Some("#dc2626".to_string()),
            "red-700" => Some("#b91c1c".to_string()),
            "red-800" => Some("#991b1b".to_string()),
            "red-900" => Some("#7f1d1d".to_string()),

            // Blue colors
            "blue-50" => Some("#eff6ff".to_string()),
            "blue-100" => Some("#dbeafe".to_string()),
            "blue-200" => Some("#bfdbfe".to_string()),
            "blue-300" => Some("#93c5fd".to_string()),
            "blue-400" => Some("#60a5fa".to_string()),
            "blue-500" => Some("#3b82f6".to_string()),
            "blue-600" => Some("#2563eb".to_string()),
            "blue-700" => Some("#1d4ed8".to_string()),
            "blue-800" => Some("#1e40af".to_string()),
            "blue-900" => Some("#1e3a8a".to_string()),

            // Green colors
            "green-50" => Some("#f0fdf4".to_string()),
            "green-100" => Some("#dcfce7".to_string()),
            "green-200" => Some("#bbf7d0".to_string()),
            "green-300" => Some("#86efac".to_string()),
            "green-400" => Some("#4ade80".to_string()),
            "green-500" => Some("#22c55e".to_string()),
            "green-600" => Some("#16a34a".to_string()),
            "green-700" => Some("#15803d".to_string()),
            "green-800" => Some("#166534".to_string()),
            "green-900" => Some("#14532d".to_string()),

            // Gray colors
            "gray-50" => Some("#f9fafb".to_string()),
            "gray-100" => Some("#f3f4f6".to_string()),
            "gray-200" => Some("#e5e7eb".to_string()),
            "gray-300" => Some("#d1d5db".to_string()),
            "gray-400" => Some("#9ca3af".to_string()),
            "gray-500" => Some("#6b7280".to_string()),
            "gray-600" => Some("#4b5563".to_string()),
            "gray-700" => Some("#374151".to_string()),
            "gray-800" => Some("#1f2937".to_string()),
            "gray-900" => Some("#111827".to_string()),

            _ => None,
        }
    }
}

impl UtilityParser for AccentColorParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_accent_color_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec!["accent-*", "accent-[*]", "accent-(*)"]
    }

    fn get_priority(&self) -> u32 {
        60 // Medium priority for accent color
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Interactive
    }
}
