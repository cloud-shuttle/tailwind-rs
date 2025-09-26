//! Sizing Utilities Parser
//!
//! This module provides parsing logic for sizing-related Tailwind CSS utilities,
//! including width, height, min-width, min-height, max-width, and max-height.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct SizingParser;

impl SizingParser {
    pub fn new() -> Self { Self }

    /// Parse width classes
    fn parse_width_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("w-") {
            let css_value = self.parse_sizing_value(value)?;
            return Some(vec![CssProperty { name: "width".to_string(), value: css_value, important: false }]);
        }
        None
    }

    /// Parse height classes
    fn parse_height_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("h-") {
            let css_value = self.parse_sizing_value(value)?;
            return Some(vec![CssProperty { name: "height".to_string(), value: css_value, important: false }]);
        }
        None
    }

    /// Parse max-width classes
    fn parse_max_width_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("max-w-") {
            let css_value = self.parse_max_width_value(value)?;
            return Some(vec![CssProperty { name: "max-width".to_string(), value: css_value, important: false }]);
        }
        None
    }

    /// Parse min-width classes
    fn parse_min_width_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("min-w-") {
            let css_value = self.parse_sizing_value(value)?;
            return Some(vec![CssProperty { name: "min-width".to_string(), value: css_value, important: false }]);
        }
        None
    }

    /// Parse max-height classes
    fn parse_max_height_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("max-h-") {
            let css_value = self.parse_sizing_value(value)?;
            return Some(vec![CssProperty { name: "max-height".to_string(), value: css_value, important: false }]);
        }
        None
    }

    /// Parse min-height classes
    fn parse_min_height_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("min-h-") {
            let css_value = self.parse_sizing_value(value)?;
            return Some(vec![CssProperty { name: "min-height".to_string(), value: css_value, important: false }]);
        }
        None
    }

    /// Parse sizing values (width, height, etc.)
    fn parse_sizing_value(&self, value: &str) -> Option<String> {
        match value {
            "0" => Some("0".to_string()),
            "px" => Some("1px".to_string()),
            "0.5" => Some("0.125rem".to_string()),
            "1" => Some("0.25rem".to_string()),
            "1.5" => Some("0.375rem".to_string()),
            "2" => Some("0.5rem".to_string()),
            "2.5" => Some("0.625rem".to_string()),
            "3" => Some("0.75rem".to_string()),
            "3.5" => Some("0.875rem".to_string()),
            "4" => Some("1rem".to_string()),
            "5" => Some("1.25rem".to_string()),
            "6" => Some("1.5rem".to_string()),
            "7" => Some("1.75rem".to_string()),
            "8" => Some("2rem".to_string()),
            "9" => Some("2.25rem".to_string()),
            "10" => Some("2.5rem".to_string()),
            "11" => Some("2.75rem".to_string()),
            "12" => Some("3rem".to_string()),
            "14" => Some("3.5rem".to_string()),
            "16" => Some("4rem".to_string()),
            "20" => Some("5rem".to_string()),
            "24" => Some("6rem".to_string()),
            "28" => Some("7rem".to_string()),
            "32" => Some("8rem".to_string()),
            "36" => Some("9rem".to_string()),
            "40" => Some("10rem".to_string()),
            "44" => Some("11rem".to_string()),
            "48" => Some("12rem".to_string()),
            "52" => Some("13rem".to_string()),
            "56" => Some("14rem".to_string()),
            "60" => Some("15rem".to_string()),
            "64" => Some("16rem".to_string()),
            "72" => Some("18rem".to_string()),
            "80" => Some("20rem".to_string()),
            "96" => Some("24rem".to_string()),
            "auto" => Some("auto".to_string()),
            "full" => Some("100%".to_string()),
            "screen" => Some("100vh".to_string()),
            "min" => Some("min-content".to_string()),
            "max" => Some("max-content".to_string()),
            "fit" => Some("fit-content".to_string()),
            _ => None,
        }
    }

    /// Parse max-width specific values
    fn parse_max_width_value(&self, value: &str) -> Option<String> {
        match value {
            "none" => Some("none".to_string()),
            "xs" => Some("20rem".to_string()),
            "sm" => Some("24rem".to_string()),
            "md" => Some("28rem".to_string()),
            "lg" => Some("32rem".to_string()),
            "xl" => Some("36rem".to_string()),
            "2xl" => Some("42rem".to_string()),
            "3xl" => Some("48rem".to_string()),
            "4xl" => Some("56rem".to_string()),
            "5xl" => Some("64rem".to_string()),
            "6xl" => Some("72rem".to_string()),
            "7xl" => Some("80rem".to_string()),
            "full" => Some("100%".to_string()),
            "min" => Some("min-content".to_string()),
            "max" => Some("max-content".to_string()),
            "fit" => Some("fit-content".to_string()),
            "prose" => Some("65ch".to_string()),
            "screen-sm" => Some("640px".to_string()),
            "screen-md" => Some("768px".to_string()),
            "screen-lg" => Some("1024px".to_string()),
            "screen-xl" => Some("1280px".to_string()),
            "screen-2xl" => Some("1536px".to_string()),
            _ => self.parse_sizing_value(value), // Fall back to regular sizing values
        }
    }
}

impl UtilityParser for SizingParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity
        if let Some(properties) = self.parse_width_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_height_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_max_width_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_min_width_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_max_height_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_min_height_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec!["w-*", "h-*", "max-w-*", "min-w-*", "max-h-*", "min-h-*"]
    }

    fn get_priority(&self) -> u32 { 80 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Layout }
}

impl Default for SizingParser {
    fn default() -> Self { Self::new() }
}
