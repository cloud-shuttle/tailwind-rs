//! Interactive Utilities Parser

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct InteractiveParser;

impl InteractiveParser {
    pub fn new() -> Self { Self }
    
    /// Parse pointer events classes
    fn parse_pointer_events_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "pointer-events-none" => Some(vec![CssProperty { name: "pointer-events".to_string(), value: "none".to_string(), important: false }]),
            "pointer-events-auto" => Some(vec![CssProperty { name: "pointer-events".to_string(), value: "auto".to_string(), important: false }]),
            _ => None,
        }
    }
    
    /// Parse cursor classes
    fn parse_cursor_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "cursor-auto" => Some(vec![CssProperty { name: "cursor".to_string(), value: "auto".to_string(), important: false }]),
            "cursor-default" => Some(vec![CssProperty { name: "cursor".to_string(), value: "default".to_string(), important: false }]),
            "cursor-pointer" => Some(vec![CssProperty { name: "cursor".to_string(), value: "pointer".to_string(), important: false }]),
            "cursor-wait" => Some(vec![CssProperty { name: "cursor".to_string(), value: "wait".to_string(), important: false }]),
            "cursor-text" => Some(vec![CssProperty { name: "cursor".to_string(), value: "text".to_string(), important: false }]),
            "cursor-move" => Some(vec![CssProperty { name: "cursor".to_string(), value: "move".to_string(), important: false }]),
            "cursor-help" => Some(vec![CssProperty { name: "cursor".to_string(), value: "help".to_string(), important: false }]),
            "cursor-not-allowed" => Some(vec![CssProperty { name: "cursor".to_string(), value: "not-allowed".to_string(), important: false }]),
            "cursor-none" => Some(vec![CssProperty { name: "cursor".to_string(), value: "none".to_string(), important: false }]),
            "cursor-context-menu" => Some(vec![CssProperty { name: "cursor".to_string(), value: "context-menu".to_string(), important: false }]),
            "cursor-progress" => Some(vec![CssProperty { name: "cursor".to_string(), value: "progress".to_string(), important: false }]),
            "cursor-cell" => Some(vec![CssProperty { name: "cursor".to_string(), value: "cell".to_string(), important: false }]),
            "cursor-crosshair" => Some(vec![CssProperty { name: "cursor".to_string(), value: "crosshair".to_string(), important: false }]),
            "cursor-vertical-text" => Some(vec![CssProperty { name: "cursor".to_string(), value: "vertical-text".to_string(), important: false }]),
            "cursor-alias" => Some(vec![CssProperty { name: "cursor".to_string(), value: "alias".to_string(), important: false }]),
            "cursor-copy" => Some(vec![CssProperty { name: "cursor".to_string(), value: "copy".to_string(), important: false }]),
            "cursor-no-drop" => Some(vec![CssProperty { name: "cursor".to_string(), value: "no-drop".to_string(), important: false }]),
            "cursor-grab" => Some(vec![CssProperty { name: "cursor".to_string(), value: "grab".to_string(), important: false }]),
            "cursor-grabbing" => Some(vec![CssProperty { name: "cursor".to_string(), value: "grabbing".to_string(), important: false }]),
            "cursor-all-scroll" => Some(vec![CssProperty { name: "cursor".to_string(), value: "all-scroll".to_string(), important: false }]),
            "cursor-col-resize" => Some(vec![CssProperty { name: "cursor".to_string(), value: "col-resize".to_string(), important: false }]),
            "cursor-row-resize" => Some(vec![CssProperty { name: "cursor".to_string(), value: "row-resize".to_string(), important: false }]),
            "cursor-n-resize" => Some(vec![CssProperty { name: "cursor".to_string(), value: "n-resize".to_string(), important: false }]),
            "cursor-e-resize" => Some(vec![CssProperty { name: "cursor".to_string(), value: "e-resize".to_string(), important: false }]),
            "cursor-s-resize" => Some(vec![CssProperty { name: "cursor".to_string(), value: "s-resize".to_string(), important: false }]),
            "cursor-w-resize" => Some(vec![CssProperty { name: "cursor".to_string(), value: "w-resize".to_string(), important: false }]),
            "cursor-ne-resize" => Some(vec![CssProperty { name: "cursor".to_string(), value: "ne-resize".to_string(), important: false }]),
            "cursor-nw-resize" => Some(vec![CssProperty { name: "cursor".to_string(), value: "nw-resize".to_string(), important: false }]),
            "cursor-se-resize" => Some(vec![CssProperty { name: "cursor".to_string(), value: "se-resize".to_string(), important: false }]),
            "cursor-sw-resize" => Some(vec![CssProperty { name: "cursor".to_string(), value: "sw-resize".to_string(), important: false }]),
            "cursor-ew-resize" => Some(vec![CssProperty { name: "cursor".to_string(), value: "ew-resize".to_string(), important: false }]),
            "cursor-ns-resize" => Some(vec![CssProperty { name: "cursor".to_string(), value: "ns-resize".to_string(), important: false }]),
            "cursor-nesw-resize" => Some(vec![CssProperty { name: "cursor".to_string(), value: "nesw-resize".to_string(), important: false }]),
            "cursor-nwse-resize" => Some(vec![CssProperty { name: "cursor".to_string(), value: "nwse-resize".to_string(), important: false }]),
            "cursor-zoom-in" => Some(vec![CssProperty { name: "cursor".to_string(), value: "zoom-in".to_string(), important: false }]),
            "cursor-zoom-out" => Some(vec![CssProperty { name: "cursor".to_string(), value: "zoom-out".to_string(), important: false }]),
            _ => None,
        }
    }
    
    /// Parse user select classes
    fn parse_user_select_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "select-none" => Some(vec![CssProperty { name: "user-select".to_string(), value: "none".to_string(), important: false }]),
            "select-text" => Some(vec![CssProperty { name: "user-select".to_string(), value: "text".to_string(), important: false }]),
            "select-all" => Some(vec![CssProperty { name: "user-select".to_string(), value: "all".to_string(), important: false }]),
            "select-auto" => Some(vec![CssProperty { name: "user-select".to_string(), value: "auto".to_string(), important: false }]),
            _ => None,
        }
    }
    
    /// Parse resize classes
    fn parse_resize_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "resize-none" => Some(vec![CssProperty { name: "resize".to_string(), value: "none".to_string(), important: false }]),
            "resize-y" => Some(vec![CssProperty { name: "resize".to_string(), value: "vertical".to_string(), important: false }]),
            "resize-x" => Some(vec![CssProperty { name: "resize".to_string(), value: "horizontal".to_string(), important: false }]),
            "resize" => Some(vec![CssProperty { name: "resize".to_string(), value: "both".to_string(), important: false }]),
            _ => None,
        }
    }
    
    /// Parse scroll behavior classes
    fn parse_scroll_behavior_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "scroll-auto" => Some(vec![CssProperty { name: "scroll-behavior".to_string(), value: "auto".to_string(), important: false }]),
            "scroll-smooth" => Some(vec![CssProperty { name: "scroll-behavior".to_string(), value: "smooth".to_string(), important: false }]),
            _ => None,
        }
    }
    
    /// Parse scroll snap classes
    fn parse_scroll_snap_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "snap-start" => Some(vec![CssProperty { name: "scroll-snap-align".to_string(), value: "start".to_string(), important: false }]),
            "snap-end" => Some(vec![CssProperty { name: "scroll-snap-align".to_string(), value: "end".to_string(), important: false }]),
            "snap-center" => Some(vec![CssProperty { name: "scroll-snap-align".to_string(), value: "center".to_string(), important: false }]),
            "snap-align-none" => Some(vec![CssProperty { name: "scroll-snap-align".to_string(), value: "none".to_string(), important: false }]),
            "snap-normal" => Some(vec![CssProperty { name: "scroll-snap-align".to_string(), value: "normal".to_string(), important: false }]),
            "snap-always" => Some(vec![CssProperty { name: "scroll-snap-stop".to_string(), value: "always".to_string(), important: false }]),
            "snap-none" => Some(vec![CssProperty { name: "scroll-snap-stop".to_string(), value: "normal".to_string(), important: false }]),
            _ => None,
        }
    }
}

impl UtilityParser for InteractiveParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try pointer events classes first
        if let Some(properties) = self.parse_pointer_events_class(class) {
            return Some(properties);
        }
        
        // Try cursor classes
        if let Some(properties) = self.parse_cursor_class(class) {
            return Some(properties);
        }
        
        // Try user select classes
        if let Some(properties) = self.parse_user_select_class(class) {
            return Some(properties);
        }
        
        // Try resize classes
        if let Some(properties) = self.parse_resize_class(class) {
            return Some(properties);
        }
        
        // Try scroll behavior classes
        if let Some(properties) = self.parse_scroll_behavior_class(class) {
            return Some(properties);
        }
        
        // Try scroll snap classes
        if let Some(properties) = self.parse_scroll_snap_class(class) {
            return Some(properties);
        }
        
        None
    }
    
    fn get_supported_patterns(&self) -> Vec<&'static str> { 
        vec![
            "pointer-events-*", "cursor-*", "select-*", "resize-*", "scroll-*", "snap-*"
        ] 
    }
    fn get_priority(&self) -> u32 { 40 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Interactive }
}

impl Default for InteractiveParser {
    fn default() -> Self { Self::new() }
}
