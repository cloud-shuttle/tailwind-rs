//! Margin Utilities Parser
//!
//! This module provides parsing logic for margin-related Tailwind CSS utilities,
//! including margin sides, margin auto, and margin values.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct MarginParser;

impl MarginParser {
    pub fn new() -> Self { Self }

    /// Parse margin auto classes
    fn parse_margin_auto_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "mx-auto" => Some(vec![
                CssProperty { name: "margin-left".to_string(), value: "auto".to_string(), important: false },
                CssProperty { name: "margin-right".to_string(), value: "auto".to_string(), important: false },
            ]),
            "my-auto" => Some(vec![
                CssProperty { name: "margin-top".to_string(), value: "auto".to_string(), important: false },
                CssProperty { name: "margin-bottom".to_string(), value: "auto".to_string(), important: false },
            ]),
            "mt-auto" => Some(vec![CssProperty { name: "margin-top".to_string(), value: "auto".to_string(), important: false }]),
            "mr-auto" => Some(vec![CssProperty { name: "margin-right".to_string(), value: "auto".to_string(), important: false }]),
            "mb-auto" => Some(vec![CssProperty { name: "margin-bottom".to_string(), value: "auto".to_string(), important: false }]),
            "ml-auto" => Some(vec![CssProperty { name: "margin-left".to_string(), value: "auto".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse margin side classes
    fn parse_margin_side_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("mt-") {
            let margin_value = self.parse_margin_value(value)?;
            return Some(vec![CssProperty { name: "margin-top".to_string(), value: margin_value, important: false }]);
        }
        if let Some(value) = class.strip_prefix("-mt-") {
            let margin_value = self.parse_margin_value(value)?;
            return Some(vec![CssProperty { name: "margin-top".to_string(), value: format!("-{}", margin_value), important: false }]);
        }
        if let Some(value) = class.strip_prefix("mr-") {
            let margin_value = self.parse_margin_value(value)?;
            return Some(vec![CssProperty { name: "margin-right".to_string(), value: margin_value, important: false }]);
        }
        if let Some(value) = class.strip_prefix("-mr-") {
            let margin_value = self.parse_margin_value(value)?;
            return Some(vec![CssProperty { name: "margin-right".to_string(), value: format!("-{}", margin_value), important: false }]);
        }
        if let Some(value) = class.strip_prefix("mb-") {
            let margin_value = self.parse_margin_value(value)?;
            return Some(vec![CssProperty { name: "margin-bottom".to_string(), value: margin_value, important: false }]);
        }
        if let Some(value) = class.strip_prefix("-mb-") {
            let margin_value = self.parse_margin_value(value)?;
            return Some(vec![CssProperty { name: "margin-bottom".to_string(), value: format!("-{}", margin_value), important: false }]);
        }
        if let Some(value) = class.strip_prefix("ml-") {
            let margin_value = self.parse_margin_value(value)?;
            return Some(vec![CssProperty { name: "margin-left".to_string(), value: margin_value, important: false }]);
        }
        if let Some(value) = class.strip_prefix("-ml-") {
            let margin_value = self.parse_margin_value(value)?;
            return Some(vec![CssProperty { name: "margin-left".to_string(), value: format!("-{}", margin_value), important: false }]);
        }
        if let Some(value) = class.strip_prefix("mx-") {
            let margin_value = self.parse_margin_value(value)?;
            return Some(vec![
                CssProperty { name: "margin-left".to_string(), value: margin_value.clone(), important: false },
                CssProperty { name: "margin-right".to_string(), value: margin_value, important: false },
            ]);
        }
        if let Some(value) = class.strip_prefix("-mx-") {
            let margin_value = self.parse_margin_value(value)?;
            return Some(vec![
                CssProperty { name: "margin-left".to_string(), value: format!("-{}", margin_value.clone()), important: false },
                CssProperty { name: "margin-right".to_string(), value: format!("-{}", margin_value), important: false },
            ]);
        }
        if let Some(value) = class.strip_prefix("my-") {
            let margin_value = self.parse_margin_value(value)?;
            return Some(vec![
                CssProperty { name: "margin-top".to_string(), value: margin_value.clone(), important: false },
                CssProperty { name: "margin-bottom".to_string(), value: margin_value, important: false },
            ]);
        }
        if let Some(value) = class.strip_prefix("-my-") {
            let margin_value = self.parse_margin_value(value)?;
            return Some(vec![
                CssProperty { name: "margin-top".to_string(), value: format!("-{}", margin_value.clone()), important: false },
                CssProperty { name: "margin-bottom".to_string(), value: format!("-{}", margin_value), important: false },
            ]);
        }
        None
    }

    /// Parse margin values
    fn parse_margin_value(&self, value: &str) -> Option<String> {
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
            _ => None,
        }
    }

    /// Parse general margin classes (m-*, -m-*)
    fn parse_margin_general_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("m-") {
            let margin_value = self.parse_margin_value(value)?;
            return Some(vec![CssProperty { name: "margin".to_string(), value: margin_value, important: false }]);
        }
        if let Some(value) = class.strip_prefix("-m-") {
            let margin_value = self.parse_margin_value(value)?;
            return Some(vec![CssProperty { name: "margin".to_string(), value: format!("-{}", margin_value), important: false }]);
        }
        None
    }
}

impl UtilityParser for MarginParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity
        if let Some(properties) = self.parse_margin_auto_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_margin_side_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_margin_general_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec!["mx-auto", "my-auto", "mt-auto", "mr-auto", "mb-auto", "ml-auto", "mt-*", "mr-*", "mb-*", "ml-*", "mx-*", "my-*"]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Spacing }
}

impl Default for MarginParser {
    fn default() -> Self { Self::new() }
}
