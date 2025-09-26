//! Advanced Spacing Utilities Parser
//!
//! This module provides parsing logic for advanced spacing-related Tailwind CSS utilities,
//! including space utilities, gap utilities, and complex spacing patterns.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct AdvancedSpacingParser;

impl AdvancedSpacingParser {
    pub fn new() -> Self { Self }

    /// Parse space utilities (space-y-*, space-x-*)
    fn parse_space_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(direction_and_value) = class.strip_prefix("space-") {
            if let Some(value) = direction_and_value.strip_prefix("y-") {
                let spacing_value = self.parse_spacing_value(value)?;
                return Some(vec![
                    CssProperty { 
                        name: "margin-top".to_string(), 
                        value: format!("calc({} * 0.5)", spacing_value), 
                        important: false 
                    },
                    CssProperty { 
                        name: "margin-bottom".to_string(), 
                        value: format!("calc({} * 0.5)", spacing_value), 
                        important: false 
                    },
                ]);
            }
            if let Some(value) = direction_and_value.strip_prefix("x-") {
                let spacing_value = self.parse_spacing_value(value)?;
                return Some(vec![
                    CssProperty { 
                        name: "margin-left".to_string(), 
                        value: format!("calc({} * 0.5)", spacing_value), 
                        important: false 
                    },
                    CssProperty { 
                        name: "margin-right".to_string(), 
                        value: format!("calc({} * 0.5)", spacing_value), 
                        important: false 
                    },
                ]);
            }
        }
        None
    }

    /// Parse gap utilities (gap-*, gap-x-*, gap-y-*)
    fn parse_gap_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("gap-") {
            let spacing_value = self.parse_spacing_value(value)?;
            return Some(vec![CssProperty { name: "gap".to_string(), value: spacing_value, important: false }]);
        }
        if let Some(value) = class.strip_prefix("gap-x-") {
            let spacing_value = self.parse_spacing_value(value)?;
            return Some(vec![CssProperty { name: "column-gap".to_string(), value: spacing_value, important: false }]);
        }
        if let Some(value) = class.strip_prefix("gap-y-") {
            let spacing_value = self.parse_spacing_value(value)?;
            return Some(vec![CssProperty { name: "row-gap".to_string(), value: spacing_value, important: false }]);
        }
        None
    }

    /// Parse spacing values
    fn parse_spacing_value(&self, value: &str) -> Option<String> {
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
            _ => None,
        }
    }
}

impl UtilityParser for AdvancedSpacingParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try space utilities first
        if let Some(properties) = self.parse_space_class(class) {
            return Some(properties);
        }
        
        // Try gap utilities
        if let Some(properties) = self.parse_gap_class(class) {
            return Some(properties);
        }
        
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec!["space-y-*", "space-x-*", "gap-*", "gap-x-*", "gap-y-*"]
    }

    fn get_priority(&self) -> u32 { 50 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Spacing }
}

impl Default for AdvancedSpacingParser {
    fn default() -> Self { Self::new() }
}
