//! Grid Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS grid utilities,
//! such as `grid`, `grid-cols-*`, `grid-rows-*`, `col-*`, `row-*`, etc.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct GridParser;

impl GridParser {
    pub fn new() -> Self { Self }

    /// Parse basic grid display class
    fn parse_grid_display_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "grid" => Some(vec![CssProperty { 
                name: "display".to_string(), 
                value: "grid".to_string(), 
                important: false 
            }]),
            _ => None,
        }
    }

    /// Parse grid template columns classes
    fn parse_grid_template_columns_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("grid-cols-") {
            match value {
                "none" => Some(vec![CssProperty { 
                    name: "grid-template-columns".to_string(), 
                    value: "none".to_string(), 
                    important: false 
                }]),
                "subgrid" => Some(vec![CssProperty { 
                    name: "grid-template-columns".to_string(), 
                    value: "subgrid".to_string(), 
                    important: false 
                }]),
                _ => {
                    // Handle numeric values (1-12)
                    if let Ok(num) = value.parse::<u32>() {
                        if num >= 1 && num <= 12 {
                            return Some(vec![CssProperty { 
                                name: "grid-template-columns".to_string(), 
                                value: format!("repeat({}, minmax(0, 1fr))", num), 
                                important: false 
                            }]);
                        }
                    }
                    None
                }
            }
        } else {
            None
        }
    }

    /// Parse grid template rows classes
    fn parse_grid_template_rows_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("grid-rows-") {
            match value {
                "none" => Some(vec![CssProperty { 
                    name: "grid-template-rows".to_string(), 
                    value: "none".to_string(), 
                    important: false 
                }]),
                "subgrid" => Some(vec![CssProperty { 
                    name: "grid-template-rows".to_string(), 
                    value: "subgrid".to_string(), 
                    important: false 
                }]),
                _ => {
                    // Handle numeric values (1-6)
                    if let Ok(num) = value.parse::<u32>() {
                        if num >= 1 && num <= 6 {
                            return Some(vec![CssProperty { 
                                name: "grid-template-rows".to_string(), 
                                value: format!("repeat({}, minmax(0, 1fr))", num), 
                                important: false 
                            }]);
                        }
                    }
                    None
                }
            }
        } else {
            None
        }
    }

    /// Parse grid column span classes
    fn parse_grid_column_span_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("col-span-") {
            match value {
                "full" => Some(vec![CssProperty { 
                    name: "grid-column".to_string(), 
                    value: "1 / -1".to_string(), 
                    important: false 
                }]),
                "auto" => Some(vec![CssProperty { 
                    name: "grid-column".to_string(), 
                    value: "auto".to_string(), 
                    important: false 
                }]),
                _ => {
                    // Handle numeric values (1-12)
                    if let Ok(num) = value.parse::<u32>() {
                        if num >= 1 && num <= 12 {
                            return Some(vec![CssProperty { 
                                name: "grid-column".to_string(), 
                                value: format!("span {}/span {}", num, num), 
                                important: false 
                            }]);
                        }
                    }
                    None
                }
            }
        } else {
            None
        }
    }

    /// Parse grid row span classes
    fn parse_grid_row_span_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("row-span-") {
            match value {
                "full" => Some(vec![CssProperty { 
                    name: "grid-row".to_string(), 
                    value: "1 / -1".to_string(), 
                    important: false 
                }]),
                "auto" => Some(vec![CssProperty { 
                    name: "grid-row".to_string(), 
                    value: "auto".to_string(), 
                    important: false 
                }]),
                _ => {
                    // Handle numeric values (1-6)
                    if let Ok(num) = value.parse::<u32>() {
                        if num >= 1 && num <= 6 {
                            return Some(vec![CssProperty { 
                                name: "grid-row".to_string(), 
                                value: format!("span {}/span {}", num, num), 
                                important: false 
                            }]);
                        }
                    }
                    None
                }
            }
        } else {
            None
        }
    }

    /// Parse grid auto flow classes
    fn parse_grid_auto_flow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "grid-flow-row" => Some(vec![CssProperty { 
                name: "grid-auto-flow".to_string(), 
                value: "row".to_string(), 
                important: false 
            }]),
            "grid-flow-col" => Some(vec![CssProperty { 
                name: "grid-auto-flow".to_string(), 
                value: "column".to_string(), 
                important: false 
            }]),
            "grid-flow-dense" => Some(vec![CssProperty { 
                name: "grid-auto-flow".to_string(), 
                value: "dense".to_string(), 
                important: false 
            }]),
            "grid-flow-row-dense" => Some(vec![CssProperty { 
                name: "grid-auto-flow".to_string(), 
                value: "row dense".to_string(), 
                important: false 
            }]),
            "grid-flow-col-dense" => Some(vec![CssProperty { 
                name: "grid-auto-flow".to_string(), 
                value: "column dense".to_string(), 
                important: false 
            }]),
            _ => None,
        }
    }

    /// Parse grid gap classes
    fn parse_grid_gap_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("gap-") {
            match value {
                "0" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "0".to_string(), 
                    important: false 
                }]),
                "1" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "0.25rem".to_string(), 
                    important: false 
                }]),
                "2" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "0.5rem".to_string(), 
                    important: false 
                }]),
                "3" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "0.75rem".to_string(), 
                    important: false 
                }]),
                "4" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "1rem".to_string(), 
                    important: false 
                }]),
                "5" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "1.25rem".to_string(), 
                    important: false 
                }]),
                "6" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "1.5rem".to_string(), 
                    important: false 
                }]),
                "8" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "2rem".to_string(), 
                    important: false 
                }]),
                "10" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "2.5rem".to_string(), 
                    important: false 
                }]),
                "12" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "3rem".to_string(), 
                    important: false 
                }]),
                "16" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "4rem".to_string(), 
                    important: false 
                }]),
                "20" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "5rem".to_string(), 
                    important: false 
                }]),
                "24" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "6rem".to_string(), 
                    important: false 
                }]),
                "32" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "8rem".to_string(), 
                    important: false 
                }]),
                "40" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "10rem".to_string(), 
                    important: false 
                }]),
                "48" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "12rem".to_string(), 
                    important: false 
                }]),
                "56" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "14rem".to_string(), 
                    important: false 
                }]),
                "64" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "16rem".to_string(), 
                    important: false 
                }]),
                "72" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "18rem".to_string(), 
                    important: false 
                }]),
                "80" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "20rem".to_string(), 
                    important: false 
                }]),
                "96" => Some(vec![CssProperty { 
                    name: "gap".to_string(), 
                    value: "24rem".to_string(), 
                    important: false 
                }]),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl UtilityParser for GridParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try all grid parsing methods in order of priority
        self.parse_grid_display_class(class)
            .or_else(|| self.parse_grid_template_columns_class(class))
            .or_else(|| self.parse_grid_template_rows_class(class))
            .or_else(|| self.parse_grid_column_span_class(class))
            .or_else(|| self.parse_grid_row_span_class(class))
            .or_else(|| self.parse_grid_auto_flow_class(class))
            .or_else(|| self.parse_grid_gap_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "grid",
            "grid-cols-*", "grid-rows-*",
            "col-span-*", "row-span-*",
            "grid-flow-*", "gap-*"
        ]
    }

    fn get_priority(&self) -> u32 { 50 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Grid }
}

impl Default for GridParser {
    fn default() -> Self { Self::new() }
}
