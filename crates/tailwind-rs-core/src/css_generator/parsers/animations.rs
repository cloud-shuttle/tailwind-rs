//! Animation Utilities Parser

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct AnimationParser;

impl AnimationParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse animation classes
    pub fn parse_animation_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "animate-none" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            "animate-spin" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "spin 1s linear infinite".to_string(),
                important: false,
            }]),
            "animate-ping" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite".to_string(),
                important: false,
            }]),
            "animate-pulse" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite".to_string(),
                important: false,
            }]),
            "animate-bounce" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "bounce 1s infinite".to_string(),
                important: false,
            }]),
            // Advanced animations
            "animate-float" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "float 3s ease-in-out infinite".to_string(),
                important: false,
            }]),
            "animate-twinkle" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "twinkle 2s ease-in-out infinite".to_string(),
                important: false,
            }]),
            "animate-rainbow" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "rainbow 3s linear infinite".to_string(),
                important: false,
            }]),
            "animate-shimmer" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "shimmer 2s linear infinite".to_string(),
                important: false,
            }]),
            "animate-drift" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "drift 4s ease-in-out infinite".to_string(),
                important: false,
            }]),
            "animate-glow" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "glow 2s ease-in-out infinite alternate".to_string(),
                important: false,
            }]),

            _ => {
                // Handle custom property animations: animate-(<custom-property>)
                if let Some(custom_prop) = class.strip_prefix("animate-(") {
                    if let Some(custom_prop) = custom_prop.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "animation".to_string(),
                            value: format!("var(--{})", custom_prop),
                            important: false,
                        }]);
                    }
                }

                // Handle arbitrary animations: animate-[<value>]
                if let Some(arbitrary_value) = class.strip_prefix("animate-[") {
                    if let Some(arbitrary_value) = arbitrary_value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "animation".to_string(),
                            value: arbitrary_value.to_string(),
                            important: false,
                        }]);
                    }
                }

                None
            },
        }
    }
}

impl UtilityParser for AnimationParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_animation_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec!["animate-"]
    }

    fn get_priority(&self) -> u32 {
        50
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Animations
    }
}

impl Default for AnimationParser {
    fn default() -> Self {
        Self::new()
    }
}
