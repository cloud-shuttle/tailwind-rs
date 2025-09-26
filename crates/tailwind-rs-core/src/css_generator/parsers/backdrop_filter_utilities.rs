//! Backdrop Filter Utilities Parser
//!
//! This module provides parsing logic for backdrop filter-related Tailwind CSS utilities,
//! including backdrop-blur, backdrop-brightness, backdrop-contrast, backdrop-grayscale,
//! backdrop-hue-rotate, backdrop-invert, backdrop-opacity, backdrop-saturate, and backdrop-sepia.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct BackdropFilterUtilitiesParser;

impl BackdropFilterUtilitiesParser {
    pub fn new() -> Self { Self }

    /// Parse backdrop-filter classes
    fn parse_backdrop_filter_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "backdrop-filter-none" => Some(vec![CssProperty {
                name: "backdrop-filter".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for backdrop-filter
                if let Some(value) = class.strip_prefix("backdrop-filter-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "backdrop-filter".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for backdrop-filter
                if let Some(value) = class.strip_prefix("backdrop-filter-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "backdrop-filter".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse backdrop-blur classes
    fn parse_backdrop_blur_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "backdrop-blur-none" => Some(vec![CssProperty {
                name: "backdrop-filter".to_string(),
                value: "blur(0)".to_string(),
                important: false,
            }]),
            "backdrop-blur-xs" => Some(vec![CssProperty {
                name: "backdrop-filter".to_string(),
                value: "blur(var(--blur-xs))".to_string(),
                important: false,
            }]),
            "backdrop-blur-sm" => Some(vec![CssProperty {
                name: "backdrop-filter".to_string(),
                value: "blur(var(--blur-sm))".to_string(),
                important: false,
            }]),
            "backdrop-blur-md" => Some(vec![CssProperty {
                name: "backdrop-filter".to_string(),
                value: "blur(var(--blur-md))".to_string(),
                important: false,
            }]),
            "backdrop-blur-lg" => Some(vec![CssProperty {
                name: "backdrop-filter".to_string(),
                value: "blur(var(--blur-lg))".to_string(),
                important: false,
            }]),
            "backdrop-blur-xl" => Some(vec![CssProperty {
                name: "backdrop-filter".to_string(),
                value: "blur(var(--blur-xl))".to_string(),
                important: false,
            }]),
            "backdrop-blur-2xl" => Some(vec![CssProperty {
                name: "backdrop-filter".to_string(),
                value: "blur(var(--blur-2xl))".to_string(),
                important: false,
            }]),
            "backdrop-blur-3xl" => Some(vec![CssProperty {
                name: "backdrop-filter".to_string(),
                value: "blur(var(--blur-3xl))".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for backdrop-blur
                if let Some(value) = class.strip_prefix("backdrop-blur-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "backdrop-filter".to_string(),
                            value: format!("blur(var({}))", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for backdrop-blur
                if let Some(value) = class.strip_prefix("backdrop-blur-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "backdrop-filter".to_string(),
                            value: format!("blur({})", value),
                            important: false,
                        }]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse backdrop-brightness classes
    fn parse_backdrop_brightness_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("backdrop-brightness-") {
            if let Some(value) = class.strip_prefix("backdrop-brightness-") {
                // Custom properties for backdrop-brightness
                if let Some(value) = value.strip_prefix("(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "backdrop-filter".to_string(),
                            value: format!("brightness(var({}))", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for backdrop-brightness
                if let Some(value) = value.strip_prefix("[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "backdrop-filter".to_string(),
                            value: format!("brightness({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Numeric values for backdrop-brightness
                if let Ok(_) = value.parse::<f32>() {
                    return Some(vec![CssProperty {
                        name: "backdrop-filter".to_string(),
                        value: format!("brightness({}%)", value),
                        important: false,
                    }]);
                }
            }
        }
        None
    }

    /// Parse backdrop-contrast classes
    fn parse_backdrop_contrast_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("backdrop-contrast-") {
            if let Some(value) = class.strip_prefix("backdrop-contrast-") {
                // Custom properties for backdrop-contrast
                if let Some(value) = value.strip_prefix("(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "backdrop-filter".to_string(),
                            value: format!("contrast(var({}))", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for backdrop-contrast
                if let Some(value) = value.strip_prefix("[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "backdrop-filter".to_string(),
                            value: format!("contrast({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Numeric values for backdrop-contrast
                if let Ok(_) = value.parse::<f32>() {
                    return Some(vec![CssProperty {
                        name: "backdrop-filter".to_string(),
                        value: format!("contrast({}%)", value),
                        important: false,
                    }]);
                }
            }
        }
        None
    }

    /// Parse backdrop-grayscale classes
    fn parse_backdrop_grayscale_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "backdrop-grayscale" => Some(vec![CssProperty {
                name: "backdrop-filter".to_string(),
                value: "grayscale(100%)".to_string(),
                important: false,
            }]),
            "backdrop-grayscale-0" => Some(vec![CssProperty {
                name: "backdrop-filter".to_string(),
                value: "grayscale(0%)".to_string(),
                important: false,
            }]),
            _ => {
                if class.starts_with("backdrop-grayscale-") {
                    if let Some(value) = class.strip_prefix("backdrop-grayscale-") {
                        // Custom properties for backdrop-grayscale
                        if let Some(value) = value.strip_prefix("(") {
                            if let Some(value) = value.strip_suffix(")") {
                                return Some(vec![CssProperty {
                                    name: "backdrop-filter".to_string(),
                                    value: format!("grayscale(var({}))", value),
                                    important: false,
                                }]);
                            }
                        }
                        
                        // Arbitrary values for backdrop-grayscale
                        if let Some(value) = value.strip_prefix("[") {
                            if let Some(value) = value.strip_suffix("]") {
                                return Some(vec![CssProperty {
                                    name: "backdrop-filter".to_string(),
                                    value: format!("grayscale({})", value),
                                    important: false,
                                }]);
                            }
                        }
                        
                        // Numeric values for backdrop-grayscale
                        if let Ok(_) = value.parse::<f32>() {
                            return Some(vec![CssProperty {
                                name: "backdrop-filter".to_string(),
                                value: format!("grayscale({}%)", value),
                                important: false,
                            }]);
                        }
                    }
                }
                None
            }
        }
    }

    /// Parse backdrop-hue-rotate classes
    fn parse_backdrop_hue_rotate_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("backdrop-hue-rotate-") {
            if let Some(value) = class.strip_prefix("backdrop-hue-rotate-") {
                // Custom properties for backdrop-hue-rotate
                if let Some(value) = value.strip_prefix("(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "backdrop-filter".to_string(),
                            value: format!("hue-rotate(var({}))", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for backdrop-hue-rotate
                if let Some(value) = value.strip_prefix("[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "backdrop-filter".to_string(),
                            value: format!("hue-rotate({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Numeric values for backdrop-hue-rotate
                if let Ok(_) = value.parse::<f32>() {
                    return Some(vec![CssProperty {
                        name: "backdrop-filter".to_string(),
                        value: format!("hue-rotate({}deg)", value),
                        important: false,
                    }]);
                }
            }
        } else if class.starts_with("-backdrop-hue-rotate-") {
            if let Some(value) = class.strip_prefix("-backdrop-hue-rotate-") {
                // Negative backdrop-hue-rotate values
                if let Ok(num) = value.parse::<f32>() {
                    return Some(vec![CssProperty {
                        name: "backdrop-filter".to_string(),
                        value: format!("hue-rotate(calc({}deg * -1))", num),
                        important: false,
                    }]);
                }
            }
        }
        None
    }

    /// Parse backdrop-invert classes
    fn parse_backdrop_invert_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "backdrop-invert" => Some(vec![CssProperty {
                name: "backdrop-filter".to_string(),
                value: "invert(100%)".to_string(),
                important: false,
            }]),
            "backdrop-invert-0" => Some(vec![CssProperty {
                name: "backdrop-filter".to_string(),
                value: "invert(0%)".to_string(),
                important: false,
            }]),
            _ => {
                if class.starts_with("backdrop-invert-") {
                    if let Some(value) = class.strip_prefix("backdrop-invert-") {
                        // Custom properties for backdrop-invert
                        if let Some(value) = value.strip_prefix("(") {
                            if let Some(value) = value.strip_suffix(")") {
                                return Some(vec![CssProperty {
                                    name: "backdrop-filter".to_string(),
                                    value: format!("invert(var({}))", value),
                                    important: false,
                                }]);
                            }
                        }
                        
                        // Arbitrary values for backdrop-invert
                        if let Some(value) = value.strip_prefix("[") {
                            if let Some(value) = value.strip_suffix("]") {
                                return Some(vec![CssProperty {
                                    name: "backdrop-filter".to_string(),
                                    value: format!("invert({})", value),
                                    important: false,
                                }]);
                            }
                        }
                        
                        // Numeric values for backdrop-invert
                        if let Ok(_) = value.parse::<f32>() {
                            return Some(vec![CssProperty {
                                name: "backdrop-filter".to_string(),
                                value: format!("invert({}%)", value),
                                important: false,
                            }]);
                        }
                    }
                }
                None
            }
        }
    }

    /// Parse backdrop-opacity classes
    fn parse_backdrop_opacity_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("backdrop-opacity-") {
            if let Some(value) = class.strip_prefix("backdrop-opacity-") {
                // Custom properties for backdrop-opacity
                if let Some(value) = value.strip_prefix("(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "backdrop-filter".to_string(),
                            value: format!("opacity(var({}))", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for backdrop-opacity
                if let Some(value) = value.strip_prefix("[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "backdrop-filter".to_string(),
                            value: format!("opacity({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Numeric values for backdrop-opacity
                if let Ok(_) = value.parse::<f32>() {
                    return Some(vec![CssProperty {
                        name: "backdrop-filter".to_string(),
                        value: format!("opacity({}%)", value),
                        important: false,
                    }]);
                }
            }
        }
        None
    }

    /// Parse backdrop-saturate classes
    fn parse_backdrop_saturate_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("backdrop-saturate-") {
            if let Some(value) = class.strip_prefix("backdrop-saturate-") {
                // Custom properties for backdrop-saturate
                if let Some(value) = value.strip_prefix("(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "backdrop-filter".to_string(),
                            value: format!("saturate(var({}))", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for backdrop-saturate
                if let Some(value) = value.strip_prefix("[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "backdrop-filter".to_string(),
                            value: format!("saturate({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Numeric values for backdrop-saturate
                if let Ok(_) = value.parse::<f32>() {
                    return Some(vec![CssProperty {
                        name: "backdrop-filter".to_string(),
                        value: format!("saturate({}%)", value),
                        important: false,
                    }]);
                }
            }
        }
        None
    }

    /// Parse backdrop-sepia classes
    fn parse_backdrop_sepia_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "backdrop-sepia" => Some(vec![CssProperty {
                name: "backdrop-filter".to_string(),
                value: "sepia(100%)".to_string(),
                important: false,
            }]),
            "backdrop-sepia-0" => Some(vec![CssProperty {
                name: "backdrop-filter".to_string(),
                value: "sepia(0%)".to_string(),
                important: false,
            }]),
            _ => {
                if class.starts_with("backdrop-sepia-") {
                    if let Some(value) = class.strip_prefix("backdrop-sepia-") {
                        // Custom properties for backdrop-sepia
                        if let Some(value) = value.strip_prefix("(") {
                            if let Some(value) = value.strip_suffix(")") {
                                return Some(vec![CssProperty {
                                    name: "backdrop-filter".to_string(),
                                    value: format!("sepia(var({}))", value),
                                    important: false,
                                }]);
                            }
                        }
                        
                        // Arbitrary values for backdrop-sepia
                        if let Some(value) = value.strip_prefix("[") {
                            if let Some(value) = value.strip_suffix("]") {
                                return Some(vec![CssProperty {
                                    name: "backdrop-filter".to_string(),
                                    value: format!("sepia({})", value),
                                    important: false,
                                }]);
                            }
                        }
                        
                        // Numeric values for backdrop-sepia
                        if let Ok(_) = value.parse::<f32>() {
                            return Some(vec![CssProperty {
                                name: "backdrop-filter".to_string(),
                                value: format!("sepia({}%)", value),
                                important: false,
                            }]);
                        }
                    }
                }
                None
            }
        }
    }
}

impl UtilityParser for BackdropFilterUtilitiesParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity
        
        // Backdrop-filter (most specific)
        if let Some(properties) = self.parse_backdrop_filter_class(class) {
            return Some(properties);
        }
        
        // Backdrop-blur
        if let Some(properties) = self.parse_backdrop_blur_class(class) {
            return Some(properties);
        }
        
        // Backdrop-brightness
        if let Some(properties) = self.parse_backdrop_brightness_class(class) {
            return Some(properties);
        }
        
        // Backdrop-contrast
        if let Some(properties) = self.parse_backdrop_contrast_class(class) {
            return Some(properties);
        }
        
        // Backdrop-grayscale
        if let Some(properties) = self.parse_backdrop_grayscale_class(class) {
            return Some(properties);
        }
        
        // Backdrop-hue-rotate
        if let Some(properties) = self.parse_backdrop_hue_rotate_class(class) {
            return Some(properties);
        }
        
        // Backdrop-invert
        if let Some(properties) = self.parse_backdrop_invert_class(class) {
            return Some(properties);
        }
        
        // Backdrop-opacity
        if let Some(properties) = self.parse_backdrop_opacity_class(class) {
            return Some(properties);
        }
        
        // Backdrop-saturate
        if let Some(properties) = self.parse_backdrop_saturate_class(class) {
            return Some(properties);
        }
        
        // Backdrop-sepia (least specific)
        if let Some(properties) = self.parse_backdrop_sepia_class(class) {
            return Some(properties);
        }
        
        None
    }
    
    fn get_supported_patterns(&self) -> Vec<&'static str> { 
        vec![
            "backdrop-filter-*", "backdrop-blur-*", "backdrop-brightness-*", "backdrop-contrast-*", 
            "backdrop-grayscale-*", "backdrop-hue-rotate-*", "backdrop-invert-*", "backdrop-opacity-*", 
            "backdrop-saturate-*", "backdrop-sepia-*"
        ] 
    }
    
    fn get_priority(&self) -> u32 { 50 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Effects }
}

impl Default for BackdropFilterUtilitiesParser {
    fn default() -> Self {
        Self::new()
    }
}
