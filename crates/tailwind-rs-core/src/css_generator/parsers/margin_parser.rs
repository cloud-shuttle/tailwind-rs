//! Margin Parser for Tailwind CSS
//!
//! This module handles parsing of margin-related utilities:
//! - All-direction margin (m-*)
//! - Axis margin (mx-*, my-*)
//! - Directional margin (mt-*, mr-*, mb-*, ml-*)
//! - Logical margin (ms-*, me-*)
//! - Arbitrary margin (mt-[...], mr-[...], etc.)
//! - Custom margin (mt-(...), mr-(...), etc.)

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

/// Parser for margin utilities
#[derive(Debug, Clone)]
pub struct MarginParser;

impl Default for MarginParser {
    fn default() -> Self {
        Self::new()
    }
}

impl MarginParser {
    /// Create a new margin parser
    pub fn new() -> Self {
        Self
    }

    /// Parse margin classes
    pub fn parse_margin_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("m-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "margin".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mx-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "margin-left".to_string(),
                    value: spacing.clone(),
                    important: false,
                }, CssProperty {
                    name: "margin-right".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("my-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "margin-top".to_string(),
                    value: spacing.clone(),
                    important: false,
                }, CssProperty {
                    name: "margin-bottom".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mt-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "margin-top".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mr-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "margin-right".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mb-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "margin-bottom".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("ml-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "margin-left".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        // Logical properties for margin
        if let Some(value) = class.strip_prefix("ms-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "margin-inline-start".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("me-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "margin-inline-end".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        // Arbitrary values for margin
        if let Some(value) = class.strip_prefix("m-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "margin".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mx-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "margin-left".to_string(),
                    value: value.to_string(),
                    important: false,
                }, CssProperty {
                    name: "margin-right".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("my-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "margin-top".to_string(),
                    value: value.to_string(),
                    important: false,
                }, CssProperty {
                    name: "margin-bottom".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mt-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "margin-top".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mr-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "margin-right".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mb-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "margin-bottom".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("ml-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "margin-left".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("ms-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "margin-inline-start".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("me-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "margin-inline-end".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        // Custom properties for margin
        if let Some(value) = class.strip_prefix("m-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "margin".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mx-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "margin-left".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }, CssProperty {
                    name: "margin-right".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("my-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "margin-top".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }, CssProperty {
                    name: "margin-bottom".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mt-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "margin-top".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mr-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "margin-right".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mb-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "margin-bottom".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("ml-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "margin-left".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("ms-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "margin-inline-start".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("me-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "margin-inline-end".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        None
    }

    /// Get spacing value for a given size
    fn get_spacing_value(&self, size: &str) -> Option<String> {
        match size {
            "0" => Some("0".to_string()),
            "px" => Some("1px".to_string()),
            "0.5" => Some("0.125rem".to_string()),
            "1" => Some("0.25rem".to_string()),
            "1.5" => Some("0.375rem".to_string()),
            "2" => Some("0.5rem".to_string()),
            "2.5" => Some("0.625rem".to_string()),
            "3" => Some("0.75rem".to_string()),
            "3.5" => Some("0.875rem".to_string()),
            "4" => Some("1rem".to_string()),
            "5" => Some("1.25rem".to_string()),
            "6" => Some("1.5rem".to_string()),
            "7" => Some("1.75rem".to_string()),
            "8" => Some("2rem".to_string()),
            "9" => Some("2.25rem".to_string()),
            "10" => Some("2.5rem".to_string()),
            "11" => Some("2.75rem".to_string()),
            "12" => Some("3rem".to_string()),
            "14" => Some("3.5rem".to_string()),
            "16" => Some("4rem".to_string()),
            "20" => Some("5rem".to_string()),
            "24" => Some("6rem".to_string()),
            "28" => Some("7rem".to_string()),
            "32" => Some("8rem".to_string()),
            "36" => Some("9rem".to_string()),
            "40" => Some("10rem".to_string()),
            "44" => Some("11rem".to_string()),
            "48" => Some("12rem".to_string()),
            "52" => Some("13rem".to_string()),
            "56" => Some("14rem".to_string()),
            "60" => Some("15rem".to_string()),
            "64" => Some("16rem".to_string()),
            "72" => Some("18rem".to_string()),
            "80" => Some("20rem".to_string()),
            "96" => Some("24rem".to_string()),
            _ => {
                // Check for arbitrary values
                if let Some(value) = size.strip_prefix("[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(value.to_string());
                    }
                }
                // Check for custom properties
                if let Some(value) = size.strip_prefix("(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(format!("var({})", value));
                    }
                }
                None
            }
        }
    }
}

impl UtilityParser for MarginParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_margin_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "m-*",
            "mx-*",
            "my-*",
            "mt-*",
            "mr-*",
            "mb-*",
            "ml-*",
            "ms-*",
            "me-*",
            "m-[*]",
            "mx-[*]",
            "my-[*]",
            "mt-[*]",
            "mr-[*]",
            "mb-[*]",
            "ml-[*]",
            "ms-[*]",
            "me-[*]",
            "m-(*)",
            "mx-(*)",
            "my-(*)",
            "mt-(*)",
            "mr-(*)",
            "mb-(*)",
            "ml-(*)",
            "ms-(*)",
            "me-(*)",
        ]
    }

    fn get_priority(&self) -> u32 {
        50 // Medium priority for margin
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Spacing
    }
}
