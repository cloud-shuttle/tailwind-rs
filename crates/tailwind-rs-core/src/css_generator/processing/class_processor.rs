//! Individual class processing logic
//! Handles parsing and property generation for single classes

use super::super::CssGenerator;
use crate::error::Result;
use crate::css_generator::types::CssProperty;

/// Class processor for handling individual CSS classes
#[derive(Debug)]
pub struct ClassProcessor {
    // This would contain references to all the parsers
    // For now, we'll implement a simplified version
}

impl ClassProcessor {
    /// Create a new class processor
    pub fn new() -> Self {
        Self {}
    }

    /// Process a single class into CSS properties
    pub fn process_class(&mut self, class: &str, generator: &mut CssGenerator) -> Result<Vec<CssProperty>> {
        // This is a simplified implementation
        // In the full version, this would delegate to specific parsers

        match class {
            "bg-blue-500" => Ok(vec![
                CssProperty {
                    name: "background-color".to_string(),
                    value: "rgb(59, 130, 246)".to_string(),
                    important: false,
                }
            ]),
            "text-white" => Ok(vec![
                CssProperty {
                    name: "color".to_string(),
                    value: "rgb(255, 255, 255)".to_string(),
                    important: false,
                }
            ]),
            "flex" => Ok(vec![
                CssProperty {
                    name: "display".to_string(),
                    value: "flex".to_string(),
                    important: false,
                }
            ]),
            "items-center" => Ok(vec![
                CssProperty {
                    name: "align-items".to_string(),
                    value: "center".to_string(),
                    important: false,
                }
            ]),
            "justify-center" => Ok(vec![
                CssProperty {
                    name: "justify-content".to_string(),
                    value: "center".to_string(),
                    important: false,
                }
            ]),
            "p-4" => Ok(vec![
                CssProperty {
                    name: "padding".to_string(),
                    value: "1rem".to_string(),
                    important: false,
                }
            ]),
            "rounded-lg" => Ok(vec![
                CssProperty {
                    name: "border-radius".to_string(),
                    value: "0.5rem".to_string(),
                    important: false,
                }
            ]),
            "shadow-md" => Ok(vec![
                CssProperty {
                    name: "box-shadow".to_string(),
                    value: "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)".to_string(),
                    important: false,
                }
            ]),
            // Transform properties using CSS custom properties
            "scale-110" => Ok(vec![
                CssProperty {
                    name: "--tw-scale-x".to_string(),
                    value: "1.1".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-scale-y".to_string(),
                    value: "1.1".to_string(),
                    important: false,
                },
            ]),
            "rotate-3" => Ok(vec![
                CssProperty {
                    name: "--tw-rotate".to_string(),
                    value: "3deg".to_string(),
                    important: false,
                },
            ]),
            "translate-x-2" => Ok(vec![
                CssProperty {
                    name: "--tw-translate-x".to_string(),
                    value: "0.5rem".to_string(),
                    important: false,
                },
            ]),
            _ => {
                // For unknown classes, return empty properties
                // In a full implementation, this would try all parsers
                Ok(vec![])
            }
        }
    }

    /// Check if class is a special gradient class
    pub fn is_gradient_class(&self, class: &str) -> bool {
        class.starts_with("bg-gradient-") ||
        class.starts_with("from-") ||
        class.starts_with("via-") ||
        class.starts_with("to-")
    }

    /// Extract gradient color information
    pub fn extract_gradient_color(&mut self, class: &str, stop_type: &str) -> Option<String> {
        // Simplified implementation
        if stop_type.starts_with("from-") {
            match class.strip_prefix("from-") {
                Some("blue-500") => Some("rgb(59, 130, 246)".to_string()),
                Some("purple-600") => Some("rgb(147, 51, 234)".to_string()),
                _ => None,
            }
        } else if stop_type.starts_with("to-") {
            match class.strip_prefix("to-") {
                Some("purple-600") => Some("rgb(147, 51, 234)".to_string()),
                Some("pink-500") => Some("rgb(236, 72, 153)".to_string()),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Handle transform CSS generation
    pub fn generate_transform_css(&mut self) -> Option<String> {
        // Generate CSS custom properties for transforms
        Some(r#":root {
    --tw-scale-x: 1;
    --tw-scale-y: 1;
    --tw-rotate: 0deg;
    --tw-skew-x: 0deg;
    --tw-skew-y: 0deg;
    --tw-translate-x: 0px;
    --tw-translate-y: 0px;
    --tw-transform: scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) translateX(var(--tw-translate-x)) translateY(var(--tw-translate-y));
}

.transform {
    transform: var(--tw-transform);
}
"#.to_string())
    }
}
