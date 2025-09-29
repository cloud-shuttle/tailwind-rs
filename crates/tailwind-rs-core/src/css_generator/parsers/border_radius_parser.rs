use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

/// Parser for border radius utilities
#[derive(Debug, Clone)]
pub struct BorderRadiusParser;

impl Default for BorderRadiusParser {
    fn default() -> Self {
        Self::new()
    }
}

impl BorderRadiusParser {
    /// Create a new BorderRadiusParser
    pub fn new() -> Self {
        Self
    }

    /// Parse border-radius classes
    pub fn parse_border_radius_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "rounded-xs" => Some(vec![CssProperty {
                name: "border-radius".to_string(),
                value: "0.25rem".to_string(),
                important: false,
            }]),
            "rounded-sm" => Some(vec![CssProperty {
                name: "border-radius".to_string(),
                value: "0.375rem".to_string(),
                important: false,
            }]),
            "rounded" => Some(vec![CssProperty {
                name: "border-radius".to_string(),
                value: "0.375rem".to_string(),
                important: false,
            }]),
            "rounded-md" => Some(vec![CssProperty {
                name: "border-radius".to_string(),
                value: "0.375rem".to_string(),
                important: false,
            }]),
            "rounded-lg" => Some(vec![CssProperty {
                name: "border-radius".to_string(),
                value: "0.5rem".to_string(),
                important: false,
            }]),
            "rounded-xl" => Some(vec![CssProperty {
                name: "border-radius".to_string(),
                value: "0.75rem".to_string(),
                important: false,
            }]),
            "rounded-2xl" => Some(vec![CssProperty {
                name: "border-radius".to_string(),
                value: "1rem".to_string(),
                important: false,
            }]),
            "rounded-3xl" => Some(vec![CssProperty {
                name: "border-radius".to_string(),
                value: "1.5rem".to_string(),
                important: false,
            }]),
            "rounded-4xl" => Some(vec![CssProperty {
                name: "border-radius".to_string(),
                value: "2rem".to_string(),
                important: false,
            }]),
            "rounded-none" => Some(vec![CssProperty {
                name: "border-radius".to_string(),
                value: "0".to_string(),
                important: false,
            }]),
            "rounded-full" => Some(vec![CssProperty {
                name: "border-radius".to_string(),
                value: "calc(infinity * 1px)".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for border radius
                if let Some(value) = class.strip_prefix("rounded-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "border-radius".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }

                // Arbitrary values for border radius
                if let Some(value) = class.strip_prefix("rounded-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "border-radius".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }

                // Basic side-specific border radius
                if class == "rounded-t" {
                    return Some(vec![
                        CssProperty {
                            name: "border-top-left-radius".to_string(),
                            value: "0.375rem".to_string(),
                            important: false,
                        },
                        CssProperty {
                            name: "border-top-right-radius".to_string(),
                            value: "0.375rem".to_string(),
                            important: false,
                        },
                    ]);
                }

                if class == "rounded-r" {
                    return Some(vec![
                        CssProperty {
                            name: "border-top-right-radius".to_string(),
                            value: "0.375rem".to_string(),
                            important: false,
                        },
                        CssProperty {
                            name: "border-bottom-right-radius".to_string(),
                            value: "0.375rem".to_string(),
                            important: false,
                        },
                    ]);
                }

                if class == "rounded-b" {
                    return Some(vec![
                        CssProperty {
                            name: "border-bottom-left-radius".to_string(),
                            value: "0.375rem".to_string(),
                            important: false,
                        },
                        CssProperty {
                            name: "border-bottom-right-radius".to_string(),
                            value: "0.375rem".to_string(),
                            important: false,
                        },
                    ]);
                }

                if class == "rounded-l" {
                    return Some(vec![
                        CssProperty {
                            name: "border-top-left-radius".to_string(),
                            value: "0.375rem".to_string(),
                            important: false,
                        },
                        CssProperty {
                            name: "border-bottom-left-radius".to_string(),
                            value: "0.375rem".to_string(),
                            important: false,
                        },
                    ]);
                }

                // Corner-specific border radius
                if class == "rounded-tl" {
                    return Some(vec![CssProperty {
                        name: "border-top-left-radius".to_string(),
                        value: "0.375rem".to_string(),
                        important: false,
                    }]);
                }

                if class == "rounded-tr" {
                    return Some(vec![CssProperty {
                        name: "border-top-right-radius".to_string(),
                        value: "0.375rem".to_string(),
                        important: false,
                    }]);
                }

                if class == "rounded-br" {
                    return Some(vec![CssProperty {
                        name: "border-bottom-right-radius".to_string(),
                        value: "0.375rem".to_string(),
                        important: false,
                    }]);
                }

                if class == "rounded-bl" {
                    return Some(vec![CssProperty {
                        name: "border-bottom-left-radius".to_string(),
                        value: "0.375rem".to_string(),
                        important: false,
                    }]);
                }

                // Side-specific border radius with sizes
                if class.starts_with("rounded-t-") {
                    if let Some(size) = class.strip_prefix("rounded-t-") {
                        if let Some(radius_value) = self.get_radius_value(size) {
                            return Some(vec![
                                CssProperty {
                                    name: "border-top-left-radius".to_string(),
                                    value: radius_value.clone(),
                                    important: false,
                                },
                                CssProperty {
                                    name: "border-top-right-radius".to_string(),
                                    value: radius_value,
                                    important: false,
                                },
                            ]);
                        }
                    }
                }

                if class.starts_with("rounded-r-") {
                    if let Some(size) = class.strip_prefix("rounded-r-") {
                        if let Some(radius_value) = self.get_radius_value(size) {
                            return Some(vec![
                                CssProperty {
                                    name: "border-top-right-radius".to_string(),
                                    value: radius_value.clone(),
                                    important: false,
                                },
                                CssProperty {
                                    name: "border-bottom-right-radius".to_string(),
                                    value: radius_value,
                                    important: false,
                                },
                            ]);
                        }
                    }
                }

                if class.starts_with("rounded-b-") {
                    if let Some(size) = class.strip_prefix("rounded-b-") {
                        if let Some(radius_value) = self.get_radius_value(size) {
                            return Some(vec![
                                CssProperty {
                                    name: "border-bottom-left-radius".to_string(),
                                    value: radius_value.clone(),
                                    important: false,
                                },
                                CssProperty {
                                    name: "border-bottom-right-radius".to_string(),
                                    value: radius_value,
                                    important: false,
                                },
                            ]);
                        }
                    }
                }

                if class.starts_with("rounded-l-") {
                    if let Some(size) = class.strip_prefix("rounded-l-") {
                        if let Some(radius_value) = self.get_radius_value(size) {
                            return Some(vec![
                                CssProperty {
                                    name: "border-top-left-radius".to_string(),
                                    value: radius_value.clone(),
                                    important: false,
                                },
                                CssProperty {
                                    name: "border-bottom-left-radius".to_string(),
                                    value: radius_value,
                                    important: false,
                                },
                            ]);
                        }
                    }
                }

                // Corner-specific border radius with sizes
                if class.starts_with("rounded-tl-") {
                    if let Some(size) = class.strip_prefix("rounded-tl-") {
                        if let Some(radius_value) = self.get_radius_value(size) {
                            return Some(vec![CssProperty {
                                name: "border-top-left-radius".to_string(),
                                value: radius_value,
                                important: false,
                            }]);
                        }
                    }
                }

                if class.starts_with("rounded-tr-") {
                    if let Some(size) = class.strip_prefix("rounded-tr-") {
                        if let Some(radius_value) = self.get_radius_value(size) {
                            return Some(vec![CssProperty {
                                name: "border-top-right-radius".to_string(),
                                value: radius_value,
                                important: false,
                            }]);
                        }
                    }
                }

                if class.starts_with("rounded-br-") {
                    if let Some(size) = class.strip_prefix("rounded-br-") {
                        if let Some(radius_value) = self.get_radius_value(size) {
                            return Some(vec![CssProperty {
                                name: "border-bottom-right-radius".to_string(),
                                value: radius_value,
                                important: false,
                            }]);
                        }
                    }
                }

                if class.starts_with("rounded-bl-") {
                    if let Some(size) = class.strip_prefix("rounded-bl-") {
                        if let Some(radius_value) = self.get_radius_value(size) {
                            return Some(vec![CssProperty {
                                name: "border-bottom-left-radius".to_string(),
                                value: radius_value,
                                important: false,
                            }]);
                        }
                    }
                }

                None
            }
        }
    }

    /// Get radius value for a given size
    fn get_radius_value(&self, size: &str) -> Option<String> {
        match size {
            "xs" => Some("0.125rem".to_string()),
            "sm" => Some("0.25rem".to_string()),
            "md" => Some("0.375rem".to_string()),
            "lg" => Some("0.5rem".to_string()),
            "xl" => Some("0.75rem".to_string()),
            "2xl" => Some("1rem".to_string()),
            "3xl" => Some("1.5rem".to_string()),
            "4xl" => Some("3rem".to_string()),
            "none" => Some("0".to_string()),
            "full" => Some("calc(infinity * 1px)".to_string()),
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

impl UtilityParser for BorderRadiusParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_border_radius_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "rounded",
            "rounded-*",
            "rounded-t*",
            "rounded-r*",
            "rounded-b*",
            "rounded-l*",
            "rounded-tl*",
            "rounded-tr*",
            "rounded-br*",
            "rounded-bl*",
            "rounded-[*]",
            "rounded-(*)",
        ]
    }

    fn get_priority(&self) -> u32 {
        50 // Medium priority for border radius
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Borders
    }
}
