//! Advanced Border Utilities Parser
//!
//! This module provides parsing logic for advanced border-related Tailwind CSS utilities,
//! including border sides, border radius, and border styles.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct AdvancedBorderParser;

impl AdvancedBorderParser {
    pub fn new() -> Self { Self }

    /// Parse border side classes
    fn parse_border_side_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "border-t" => Some(vec![CssProperty { name: "border-top-width".to_string(), value: "1px".to_string(), important: false }]),
            "border-r" => Some(vec![CssProperty { name: "border-right-width".to_string(), value: "1px".to_string(), important: false }]),
            "border-b" => Some(vec![CssProperty { name: "border-bottom-width".to_string(), value: "1px".to_string(), important: false }]),
            "border-l" => Some(vec![CssProperty { name: "border-left-width".to_string(), value: "1px".to_string(), important: false }]),
            "border-x" => Some(vec![
                CssProperty { name: "border-left-width".to_string(), value: "1px".to_string(), important: false },
                CssProperty { name: "border-right-width".to_string(), value: "1px".to_string(), important: false },
            ]),
            "border-y" => Some(vec![
                CssProperty { name: "border-top-width".to_string(), value: "1px".to_string(), important: false },
                CssProperty { name: "border-bottom-width".to_string(), value: "1px".to_string(), important: false },
            ]),
            _ => None,
        }
    }

    /// Parse border radius classes
    fn parse_border_radius_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "rounded-none" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "0".to_string(), important: false }]),
            "rounded-sm" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "0.125rem".to_string(), important: false }]),
            "rounded" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "0.25rem".to_string(), important: false }]),
            "rounded-md" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "0.375rem".to_string(), important: false }]),
            "rounded-lg" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "0.5rem".to_string(), important: false }]),
            "rounded-xl" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "0.75rem".to_string(), important: false }]),
            "rounded-2xl" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "1rem".to_string(), important: false }]),
            "rounded-3xl" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "1.5rem".to_string(), important: false }]),
            "rounded-full" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "9999px".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse border radius side classes
    fn parse_border_radius_side_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(radius_part) = class.strip_prefix("rounded-") {
            if let Some((side, radius)) = radius_part.split_once('-') {
                let radius_value = self.parse_radius_value(radius)?;
                let property_name = match side {
                    "t" => "border-top-left-radius",
                    "r" => "border-top-right-radius", 
                    "b" => "border-bottom-right-radius",
                    "l" => "border-bottom-left-radius",
                    "tl" => "border-top-left-radius",
                    "tr" => "border-top-right-radius",
                    "br" => "border-bottom-right-radius",
                    "bl" => "border-bottom-left-radius",
                    _ => return None,
                };
                return Some(vec![CssProperty { name: property_name.to_string(), value: radius_value, important: false }]);
            }
        }
        None
    }

    /// Parse border style classes
    fn parse_border_style_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "border-solid" => Some(vec![CssProperty { name: "border-style".to_string(), value: "solid".to_string(), important: false }]),
            "border-dashed" => Some(vec![CssProperty { name: "border-style".to_string(), value: "dashed".to_string(), important: false }]),
            "border-dotted" => Some(vec![CssProperty { name: "border-style".to_string(), value: "dotted".to_string(), important: false }]),
            "border-double" => Some(vec![CssProperty { name: "border-style".to_string(), value: "double".to_string(), important: false }]),
            "border-none" => Some(vec![CssProperty { name: "border-style".to_string(), value: "none".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse radius values
    fn parse_radius_value(&self, value: &str) -> Option<String> {
        match value {
            "none" => Some("0".to_string()),
            "sm" => Some("0.125rem".to_string()),
            "" => Some("0.25rem".to_string()), // Default rounded
            "md" => Some("0.375rem".to_string()),
            "lg" => Some("0.5rem".to_string()),
            "xl" => Some("0.75rem".to_string()),
            "2xl" => Some("1rem".to_string()),
            "3xl" => Some("1.5rem".to_string()),
            "full" => Some("9999px".to_string()),
            _ => None,
        }
    }
}

impl UtilityParser for AdvancedBorderParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity
        if let Some(properties) = self.parse_border_side_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_border_radius_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_border_radius_side_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_border_style_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec!["border-t", "border-r", "border-b", "border-l", "border-x", "border-y", 
             "rounded-*", "border-solid", "border-dashed", "border-dotted", "border-double", "border-none"]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Borders }
}

impl Default for AdvancedBorderParser {
    fn default() -> Self { Self::new() }
}
