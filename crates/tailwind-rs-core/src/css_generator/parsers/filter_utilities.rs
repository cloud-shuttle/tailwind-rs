//! Filter Utilities Parser
//!
//! This module provides parsing logic for filter-related Tailwind CSS utilities,
//! including blur, brightness, contrast, drop-shadow, grayscale, hue-rotate, invert, saturate, and sepia.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct FilterUtilitiesParser;

impl FilterUtilitiesParser {
    pub fn new() -> Self { Self }

    /// Parse filter classes
    fn parse_filter_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "filter-none" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for filter
                if let Some(value) = class.strip_prefix("filter-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "filter".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for filter
                if let Some(value) = class.strip_prefix("filter-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "filter".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse blur classes
    fn parse_blur_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "blur-none" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "blur(0)".to_string(),
                important: false,
            }]),
            "blur-xs" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "blur(var(--blur-xs))".to_string(),
                important: false,
            }]),
            "blur-sm" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "blur(var(--blur-sm))".to_string(),
                important: false,
            }]),
            "blur-md" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "blur(var(--blur-md))".to_string(),
                important: false,
            }]),
            "blur-lg" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "blur(var(--blur-lg))".to_string(),
                important: false,
            }]),
            "blur-xl" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "blur(var(--blur-xl))".to_string(),
                important: false,
            }]),
            "blur-2xl" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "blur(var(--blur-2xl))".to_string(),
                important: false,
            }]),
            "blur-3xl" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "blur(var(--blur-3xl))".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for blur
                if let Some(value) = class.strip_prefix("blur-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "filter".to_string(),
                            value: format!("blur(var({}))", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for blur
                if let Some(value) = class.strip_prefix("blur-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "filter".to_string(),
                            value: format!("blur({})", value),
                            important: false,
                        }]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse brightness classes
    fn parse_brightness_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("brightness-") {
            if let Some(value) = class.strip_prefix("brightness-") {
                // Custom properties for brightness
                if let Some(value) = value.strip_prefix("(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "filter".to_string(),
                            value: format!("brightness(var({}))", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for brightness
                if let Some(value) = value.strip_prefix("[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "filter".to_string(),
                            value: format!("brightness({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Numeric values for brightness
                if value.parse::<f32>().is_ok() {
                    return Some(vec![CssProperty {
                        name: "filter".to_string(),
                        value: format!("brightness({}%)", value),
                        important: false,
                    }]);
                }
            }
        }
        None
    }

    /// Parse contrast classes
    fn parse_contrast_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("contrast-") {
            if let Some(value) = class.strip_prefix("contrast-") {
                // Custom properties for contrast
                if let Some(value) = value.strip_prefix("(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "filter".to_string(),
                            value: format!("contrast(var({}))", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for contrast
                if let Some(value) = value.strip_prefix("[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "filter".to_string(),
                            value: format!("contrast({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Numeric values for contrast
                if value.parse::<f32>().is_ok() {
                    return Some(vec![CssProperty {
                        name: "filter".to_string(),
                        value: format!("contrast({}%)", value),
                        important: false,
                    }]);
                }
            }
        }
        None
    }

    /// Parse drop-shadow classes
    fn parse_drop_shadow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "drop-shadow-none" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "drop-shadow(0 0 #0000)".to_string(),
                important: false,
            }]),
            "drop-shadow-xs" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "drop-shadow(var(--drop-shadow-xs))".to_string(),
                important: false,
            }]),
            "drop-shadow-sm" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "drop-shadow(var(--drop-shadow-sm))".to_string(),
                important: false,
            }]),
            "drop-shadow-md" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "drop-shadow(var(--drop-shadow-md))".to_string(),
                important: false,
            }]),
            "drop-shadow-lg" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "drop-shadow(var(--drop-shadow-lg))".to_string(),
                important: false,
            }]),
            "drop-shadow-xl" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "drop-shadow(var(--drop-shadow-xl))".to_string(),
                important: false,
            }]),
            "drop-shadow-2xl" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "drop-shadow(var(--drop-shadow-2xl))".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for drop-shadow
                if let Some(value) = class.strip_prefix("drop-shadow-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "filter".to_string(),
                            value: format!("drop-shadow(var({}))", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for drop-shadow
                if let Some(value) = class.strip_prefix("drop-shadow-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "filter".to_string(),
                            value: format!("drop-shadow({})", value),
                            important: false,
                        }]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse grayscale classes
    fn parse_grayscale_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "grayscale" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "grayscale(100%)".to_string(),
                important: false,
            }]),
            "grayscale-0" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "grayscale(0%)".to_string(),
                important: false,
            }]),
            _ => {
                if class.starts_with("grayscale-") {
                    if let Some(value) = class.strip_prefix("grayscale-") {
                        // Custom properties for grayscale
                        if let Some(value) = value.strip_prefix("(") {
                            if let Some(value) = value.strip_suffix(")") {
                                return Some(vec![CssProperty {
                                    name: "filter".to_string(),
                                    value: format!("grayscale(var({}))", value),
                                    important: false,
                                }]);
                            }
                        }
                        
                        // Arbitrary values for grayscale
                        if let Some(value) = value.strip_prefix("[") {
                            if let Some(value) = value.strip_suffix("]") {
                                return Some(vec![CssProperty {
                                    name: "filter".to_string(),
                                    value: format!("grayscale({})", value),
                                    important: false,
                                }]);
                            }
                        }
                        
                        // Numeric values for grayscale
                        if value.parse::<f32>().is_ok() {
                            return Some(vec![CssProperty {
                                name: "filter".to_string(),
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

    /// Parse hue-rotate classes
    fn parse_hue_rotate_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("hue-rotate-") {
            if let Some(value) = class.strip_prefix("hue-rotate-") {
                // Custom properties for hue-rotate
                if let Some(value) = value.strip_prefix("(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "filter".to_string(),
                            value: format!("hue-rotate(var({}))", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for hue-rotate
                if let Some(value) = value.strip_prefix("[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "filter".to_string(),
                            value: format!("hue-rotate({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Numeric values for hue-rotate
                if value.parse::<f32>().is_ok() {
                    return Some(vec![CssProperty {
                        name: "filter".to_string(),
                        value: format!("hue-rotate({}deg)", value),
                        important: false,
                    }]);
                }
            }
        } else if class.starts_with("-hue-rotate-") {
            if let Some(value) = class.strip_prefix("-hue-rotate-") {
                // Negative hue-rotate values
                if let Ok(num) = value.parse::<f32>() {
                    return Some(vec![CssProperty {
                        name: "filter".to_string(),
                        value: format!("hue-rotate(calc({}deg * -1))", num),
                        important: false,
                    }]);
                }
            }
        }
        None
    }

    /// Parse invert classes
    fn parse_invert_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "invert" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "invert(100%)".to_string(),
                important: false,
            }]),
            "invert-0" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "invert(0%)".to_string(),
                important: false,
            }]),
            _ => {
                if class.starts_with("invert-") {
                    if let Some(value) = class.strip_prefix("invert-") {
                        // Custom properties for invert
                        if let Some(value) = value.strip_prefix("(") {
                            if let Some(value) = value.strip_suffix(")") {
                                return Some(vec![CssProperty {
                                    name: "filter".to_string(),
                                    value: format!("invert(var({}))", value),
                                    important: false,
                                }]);
                            }
                        }
                        
                        // Arbitrary values for invert
                        if let Some(value) = value.strip_prefix("[") {
                            if let Some(value) = value.strip_suffix("]") {
                                return Some(vec![CssProperty {
                                    name: "filter".to_string(),
                                    value: format!("invert({})", value),
                                    important: false,
                                }]);
                            }
                        }
                        
                        // Numeric values for invert
                        if value.parse::<f32>().is_ok() {
                            return Some(vec![CssProperty {
                                name: "filter".to_string(),
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

    /// Parse saturate classes
    fn parse_saturate_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("saturate-") {
            if let Some(value) = class.strip_prefix("saturate-") {
                // Custom properties for saturate
                if let Some(value) = value.strip_prefix("(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "filter".to_string(),
                            value: format!("saturate(var({}))", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for saturate
                if let Some(value) = value.strip_prefix("[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "filter".to_string(),
                            value: format!("saturate({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Numeric values for saturate
                if value.parse::<f32>().is_ok() {
                    return Some(vec![CssProperty {
                        name: "filter".to_string(),
                        value: format!("saturate({}%)", value),
                        important: false,
                    }]);
                }
            }
        }
        None
    }

    /// Parse sepia classes
    fn parse_sepia_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "sepia" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "sepia(100%)".to_string(),
                important: false,
            }]),
            "sepia-0" => Some(vec![CssProperty {
                name: "filter".to_string(),
                value: "sepia(0%)".to_string(),
                important: false,
            }]),
            _ => {
                if class.starts_with("sepia-") {
                    if let Some(value) = class.strip_prefix("sepia-") {
                        // Custom properties for sepia
                        if let Some(value) = value.strip_prefix("(") {
                            if let Some(value) = value.strip_suffix(")") {
                                return Some(vec![CssProperty {
                                    name: "filter".to_string(),
                                    value: format!("sepia(var({}))", value),
                                    important: false,
                                }]);
                            }
                        }
                        
                        // Arbitrary values for sepia
                        if let Some(value) = value.strip_prefix("[") {
                            if let Some(value) = value.strip_suffix("]") {
                                return Some(vec![CssProperty {
                                    name: "filter".to_string(),
                                    value: format!("sepia({})", value),
                                    important: false,
                                }]);
                            }
                        }
                        
                        // Numeric values for sepia
                        if value.parse::<f32>().is_ok() {
                            return Some(vec![CssProperty {
                                name: "filter".to_string(),
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

impl UtilityParser for FilterUtilitiesParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity
        
        // Filter (most specific)
        if let Some(properties) = self.parse_filter_class(class) {
            return Some(properties);
        }
        
        // Blur
        if let Some(properties) = self.parse_blur_class(class) {
            return Some(properties);
        }
        
        // Brightness
        if let Some(properties) = self.parse_brightness_class(class) {
            return Some(properties);
        }
        
        // Contrast
        if let Some(properties) = self.parse_contrast_class(class) {
            return Some(properties);
        }
        
        // Drop shadow
        if let Some(properties) = self.parse_drop_shadow_class(class) {
            return Some(properties);
        }
        
        // Grayscale
        if let Some(properties) = self.parse_grayscale_class(class) {
            return Some(properties);
        }
        
        // Hue rotate
        if let Some(properties) = self.parse_hue_rotate_class(class) {
            return Some(properties);
        }
        
        // Invert
        if let Some(properties) = self.parse_invert_class(class) {
            return Some(properties);
        }
        
        // Saturate
        if let Some(properties) = self.parse_saturate_class(class) {
            return Some(properties);
        }
        
        // Sepia (least specific)
        if let Some(properties) = self.parse_sepia_class(class) {
            return Some(properties);
        }
        
        None
    }
    
    fn get_supported_patterns(&self) -> Vec<&'static str> { 
        vec![
            "filter-*", "blur-*", "brightness-*", "contrast-*", "drop-shadow-*", 
            "grayscale-*", "hue-rotate-*", "invert-*", "saturate-*", "sepia-*"
        ] 
    }
    
    fn get_priority(&self) -> u32 { 50 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Effects }
}

impl Default for FilterUtilitiesParser {
    fn default() -> Self {
        Self::new()
    }
}
