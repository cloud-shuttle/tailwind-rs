//! Interactivity utilities for Tailwind-RS
//! 
//! This module provides utilities for handling interactive elements like hover, focus, and active states.

use crate::css_generator::types::CssProperty;

/// Parser for interactivity utilities
#[derive(Debug, Clone)]
pub struct InteractivityParser;

impl InteractivityParser {
    /// Create a new InteractivityParser
    pub fn new() -> Self {
        Self
    }

    /// Parse interactivity classes
    pub fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            // Cursor utilities
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

            // Resize utilities
            "resize-none" => Some(vec![CssProperty { name: "resize".to_string(), value: "none".to_string(), important: false }]),
            "resize-y" => Some(vec![CssProperty { name: "resize".to_string(), value: "vertical".to_string(), important: false }]),
            "resize-x" => Some(vec![CssProperty { name: "resize".to_string(), value: "horizontal".to_string(), important: false }]),
            "resize" => Some(vec![CssProperty { name: "resize".to_string(), value: "both".to_string(), important: false }]),

            // User select utilities
            "select-none" => Some(vec![CssProperty { name: "user-select".to_string(), value: "none".to_string(), important: false }]),
            "select-text" => Some(vec![CssProperty { name: "user-select".to_string(), value: "text".to_string(), important: false }]),
            "select-all" => Some(vec![CssProperty { name: "user-select".to_string(), value: "all".to_string(), important: false }]),
            "select-auto" => Some(vec![CssProperty { name: "user-select".to_string(), value: "auto".to_string(), important: false }]),

            // Scroll behavior utilities
            "scroll-auto" => Some(vec![CssProperty { name: "scroll-behavior".to_string(), value: "auto".to_string(), important: false }]),
            "scroll-smooth" => Some(vec![CssProperty { name: "scroll-behavior".to_string(), value: "smooth".to_string(), important: false }]),

            // Touch action utilities
            "touch-auto" => Some(vec![CssProperty { name: "touch-action".to_string(), value: "auto".to_string(), important: false }]),
            "touch-none" => Some(vec![CssProperty { name: "touch-action".to_string(), value: "none".to_string(), important: false }]),
            "touch-pan-x" => Some(vec![CssProperty { name: "touch-action".to_string(), value: "pan-x".to_string(), important: false }]),
            "touch-pan-left" => Some(vec![CssProperty { name: "touch-action".to_string(), value: "pan-left".to_string(), important: false }]),
            "touch-pan-right" => Some(vec![CssProperty { name: "touch-action".to_string(), value: "pan-right".to_string(), important: false }]),
            "touch-pan-y" => Some(vec![CssProperty { name: "touch-action".to_string(), value: "pan-y".to_string(), important: false }]),
            "touch-pan-up" => Some(vec![CssProperty { name: "touch-action".to_string(), value: "pan-up".to_string(), important: false }]),
            "touch-pan-down" => Some(vec![CssProperty { name: "touch-action".to_string(), value: "pan-down".to_string(), important: false }]),
            "touch-pinch-zoom" => Some(vec![CssProperty { name: "touch-action".to_string(), value: "pinch-zoom".to_string(), important: false }]),
            "touch-manipulation" => Some(vec![CssProperty { name: "touch-action".to_string(), value: "manipulation".to_string(), important: false }]),

            // Will change utilities
            "will-change-auto" => Some(vec![CssProperty { name: "will-change".to_string(), value: "auto".to_string(), important: false }]),
            "will-change-scroll" => Some(vec![CssProperty { name: "will-change".to_string(), value: "scroll-position".to_string(), important: false }]),
            "will-change-contents" => Some(vec![CssProperty { name: "will-change".to_string(), value: "contents".to_string(), important: false }]),
            "will-change-transform" => Some(vec![CssProperty { name: "will-change".to_string(), value: "transform".to_string(), important: false }]),

            _ => None,
        }
    }
}
