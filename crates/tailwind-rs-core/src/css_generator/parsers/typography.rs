//! Typography Utilities Parser

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct TypographyParser;

impl TypographyParser {
    pub fn new() -> Self { Self }
    
    /// Parse text color classes
    fn parse_text_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(color) = class.strip_prefix("text-") {
            let color_value = self.get_color_value(color);
            return Some(vec![CssProperty { name: "color".to_string(), value: color_value, important: false }]);
        }
        None
    }
    
    /// Parse font size classes
    fn parse_font_size_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "text-xs" => Some(vec![CssProperty { name: "font-size".to_string(), value: "0.75rem".to_string(), important: false }]),
            "text-sm" => Some(vec![CssProperty { name: "font-size".to_string(), value: "0.875rem".to_string(), important: false }]),
            "text-base" => Some(vec![CssProperty { name: "font-size".to_string(), value: "1rem".to_string(), important: false }]),
            "text-lg" => Some(vec![CssProperty { name: "font-size".to_string(), value: "1.125rem".to_string(), important: false }]),
            "text-xl" => Some(vec![CssProperty { name: "font-size".to_string(), value: "1.25rem".to_string(), important: false }]),
            "text-2xl" => Some(vec![CssProperty { name: "font-size".to_string(), value: "1.5rem".to_string(), important: false }]),
            "text-3xl" => Some(vec![CssProperty { name: "font-size".to_string(), value: "1.875rem".to_string(), important: false }]),
            "text-4xl" => Some(vec![CssProperty { name: "font-size".to_string(), value: "2.25rem".to_string(), important: false }]),
            "text-5xl" => Some(vec![CssProperty { name: "font-size".to_string(), value: "3rem".to_string(), important: false }]),
            "text-6xl" => Some(vec![CssProperty { name: "font-size".to_string(), value: "3.75rem".to_string(), important: false }]),
            "text-7xl" => Some(vec![CssProperty { name: "font-size".to_string(), value: "4.5rem".to_string(), important: false }]),
            "text-8xl" => Some(vec![CssProperty { name: "font-size".to_string(), value: "6rem".to_string(), important: false }]),
            "text-9xl" => Some(vec![CssProperty { name: "font-size".to_string(), value: "8rem".to_string(), important: false }]),
            _ => None,
        }
    }
    
    /// Parse font weight classes
    fn parse_font_weight_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "font-thin" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "100".to_string(), important: false }]),
            "font-extralight" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "200".to_string(), important: false }]),
            "font-light" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "300".to_string(), important: false }]),
            "font-normal" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "400".to_string(), important: false }]),
            "font-medium" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "500".to_string(), important: false }]),
            "font-semibold" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "600".to_string(), important: false }]),
            "font-bold" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "700".to_string(), important: false }]),
            "font-extrabold" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "800".to_string(), important: false }]),
            "font-black" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "900".to_string(), important: false }]),
            _ => None,
        }
    }
    
    /// Parse text alignment classes
    fn parse_text_align_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "text-left" => Some(vec![CssProperty { name: "text-align".to_string(), value: "left".to_string(), important: false }]),
            "text-center" => Some(vec![CssProperty { name: "text-align".to_string(), value: "center".to_string(), important: false }]),
            "text-right" => Some(vec![CssProperty { name: "text-align".to_string(), value: "right".to_string(), important: false }]),
            "text-justify" => Some(vec![CssProperty { name: "text-align".to_string(), value: "justify".to_string(), important: false }]),
            _ => None,
        }
    }
    
    /// Parse text decoration classes
    fn parse_text_decoration_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "underline" => Some(vec![CssProperty { name: "text-decoration-line".to_string(), value: "underline".to_string(), important: false }]),
            "overline" => Some(vec![CssProperty { name: "text-decoration-line".to_string(), value: "overline".to_string(), important: false }]),
            "line-through" => Some(vec![CssProperty { name: "text-decoration-line".to_string(), value: "line-through".to_string(), important: false }]),
            "no-underline" => Some(vec![CssProperty { name: "text-decoration-line".to_string(), value: "none".to_string(), important: false }]),
            _ => None,
        }
    }
    
    /// Parse text transform classes
    fn parse_text_transform_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "uppercase" => Some(vec![CssProperty { name: "text-transform".to_string(), value: "uppercase".to_string(), important: false }]),
            "lowercase" => Some(vec![CssProperty { name: "text-transform".to_string(), value: "lowercase".to_string(), important: false }]),
            "capitalize" => Some(vec![CssProperty { name: "text-transform".to_string(), value: "capitalize".to_string(), important: false }]),
            "normal-case" => Some(vec![CssProperty { name: "text-transform".to_string(), value: "none".to_string(), important: false }]),
            _ => None,
        }
    }
    
    /// Parse text opacity classes
    fn parse_text_opacity_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(opacity) = class.strip_prefix("text-opacity-") {
            let opacity_value = format!("{}%", opacity);
            return Some(vec![CssProperty { name: "opacity".to_string(), value: opacity_value, important: false }]);
        }
        None
    }
    
    /// Parse text transparent class
    fn parse_text_transparent_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class == "text-transparent" {
            return Some(vec![CssProperty { name: "color".to_string(), value: "transparent".to_string(), important: false }]);
        }
        None
    }
    
    /// Parse letter spacing classes
    fn parse_letter_spacing_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "tracking-tighter" => Some(vec![CssProperty { name: "letter-spacing".to_string(), value: "-0.05em".to_string(), important: false }]),
            "tracking-tight" => Some(vec![CssProperty { name: "letter-spacing".to_string(), value: "-0.025em".to_string(), important: false }]),
            "tracking-normal" => Some(vec![CssProperty { name: "letter-spacing".to_string(), value: "0em".to_string(), important: false }]),
            "tracking-wide" => Some(vec![CssProperty { name: "letter-spacing".to_string(), value: "0.025em".to_string(), important: false }]),
            "tracking-wider" => Some(vec![CssProperty { name: "letter-spacing".to_string(), value: "0.05em".to_string(), important: false }]),
            "tracking-widest" => Some(vec![CssProperty { name: "letter-spacing".to_string(), value: "0.1em".to_string(), important: false }]),
            _ => None,
        }
    }
    
    /// Parse line height classes
    fn parse_line_height_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "leading-none" => Some(vec![CssProperty { name: "line-height".to_string(), value: "1".to_string(), important: false }]),
            "leading-tight" => Some(vec![CssProperty { name: "line-height".to_string(), value: "1.25".to_string(), important: false }]),
            "leading-snug" => Some(vec![CssProperty { name: "line-height".to_string(), value: "1.375".to_string(), important: false }]),
            "leading-normal" => Some(vec![CssProperty { name: "line-height".to_string(), value: "1.5".to_string(), important: false }]),
            "leading-relaxed" => Some(vec![CssProperty { name: "line-height".to_string(), value: "1.625".to_string(), important: false }]),
            "leading-loose" => Some(vec![CssProperty { name: "line-height".to_string(), value: "2".to_string(), important: false }]),
            _ => None,
        }
    }
    
    /// Parse order classes
    fn parse_order_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "order-first" => Some(vec![CssProperty { name: "order".to_string(), value: "-9999".to_string(), important: false }]),
            "order-last" => Some(vec![CssProperty { name: "order".to_string(), value: "9999".to_string(), important: false }]),
            "order-none" => Some(vec![CssProperty { name: "order".to_string(), value: "0".to_string(), important: false }]),
            _ => None,
        }
    }
    
    /// Get color value for text colors
    fn get_color_value(&self, color: &str) -> String {
        match color {
            "transparent" => "transparent".to_string(),
            "current" => "currentColor".to_string(),
            "black" => "#000000".to_string(),
            "white" => "#ffffff".to_string(),
            "gray-50" => "#f9fafb".to_string(),
            "gray-100" => "#f3f4f6".to_string(),
            "gray-200" => "#e5e7eb".to_string(),
            "gray-300" => "#d1d5db".to_string(),
            "gray-400" => "#9ca3af".to_string(),
            "gray-500" => "#6b7280".to_string(),
            "gray-600" => "#4b5563".to_string(),
            "gray-700" => "#374151".to_string(),
            "gray-800" => "#1f2937".to_string(),
            "gray-900" => "#111827".to_string(),
            "zinc-50" => "#fafafa".to_string(),
            "zinc-100" => "#f4f4f5".to_string(),
            "zinc-200" => "#e4e4e7".to_string(),
            "zinc-300" => "#d4d4d8".to_string(),
            "zinc-400" => "#a1a1aa".to_string(),
            "zinc-500" => "#71717a".to_string(),
            "zinc-600" => "#52525b".to_string(),
            "zinc-700" => "#3f3f46".to_string(),
            "zinc-800" => "#27272a".to_string(),
            "zinc-900" => "#18181b".to_string(),
            "red-50" => "#fef2f2".to_string(),
            "red-100" => "#fee2e2".to_string(),
            "red-200" => "#fecaca".to_string(),
            "red-300" => "#fca5a5".to_string(),
            "red-400" => "#f87171".to_string(),
            "red-500" => "#ef4444".to_string(),
            "red-600" => "#dc2626".to_string(),
            "red-700" => "#b91c1c".to_string(),
            "red-800" => "#991b1b".to_string(),
            "red-900" => "#7f1d1d".to_string(),
            "blue-50" => "#eff6ff".to_string(),
            "blue-100" => "#dbeafe".to_string(),
            "blue-200" => "#bfdbfe".to_string(),
            "blue-300" => "#93c5fd".to_string(),
            "blue-400" => "#60a5fa".to_string(),
            "blue-500" => "#3b82f6".to_string(),
            "blue-600" => "#2563eb".to_string(),
            "blue-700" => "#1d4ed8".to_string(),
            "blue-800" => "#1e40af".to_string(),
            "blue-900" => "#1e3a8a".to_string(),
            "green-50" => "#f0fdf4".to_string(),
            "green-100" => "#dcfce7".to_string(),
            "green-200" => "#bbf7d0".to_string(),
            "green-300" => "#86efac".to_string(),
            "green-400" => "#4ade80".to_string(),
            "green-500" => "#22c55e".to_string(),
            "green-600" => "#16a34a".to_string(),
            "green-700" => "#15803d".to_string(),
            "green-800" => "#166534".to_string(),
            "green-900" => "#14532d".to_string(),
            "teal-50" => "#f0fdfa".to_string(),
            "teal-100" => "#ccfbf1".to_string(),
            "teal-200" => "#99f6e4".to_string(),
            "teal-300" => "#5eead4".to_string(),
            "teal-400" => "#2dd4bf".to_string(),
            "teal-500" => "#14b8a6".to_string(),
            "teal-600" => "#0d9488".to_string(),
            "teal-700" => "#0f766e".to_string(),
            "teal-800" => "#115e59".to_string(),
            "teal-900" => "#134e4a".to_string(),
            _ => "#000000".to_string(), // Default to black for unknown colors
        }.to_string()
    }
}

impl UtilityParser for TypographyParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try text color classes first
        if let Some(properties) = self.parse_text_color_class(class) {
            return Some(properties);
        }
        
        // Try font size classes
        if let Some(properties) = self.parse_font_size_class(class) {
            return Some(properties);
        }
        
        // Try font weight classes
        if let Some(properties) = self.parse_font_weight_class(class) {
            return Some(properties);
        }
        
        // Try text alignment classes
        if let Some(properties) = self.parse_text_align_class(class) {
            return Some(properties);
        }
        
        // Try text decoration classes
        if let Some(properties) = self.parse_text_decoration_class(class) {
            return Some(properties);
        }
        
        // Try text transform classes
        if let Some(properties) = self.parse_text_transform_class(class) {
            return Some(properties);
        }
        
        // Try text opacity classes
        if let Some(properties) = self.parse_text_opacity_class(class) {
            return Some(properties);
        }
        
        // Try text transparent class
        if let Some(properties) = self.parse_text_transparent_class(class) {
            return Some(properties);
        }
        
        // Try letter spacing classes
        if let Some(properties) = self.parse_letter_spacing_class(class) {
            return Some(properties);
        }
        
        // Try line height classes
        if let Some(properties) = self.parse_line_height_class(class) {
            return Some(properties);
        }
        
        // Try order classes
        if let Some(properties) = self.parse_order_class(class) {
            return Some(properties);
        }
        
        None
    }
    
    fn get_supported_patterns(&self) -> Vec<&'static str> { 
        vec![
            "text-*", "font-*", "text-xs", "text-sm", "text-base", "text-lg", "text-xl", 
            "text-2xl", "text-3xl", "text-4xl", "text-5xl", "text-6xl", "text-7xl", 
            "text-8xl", "text-9xl", "font-thin", "font-extralight", "font-light", 
            "font-normal", "font-medium", "font-semibold", "font-bold", "font-extrabold", 
            "font-black", "text-left", "text-center", "text-right", "text-justify",
            "underline", "overline", "line-through", "no-underline", "uppercase", 
            "lowercase", "capitalize", "normal-case", "text-opacity-*", "text-transparent",
            "tracking-*", "leading-*", "order-*"
        ] 
    }
    fn get_priority(&self) -> u32 { 80 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Typography }
}

impl Default for TypographyParser {
    fn default() -> Self { Self::new() }
}
