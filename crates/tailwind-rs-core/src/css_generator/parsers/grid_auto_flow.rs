//! Grid Auto Flow Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS grid-auto-flow utilities,
//! such as `grid-flow-row`, `grid-flow-col`, `grid-flow-dense`, etc.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GridAutoFlowParser {
    grid_flow_map: HashMap<String, String>,
}

impl GridAutoFlowParser {
    pub fn new() -> Self {
        let mut grid_flow_map = HashMap::new();
        grid_flow_map.insert("grid-flow-row".to_string(), "row".to_string());
        grid_flow_map.insert("grid-flow-col".to_string(), "column".to_string());
        grid_flow_map.insert("grid-flow-dense".to_string(), "dense".to_string());
        grid_flow_map.insert("grid-flow-row-dense".to_string(), "row dense".to_string());
        grid_flow_map.insert(
            "grid-flow-col-dense".to_string(),
            "column dense".to_string(),
        );

        Self { grid_flow_map }
    }

    fn parse_grid_flow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(grid_flow_value) = self.grid_flow_map.get(class) {
            return Some(vec![CssProperty {
                name: "grid-auto-flow".to_string(),
                value: grid_flow_value.clone(),
                important: false,
            }]);
        }
        None
    }
}

impl UtilityParser for GridAutoFlowParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_grid_flow_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "grid-flow-row",
            "grid-flow-col",
            "grid-flow-dense",
            "grid-flow-row-dense",
            "grid-flow-col-dense",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Grid
    }
}

impl Default for GridAutoFlowParser {
    fn default() -> Self {
        Self::new()
    }
}
