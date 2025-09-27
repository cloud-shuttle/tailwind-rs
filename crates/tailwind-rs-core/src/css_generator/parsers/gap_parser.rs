//! Gap Parser for Tailwind CSS
//!
//! This module handles parsing of gap-related utilities:
//! - Gap (gap-*)
//! - Column gap (gap-x-*)
//! - Row gap (gap-y-*)
//! - Space utilities (space-x-*, space-y-*)
//! - Space reverse utilities (space-x-reverse, space-y-reverse)
//! - Arbitrary space utilities (space-x-[...], space-y-[...])

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

/// Parser for gap utilities
#[derive(Debug, Clone)]
pub struct GapParser;

impl Default for GapParser {
    fn default() -> Self {
        Self::new()
    }
}

impl GapParser {
    /// Create a new gap parser
    pub fn new() -> Self {
        Self
    }

    /// Parse gap classes
    pub fn parse_gap_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("gap-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "gap".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("gap-x-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "column-gap".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("gap-y-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "row-gap".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        // Space utilities
        if let Some(direction_and_value) = class.strip_prefix("space-") {
            if let Some(value) = direction_and_value.strip_prefix("x-") {
                if let Some(spacing) = self.get_spacing_value(value) {
                    return Some(vec![
                        CssProperty { 
                            name: "margin-left".to_string(), 
                            value: format!("calc({} * 0.5)", spacing), 
                            important: false 
                        },
                        CssProperty { 
                            name: "margin-right".to_string(), 
                            value: format!("calc({} * 0.5)", spacing), 
                            important: false 
                        },
                    ]);
                }
            }
            if let Some(value) = direction_and_value.strip_prefix("y-") {
                if let Some(spacing) = self.get_spacing_value(value) {
                    return Some(vec![
                        CssProperty { 
                            name: "margin-top".to_string(), 
                            value: format!("calc({} * 0.5)", spacing), 
                            important: false 
                        },
                        CssProperty { 
                            name: "margin-bottom".to_string(), 
                            value: format!("calc({} * 0.5)", spacing), 
                            important: false 
                        },
                    ]);
                }
            }
        }
        
        // Space reverse utilities
        if class == "space-x-reverse" {
            return Some(vec![CssProperty {
                name: "--tw-space-x-reverse".to_string(),
                value: "1".to_string(),
                important: false,
            }]);
        }
        
        if class == "space-y-reverse" {
            return Some(vec![CssProperty {
                name: "--tw-space-y-reverse".to_string(),
                value: "1".to_string(),
                important: false,
            }]);
        }
        
        // Arbitrary space utilities
        if let Some(value) = class.strip_prefix("space-x-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![
                    CssProperty { 
                        name: "margin-left".to_string(), 
                        value: format!("calc({} * 0.5)", value), 
                        important: false 
                    },
                    CssProperty { 
                        name: "margin-right".to_string(), 
                        value: format!("calc({} * 0.5)", value), 
                        important: false 
                    },
                ]);
            }
        }
        
        if let Some(value) = class.strip_prefix("space-y-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![
                    CssProperty { 
                        name: "margin-top".to_string(), 
                        value: format!("calc({} * 0.5)", value), 
                        important: false 
                    },
                    CssProperty { 
                        name: "margin-bottom".to_string(), 
                        value: format!("calc({} * 0.5)", value), 
                        important: false 
                    },
                ]);
            }
        }
        
        None
    }

    /// Get spacing value for a given size
    fn get_spacing_value(&self, size: &str) -> Option<String> {
        match size {
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
            _ => {
                // Check for arbitrary values
                if let Some(value) = size.strip_prefix("[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(value.to_string());
                    }
                }
                // Check for custom properties
                if let Some(value) = size.strip_prefix("(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(format!("var({})", value));
                    }
                }
                None
            }
        }
    }
}

impl UtilityParser for GapParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_gap_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "gap-*",
            "gap-x-*",
            "gap-y-*",
            "space-x-*",
            "space-y-*",
            "space-x-reverse",
            "space-y-reverse",
            "space-x-[*]",
            "space-y-[*]",
        ]
    }

    fn get_priority(&self) -> u32 {
        50 // Medium priority for gap
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Spacing
    }
}
