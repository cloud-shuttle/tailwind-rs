//! Advanced Grid Utilities Parser
//!
//! This module provides parsing logic for advanced grid-related Tailwind CSS utilities,
//! including grid columns, grid rows, and grid gaps.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct AdvancedGridParser;

impl AdvancedGridParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse grid columns classes
    fn parse_grid_columns_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(columns) = class.strip_prefix("grid-cols-") {
            let columns_value = self.parse_grid_columns_value(columns)?;
            return Some(vec![CssProperty {
                name: "grid-template-columns".to_string(),
                value: columns_value,
                important: false,
            }]);
        }
        None
    }

    /// Parse grid rows classes
    fn parse_grid_rows_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(rows) = class.strip_prefix("grid-rows-") {
            let rows_value = self.parse_grid_rows_value(rows)?;
            return Some(vec![CssProperty {
                name: "grid-template-rows".to_string(),
                value: rows_value,
                important: false,
            }]);
        }
        None
    }

    /// Parse grid column span classes
    fn parse_col_span_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(span) = class.strip_prefix("col-span-") {
            let span_value = self.parse_span_value(span)?;
            return Some(vec![CssProperty {
                name: "grid-column".to_string(),
                value: format!("span {} / span {}", span_value, span_value),
                important: false,
            }]);
        }
        None
    }

    /// Parse grid row span classes
    fn parse_row_span_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(span) = class.strip_prefix("row-span-") {
            let span_value = self.parse_span_value(span)?;
            return Some(vec![CssProperty {
                name: "grid-row".to_string(),
                value: format!("span {} / span {}", span_value, span_value),
                important: false,
            }]);
        }
        None
    }

    /// Parse grid columns values
    fn parse_grid_columns_value(&self, columns: &str) -> Option<String> {
        match columns {
            "1" => Some("repeat(1, minmax(0, 1fr))".to_string()),
            "2" => Some("repeat(2, minmax(0, 1fr))".to_string()),
            "3" => Some("repeat(3, minmax(0, 1fr))".to_string()),
            "4" => Some("repeat(4, minmax(0, 1fr))".to_string()),
            "5" => Some("repeat(5, minmax(0, 1fr))".to_string()),
            "6" => Some("repeat(6, minmax(0, 1fr))".to_string()),
            "7" => Some("repeat(7, minmax(0, 1fr))".to_string()),
            "8" => Some("repeat(8, minmax(0, 1fr))".to_string()),
            "9" => Some("repeat(9, minmax(0, 1fr))".to_string()),
            "10" => Some("repeat(10, minmax(0, 1fr))".to_string()),
            "11" => Some("repeat(11, minmax(0, 1fr))".to_string()),
            "12" => Some("repeat(12, minmax(0, 1fr))".to_string()),
            "none" => Some("none".to_string()),
            _ => None,
        }
    }

    /// Parse grid rows values
    fn parse_grid_rows_value(&self, rows: &str) -> Option<String> {
        match rows {
            "1" => Some("repeat(1, minmax(0, 1fr))".to_string()),
            "2" => Some("repeat(2, minmax(0, 1fr))".to_string()),
            "3" => Some("repeat(3, minmax(0, 1fr))".to_string()),
            "4" => Some("repeat(4, minmax(0, 1fr))".to_string()),
            "5" => Some("repeat(5, minmax(0, 1fr))".to_string()),
            "6" => Some("repeat(6, minmax(0, 1fr))".to_string()),
            "none" => Some("none".to_string()),
            _ => None,
        }
    }

    /// Parse span values
    fn parse_span_value(&self, span: &str) -> Option<String> {
        match span {
            "1" => Some("1".to_string()),
            "2" => Some("2".to_string()),
            "3" => Some("3".to_string()),
            "4" => Some("4".to_string()),
            "5" => Some("5".to_string()),
            "6" => Some("6".to_string()),
            "7" => Some("7".to_string()),
            "8" => Some("8".to_string()),
            "9" => Some("9".to_string()),
            "10" => Some("10".to_string()),
            "11" => Some("11".to_string()),
            "12" => Some("12".to_string()),
            "full" => Some("1 / -1".to_string()),
            "auto" => Some("auto".to_string()),
            _ => None,
        }
    }
}

impl UtilityParser for AdvancedGridParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity
        if let Some(properties) = self.parse_grid_columns_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_grid_rows_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_col_span_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_row_span_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec!["grid-cols-*", "grid-rows-*", "col-span-*", "row-span-*"]
    }

    fn get_priority(&self) -> u32 {
        80
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Grid
    }
}

impl Default for AdvancedGridParser {
    fn default() -> Self {
        Self::new()
    }
}
