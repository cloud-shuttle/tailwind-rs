//! Layout Utilities Parser
//!
//! This module provides parsing logic for layout utilities,
//! such as `float`, `clear`, `isolation`, and `object-position`.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct LayoutUtilitiesParser;

impl LayoutUtilitiesParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse float classes
    fn parse_float_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "float-right" => Some(vec![CssProperty {
                name: "float".to_string(),
                value: "right".to_string(),
                important: false,
            }]),
            "float-left" => Some(vec![CssProperty {
                name: "float".to_string(),
                value: "left".to_string(),
                important: false,
            }]),
            "float-start" => Some(vec![CssProperty {
                name: "float".to_string(),
                value: "inline-start".to_string(),
                important: false,
            }]),
            "float-end" => Some(vec![CssProperty {
                name: "float".to_string(),
                value: "inline-end".to_string(),
                important: false,
            }]),
            "float-none" => Some(vec![CssProperty {
                name: "float".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse clear classes
    fn parse_clear_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "clear-left" => Some(vec![CssProperty {
                name: "clear".to_string(),
                value: "left".to_string(),
                important: false,
            }]),
            "clear-right" => Some(vec![CssProperty {
                name: "clear".to_string(),
                value: "right".to_string(),
                important: false,
            }]),
            "clear-both" => Some(vec![CssProperty {
                name: "clear".to_string(),
                value: "both".to_string(),
                important: false,
            }]),
            "clear-start" => Some(vec![CssProperty {
                name: "clear".to_string(),
                value: "inline-start".to_string(),
                important: false,
            }]),
            "clear-end" => Some(vec![CssProperty {
                name: "clear".to_string(),
                value: "inline-end".to_string(),
                important: false,
            }]),
            "clear-none" => Some(vec![CssProperty {
                name: "clear".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse isolation classes
    fn parse_isolation_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "isolate" => Some(vec![CssProperty {
                name: "isolation".to_string(),
                value: "isolate".to_string(),
                important: false,
            }]),
            "isolation-auto" => Some(vec![CssProperty {
                name: "isolation".to_string(),
                value: "auto".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse object-position classes
    fn parse_object_position_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "object-top-left" => Some(vec![CssProperty {
                name: "object-position".to_string(),
                value: "top left".to_string(),
                important: false,
            }]),
            "object-top" => Some(vec![CssProperty {
                name: "object-position".to_string(),
                value: "top".to_string(),
                important: false,
            }]),
            "object-top-right" => Some(vec![CssProperty {
                name: "object-position".to_string(),
                value: "top right".to_string(),
                important: false,
            }]),
            "object-left" => Some(vec![CssProperty {
                name: "object-position".to_string(),
                value: "left".to_string(),
                important: false,
            }]),
            "object-center" => Some(vec![CssProperty {
                name: "object-position".to_string(),
                value: "center".to_string(),
                important: false,
            }]),
            "object-right" => Some(vec![CssProperty {
                name: "object-position".to_string(),
                value: "right".to_string(),
                important: false,
            }]),
            "object-bottom-left" => Some(vec![CssProperty {
                name: "object-position".to_string(),
                value: "bottom left".to_string(),
                important: false,
            }]),
            "object-bottom" => Some(vec![CssProperty {
                name: "object-position".to_string(),
                value: "bottom".to_string(),
                important: false,
            }]),
            "object-bottom-right" => Some(vec![CssProperty {
                name: "object-position".to_string(),
                value: "bottom right".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse arbitrary object-position classes
    fn parse_arbitrary_object_position_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("object-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "object-position".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        None
    }

    /// Parse custom property object-position classes
    fn parse_custom_property_object_position_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(prop) = class.strip_prefix("object-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "object-position".to_string(),
                    value: format!("var({})", prop),
                    important: false,
                }]);
            }
        }
        None
    }
}

impl UtilityParser for LayoutUtilitiesParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(properties) = self.parse_float_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_clear_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_isolation_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_object_position_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_arbitrary_object_position_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_custom_property_object_position_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "float-right",
            "float-left",
            "float-start",
            "float-end",
            "float-none",
            "clear-left",
            "clear-right",
            "clear-both",
            "clear-start",
            "clear-end",
            "clear-none",
            "isolate",
            "isolation-auto",
            "object-top-left",
            "object-top",
            "object-top-right",
            "object-left",
            "object-center",
            "object-right",
            "object-bottom-left",
            "object-bottom",
            "object-bottom-right",
            "object-[*]",
            "object-(*)",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Layout
    }
}

impl Default for LayoutUtilitiesParser {
    fn default() -> Self {
        Self::new()
    }
}
