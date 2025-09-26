//! Columns Utilities Parser
//!
//! This module provides parsing logic for column utilities,
//! such as `columns-3`, `columns-xs`, `columns-sm`, etc.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct ColumnsParser;

impl ColumnsParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse column number classes
    fn parse_column_number_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(number) = class.strip_prefix("columns-") {
            // Check if it's a number (1-12)
            if let Ok(_) = number.parse::<u32>() {
                return Some(vec![CssProperty {
                    name: "columns".to_string(),
                    value: number.to_string(),
                    important: false,
                }]);
            }
        }
        None
    }

    /// Parse column size classes
    fn parse_column_size_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "columns-3xs" => Some(vec![CssProperty {
                name: "columns".to_string(),
                value: "16rem".to_string(),
                important: false,
            }]),
            "columns-2xs" => Some(vec![CssProperty {
                name: "columns".to_string(),
                value: "18rem".to_string(),
                important: false,
            }]),
            "columns-xs" => Some(vec![CssProperty {
                name: "columns".to_string(),
                value: "20rem".to_string(),
                important: false,
            }]),
            "columns-sm" => Some(vec![CssProperty {
                name: "columns".to_string(),
                value: "24rem".to_string(),
                important: false,
            }]),
            "columns-md" => Some(vec![CssProperty {
                name: "columns".to_string(),
                value: "28rem".to_string(),
                important: false,
            }]),
            "columns-lg" => Some(vec![CssProperty {
                name: "columns".to_string(),
                value: "32rem".to_string(),
                important: false,
            }]),
            "columns-xl" => Some(vec![CssProperty {
                name: "columns".to_string(),
                value: "36rem".to_string(),
                important: false,
            }]),
            "columns-2xl" => Some(vec![CssProperty {
                name: "columns".to_string(),
                value: "42rem".to_string(),
                important: false,
            }]),
            "columns-3xl" => Some(vec![CssProperty {
                name: "columns".to_string(),
                value: "48rem".to_string(),
                important: false,
            }]),
            "columns-4xl" => Some(vec![CssProperty {
                name: "columns".to_string(),
                value: "56rem".to_string(),
                important: false,
            }]),
            "columns-5xl" => Some(vec![CssProperty {
                name: "columns".to_string(),
                value: "64rem".to_string(),
                important: false,
            }]),
            "columns-6xl" => Some(vec![CssProperty {
                name: "columns".to_string(),
                value: "72rem".to_string(),
                important: false,
            }]),
            "columns-7xl" => Some(vec![CssProperty {
                name: "columns".to_string(),
                value: "80rem".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse arbitrary column classes
    fn parse_arbitrary_column_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("columns-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "columns".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        None
    }

    /// Parse custom property column classes
    fn parse_custom_property_column_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(prop) = class.strip_prefix("columns-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "columns".to_string(),
                    value: format!("var({})", prop),
                    important: false,
                }]);
            }
        }
        None
    }
}

impl UtilityParser for ColumnsParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(properties) = self.parse_column_number_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_column_size_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_arbitrary_column_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_custom_property_column_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "columns-1",
            "columns-2",
            "columns-3",
            "columns-4",
            "columns-5",
            "columns-6",
            "columns-3xs",
            "columns-2xs",
            "columns-xs",
            "columns-sm",
            "columns-md",
            "columns-lg",
            "columns-xl",
            "columns-2xl",
            "columns-3xl",
            "columns-4xl",
            "columns-5xl",
            "columns-6xl",
            "columns-7xl",
            "columns-[*]",
            "columns-(*)",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Layout
    }
}

impl Default for ColumnsParser {
    fn default() -> Self {
        Self::new()
    }
}
