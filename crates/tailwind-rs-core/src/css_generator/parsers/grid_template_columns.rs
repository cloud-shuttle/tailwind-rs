//! Grid Template Columns Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS grid-template-columns utilities,
//! such as `grid-cols-1`, `grid-cols-2`, `grid-cols-none`, `grid-cols-subgrid`, etc.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GridTemplateColumnsParser {
    grid_cols_map: HashMap<String, String>,
}

impl GridTemplateColumnsParser {
    pub fn new() -> Self {
        let mut grid_cols_map = HashMap::new();
        grid_cols_map.insert("grid-cols-none".to_string(), "none".to_string());
        grid_cols_map.insert("grid-cols-subgrid".to_string(), "subgrid".to_string());

        // Add grid-cols-1 through grid-cols-12
        for i in 1..=12 {
            grid_cols_map.insert(
                format!("grid-cols-{}", i),
                format!("repeat({}, minmax(0, 1fr))", i),
            );
        }

        Self { grid_cols_map }
    }

    fn parse_grid_cols_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(grid_cols_value) = self.grid_cols_map.get(class) {
            return Some(vec![CssProperty {
                name: "grid-template-columns".to_string(),
                value: grid_cols_value.clone(),
                important: false,
            }]);
        }
        None
    }

    fn parse_arbitrary_grid_cols_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("grid-cols-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "grid-template-columns".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        None
    }

    fn parse_custom_property_grid_cols_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(prop) = class.strip_prefix("grid-cols-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "grid-template-columns".to_string(),
                    value: format!("var({})", prop),
                    important: false,
                }]);
            }
        }
        None
    }
}

impl UtilityParser for GridTemplateColumnsParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_grid_cols_class(class)
            .or_else(|| self.parse_arbitrary_grid_cols_class(class))
            .or_else(|| self.parse_custom_property_grid_cols_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "grid-cols-1",
            "grid-cols-2",
            "grid-cols-3",
            "grid-cols-4",
            "grid-cols-5",
            "grid-cols-6",
            "grid-cols-7",
            "grid-cols-8",
            "grid-cols-9",
            "grid-cols-10",
            "grid-cols-11",
            "grid-cols-12",
            "grid-cols-none",
            "grid-cols-subgrid",
            "grid-cols-[*]",
            "grid-cols-(*)",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Grid
    }
}

impl Default for GridTemplateColumnsParser {
    fn default() -> Self {
        Self::new()
    }
}
