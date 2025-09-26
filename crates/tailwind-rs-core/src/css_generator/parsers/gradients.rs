//! Gradient Utilities Parser
//!
//! This module provides parsing logic for gradient-related Tailwind CSS utilities,
//! such as `bg-linear-to-r`, `from-teal-500/0`, `via-teal-500/40`, `to-teal-500/0`.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct GradientParser;

impl GradientParser {
    pub fn new() -> Self { Self }

    /// Parse gradient direction classes
    fn parse_gradient_direction_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-gradient-to-r" | "bg-linear-to-r" => Some(vec![CssProperty { name: "background-image".to_string(), value: "linear-gradient(to right, var(--tw-gradient-stops))".to_string(), important: false }]),
            "bg-linear-to-l" => Some(vec![CssProperty { name: "background-image".to_string(), value: "linear-gradient(to left, var(--tw-gradient-stops))".to_string(), important: false }]),
            "bg-linear-to-t" => Some(vec![CssProperty { name: "background-image".to_string(), value: "linear-gradient(to top, var(--tw-gradient-stops))".to_string(), important: false }]),
            "bg-linear-to-b" => Some(vec![CssProperty { name: "background-image".to_string(), value: "linear-gradient(to bottom, var(--tw-gradient-stops))".to_string(), important: false }]),
            "bg-linear-to-tr" => Some(vec![CssProperty { name: "background-image".to_string(), value: "linear-gradient(to top right, var(--tw-gradient-stops))".to_string(), important: false }]),
            "bg-linear-to-tl" => Some(vec![CssProperty { name: "background-image".to_string(), value: "linear-gradient(to top left, var(--tw-gradient-stops))".to_string(), important: false }]),
            "bg-linear-to-br" => Some(vec![CssProperty { name: "background-image".to_string(), value: "linear-gradient(to bottom right, var(--tw-gradient-stops))".to_string(), important: false }]),
            "bg-linear-to-bl" => Some(vec![CssProperty { name: "background-image".to_string(), value: "linear-gradient(to bottom left, var(--tw-gradient-stops))".to_string(), important: false }]),
            "bg-radial" => Some(vec![CssProperty { name: "background-image".to_string(), value: "radial-gradient(ellipse at center, var(--tw-gradient-stops))".to_string(), important: false }]),
            "bg-conic" => Some(vec![CssProperty { name: "background-image".to_string(), value: "conic-gradient(from 0deg, var(--tw-gradient-stops))".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse gradient stop classes
    fn parse_gradient_stop_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(color_part) = class.strip_prefix("from-") {
            let color_value = self.get_gradient_color_value(color_part)?;
            return Some(vec![CssProperty { name: "--tw-gradient-from".to_string(), value: color_value, important: false }]);
        }
        if let Some(color_part) = class.strip_prefix("via-") {
            let color_value = self.get_gradient_color_value(color_part)?;
            return Some(vec![CssProperty { name: "--tw-gradient-via".to_string(), value: color_value, important: false }]);
        }
        if let Some(color_part) = class.strip_prefix("to-") {
            let color_value = self.get_gradient_color_value(color_part)?;
            return Some(vec![CssProperty { name: "--tw-gradient-to".to_string(), value: color_value, important: false }]);
        }
        None
    }

    /// Get gradient color value
    fn get_gradient_color_value(&self, color: &str) -> Option<String> {
        // Handle opacity with slash notation (e.g., teal-500/0, teal-500/40)
        if let Some((color_name, opacity)) = color.split_once('/') {
            let base_color = self.get_base_color_value(color_name)?;
            let opacity_value = self.parse_opacity_value(opacity)?;
            return Some(self.apply_opacity_to_color(&base_color, &opacity_value));
        }
        
        self.get_base_color_value(color)
    }

    /// Get base color value
    fn get_base_color_value(&self, color: &str) -> Option<String> {
        match color {
            // Teal colors
            "teal-50" => Some("#f0fdfa".to_string()),
            "teal-100" => Some("#ccfbf1".to_string()),
            "teal-200" => Some("#99f6e4".to_string()),
            "teal-300" => Some("#5eead4".to_string()),
            "teal-400" => Some("#2dd4bf".to_string()),
            "teal-500" => Some("#14b8a6".to_string()),
            "teal-600" => Some("#0d9488".to_string()),
            "teal-700" => Some("#0f766e".to_string()),
            "teal-800" => Some("#115e59".to_string()),
            "teal-900" => Some("#134e4a".to_string()),
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
            // Purple colors
            "purple-50" => Some("#faf5ff".to_string()),
            "purple-100" => Some("#f3e8ff".to_string()),
            "purple-200" => Some("#e9d5ff".to_string()),
            "purple-300" => Some("#d8b4fe".to_string()),
            "purple-400" => Some("#c084fc".to_string()),
            "purple-500" => Some("#a855f7".to_string()),
            "purple-600" => Some("#9333ea".to_string()),
            "purple-700" => Some("#7c3aed".to_string()),
            "purple-800" => Some("#6b21a8".to_string()),
            "purple-900" => Some("#581c87".to_string()),
            // Basic colors
            "white" => Some("#ffffff".to_string()),
            "black" => Some("#000000".to_string()),
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

impl UtilityParser for GradientParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(properties) = self.parse_gradient_direction_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_gradient_stop_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "bg-linear-to-*", "bg-radial", "bg-conic",
            "from-*", "via-*", "to-*"
        ]
    }

    fn get_priority(&self) -> u32 { 95 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Color }
}

impl Default for GradientParser {
    fn default() -> Self { Self::new() }
}
