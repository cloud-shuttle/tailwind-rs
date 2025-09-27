//! Ring Utilities Parser
//!
//! This module provides parsing logic for ring-related Tailwind CSS utilities,
//! including ring width, ring color, and ring offset.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct RingParser;

impl RingParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse ring width classes
    fn parse_ring_width_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "ring-0" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "0 0 0 0 #000".to_string(),
                important: false,
            }]),
            "ring-1" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "0 0 0 1px #000".to_string(),
                important: false,
            }]),
            "ring-2" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "0 0 0 2px #000".to_string(),
                important: false,
            }]),
            "ring-4" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "0 0 0 4px #000".to_string(),
                important: false,
            }]),
            "ring-8" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "0 0 0 8px #000".to_string(),
                important: false,
            }]),
            "ring" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "0 0 0 3px #000".to_string(),
                important: false,
            }]),
            "ring-inset" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "inset 0 0 0 3px #000".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse ring color classes
    fn parse_ring_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(color_part) = class.strip_prefix("ring-") {
            // Handle opacity modifiers (e.g., ring-zinc-900/5)
            if let Some((color_name, opacity)) = color_part.split_once('/') {
                let color_value = self.get_ring_color_value(color_name)?;
                let opacity_value = self.parse_opacity_value(opacity)?;
                let final_color = self.apply_opacity_to_color(&color_value, &opacity_value);
                return Some(vec![CssProperty {
                    name: "box-shadow".to_string(),
                    value: format!("0 0 0 3px {}", final_color),
                    important: false,
                }]);
            }
            let color_value = self.get_ring_color_value(color_part)?;
            return Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: format!("0 0 0 3px {}", color_value),
                important: false,
            }]);
        }
        None
    }

    /// Parse ring offset classes
    fn parse_ring_offset_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("ring-offset-") {
            let offset_value = self.parse_ring_offset_value(value)?;
            return Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: format!("0 0 0 {} #000", offset_value),
                important: false,
            }]);
        }
        None
    }

    /// Get ring color values
    fn get_ring_color_value(&self, color: &str) -> Option<String> {
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
            _ => return None,
        };
        Some(color_value.to_string())
    }

    /// Parse ring offset values
    fn parse_ring_offset_value(&self, value: &str) -> Option<String> {
        match value {
            "0" => Some("0".to_string()),
            "1" => Some("1px".to_string()),
            "2" => Some("2px".to_string()),
            "4" => Some("4px".to_string()),
            "8" => Some("8px".to_string()),
            _ => None,
        }
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

impl UtilityParser for RingParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity
        if let Some(properties) = self.parse_ring_width_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_ring_color_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_ring_offset_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "ring-0",
            "ring-1",
            "ring-2",
            "ring-4",
            "ring-8",
            "ring",
            "ring-inset",
            "ring-*",
            "ring-offset-*",
        ]
    }

    fn get_priority(&self) -> u32 {
        85
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Effects
    }
}

impl Default for RingParser {
    fn default() -> Self {
        Self::new()
    }
}
