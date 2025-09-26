use crate::css_generator::types::CssProperty;
use super::{UtilityParser, ParserCategory};

/// Parser for background utilities
#[derive(Debug, Clone)]
pub struct BackgroundParser;

impl BackgroundParser {
    /// Create a new BackgroundParser
    pub fn new() -> Self {
        Self
    }
}

impl BackgroundParser {
    /// Parse background-attachment classes
    fn parse_background_attachment_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-fixed" => Some(vec![CssProperty { name: "background-attachment".to_string(), value: "fixed".to_string(), important: false }]),
            "bg-local" => Some(vec![CssProperty { name: "background-attachment".to_string(), value: "local".to_string(), important: false }]),
            "bg-scroll" => Some(vec![CssProperty { name: "background-attachment".to_string(), value: "scroll".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse background-clip classes
    fn parse_background_clip_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-clip-border" => Some(vec![CssProperty { name: "background-clip".to_string(), value: "border-box".to_string(), important: false }]),
            "bg-clip-padding" => Some(vec![CssProperty { name: "background-clip".to_string(), value: "padding-box".to_string(), important: false }]),
            "bg-clip-content" => Some(vec![CssProperty { name: "background-clip".to_string(), value: "content-box".to_string(), important: false }]),
            "bg-clip-text" => Some(vec![CssProperty { name: "background-clip".to_string(), value: "text".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse background-color classes
    fn parse_background_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-inherit" => Some(vec![CssProperty { name: "background-color".to_string(), value: "inherit".to_string(), important: false }]),
            "bg-current" => Some(vec![CssProperty { name: "background-color".to_string(), value: "currentColor".to_string(), important: false }]),
            "bg-transparent" => Some(vec![CssProperty { name: "background-color".to_string(), value: "transparent".to_string(), important: false }]),
            "bg-black" => Some(vec![CssProperty { name: "background-color".to_string(), value: "var(--color-black)".to_string(), important: false }]),
            "bg-white" => Some(vec![CssProperty { name: "background-color".to_string(), value: "var(--color-white)".to_string(), important: false }]),
            _ => {
                // Custom properties for background color
                if let Some(value) = class.strip_prefix("bg-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "background-color".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for background color
                if let Some(value) = class.strip_prefix("bg-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "background-color".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                // Color with opacity modifier (e.g., bg-blue-600/50)
                if class.contains("/") {
                    let parts: Vec<&str> = class.split("/").collect();
                    if parts.len() == 2 {
                        let base_color = parts[0];
                        let opacity = parts[1];
                        if let Some(color_value) = self.get_color_value(base_color) {
                            return Some(vec![CssProperty {
                                name: "background-color".to_string(),
                                value: format!("{}/{}", color_value, opacity),
                                important: false,
                            }]);
                        }
                    }
                }
                
                // Standard color classes (bg-red-500, bg-blue-600, etc.)
                if let Some(color_value) = self.get_color_value(class) {
                    return Some(vec![CssProperty {
                        name: "background-color".to_string(),
                        value: color_value,
                        important: false,
                    }]);
                }
                
                None
            }
        }
    }

    /// Parse background-image classes
    fn parse_background_image_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-none" => Some(vec![CssProperty { name: "background-image".to_string(), value: "none".to_string(), important: false }]),
            "bg-linear-to-t" => Some(vec![CssProperty { name: "background-image".to_string(), value: "linear-gradient(to top, var(--tw-gradient-stops))".to_string(), important: false }]),
            "bg-linear-to-tr" => Some(vec![CssProperty { name: "background-image".to_string(), value: "linear-gradient(to top right, var(--tw-gradient-stops))".to_string(), important: false }]),
            "bg-linear-to-r" => Some(vec![CssProperty { name: "background-image".to_string(), value: "linear-gradient(to right, var(--tw-gradient-stops))".to_string(), important: false }]),
            "bg-linear-to-br" => Some(vec![CssProperty { name: "background-image".to_string(), value: "linear-gradient(to bottom right, var(--tw-gradient-stops))".to_string(), important: false }]),
            "bg-linear-to-b" => Some(vec![CssProperty { name: "background-image".to_string(), value: "linear-gradient(to bottom, var(--tw-gradient-stops))".to_string(), important: false }]),
            "bg-linear-to-bl" => Some(vec![CssProperty { name: "background-image".to_string(), value: "linear-gradient(to bottom left, var(--tw-gradient-stops))".to_string(), important: false }]),
            "bg-linear-to-l" => Some(vec![CssProperty { name: "background-image".to_string(), value: "linear-gradient(to left, var(--tw-gradient-stops))".to_string(), important: false }]),
            "bg-linear-to-tl" => Some(vec![CssProperty { name: "background-image".to_string(), value: "linear-gradient(to top left, var(--tw-gradient-stops))".to_string(), important: false }]),
            _ => {
                // Custom properties for background image
                if let Some(value) = class.strip_prefix("bg-(image:") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "background-image".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for background image
                if let Some(value) = class.strip_prefix("bg-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "background-image".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                // Linear gradient with angle
                if let Some(angle) = class.strip_prefix("bg-linear-") {
                    if let Some(angle) = angle.strip_suffix("deg") {
                        if let Ok(_) = angle.parse::<f32>() {
                            return Some(vec![CssProperty {
                                name: "background-image".to_string(),
                                value: format!("linear-gradient({}deg, var(--tw-gradient-stops))", angle),
                                important: false,
                            }]);
                        }
                    }
                }
                
                // Radial gradient
                if class.starts_with("bg-radial") {
                    if class == "bg-radial" {
                        return Some(vec![CssProperty {
                            name: "background-image".to_string(),
                            value: "radial-gradient(var(--tw-gradient-stops))".to_string(),
                            important: false,
                        }]);
                    }
                    
                    // Radial gradient with position
                    if let Some(position) = class.strip_prefix("bg-radial-[at_") {
                        if let Some(position) = position.strip_suffix("]") {
                            return Some(vec![CssProperty {
                                name: "background-image".to_string(),
                                value: format!("radial-gradient(at {}, var(--tw-gradient-stops))", position),
                                important: false,
                            }]);
                        }
                    }
                }
                
                // Conic gradient
                if class.starts_with("bg-conic") {
                    if class == "bg-conic" {
                        return Some(vec![CssProperty {
                            name: "background-image".to_string(),
                            value: "conic-gradient(var(--tw-gradient-stops))".to_string(),
                            important: false,
                        }]);
                    }
                    
                    // Conic gradient with angle
                    if let Some(angle) = class.strip_prefix("bg-conic-") {
                        if let Some(angle) = angle.strip_suffix("deg") {
                            if let Ok(_) = angle.parse::<f32>() {
                                return Some(vec![CssProperty {
                                    name: "background-image".to_string(),
                                    value: format!("conic-gradient({}deg, var(--tw-gradient-stops))", angle),
                                    important: false,
                                }]);
                            }
                        }
                    }
                }
                
                None
            }
        }
    }

    /// Parse background-origin classes
    fn parse_background_origin_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-origin-border" => Some(vec![CssProperty { name: "background-origin".to_string(), value: "border-box".to_string(), important: false }]),
            "bg-origin-padding" => Some(vec![CssProperty { name: "background-origin".to_string(), value: "padding-box".to_string(), important: false }]),
            "bg-origin-content" => Some(vec![CssProperty { name: "background-origin".to_string(), value: "content-box".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse background-position classes
    fn parse_background_position_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-top-left" => Some(vec![CssProperty { name: "background-position".to_string(), value: "top left".to_string(), important: false }]),
            "bg-top" => Some(vec![CssProperty { name: "background-position".to_string(), value: "top".to_string(), important: false }]),
            "bg-top-right" => Some(vec![CssProperty { name: "background-position".to_string(), value: "top right".to_string(), important: false }]),
            "bg-left" => Some(vec![CssProperty { name: "background-position".to_string(), value: "left".to_string(), important: false }]),
            "bg-center" => Some(vec![CssProperty { name: "background-position".to_string(), value: "center".to_string(), important: false }]),
            "bg-right" => Some(vec![CssProperty { name: "background-position".to_string(), value: "right".to_string(), important: false }]),
            "bg-bottom-left" => Some(vec![CssProperty { name: "background-position".to_string(), value: "bottom left".to_string(), important: false }]),
            "bg-bottom" => Some(vec![CssProperty { name: "background-position".to_string(), value: "bottom".to_string(), important: false }]),
            "bg-bottom-right" => Some(vec![CssProperty { name: "background-position".to_string(), value: "bottom right".to_string(), important: false }]),
            _ => {
                // Custom properties for background position
                if let Some(value) = class.strip_prefix("bg-position-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "background-position".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for background position
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
        }
    }

    /// Parse background-repeat classes
    fn parse_background_repeat_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-repeat" => Some(vec![CssProperty { name: "background-repeat".to_string(), value: "repeat".to_string(), important: false }]),
            "bg-repeat-x" => Some(vec![CssProperty { name: "background-repeat".to_string(), value: "repeat-x".to_string(), important: false }]),
            "bg-repeat-y" => Some(vec![CssProperty { name: "background-repeat".to_string(), value: "repeat-y".to_string(), important: false }]),
            "bg-repeat-space" => Some(vec![CssProperty { name: "background-repeat".to_string(), value: "space".to_string(), important: false }]),
            "bg-repeat-round" => Some(vec![CssProperty { name: "background-repeat".to_string(), value: "round".to_string(), important: false }]),
            "bg-no-repeat" => Some(vec![CssProperty { name: "background-repeat".to_string(), value: "no-repeat".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse background-size classes
    fn parse_background_size_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-auto" => Some(vec![CssProperty { name: "background-size".to_string(), value: "auto".to_string(), important: false }]),
            "bg-cover" => Some(vec![CssProperty { name: "background-size".to_string(), value: "cover".to_string(), important: false }]),
            "bg-contain" => Some(vec![CssProperty { name: "background-size".to_string(), value: "contain".to_string(), important: false }]),
            _ => {
                // Custom properties for background size
                if let Some(value) = class.strip_prefix("bg-size-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "background-size".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for background size
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
        }
    }

    /// Get color value for background colors
    fn get_color_value(&self, class: &str) -> Option<String> {
        // This is a simplified color mapping - in a real implementation,
        // you'd want a comprehensive color system
        match class {
            "bg-red-50" => Some("var(--color-red-50)".to_string()),
            "bg-red-100" => Some("var(--color-red-100)".to_string()),
            "bg-red-200" => Some("var(--color-red-200)".to_string()),
            "bg-red-300" => Some("var(--color-red-300)".to_string()),
            "bg-red-400" => Some("var(--color-red-400)".to_string()),
            "bg-red-500" => Some("var(--color-red-500)".to_string()),
            "bg-red-600" => Some("var(--color-red-600)".to_string()),
            "bg-red-700" => Some("var(--color-red-700)".to_string()),
            "bg-red-800" => Some("var(--color-red-800)".to_string()),
            "bg-red-900" => Some("var(--color-red-900)".to_string()),
            "bg-red-950" => Some("var(--color-red-950)".to_string()),
            "bg-blue-50" => Some("var(--color-blue-50)".to_string()),
            "bg-blue-100" => Some("var(--color-blue-100)".to_string()),
            "bg-blue-200" => Some("var(--color-blue-200)".to_string()),
            "bg-blue-300" => Some("var(--color-blue-300)".to_string()),
            "bg-blue-400" => Some("var(--color-blue-400)".to_string()),
            "bg-blue-500" => Some("var(--color-blue-500)".to_string()),
            "bg-blue-600" => Some("var(--color-blue-600)".to_string()),
            "bg-blue-700" => Some("var(--color-blue-700)".to_string()),
            "bg-blue-800" => Some("var(--color-blue-800)".to_string()),
            "bg-blue-900" => Some("var(--color-blue-900)".to_string()),
            "bg-blue-950" => Some("var(--color-blue-950)".to_string()),
            "bg-green-50" => Some("var(--color-green-50)".to_string()),
            "bg-green-100" => Some("var(--color-green-100)".to_string()),
            "bg-green-200" => Some("var(--color-green-200)".to_string()),
            "bg-green-300" => Some("var(--color-green-300)".to_string()),
            "bg-green-400" => Some("var(--color-green-400)".to_string()),
            "bg-green-500" => Some("var(--color-green-500)".to_string()),
            "bg-green-600" => Some("var(--color-green-600)".to_string()),
            "bg-green-700" => Some("var(--color-green-700)".to_string()),
            "bg-green-800" => Some("var(--color-green-800)".to_string()),
            "bg-green-900" => Some("var(--color-green-900)".to_string()),
            "bg-green-950" => Some("var(--color-green-950)".to_string()),
            _ => None,
        }
    }
}

impl UtilityParser for BackgroundParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity
        
        // Background attachment (most specific)
        if let Some(properties) = self.parse_background_attachment_class(class) {
            return Some(properties);
        }
        
        // Background clip
        if let Some(properties) = self.parse_background_clip_class(class) {
            return Some(properties);
        }
        
        // Background origin
        if let Some(properties) = self.parse_background_origin_class(class) {
            return Some(properties);
        }
        
        // Background position
        if let Some(properties) = self.parse_background_position_class(class) {
            return Some(properties);
        }
        
        // Background repeat
        if let Some(properties) = self.parse_background_repeat_class(class) {
            return Some(properties);
        }
        
        // Background size
        if let Some(properties) = self.parse_background_size_class(class) {
            return Some(properties);
        }
        
        // Background image
        if let Some(properties) = self.parse_background_image_class(class) {
            return Some(properties);
        }
        
        // Background color (least specific)
        if let Some(properties) = self.parse_background_color_class(class) {
            return Some(properties);
        }
        
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "bg-*", "bg-fixed", "bg-local", "bg-scroll", "bg-clip-*", "bg-origin-*",
            "bg-top-*", "bg-left", "bg-center", "bg-right", "bg-bottom-*",
            "bg-repeat-*", "bg-no-repeat", "bg-auto", "bg-cover", "bg-contain",
            "bg-linear-*", "bg-radial*", "bg-conic*", "bg-none"
        ]
    }

    fn get_priority(&self) -> u32 { 80 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Background }
}
