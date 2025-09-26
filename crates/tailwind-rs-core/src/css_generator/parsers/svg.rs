//! SVG Utilities Parser
//!
//! This module provides parsing logic for SVG-related Tailwind CSS utilities,
//! including stroke, fill, and other SVG properties.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct SvgParser;

impl SvgParser {
    pub fn new() -> Self { Self }

    /// Parse stroke color classes
    fn parse_stroke_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "stroke-none" => Some(vec![CssProperty { name: "stroke".to_string(), value: "none".to_string(), important: false }]),
            "stroke-inherit" => Some(vec![CssProperty { name: "stroke".to_string(), value: "inherit".to_string(), important: false }]),
            "stroke-current" => Some(vec![CssProperty { name: "stroke".to_string(), value: "currentColor".to_string(), important: false }]),
            "stroke-transparent" => Some(vec![CssProperty { name: "stroke".to_string(), value: "transparent".to_string(), important: false }]),
            _ => {
                if let Some(color_part) = class.strip_prefix("stroke-") {
                    let color_value = self.get_stroke_color_value(color_part)?;
                    return Some(vec![CssProperty { 
                        name: "stroke".to_string(), 
                        value: color_value, 
                        important: false 
                    }]);
                }
                None
            }
        }
    }

    /// Parse fill color classes
    fn parse_fill_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "fill-none" => Some(vec![CssProperty { name: "fill".to_string(), value: "none".to_string(), important: false }]),
            "fill-inherit" => Some(vec![CssProperty { name: "fill".to_string(), value: "inherit".to_string(), important: false }]),
            "fill-current" => Some(vec![CssProperty { name: "fill".to_string(), value: "currentColor".to_string(), important: false }]),
            "fill-transparent" => Some(vec![CssProperty { name: "fill".to_string(), value: "transparent".to_string(), important: false }]),
            _ => {
                if let Some(color_part) = class.strip_prefix("fill-") {
                    let color_value = self.get_fill_color_value(color_part)?;
                    return Some(vec![CssProperty { 
                        name: "fill".to_string(), 
                        value: color_value, 
                        important: false 
                    }]);
                }
                None
            }
        }
    }

    /// Parse stroke width classes
    fn parse_stroke_width_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(width) = class.strip_prefix("stroke-") {
            let width_value = self.parse_stroke_width_value(width)?;
            return Some(vec![CssProperty { 
                name: "stroke-width".to_string(), 
                value: width_value, 
                important: false 
            }]);
        }
        None
    }

    /// Get stroke color values
    fn get_stroke_color_value(&self, color: &str) -> Option<String> {
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

    /// Get fill color values
    fn get_fill_color_value(&self, color: &str) -> Option<String> {
        // Same as stroke colors for now
        self.get_stroke_color_value(color)
    }

    /// Parse stroke width values
    fn parse_stroke_width_value(&self, width: &str) -> Option<String> {
        match width {
            "0" => Some("0".to_string()),
            "1" => Some("1".to_string()),
            "2" => Some("2".to_string()),
            _ => None,
        }
    }
}

impl UtilityParser for SvgParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity
        if let Some(properties) = self.parse_stroke_color_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_fill_color_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_stroke_width_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec!["stroke-*", "fill-*"]
    }

    fn get_priority(&self) -> u32 { 90 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Effects }
}

impl Default for SvgParser {
    fn default() -> Self { Self::new() }
}
