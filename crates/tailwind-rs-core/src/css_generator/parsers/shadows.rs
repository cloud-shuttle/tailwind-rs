//! Shadow Utilities Parser
//!
//! This module provides parsing logic for shadow-related Tailwind CSS utilities,
//! including box-shadow, drop-shadow, and shadow colors.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct ShadowParser;

impl ShadowParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse shadow size classes
    fn parse_shadow_size_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "shadow-sm" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string(),
                important: false,
            }]),
            "shadow" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)".to_string(),
                important: false,
            }]),
            "shadow-md" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)"
                    .to_string(),
                important: false,
            }]),
            "shadow-lg" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)"
                    .to_string(),
                important: false,
            }]),
            "shadow-xl" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)"
                    .to_string(),
                important: false,
            }]),
            "shadow-2xl" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "0 25px 50px -12px rgb(0 0 0 / 0.25)".to_string(),
                important: false,
            }]),
            "shadow-none" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "0 0 #0000".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse shadow color classes
    fn parse_shadow_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(color_part) = class.strip_prefix("shadow-") {
            // Handle opacity modifiers (e.g., shadow-zinc-800/5)
            if let Some((color_name, opacity)) = color_part.split_once('/') {
                let color_value = self.get_shadow_color_value(color_name)?;
                let opacity_value = self.parse_opacity_value(opacity)?;
                let final_color = self.apply_opacity_to_color(&color_value, &opacity_value);
                return Some(vec![CssProperty {
                    name: "box-shadow".to_string(),
                    value: format!(
                        "0 1px 3px 0 {}, 0 1px 2px -1px {}",
                        final_color, final_color
                    ),
                    important: false,
                }]);
            }
            let color_value = self.get_shadow_color_value(color_part)?;
            return Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: format!(
                    "0 1px 3px 0 {}, 0 1px 2px -1px {}",
                    color_value, color_value
                ),
                important: false,
            }]);
        }
        None
    }

    /// Get shadow color values
    fn get_shadow_color_value(&self, color: &str) -> Option<String> {
        let color_value = match color {
            "transparent" => "#00000000",
            "current" => "currentColor",
            "black" => "#000000",
            "white" => "#ffffff",
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

impl UtilityParser for ShadowParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity
        if let Some(properties) = self.parse_shadow_size_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_shadow_color_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "shadow-sm",
            "shadow",
            "shadow-md",
            "shadow-lg",
            "shadow-xl",
            "shadow-2xl",
            "shadow-none",
            "shadow-*",
        ]
    }

    fn get_priority(&self) -> u32 {
        85
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Effects
    }
}

impl Default for ShadowParser {
    fn default() -> Self {
        Self::new()
    }
}
