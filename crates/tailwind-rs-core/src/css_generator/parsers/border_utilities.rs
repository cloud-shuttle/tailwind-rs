use crate::css_generator::types::CssProperty;
use super::{UtilityParser, ParserCategory};

/// Parser for border utilities
#[derive(Debug, Clone)]
pub struct BorderUtilitiesParser;

impl BorderUtilitiesParser {
    /// Create a new BorderUtilitiesParser
    pub fn new() -> Self {
        Self
    }

    /// Parse border-radius classes
    fn parse_border_radius_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "rounded-xs" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "var(--radius-xs)".to_string(), important: false }]),
            "rounded-sm" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "var(--radius-sm)".to_string(), important: false }]),
            "rounded" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "var(--radius-md)".to_string(), important: false }]),
            "rounded-md" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "var(--radius-md)".to_string(), important: false }]),
            "rounded-lg" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "var(--radius-lg)".to_string(), important: false }]),
            "rounded-xl" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "var(--radius-xl)".to_string(), important: false }]),
            "rounded-2xl" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "var(--radius-2xl)".to_string(), important: false }]),
            "rounded-3xl" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "var(--radius-3xl)".to_string(), important: false }]),
            "rounded-4xl" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "var(--radius-4xl)".to_string(), important: false }]),
            "rounded-none" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "0".to_string(), important: false }]),
            "rounded-full" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "calc(infinity * 1px)".to_string(), important: false }]),
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
                
                // Side-specific border radius
                if class.starts_with("rounded-t-") {
                    if let Some(size) = class.strip_prefix("rounded-t-") {
                        if let Some(radius_value) = self.get_radius_value(size) {
                            return Some(vec![CssProperty {
                                name: "border-top-left-radius".to_string(),
                                value: radius_value.clone(),
                                important: false,
                            }, CssProperty {
                                name: "border-top-right-radius".to_string(),
                                value: radius_value,
                                important: false,
                            }]);
                        }
                    }
                }
                
                if class.starts_with("rounded-r-") {
                    if let Some(size) = class.strip_prefix("rounded-r-") {
                        if let Some(radius_value) = self.get_radius_value(size) {
                            return Some(vec![CssProperty {
                                name: "border-top-right-radius".to_string(),
                                value: radius_value.clone(),
                                important: false,
                            }, CssProperty {
                                name: "border-bottom-right-radius".to_string(),
                                value: radius_value,
                                important: false,
                            }]);
                        }
                    }
                }
                
                if class.starts_with("rounded-b-") {
                    if let Some(size) = class.strip_prefix("rounded-b-") {
                        if let Some(radius_value) = self.get_radius_value(size) {
                            return Some(vec![CssProperty {
                                name: "border-bottom-left-radius".to_string(),
                                value: radius_value.clone(),
                                important: false,
                            }, CssProperty {
                                name: "border-bottom-right-radius".to_string(),
                                value: radius_value,
                                important: false,
                            }]);
                        }
                    }
                }
                
                if class.starts_with("rounded-l-") {
                    if let Some(size) = class.strip_prefix("rounded-l-") {
                        if let Some(radius_value) = self.get_radius_value(size) {
                            return Some(vec![CssProperty {
                                name: "border-top-left-radius".to_string(),
                                value: radius_value.clone(),
                                important: false,
                            }, CssProperty {
                                name: "border-bottom-left-radius".to_string(),
                                value: radius_value,
                                important: false,
                            }]);
                        }
                    }
                }
                
                // Corner-specific border radius
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

    /// Parse border-width classes
    fn parse_border_width_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "border" => Some(vec![CssProperty { name: "border-width".to_string(), value: "1px".to_string(), important: false }]),
            "border-0" => Some(vec![CssProperty { name: "border-width".to_string(), value: "0px".to_string(), important: false }]),
            _ => {
                // Numeric border widths
                if class.starts_with("border-") && class.len() > 7 {
                    let number = &class[7..];
                    if let Ok(_) = number.parse::<u32>() {
                        return Some(vec![CssProperty {
                            name: "border-width".to_string(),
                            value: format!("{}px", number),
                            important: false,
                        }]);
                    }
                }
                
                // Custom properties for border width
                if let Some(value) = class.strip_prefix("border-(length:") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "border-width".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for border width
                if let Some(value) = class.strip_prefix("border-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "border-width".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                // Side-specific border widths
                if class.starts_with("border-t-") {
                    if let Some(number) = class.strip_prefix("border-t-") {
                        if let Ok(_) = number.parse::<u32>() {
                            return Some(vec![CssProperty {
                                name: "border-top-width".to_string(),
                                value: format!("{}px", number),
                                important: false,
                            }]);
                        }
                    }
                }
                
                if class.starts_with("border-r-") {
                    if let Some(number) = class.strip_prefix("border-r-") {
                        if let Ok(_) = number.parse::<u32>() {
                            return Some(vec![CssProperty {
                                name: "border-right-width".to_string(),
                                value: format!("{}px", number),
                                important: false,
                            }]);
                        }
                    }
                }
                
                if class.starts_with("border-b-") {
                    if let Some(number) = class.strip_prefix("border-b-") {
                        if let Ok(_) = number.parse::<u32>() {
                            return Some(vec![CssProperty {
                                name: "border-bottom-width".to_string(),
                                value: format!("{}px", number),
                                important: false,
                            }]);
                        }
                    }
                }
                
                if class.starts_with("border-l-") {
                    if let Some(number) = class.strip_prefix("border-l-") {
                        if let Ok(_) = number.parse::<u32>() {
                            return Some(vec![CssProperty {
                                name: "border-left-width".to_string(),
                                value: format!("{}px", number),
                                important: false,
                            }]);
                        }
                    }
                }
                
                // Horizontal and vertical borders
                if class.starts_with("border-x-") {
                    if let Some(number) = class.strip_prefix("border-x-") {
                        if let Ok(_) = number.parse::<u32>() {
                            return Some(vec![CssProperty {
                                name: "border-inline-width".to_string(),
                                value: format!("{}px", number),
                                important: false,
                            }]);
                        }
                    }
                }
                
                if class.starts_with("border-y-") {
                    if let Some(number) = class.strip_prefix("border-y-") {
                        if let Ok(_) = number.parse::<u32>() {
                            return Some(vec![CssProperty {
                                name: "border-block-width".to_string(),
                                value: format!("{}px", number),
                                important: false,
                            }]);
                        }
                    }
                }
                
                None
            }
        }
    }

    /// Parse border-color classes
    fn parse_border_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "border-inherit" => Some(vec![CssProperty { name: "border-color".to_string(), value: "inherit".to_string(), important: false }]),
            "border-current" => Some(vec![CssProperty { name: "border-color".to_string(), value: "currentColor".to_string(), important: false }]),
            "border-transparent" => Some(vec![CssProperty { name: "border-color".to_string(), value: "transparent".to_string(), important: false }]),
            "border-black" => Some(vec![CssProperty { name: "border-color".to_string(), value: "var(--color-black)".to_string(), important: false }]),
            "border-white" => Some(vec![CssProperty { name: "border-color".to_string(), value: "var(--color-white)".to_string(), important: false }]),
            _ => {
                // Custom properties for border color
                if let Some(value) = class.strip_prefix("border-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "border-color".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for border color
                if let Some(value) = class.strip_prefix("border-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "border-color".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                // Color with opacity modifier (e.g., border-blue-600/50)
                if class.contains("/") {
                    let parts: Vec<&str> = class.split("/").collect();
                    if parts.len() == 2 {
                        let base_color = parts[0];
                        let opacity = parts[1];
                        if let Some(color_value) = self.get_color_value(base_color) {
                            return Some(vec![CssProperty {
                                name: "border-color".to_string(),
                                value: format!("{}/{}", color_value, opacity),
                                important: false,
                            }]);
                        }
                    }
                }
                
                // Standard color classes (border-red-500, border-blue-600, etc.)
                if let Some(color_value) = self.get_color_value(class) {
                    return Some(vec![CssProperty {
                        name: "border-color".to_string(),
                        value: color_value,
                        important: false,
                    }]);
                }
                
                // Side-specific border colors
                if class.starts_with("border-t-") {
                    if let Some(color_class) = class.strip_prefix("border-t-") {
                        if let Some(color_value) = self.get_color_value(&format!("border-{}", color_class)) {
                            return Some(vec![CssProperty {
                                name: "border-top-color".to_string(),
                                value: color_value,
                                important: false,
                            }]);
                        }
                    }
                }
                
                if class.starts_with("border-r-") {
                    if let Some(color_class) = class.strip_prefix("border-r-") {
                        if let Some(color_value) = self.get_color_value(&format!("border-{}", color_class)) {
                            return Some(vec![CssProperty {
                                name: "border-right-color".to_string(),
                                value: color_value,
                                important: false,
                            }]);
                        }
                    }
                }
                
                if class.starts_with("border-b-") {
                    if let Some(color_class) = class.strip_prefix("border-b-") {
                        if let Some(color_value) = self.get_color_value(&format!("border-{}", color_class)) {
                            return Some(vec![CssProperty {
                                name: "border-bottom-color".to_string(),
                                value: color_value,
                                important: false,
                            }]);
                        }
                    }
                }
                
                if class.starts_with("border-l-") {
                    if let Some(color_class) = class.strip_prefix("border-l-") {
                        if let Some(color_value) = self.get_color_value(&format!("border-{}", color_class)) {
                            return Some(vec![CssProperty {
                                name: "border-left-color".to_string(),
                                value: color_value,
                                important: false,
                            }]);
                        }
                    }
                }
                
                // Horizontal and vertical border colors
                if class.starts_with("border-x-") {
                    if let Some(color_class) = class.strip_prefix("border-x-") {
                        if let Some(color_value) = self.get_color_value(&format!("border-{}", color_class)) {
                            return Some(vec![CssProperty {
                                name: "border-inline-color".to_string(),
                                value: color_value,
                                important: false,
                            }]);
                        }
                    }
                }
                
                if class.starts_with("border-y-") {
                    if let Some(color_class) = class.strip_prefix("border-y-") {
                        if let Some(color_value) = self.get_color_value(&format!("border-{}", color_class)) {
                            return Some(vec![CssProperty {
                                name: "border-block-color".to_string(),
                                value: color_value,
                                important: false,
                            }]);
                        }
                    }
                }
                
                None
            }
        }
    }

    /// Parse border-style classes
    fn parse_border_style_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "border-solid" => Some(vec![CssProperty { name: "border-style".to_string(), value: "solid".to_string(), important: false }]),
            "border-dashed" => Some(vec![CssProperty { name: "border-style".to_string(), value: "dashed".to_string(), important: false }]),
            "border-dotted" => Some(vec![CssProperty { name: "border-style".to_string(), value: "dotted".to_string(), important: false }]),
            "border-double" => Some(vec![CssProperty { name: "border-style".to_string(), value: "double".to_string(), important: false }]),
            "border-hidden" => Some(vec![CssProperty { name: "border-style".to_string(), value: "hidden".to_string(), important: false }]),
            "border-none" => Some(vec![CssProperty { name: "border-style".to_string(), value: "none".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse outline-width classes
    fn parse_outline_width_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "outline" => Some(vec![CssProperty { name: "outline-width".to_string(), value: "1px".to_string(), important: false }]),
            "outline-0" => Some(vec![CssProperty { name: "outline-width".to_string(), value: "0px".to_string(), important: false }]),
            _ => {
                // Numeric outline widths
                if class.starts_with("outline-") && class.len() > 8 {
                    let number = &class[8..];
                    if let Ok(_) = number.parse::<u32>() {
                        return Some(vec![CssProperty {
                            name: "outline-width".to_string(),
                            value: format!("{}px", number),
                            important: false,
                        }]);
                    }
                }
                
                // Custom properties for outline width
                if let Some(value) = class.strip_prefix("outline-(length:") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "outline-width".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for outline width
                if let Some(value) = class.strip_prefix("outline-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "outline-width".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse outline-color classes
    fn parse_outline_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "outline-inherit" => Some(vec![CssProperty { name: "outline-color".to_string(), value: "inherit".to_string(), important: false }]),
            "outline-current" => Some(vec![CssProperty { name: "outline-color".to_string(), value: "currentColor".to_string(), important: false }]),
            "outline-transparent" => Some(vec![CssProperty { name: "outline-color".to_string(), value: "transparent".to_string(), important: false }]),
            "outline-black" => Some(vec![CssProperty { name: "outline-color".to_string(), value: "var(--color-black)".to_string(), important: false }]),
            "outline-white" => Some(vec![CssProperty { name: "outline-color".to_string(), value: "var(--color-white)".to_string(), important: false }]),
            _ => {
                // Custom properties for outline color
                if let Some(value) = class.strip_prefix("outline-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "outline-color".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for outline color
                if let Some(value) = class.strip_prefix("outline-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "outline-color".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                // Color with opacity modifier (e.g., outline-blue-600/50)
                if class.contains("/") {
                    let parts: Vec<&str> = class.split("/").collect();
                    if parts.len() == 2 {
                        let base_color = parts[0];
                        let opacity = parts[1];
                        if let Some(color_value) = self.get_color_value(base_color) {
                            return Some(vec![CssProperty {
                                name: "outline-color".to_string(),
                                value: format!("{}/{}", color_value, opacity),
                                important: false,
                            }]);
                        }
                    }
                }
                
                // Standard color classes (outline-red-500, outline-blue-600, etc.)
                if let Some(color_value) = self.get_color_value(class) {
                    return Some(vec![CssProperty {
                        name: "outline-color".to_string(),
                        value: color_value,
                        important: false,
                    }]);
                }
                
                None
            }
        }
    }

    /// Parse outline-style classes
    fn parse_outline_style_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "outline-solid" => Some(vec![CssProperty { name: "outline-style".to_string(), value: "solid".to_string(), important: false }]),
            "outline-dashed" => Some(vec![CssProperty { name: "outline-style".to_string(), value: "dashed".to_string(), important: false }]),
            "outline-dotted" => Some(vec![CssProperty { name: "outline-style".to_string(), value: "dotted".to_string(), important: false }]),
            "outline-double" => Some(vec![CssProperty { name: "outline-style".to_string(), value: "double".to_string(), important: false }]),
            "outline-none" => Some(vec![CssProperty { name: "outline-style".to_string(), value: "none".to_string(), important: false }]),
            "outline-hidden" => Some(vec![
                CssProperty { name: "outline".to_string(), value: "2px solid transparent".to_string(), important: false },
                CssProperty { name: "outline-offset".to_string(), value: "2px".to_string(), important: false }
            ]),
            _ => None,
        }
    }

    /// Parse outline-offset classes
    fn parse_outline_offset_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("outline-offset-") {
            let number = &class[15..];
            if let Ok(_) = number.parse::<u32>() {
                return Some(vec![CssProperty {
                    name: "outline-offset".to_string(),
                    value: format!("{}px", number),
                    important: false,
                }]);
            }
        }
        
        if class.starts_with("-outline-offset-") {
            let number = &class[16..];
            if let Ok(_) = number.parse::<u32>() {
                return Some(vec![CssProperty {
                    name: "outline-offset".to_string(),
                    value: format!("calc({}px * -1)", number),
                    important: false,
                }]);
            }
        }
        
        // Custom properties for outline offset
        if let Some(value) = class.strip_prefix("outline-offset-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "outline-offset".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        // Arbitrary values for outline offset
        if let Some(value) = class.strip_prefix("outline-offset-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "outline-offset".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        None
    }

    /// Get radius value for border radius
    fn get_radius_value(&self, size: &str) -> Option<String> {
        match size {
            "xs" => Some("var(--radius-xs)".to_string()),
            "sm" => Some("var(--radius-sm)".to_string()),
            "md" => Some("var(--radius-md)".to_string()),
            "lg" => Some("var(--radius-lg)".to_string()),
            "xl" => Some("var(--radius-xl)".to_string()),
            "2xl" => Some("var(--radius-2xl)".to_string()),
            "3xl" => Some("var(--radius-3xl)".to_string()),
            "4xl" => Some("var(--radius-4xl)".to_string()),
            "none" => Some("0".to_string()),
            "full" => Some("calc(infinity * 1px)".to_string()),
            _ => None,
        }
    }

    /// Get color value for border and outline colors
    fn get_color_value(&self, class: &str) -> Option<String> {
        // This is a simplified color mapping - in a real implementation,
        // you'd want a comprehensive color system
        match class {
            "border-red-50" => Some("var(--color-red-50)".to_string()),
            "border-red-100" => Some("var(--color-red-100)".to_string()),
            "border-red-200" => Some("var(--color-red-200)".to_string()),
            "border-red-300" => Some("var(--color-red-300)".to_string()),
            "border-red-400" => Some("var(--color-red-400)".to_string()),
            "border-red-500" => Some("var(--color-red-500)".to_string()),
            "border-red-600" => Some("var(--color-red-600)".to_string()),
            "border-red-700" => Some("var(--color-red-700)".to_string()),
            "border-red-800" => Some("var(--color-red-800)".to_string()),
            "border-red-900" => Some("var(--color-red-900)".to_string()),
            "border-red-950" => Some("var(--color-red-950)".to_string()),
            "border-blue-50" => Some("var(--color-blue-50)".to_string()),
            "border-blue-100" => Some("var(--color-blue-100)".to_string()),
            "border-blue-200" => Some("var(--color-blue-200)".to_string()),
            "border-blue-300" => Some("var(--color-blue-300)".to_string()),
            "border-blue-400" => Some("var(--color-blue-400)".to_string()),
            "border-blue-500" => Some("var(--color-blue-500)".to_string()),
            "border-blue-600" => Some("var(--color-blue-600)".to_string()),
            "border-blue-700" => Some("var(--color-blue-700)".to_string()),
            "border-blue-800" => Some("var(--color-blue-800)".to_string()),
            "border-blue-900" => Some("var(--color-blue-900)".to_string()),
            "border-blue-950" => Some("var(--color-blue-950)".to_string()),
            "border-green-50" => Some("var(--color-green-50)".to_string()),
            "border-green-100" => Some("var(--color-green-100)".to_string()),
            "border-green-200" => Some("var(--color-green-200)".to_string()),
            "border-green-300" => Some("var(--color-green-300)".to_string()),
            "border-green-400" => Some("var(--color-green-400)".to_string()),
            "border-green-500" => Some("var(--color-green-500)".to_string()),
            "border-green-600" => Some("var(--color-green-600)".to_string()),
            "border-green-700" => Some("var(--color-green-700)".to_string()),
            "border-green-800" => Some("var(--color-green-800)".to_string()),
            "border-green-900" => Some("var(--color-green-900)".to_string()),
            "border-green-950" => Some("var(--color-green-950)".to_string()),
            "outline-red-50" => Some("var(--color-red-50)".to_string()),
            "outline-red-100" => Some("var(--color-red-100)".to_string()),
            "outline-red-200" => Some("var(--color-red-200)".to_string()),
            "outline-red-300" => Some("var(--color-red-300)".to_string()),
            "outline-red-400" => Some("var(--color-red-400)".to_string()),
            "outline-red-500" => Some("var(--color-red-500)".to_string()),
            "outline-red-600" => Some("var(--color-red-600)".to_string()),
            "outline-red-700" => Some("var(--color-red-700)".to_string()),
            "outline-red-800" => Some("var(--color-red-800)".to_string()),
            "outline-red-900" => Some("var(--color-red-900)".to_string()),
            "outline-red-950" => Some("var(--color-red-950)".to_string()),
            "outline-blue-50" => Some("var(--color-blue-50)".to_string()),
            "outline-blue-100" => Some("var(--color-blue-100)".to_string()),
            "outline-blue-200" => Some("var(--color-blue-200)".to_string()),
            "outline-blue-300" => Some("var(--color-blue-300)".to_string()),
            "outline-blue-400" => Some("var(--color-blue-400)".to_string()),
            "outline-blue-500" => Some("var(--color-blue-500)".to_string()),
            "outline-blue-600" => Some("var(--color-blue-600)".to_string()),
            "outline-blue-700" => Some("var(--color-blue-700)".to_string()),
            "outline-blue-800" => Some("var(--color-blue-800)".to_string()),
            "outline-blue-900" => Some("var(--color-blue-900)".to_string()),
            "outline-blue-950" => Some("var(--color-blue-950)".to_string()),
            "outline-green-50" => Some("var(--color-green-50)".to_string()),
            "outline-green-100" => Some("var(--color-green-100)".to_string()),
            "outline-green-200" => Some("var(--color-green-200)".to_string()),
            "outline-green-300" => Some("var(--color-green-300)".to_string()),
            "outline-green-400" => Some("var(--color-green-400)".to_string()),
            "outline-green-500" => Some("var(--color-green-500)".to_string()),
            "outline-green-600" => Some("var(--color-green-600)".to_string()),
            "outline-green-700" => Some("var(--color-green-700)".to_string()),
            "outline-green-800" => Some("var(--color-green-800)".to_string()),
            "outline-green-900" => Some("var(--color-green-900)".to_string()),
            "outline-green-950" => Some("var(--color-green-950)".to_string()),
            _ => None,
        }
    }
}

impl UtilityParser for BorderUtilitiesParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity
        
        // Outline offset (most specific)
        if let Some(properties) = self.parse_outline_offset_class(class) {
            return Some(properties);
        }
        
        // Outline style
        if let Some(properties) = self.parse_outline_style_class(class) {
            return Some(properties);
        }
        
        // Outline color
        if let Some(properties) = self.parse_outline_color_class(class) {
            return Some(properties);
        }
        
        // Outline width
        if let Some(properties) = self.parse_outline_width_class(class) {
            return Some(properties);
        }
        
        // Border style
        if let Some(properties) = self.parse_border_style_class(class) {
            return Some(properties);
        }
        
        // Border color
        if let Some(properties) = self.parse_border_color_class(class) {
            return Some(properties);
        }
        
        // Border width
        if let Some(properties) = self.parse_border_width_class(class) {
            return Some(properties);
        }
        
        // Border radius (least specific)
        if let Some(properties) = self.parse_border_radius_class(class) {
            return Some(properties);
        }
        
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "rounded-*", "border-*", "outline-*", "outline-offset-*",
            "rounded-t-*", "rounded-r-*", "rounded-b-*", "rounded-l-*",
            "rounded-tl-*", "rounded-tr-*", "rounded-br-*", "rounded-bl-*",
            "border-t-*", "border-r-*", "border-b-*", "border-l-*",
            "border-x-*", "border-y-*", "border-solid", "border-dashed",
            "border-dotted", "border-double", "border-hidden", "border-none",
            "outline-solid", "outline-dashed", "outline-dotted", "outline-double",
            "outline-none", "outline-hidden"
        ]
    }

    fn get_priority(&self) -> u32 { 85 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Borders }
}
