//! Background Properties Parser
//!
//! This module provides parsing logic for background property classes in Tailwind CSS,
//! such as `bg-size-[530px_430px]`, `bg-position-[center_-75px]`, `bg-no-repeat`.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct BackgroundPropertiesParser;

impl BackgroundPropertiesParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse background size classes (bg-size-[530px_430px])
    fn parse_bg_size_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("bg-size-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "background-size".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        None
    }

    /// Parse background position classes (bg-position-[center_-75px])
    fn parse_bg_position_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("bg-position-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "background-position".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        None
    }

    /// Parse background repeat classes (bg-no-repeat, bg-repeat, bg-repeat-x, bg-repeat-y)
    fn parse_bg_repeat_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-no-repeat" => Some(vec![CssProperty {
                name: "background-repeat".to_string(),
                value: "no-repeat".to_string(),
                important: false,
            }]),
            "bg-repeat" => Some(vec![CssProperty {
                name: "background-repeat".to_string(),
                value: "repeat".to_string(),
                important: false,
            }]),
            "bg-repeat-x" => Some(vec![CssProperty {
                name: "background-repeat".to_string(),
                value: "repeat-x".to_string(),
                important: false,
            }]),
            "bg-repeat-y" => Some(vec![CssProperty {
                name: "background-repeat".to_string(),
                value: "repeat-y".to_string(),
                important: false,
            }]),
            "bg-repeat-round" => Some(vec![CssProperty {
                name: "background-repeat".to_string(),
                value: "round".to_string(),
                important: false,
            }]),
            "bg-repeat-space" => Some(vec![CssProperty {
                name: "background-repeat".to_string(),
                value: "space".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse background attachment classes (bg-fixed, bg-local, bg-scroll)
    fn parse_bg_attachment_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-fixed" => Some(vec![CssProperty {
                name: "background-attachment".to_string(),
                value: "fixed".to_string(),
                important: false,
            }]),
            "bg-local" => Some(vec![CssProperty {
                name: "background-attachment".to_string(),
                value: "local".to_string(),
                important: false,
            }]),
            "bg-scroll" => Some(vec![CssProperty {
                name: "background-attachment".to_string(),
                value: "scroll".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse background clip classes (bg-clip-border, bg-clip-padding, bg-clip-content, bg-clip-text)
    fn parse_bg_clip_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-clip-border" => Some(vec![CssProperty {
                name: "background-clip".to_string(),
                value: "border-box".to_string(),
                important: false,
            }]),
            "bg-clip-padding" => Some(vec![CssProperty {
                name: "background-clip".to_string(),
                value: "padding-box".to_string(),
                important: false,
            }]),
            "bg-clip-content" => Some(vec![CssProperty {
                name: "background-clip".to_string(),
                value: "content-box".to_string(),
                important: false,
            }]),
            "bg-clip-text" => Some(vec![CssProperty {
                name: "background-clip".to_string(),
                value: "text".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }
}

impl UtilityParser for BackgroundPropertiesParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(properties) = self.parse_bg_size_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_bg_position_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_bg_repeat_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_bg_attachment_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_bg_clip_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "bg-size-[*]",
            "bg-position-[*]",
            "bg-no-repeat",
            "bg-repeat",
            "bg-repeat-x",
            "bg-repeat-y",
            "bg-repeat-round",
            "bg-repeat-space",
            "bg-fixed",
            "bg-local",
            "bg-scroll",
            "bg-clip-border",
            "bg-clip-padding",
            "bg-clip-content",
            "bg-clip-text",
        ]
    }

    fn get_priority(&self) -> u32 {
        85
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Background
    }
}

impl Default for BackgroundPropertiesParser {
    fn default() -> Self {
        Self::new()
    }
}
