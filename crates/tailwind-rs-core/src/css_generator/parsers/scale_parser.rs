//! Scale Parser
//!
//! This module provides parsing logic for scale utilities,
//! including scale-x and scale-y classes that were previously missing.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ScaleParser {
    scale_x_map: HashMap<String, String>,
    scale_y_map: HashMap<String, String>,
}

impl ScaleParser {
    pub fn new() -> Self {
        let mut scale_x_map = HashMap::new();
        let mut scale_y_map = HashMap::new();

        // Initialize scale-x values
        scale_x_map.insert("0".to_string(), "scaleX(0)".to_string());
        scale_x_map.insert("50".to_string(), "scaleX(0.5)".to_string());
        scale_x_map.insert("75".to_string(), "scaleX(0.75)".to_string());
        scale_x_map.insert("90".to_string(), "scaleX(0.9)".to_string());
        scale_x_map.insert("95".to_string(), "scaleX(0.95)".to_string());
        scale_x_map.insert("100".to_string(), "scaleX(1)".to_string());
        scale_x_map.insert("105".to_string(), "scaleX(1.05)".to_string());
        scale_x_map.insert("110".to_string(), "scaleX(1.1)".to_string());
        scale_x_map.insert("125".to_string(), "scaleX(1.25)".to_string());
        scale_x_map.insert("150".to_string(), "scaleX(1.5)".to_string());

        // Initialize scale-y values (same as scale-x)
        scale_y_map.insert("0".to_string(), "scaleY(0)".to_string());
        scale_y_map.insert("50".to_string(), "scaleY(0.5)".to_string());
        scale_y_map.insert("75".to_string(), "scaleY(0.75)".to_string());
        scale_y_map.insert("90".to_string(), "scaleY(0.9)".to_string());
        scale_y_map.insert("95".to_string(), "scaleY(0.95)".to_string());
        scale_y_map.insert("100".to_string(), "scaleY(1)".to_string());
        scale_y_map.insert("105".to_string(), "scaleY(1.05)".to_string());
        scale_y_map.insert("110".to_string(), "scaleY(1.1)".to_string());
        scale_y_map.insert("125".to_string(), "scaleY(1.25)".to_string());
        scale_y_map.insert("150".to_string(), "scaleY(1.5)".to_string());

        Self {
            scale_x_map,
            scale_y_map,
        }
    }

    /// Parse scale-x classes
    fn parse_scale_x_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("scale-x-") {
            if let Some(transform_value) = self.scale_x_map.get(value) {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: transform_value.clone(),
                    important: false,
                }]);
            }
        }
        None
    }

    /// Parse scale-y classes
    fn parse_scale_y_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("scale-y-") {
            if let Some(transform_value) = self.scale_y_map.get(value) {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: transform_value.clone(),
                    important: false,
                }]);
            }
        }
        None
    }
}

impl UtilityParser for ScaleParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_scale_x_class(class)
            .or_else(|| self.parse_scale_y_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "scale-x-0", "scale-x-50", "scale-x-75", "scale-x-90", "scale-x-95",
            "scale-x-100", "scale-x-105", "scale-x-110", "scale-x-125", "scale-x-150",
            "scale-y-0", "scale-y-50", "scale-y-75", "scale-y-90", "scale-y-95",
            "scale-y-100", "scale-y-105", "scale-y-110", "scale-y-125", "scale-y-150",
        ]
    }

    fn get_priority(&self) -> u32 { 80 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Transforms }
}

impl Default for ScaleParser {
    fn default() -> Self { Self::new() }
}
