//! Grid Row Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS grid-row utilities,
//! such as `row-span-1`, `row-span-2`, `row-start-1`, `row-end-2`, etc.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GridRowParser {
    row_span_map: HashMap<String, String>,
    row_start_map: HashMap<String, String>,
    row_end_map: HashMap<String, String>,
}

impl GridRowParser {
    pub fn new() -> Self {
        let mut row_span_map = HashMap::new();
        row_span_map.insert("row-span-full".to_string(), "1 / -1".to_string());

        // Add row-span-1 through row-span-12
        for i in 1..=12 {
            row_span_map.insert(
                format!("row-span-{}", i),
                format!("span {} / span {}", i, i),
            );
        }

        let mut row_start_map = HashMap::new();
        row_start_map.insert("row-start-auto".to_string(), "auto".to_string());

        // Add row-start-1 through row-start-12
        for i in 1..=12 {
            row_start_map.insert(format!("row-start-{}", i), i.to_string());
        }

        // Add negative row-start values
        for i in 1..=12 {
            row_start_map.insert(format!("-row-start-{}", i), format!("calc({} * -1)", i));
        }

        let mut row_end_map = HashMap::new();
        row_end_map.insert("row-end-auto".to_string(), "auto".to_string());

        // Add row-end-1 through row-end-12
        for i in 1..=12 {
            row_end_map.insert(format!("row-end-{}", i), i.to_string());
        }

        // Add negative row-end values
        for i in 1..=12 {
            row_end_map.insert(format!("-row-end-{}", i), format!("calc({} * -1)", i));
        }

        Self {
            row_span_map,
            row_start_map,
            row_end_map,
        }
    }

    fn parse_row_span_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(row_span_value) = self.row_span_map.get(class) {
            return Some(vec![CssProperty {
                name: "grid-row".to_string(),
                value: row_span_value.clone(),
                important: false,
            }]);
        }
        None
    }

    fn parse_row_start_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(row_start_value) = self.row_start_map.get(class) {
            return Some(vec![CssProperty {
                name: "grid-row-start".to_string(),
                value: row_start_value.clone(),
                important: false,
            }]);
        }
        None
    }

    fn parse_row_end_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(row_end_value) = self.row_end_map.get(class) {
            return Some(vec![CssProperty {
                name: "grid-row-end".to_string(),
                value: row_end_value.clone(),
                important: false,
            }]);
        }
        None
    }

    fn parse_arbitrary_row_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("row-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "grid-row".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        if let Some(value) = class.strip_prefix("row-span-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "grid-row".to_string(),
                    value: format!("span {} / span {}", value, value),
                    important: false,
                }]);
            }
        }
        if let Some(value) = class.strip_prefix("row-start-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "grid-row-start".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        if let Some(value) = class.strip_prefix("row-end-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "grid-row-end".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        None
    }

    fn parse_custom_property_row_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(prop) = class.strip_prefix("row-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "grid-row".to_string(),
                    value: format!("var({})", prop),
                    important: false,
                }]);
            }
        }
        if let Some(prop) = class.strip_prefix("row-span-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "grid-row".to_string(),
                    value: format!("span var({}) / span var({})", prop, prop),
                    important: false,
                }]);
            }
        }
        if let Some(prop) = class.strip_prefix("row-start-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "grid-row-start".to_string(),
                    value: format!("var({})", prop),
                    important: false,
                }]);
            }
        }
        if let Some(prop) = class.strip_prefix("row-end-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "grid-row-end".to_string(),
                    value: format!("var({})", prop),
                    important: false,
                }]);
            }
        }
        None
    }
}

impl UtilityParser for GridRowParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_row_span_class(class)
            .or_else(|| self.parse_row_start_class(class))
            .or_else(|| self.parse_row_end_class(class))
            .or_else(|| self.parse_arbitrary_row_class(class))
            .or_else(|| self.parse_custom_property_row_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "row-span-1",
            "row-span-2",
            "row-span-3",
            "row-span-4",
            "row-span-5",
            "row-span-6",
            "row-span-7",
            "row-span-8",
            "row-span-9",
            "row-span-10",
            "row-span-11",
            "row-span-12",
            "row-span-full",
            "row-start-1",
            "row-start-2",
            "row-start-3",
            "row-start-4",
            "row-start-5",
            "row-start-6",
            "row-start-7",
            "row-start-8",
            "row-start-9",
            "row-start-10",
            "row-start-11",
            "row-start-12",
            "row-start-auto",
            "-row-start-1",
            "-row-start-2",
            "-row-start-3",
            "-row-start-4",
            "-row-start-5",
            "-row-start-6",
            "-row-start-7",
            "-row-start-8",
            "-row-start-9",
            "-row-start-10",
            "-row-start-11",
            "-row-start-12",
            "row-end-1",
            "row-end-2",
            "row-end-3",
            "row-end-4",
            "row-end-5",
            "row-end-6",
            "row-end-7",
            "row-end-8",
            "row-end-9",
            "row-end-10",
            "row-end-11",
            "row-end-12",
            "row-end-auto",
            "-row-end-1",
            "-row-end-2",
            "-row-end-3",
            "-row-end-4",
            "-row-end-5",
            "-row-end-6",
            "-row-end-7",
            "-row-end-8",
            "-row-end-9",
            "-row-end-10",
            "-row-end-11",
            "-row-end-12",
            "row-[*]",
            "row-span-[*]",
            "row-start-[*]",
            "row-end-[*]",
            "row-(*)",
            "row-span-(*)",
            "row-start-(*)",
            "row-end-(*)",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Grid
    }
}

impl Default for GridRowParser {
    fn default() -> Self {
        Self::new()
    }
}
