//! Data Attributes Parser
//!
//! This module provides parsing logic for data attribute classes in Tailwind CSS,
//! such as `data-hover:bg-black/2.5`, `data-hover:bg-black/5`.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct DataAttributeParser;

impl DataAttributeParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse data attribute classes
    fn parse_data_attribute_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Handle data-hover: classes
        if let Some(property_part) = class.strip_prefix("data-hover:") {
            return self.parse_property_with_data_attribute(property_part, "data-hover");
        }

        // Handle data-active: classes
        if let Some(property_part) = class.strip_prefix("data-active:") {
            return self.parse_property_with_data_attribute(property_part, "data-active");
        }

        // Handle data-selected: classes
        if let Some(property_part) = class.strip_prefix("data-selected:") {
            return self.parse_property_with_data_attribute(property_part, "data-selected");
        }

        // Handle data-disabled: classes
        if let Some(property_part) = class.strip_prefix("data-disabled:") {
            return self.parse_property_with_data_attribute(property_part, "data-disabled");
        }

        // Handle data-open: classes
        if let Some(property_part) = class.strip_prefix("data-open:") {
            return self.parse_property_with_data_attribute(property_part, "data-open");
        }

        // Handle data-closed: classes
        if let Some(property_part) = class.strip_prefix("data-closed:") {
            return self.parse_property_with_data_attribute(property_part, "data-closed");
        }

        // Handle data-enter: classes
        if let Some(property_part) = class.strip_prefix("data-enter:") {
            return self.parse_property_with_data_attribute(property_part, "data-enter");
        }

        // Handle data-leave: classes
        if let Some(property_part) = class.strip_prefix("data-leave:") {
            return self.parse_property_with_data_attribute(property_part, "data-leave");
        }

        None
    }

    /// Parse a property with data attribute modifier
    fn parse_property_with_data_attribute(
        &self,
        property_class: &str,
        data_attribute: &str,
    ) -> Option<Vec<CssProperty>> {
        // Handle background colors with opacity
        if let Some(color_part) = property_class.strip_prefix("bg-") {
            if let Some((color_name, opacity)) = color_part.split_once('/') {
                let color_value = self.get_color_value(color_name)?;
                let opacity_value = self.parse_opacity_value(opacity)?;
                let final_color = self.apply_opacity_to_color(&color_value, &opacity_value);
                return Some(vec![CssProperty {
                    name: "background-color".to_string(),
                    value: final_color,
                    important: false,
                }]);
            } else {
                let color_value = self.get_color_value(color_part)?;
                return Some(vec![CssProperty {
                    name: "background-color".to_string(),
                    value: color_value,
                    important: false,
                }]);
            }
        }

        // Handle text colors with opacity
        if let Some(color_part) = property_class.strip_prefix("text-") {
            if let Some((color_name, opacity)) = color_part.split_once('/') {
                let color_value = self.get_color_value(color_name)?;
                let opacity_value = self.parse_opacity_value(opacity)?;
                let final_color = self.apply_opacity_to_color(&color_value, &opacity_value);
                return Some(vec![CssProperty {
                    name: "color".to_string(),
                    value: final_color,
                    important: false,
                }]);
            } else {
                let color_value = self.get_color_value(color_part)?;
                return Some(vec![CssProperty {
                    name: "color".to_string(),
                    value: color_value,
                    important: false,
                }]);
            }
        }

        // Handle border colors with opacity
        if let Some(color_part) = property_class.strip_prefix("border-") {
            if let Some((color_name, opacity)) = color_part.split_once('/') {
                let color_value = self.get_color_value(color_name)?;
                let opacity_value = self.parse_opacity_value(opacity)?;
                let final_color = self.apply_opacity_to_color(&color_value, &opacity_value);
                return Some(vec![CssProperty {
                    name: "border-color".to_string(),
                    value: final_color,
                    important: false,
                }]);
            } else {
                let color_value = self.get_color_value(color_part)?;
                return Some(vec![CssProperty {
                    name: "border-color".to_string(),
                    value: color_value,
                    important: false,
                }]);
            }
        }

        // Handle ring colors with opacity
        if let Some(color_part) = property_class.strip_prefix("ring-") {
            if let Some((color_name, opacity)) = color_part.split_once('/') {
                let color_value = self.get_color_value(color_name)?;
                let opacity_value = self.parse_opacity_value(opacity)?;
                let final_color = self.apply_opacity_to_color(&color_value, &opacity_value);
                return Some(vec![CssProperty {
                    name: "box-shadow".to_string(),
                    value: format!("0 0 0 3px {}", final_color),
                    important: false,
                }]);
            } else {
                let color_value = self.get_color_value(color_part)?;
                return Some(vec![CssProperty {
                    name: "box-shadow".to_string(),
                    value: format!("0 0 0 3px {}", color_value),
                    important: false,
                }]);
            }
        }

        // Handle transition timing functions
        match property_class {
            "ease-linear" => Some(vec![CssProperty {
                name: "transition-timing-function".to_string(),
                value: "linear".to_string(),
                important: false,
            }]),
            "ease-in" => Some(vec![CssProperty {
                name: "transition-timing-function".to_string(),
                value: "cubic-bezier(0.4, 0, 1, 1)".to_string(),
                important: false,
            }]),
            "ease-out" => Some(vec![CssProperty {
                name: "transition-timing-function".to_string(),
                value: "cubic-bezier(0, 0, 0.2, 1)".to_string(),
                important: false,
            }]),
            "ease-in-out" => Some(vec![CssProperty {
                name: "transition-timing-function".to_string(),
                value: "cubic-bezier(0.4, 0, 0.2, 1)".to_string(),
                important: false,
            }]),
            _ => None,
        }
        .or_else(|| {
            // Handle scale properties
            match property_class {
                "scale-0" => Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "scale(0)".to_string(),
                    important: false,
                }]),
                "scale-50" => Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "scale(0.5)".to_string(),
                    important: false,
                }]),
                "scale-75" => Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "scale(0.75)".to_string(),
                    important: false,
                }]),
                "scale-90" => Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "scale(0.9)".to_string(),
                    important: false,
                }]),
                "scale-95" => Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "scale(0.95)".to_string(),
                    important: false,
                }]),
                "scale-100" => Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "scale(1)".to_string(),
                    important: false,
                }]),
                "scale-105" => Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "scale(1.05)".to_string(),
                    important: false,
                }]),
                "scale-110" => Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "scale(1.1)".to_string(),
                    important: false,
                }]),
                "scale-125" => Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "scale(1.25)".to_string(),
                    important: false,
                }]),
                "scale-150" => Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: "scale(1.5)".to_string(),
                    important: false,
                }]),
                _ => None,
            }
        })
        .or_else(|| {
            // Handle opacity properties
            match property_class {
                "opacity-0" => Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: "0".to_string(),
                    important: false,
                }]),
                "opacity-5" => Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: "0.05".to_string(),
                    important: false,
                }]),
                "opacity-10" => Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: "0.1".to_string(),
                    important: false,
                }]),
                "opacity-20" => Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: "0.2".to_string(),
                    important: false,
                }]),
                "opacity-25" => Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: "0.25".to_string(),
                    important: false,
                }]),
                "opacity-30" => Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: "0.3".to_string(),
                    important: false,
                }]),
                "opacity-40" => Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: "0.4".to_string(),
                    important: false,
                }]),
                "opacity-50" => Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: "0.5".to_string(),
                    important: false,
                }]),
                "opacity-60" => Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: "0.6".to_string(),
                    important: false,
                }]),
                "opacity-70" => Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: "0.7".to_string(),
                    important: false,
                }]),
                "opacity-75" => Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: "0.75".to_string(),
                    important: false,
                }]),
                "opacity-80" => Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: "0.8".to_string(),
                    important: false,
                }]),
                "opacity-90" => Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: "0.9".to_string(),
                    important: false,
                }]),
                "opacity-95" => Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: "0.95".to_string(),
                    important: false,
                }]),
                "opacity-100" => Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: "1".to_string(),
                    important: false,
                }]),
                _ => None,
            }
        })
    }

    /// Get color value
    fn get_color_value(&self, color: &str) -> Option<String> {
        match color {
            "black" => Some("#000000".to_string()),
            "white" => Some("#ffffff".to_string()),
            "transparent" => Some("transparent".to_string()),
            "current" => Some("currentColor".to_string()),
            // Add more colors as needed
            _ => None,
        }
    }

    /// Parse opacity value
    fn parse_opacity_value(&self, opacity: &str) -> Option<f32> {
        opacity.parse::<f32>().ok().map(|o| o / 100.0)
    }

    /// Apply opacity to a color
    fn apply_opacity_to_color(&self, color: &str, opacity: &f32) -> String {
        if color.starts_with('#') && color.len() == 7 {
            let r = u8::from_str_radix(&color[1..3], 16).unwrap_or(0);
            let g = u8::from_str_radix(&color[3..5], 16).unwrap_or(0);
            let b = u8::from_str_radix(&color[5..7], 16).unwrap_or(0);
            format!("rgba({}, {}, {}, {})", r, g, b, opacity)
        } else {
            format!("{} / {}", color, opacity)
        }
    }
}

impl UtilityParser for DataAttributeParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(properties) = self.parse_data_attribute_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "data-hover:*",
            "data-active:*",
            "data-selected:*",
            "data-disabled:*",
            "data-open:*",
            "data-closed:*",
            "data-enter:*",
            "data-leave:*",
        ]
    }

    fn get_priority(&self) -> u32 {
        90
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Interactive
    }
}

impl Default for DataAttributeParser {
    fn default() -> Self {
        Self::new()
    }
}
