//! Divide Utilities Parser
//!
//! This module provides parsing logic for divide-related Tailwind CSS utilities,
//! such as `divide-y`, `divide-x`, and `divide-*` utilities.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct DivideParser;

impl DivideParser {
    pub fn new() -> Self { Self }

    /// Parse divide direction classes
    fn parse_divide_direction_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "divide-y" => Some(vec![
                CssProperty { name: "--tw-divide-y-reverse".to_string(), value: "0".to_string(), important: false },
                CssProperty { name: "border-top-width".to_string(), value: "calc(1px * calc(1 - var(--tw-divide-y-reverse)))".to_string(), important: false },
                CssProperty { name: "border-bottom-width".to_string(), value: "calc(1px * var(--tw-divide-y-reverse))".to_string(), important: false },
            ]),
            "divide-x" => Some(vec![
                CssProperty { name: "--tw-divide-x-reverse".to_string(), value: "0".to_string(), important: false },
                CssProperty { name: "border-right-width".to_string(), value: "calc(1px * var(--tw-divide-x-reverse))".to_string(), important: false },
                CssProperty { name: "border-left-width".to_string(), value: "calc(1px * calc(1 - var(--tw-divide-x-reverse)))".to_string(), important: false },
            ]),
            "divide-y-reverse" => Some(vec![
                CssProperty { name: "--tw-divide-y-reverse".to_string(), value: "1".to_string(), important: false },
            ]),
            "divide-x-reverse" => Some(vec![
                CssProperty { name: "--tw-divide-x-reverse".to_string(), value: "1".to_string(), important: false },
            ]),
            _ => None,
        }
    }

    /// Parse divide color classes
    fn parse_divide_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(color_part) = class.strip_prefix("divide-") {
            // Handle divide color classes like divide-zinc-100, divide-gray-200
            if let Some((color_name, opacity)) = color_part.split_once('/') {
                let color_value = self.get_color_value(color_name)?;
                let opacity_value = self.parse_opacity_value(opacity)?;
                let final_color = self.apply_opacity_to_color(&color_value, &opacity_value);
                return Some(vec![CssProperty { name: "border-color".to_string(), value: final_color, important: false }]);
            } else {
                let color_value = self.get_color_value(color_part)?;
                return Some(vec![CssProperty { name: "border-color".to_string(), value: color_value, important: false }]);
            }
        }
        None
    }

    /// Get color value
    fn get_color_value(&self, color: &str) -> Option<String> {
        match color {
            "gray-100" => Some("#f3f4f6".to_string()),
            "gray-200" => Some("#e5e7eb".to_string()),
            "gray-300" => Some("#d1d5db".to_string()),
            "gray-400" => Some("#9ca3af".to_string()),
            "gray-500" => Some("#6b7280".to_string()),
            "gray-600" => Some("#4b5563".to_string()),
            "gray-700" => Some("#374151".to_string()),
            "gray-800" => Some("#1f2937".to_string()),
            "gray-900" => Some("#111827".to_string()),
            "zinc-100" => Some("#f4f4f5".to_string()),
            "zinc-200" => Some("#e4e4e7".to_string()),
            "zinc-300" => Some("#d4d4d8".to_string()),
            "zinc-400" => Some("#a1a1aa".to_string()),
            "zinc-500" => Some("#71717a".to_string()),
            "zinc-600" => Some("#52525b".to_string()),
            "zinc-700" => Some("#3f3f46".to_string()),
            "zinc-800" => Some("#27272a".to_string()),
            "zinc-900" => Some("#18181b".to_string()),
            "black" => Some("#000000".to_string()),
            "white" => Some("#ffffff".to_string()),
            "transparent" => Some("transparent".to_string()),
            "current" => Some("currentColor".to_string()),
            _ => None,
        }
    }

    /// Parse opacity value
    fn parse_opacity_value(&self, opacity: &str) -> Option<f32> {
        opacity.parse::<f32>().ok().map(|o| o / 100.0)
    }

    /// Apply opacity to a color
    fn apply_opacity_to_color(&self, color: &str, opacity: &f32) -> String {
        if color.starts_with('#') && color.len() == 7 {
            let r = u8::from_str_radix(&color[1..3], 16).unwrap_or(0);
            let g = u8::from_str_radix(&color[3..5], 16).unwrap_or(0);
            let b = u8::from_str_radix(&color[5..7], 16).unwrap_or(0);
            format!("rgba({}, {}, {}, {})", r, g, b, opacity)
        } else {
            format!("{} / {}", color, opacity)
        }
    }
}

impl UtilityParser for DivideParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(properties) = self.parse_divide_direction_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_divide_color_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec!["divide-y", "divide-x", "divide-y-reverse", "divide-x-reverse", "divide-*"]
    }

    fn get_priority(&self) -> u32 { 85 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Layout }
}

impl Default for DivideParser {
    fn default() -> Self { Self::new() }
}
