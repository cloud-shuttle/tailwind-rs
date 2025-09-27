//! Flexbox Utilities Parser
//!
//! This module provides parsing logic for flexbox-related Tailwind CSS utilities,
//! including display, direction, wrapping, alignment, and justification.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct FlexboxParser;

impl FlexboxParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse flex display classes
    fn parse_flex_display_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "flex" => Some(vec![CssProperty {
                name: "display".to_string(),
                value: "flex".to_string(),
                important: false,
            }]),
            "inline-flex" => Some(vec![CssProperty {
                name: "display".to_string(),
                value: "inline-flex".to_string(),
                important: false,
            }]),
            "flex-none" => Some(vec![CssProperty {
                name: "flex".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            "flex-grow" => Some(vec![CssProperty {
                name: "flex-grow".to_string(),
                value: "1".to_string(),
                important: false,
            }]),
            "flex-shrink" => Some(vec![CssProperty {
                name: "flex-shrink".to_string(),
                value: "1".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse flex direction classes
    fn parse_flex_direction_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "flex-row" => Some(vec![CssProperty {
                name: "flex-direction".to_string(),
                value: "row".to_string(),
                important: false,
            }]),
            "flex-row-reverse" => Some(vec![CssProperty {
                name: "flex-direction".to_string(),
                value: "row-reverse".to_string(),
                important: false,
            }]),
            "flex-col" => Some(vec![CssProperty {
                name: "flex-direction".to_string(),
                value: "column".to_string(),
                important: false,
            }]),
            "flex-col-reverse" => Some(vec![CssProperty {
                name: "flex-direction".to_string(),
                value: "column-reverse".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse flex wrap classes
    fn parse_flex_wrap_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "flex-wrap" => Some(vec![CssProperty {
                name: "flex-wrap".to_string(),
                value: "wrap".to_string(),
                important: false,
            }]),
            "flex-wrap-reverse" => Some(vec![CssProperty {
                name: "flex-wrap".to_string(),
                value: "wrap-reverse".to_string(),
                important: false,
            }]),
            "flex-nowrap" => Some(vec![CssProperty {
                name: "flex-wrap".to_string(),
                value: "nowrap".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse justify content classes
    fn parse_justify_content_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "justify-start" => Some(vec![CssProperty {
                name: "justify-content".to_string(),
                value: "flex-start".to_string(),
                important: false,
            }]),
            "justify-end" => Some(vec![CssProperty {
                name: "justify-content".to_string(),
                value: "flex-end".to_string(),
                important: false,
            }]),
            "justify-center" => Some(vec![CssProperty {
                name: "justify-content".to_string(),
                value: "center".to_string(),
                important: false,
            }]),
            "justify-between" => Some(vec![CssProperty {
                name: "justify-content".to_string(),
                value: "space-between".to_string(),
                important: false,
            }]),
            "justify-around" => Some(vec![CssProperty {
                name: "justify-content".to_string(),
                value: "space-around".to_string(),
                important: false,
            }]),
            "justify-evenly" => Some(vec![CssProperty {
                name: "justify-content".to_string(),
                value: "space-evenly".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse align items classes
    fn parse_align_items_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "items-start" => Some(vec![CssProperty {
                name: "align-items".to_string(),
                value: "flex-start".to_string(),
                important: false,
            }]),
            "items-end" => Some(vec![CssProperty {
                name: "align-items".to_string(),
                value: "flex-end".to_string(),
                important: false,
            }]),
            "items-center" => Some(vec![CssProperty {
                name: "align-items".to_string(),
                value: "center".to_string(),
                important: false,
            }]),
            "items-baseline" => Some(vec![CssProperty {
                name: "align-items".to_string(),
                value: "baseline".to_string(),
                important: false,
            }]),
            "items-stretch" => Some(vec![CssProperty {
                name: "align-items".to_string(),
                value: "stretch".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse align self classes
    fn parse_align_self_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "self-auto" => Some(vec![CssProperty {
                name: "align-self".to_string(),
                value: "auto".to_string(),
                important: false,
            }]),
            "self-start" => Some(vec![CssProperty {
                name: "align-self".to_string(),
                value: "flex-start".to_string(),
                important: false,
            }]),
            "self-end" => Some(vec![CssProperty {
                name: "align-self".to_string(),
                value: "flex-end".to_string(),
                important: false,
            }]),
            "self-center" => Some(vec![CssProperty {
                name: "align-self".to_string(),
                value: "center".to_string(),
                important: false,
            }]),
            "self-stretch" => Some(vec![CssProperty {
                name: "align-self".to_string(),
                value: "stretch".to_string(),
                important: false,
            }]),
            "self-baseline" => Some(vec![CssProperty {
                name: "align-self".to_string(),
                value: "baseline".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse flex grow/shrink classes
    fn parse_flex_grow_shrink_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "flex-1" => Some(vec![CssProperty {
                name: "flex".to_string(),
                value: "1 1 0%".to_string(),
                important: false,
            }]),
            "flex-auto" => Some(vec![CssProperty {
                name: "flex".to_string(),
                value: "1 1 auto".to_string(),
                important: false,
            }]),
            "flex-initial" => Some(vec![CssProperty {
                name: "flex".to_string(),
                value: "0 1 auto".to_string(),
                important: false,
            }]),
            "flex-none" => Some(vec![CssProperty {
                name: "flex".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }
}

impl UtilityParser for FlexboxParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity
        if let Some(properties) = self.parse_flex_display_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_flex_direction_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_flex_wrap_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_justify_content_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_align_items_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_align_self_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_flex_grow_shrink_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "flex",
            "inline-flex",
            "flex-none",
            "flex-row",
            "flex-row-reverse",
            "flex-col",
            "flex-col-reverse",
            "flex-wrap",
            "flex-wrap-reverse",
            "flex-nowrap",
            "justify-*",
            "items-*",
            "self-*",
            "flex-1",
            "flex-auto",
            "flex-initial",
            "flex-none",
        ]
    }

    fn get_priority(&self) -> u32 {
        60
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Flexbox
    }
}

impl Default for FlexboxParser {
    fn default() -> Self {
        Self::new()
    }
}
