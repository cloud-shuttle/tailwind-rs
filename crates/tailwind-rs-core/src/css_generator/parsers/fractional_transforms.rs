//! Fractional Transforms Parser
//!
//! This module provides parsing logic for fractional transform classes in Tailwind CSS,
//! such as `-translate-x-1/2`, `translate-x-1/2`, `translate-y-1/2`.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct FractionalTransformsParser;

impl FractionalTransformsParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse fractional translate classes
    fn parse_fractional_translate_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Handle -translate-x-1/2, -translate-y-1/2
        if let Some(value) = class.strip_prefix("-translate-x-") {
            if value == "1/2" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateX(-50%)".to_string(),
                    important: false,
                }]);
            }
            if value == "1/3" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateX(-33.333333%)".to_string(),
                    important: false,
                }]);
            }
            if value == "2/3" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateX(-66.666667%)".to_string(),
                    important: false,
                }]);
            }
            if value == "1/4" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateX(-25%)".to_string(),
                    important: false,
                }]);
            }
            if value == "3/4" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateX(-75%)".to_string(),
                    important: false,
                }]);
            }
        }

        if let Some(value) = class.strip_prefix("-translate-y-") {
            if value == "1/2" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateY(-50%)".to_string(),
                    important: false,
                }]);
            }
            if value == "1/3" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateY(-33.333333%)".to_string(),
                    important: false,
                }]);
            }
            if value == "2/3" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateY(-66.666667%)".to_string(),
                    important: false,
                }]);
            }
            if value == "1/4" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateY(-25%)".to_string(),
                    important: false,
                }]);
            }
            if value == "3/4" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateY(-75%)".to_string(),
                    important: false,
                }]);
            }
        }

        // Handle translate-x-1/2, translate-y-1/2
        if let Some(value) = class.strip_prefix("translate-x-") {
            if value == "1/2" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateX(50%)".to_string(),
                    important: false,
                }]);
            }
            if value == "1/3" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateX(33.333333%)".to_string(),
                    important: false,
                }]);
            }
            if value == "2/3" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateX(66.666667%)".to_string(),
                    important: false,
                }]);
            }
            if value == "1/4" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateX(25%)".to_string(),
                    important: false,
                }]);
            }
            if value == "3/4" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateX(75%)".to_string(),
                    important: false,
                }]);
            }
        }

        if let Some(value) = class.strip_prefix("translate-y-") {
            if value == "1/2" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateY(50%)".to_string(),
                    important: false,
                }]);
            }
            if value == "1/3" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateY(33.333333%)".to_string(),
                    important: false,
                }]);
            }
            if value == "2/3" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateY(66.666667%)".to_string(),
                    important: false,
                }]);
            }
            if value == "1/4" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateY(25%)".to_string(),
                    important: false,
                }]);
            }
            if value == "3/4" {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "translateY(75%)".to_string(),
                    important: false,
                }]);
            }
        }

        None
    }
}

impl UtilityParser for FractionalTransformsParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(properties) = self.parse_fractional_translate_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "-translate-x-1/2",
            "-translate-x-1/3",
            "-translate-x-2/3",
            "-translate-x-1/4",
            "-translate-x-3/4",
            "-translate-y-1/2",
            "-translate-y-1/3",
            "-translate-y-2/3",
            "-translate-y-1/4",
            "-translate-y-3/4",
            "translate-x-1/2",
            "translate-x-1/3",
            "translate-x-2/3",
            "translate-x-1/4",
            "translate-x-3/4",
            "translate-y-1/2",
            "translate-y-1/3",
            "translate-y-2/3",
            "translate-y-1/4",
            "translate-y-3/4",
        ]
    }

    fn get_priority(&self) -> u32 {
        90
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Transforms
    }
}

impl Default for FractionalTransformsParser {
    fn default() -> Self {
        Self::new()
    }
}
