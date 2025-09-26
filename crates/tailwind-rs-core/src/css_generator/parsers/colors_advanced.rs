//! Advanced Color Utilities Parser
//!
//! This module provides parsing logic for advanced color-related Tailwind CSS utilities,
//! including complex color variants, opacity modifiers, and advanced color patterns.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct AdvancedColorParser;

impl AdvancedColorParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse complex background color classes with opacity
    fn parse_complex_bg_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Handle classes like bg-green-50, dark:bg-green-900/20, bg-white/90
        if let Some(color_part) = class.strip_prefix("bg-") {
            // Check for opacity modifier (e.g., green-900/20, white/90)
            if let Some((color_name, opacity)) = color_part.split_once('/') {
                let color_value = self.get_advanced_color_value(color_name)?;
                let opacity_value = self.parse_opacity_value(opacity)?;
                let final_color = self.apply_opacity_to_color(&color_value, &opacity_value);
                return Some(vec![CssProperty {
                    name: "background-color".to_string(),
                    value: final_color,
                    important: false,
                }]);
            } else {
                let color_value = self.get_advanced_color_value(color_part)?;
                return Some(vec![CssProperty {
                    name: "background-color".to_string(),
                    value: color_value,
                    important: false,
                }]);
            }
        }
        None
    }

    /// Parse complex border color classes with opacity
    fn parse_complex_border_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(color_part) = class.strip_prefix("border-") {
            // Check for opacity modifier
            if let Some((color_name, opacity)) = color_part.split_once('/') {
                let color_value = self.get_advanced_color_value(color_name)?;
                let opacity_value = self.parse_opacity_value(opacity)?;
                let final_color = self.apply_opacity_to_color(&color_value, &opacity_value);
                return Some(vec![CssProperty {
                    name: "border-color".to_string(),
                    value: final_color,
                    important: false,
                }]);
            } else {
                let color_value = self.get_advanced_color_value(color_part)?;
                return Some(vec![CssProperty {
                    name: "border-color".to_string(),
                    value: color_value,
                    important: false,
                }]);
            }
        }
        None
    }

    /// Get advanced color values including all Tailwind colors
    fn get_advanced_color_value(&self, color: &str) -> Option<String> {
        let color_value = match color {
            // Basic colors
            "white" => "#ffffff",
            "black" => "#000000",
            "transparent" => "transparent",
            "current" => "currentColor",
            // Green colors
            "green-50" => "#f0fdf4",
            "green-100" => "#dcfce7",
            "green-200" => "#bbf7d0",
            "green-300" => "#86efac",
            "green-400" => "#4ade80",
            "green-500" => "#22c55e",
            "green-600" => "#16a34a",
            "green-700" => "#15803d",
            "green-800" => "#166534",
            "green-900" => "#14532d",

            // Yellow colors
            "yellow-50" => "#fefce8",
            "yellow-100" => "#fef3c7",
            "yellow-200" => "#fde68a",
            "yellow-300" => "#fcd34d",
            "yellow-400" => "#fbbf24",
            "yellow-500" => "#f59e0b",
            "yellow-600" => "#d97706",
            "yellow-700" => "#b45309",
            "yellow-800" => "#92400e",
            "yellow-900" => "#78350f",

            // Red colors
            "red-50" => "#fef2f2",
            "red-100" => "#fee2e2",
            "red-200" => "#fecaca",
            "red-300" => "#fca5a5",
            "red-400" => "#f87171",
            "red-500" => "#ef4444",
            "red-600" => "#dc2626",
            "red-700" => "#b91c1c",
            "red-800" => "#991b1b",
            "red-900" => "#7f1d1d",

            // Blue colors
            "blue-50" => "#eff6ff",
            "blue-100" => "#dbeafe",
            "blue-200" => "#bfdbfe",
            "blue-300" => "#93c5fd",
            "blue-400" => "#60a5fa",
            "blue-500" => "#3b82f6",
            "blue-600" => "#2563eb",
            "blue-700" => "#1d4ed8",
            "blue-800" => "#1e40af",
            "blue-900" => "#1e3a8a",

            // Gray colors
            "gray-50" => "#f9fafb",
            "gray-100" => "#f3f4f6",
            "gray-200" => "#e5e7eb",
            "gray-300" => "#d1d5db",
            "gray-400" => "#9ca3af",
            "gray-500" => "#6b7280",
            "gray-600" => "#4b5563",
            "gray-700" => "#374151",
            "gray-800" => "#1f2937",
            "gray-900" => "#111827",

            // Zinc colors
            "zinc-50" => "#fafafa",
            "zinc-100" => "#f4f4f5",
            "zinc-200" => "#e4e4e7",
            "zinc-300" => "#d4d4d8",
            "zinc-400" => "#a1a1aa",
            "zinc-500" => "#71717a",
            "zinc-600" => "#52525b",
            "zinc-700" => "#3f3f46",
            "zinc-800" => "#27272a",
            "zinc-900" => "#18181b",

            _ => return None,
        };
        Some(color_value.to_string())
    }

    /// Parse opacity values
    fn parse_opacity_value(&self, opacity: &str) -> Option<String> {
        match opacity {
            "0" => Some("0".to_string()),
            "5" => Some("0.05".to_string()),
            "10" => Some("0.1".to_string()),
            "20" => Some("0.2".to_string()),
            "25" => Some("0.25".to_string()),
            "30" => Some("0.3".to_string()),
            "40" => Some("0.4".to_string()),
            "50" => Some("0.5".to_string()),
            "60" => Some("0.6".to_string()),
            "70" => Some("0.7".to_string()),
            "75" => Some("0.75".to_string()),
            "80" => Some("0.8".to_string()),
            "90" => Some("0.9".to_string()),
            "95" => Some("0.95".to_string()),
            "100" => Some("1".to_string()),
            _ => None,
        }
    }

    /// Apply opacity to a color
    fn apply_opacity_to_color(&self, color: &str, opacity: &str) -> String {
        // Convert hex to rgba with opacity
        if color.starts_with('#') && color.len() == 7 {
            let r = u8::from_str_radix(&color[1..3], 16).unwrap_or(0);
            let g = u8::from_str_radix(&color[3..5], 16).unwrap_or(0);
            let b = u8::from_str_radix(&color[5..7], 16).unwrap_or(0);
            format!("rgba({}, {}, {}, {})", r, g, b, opacity)
        } else {
            color.to_string() // Return original if not a hex color
        }
    }
}

impl UtilityParser for AdvancedColorParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try complex background color classes
        if let Some(properties) = self.parse_complex_bg_color_class(class) {
            return Some(properties);
        }

        // Try complex border color classes
        if let Some(properties) = self.parse_complex_border_color_class(class) {
            return Some(properties);
        }

        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec!["bg-*-*", "bg-*-*/*", "border-*-*", "border-*-*/*"]
    }

    fn get_priority(&self) -> u32 {
        90
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Color
    }
}

impl Default for AdvancedColorParser {
    fn default() -> Self {
        Self::new()
    }
}
