//! Grid Template Rows Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS grid-template-rows utilities,
//! such as `grid-rows-1`, `grid-rows-2`, `grid-rows-none`, `grid-rows-subgrid`, etc.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GridTemplateRowsParser {
    grid_rows_map: HashMap<String, String>,
}

impl GridTemplateRowsParser {
    pub fn new() -> Self {
        let mut grid_rows_map = HashMap::new();
        grid_rows_map.insert("grid-rows-none".to_string(), "none".to_string());
        grid_rows_map.insert("grid-rows-subgrid".to_string(), "subgrid".to_string());

        // Add grid-rows-1 through grid-rows-12
        for i in 1..=12 {
            grid_rows_map.insert(
                format!("grid-rows-{}", i),
                format!("repeat({}, minmax(0, 1fr))", i),
            );
        }

        Self { grid_rows_map }
    }

    fn parse_grid_rows_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(grid_rows_value) = self.grid_rows_map.get(class) {
            return Some(vec![CssProperty {
                name: "grid-template-rows".to_string(),
                value: grid_rows_value.clone(),
                important: false,
            }]);
        }
        None
    }

    fn parse_arbitrary_grid_rows_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("grid-rows-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "grid-template-rows".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        None
    }

    fn parse_custom_property_grid_rows_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(prop) = class.strip_prefix("grid-rows-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "grid-template-rows".to_string(),
                    value: format!("var({})", prop),
                    important: false,
                }]);
            }
        }
        None
    }
}

impl UtilityParser for GridTemplateRowsParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_grid_rows_class(class)
            .or_else(|| self.parse_arbitrary_grid_rows_class(class))
            .or_else(|| self.parse_custom_property_grid_rows_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "grid-rows-1",
            "grid-rows-2",
            "grid-rows-3",
            "grid-rows-4",
            "grid-rows-5",
            "grid-rows-6",
            "grid-rows-7",
            "grid-rows-8",
            "grid-rows-9",
            "grid-rows-10",
            "grid-rows-11",
            "grid-rows-12",
            "grid-rows-none",
            "grid-rows-subgrid",
            "grid-rows-[*]",
            "grid-rows-(*)",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Grid
    }
}

impl Default for GridTemplateRowsParser {
    fn default() -> Self {
        Self::new()
    }
}
