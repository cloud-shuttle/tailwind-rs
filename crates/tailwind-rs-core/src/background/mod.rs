//! Background Parser Module
//!
//! Comprehensive background utilities parser that handles all background-related
//! CSS properties including colors, gradients, positioning, and more.

use crate::css_generator::types::CssProperty;
use super::super::css_generator::parsers::{ParserCategory, UtilityParser};

pub mod attachment;
pub mod clip;
pub mod color;
pub mod gradient;
pub mod image;
pub mod origin;
pub mod position;
pub mod repeat;
pub mod size;
pub mod utils;

pub use attachment::BackgroundAttachmentParser;
pub use clip::BackgroundClipParser;
pub use color::BackgroundColorParser;
pub use gradient::BackgroundGradientParser;
pub use image::BackgroundImageParser;
pub use origin::BackgroundOriginParser;
pub use position::BackgroundPositionParser;
pub use repeat::BackgroundRepeatParser;
pub use size::BackgroundSizeParser;
pub use utils::BackgroundColorUtils;

/// Main background parser that coordinates all background utilities
#[derive(Debug, Clone)]
pub struct BackgroundParser {
    attachment_parser: BackgroundAttachmentParser,
    clip_parser: BackgroundClipParser,
    color_parser: BackgroundColorParser,
    gradient_parser: BackgroundGradientParser,
    image_parser: BackgroundImageParser,
    origin_parser: BackgroundOriginParser,
    position_parser: BackgroundPositionParser,
    repeat_parser: BackgroundRepeatParser,
    size_parser: BackgroundSizeParser,
}

impl BackgroundParser {
    /// Create a new background parser
    pub fn new() -> Self {
        Self {
            attachment_parser: BackgroundAttachmentParser::new(),
            clip_parser: BackgroundClipParser::new(),
            color_parser: BackgroundColorParser::new(),
            gradient_parser: BackgroundGradientParser::new(),
            image_parser: BackgroundImageParser::new(),
            origin_parser: BackgroundOriginParser::new(),
            position_parser: BackgroundPositionParser::new(),
            repeat_parser: BackgroundRepeatParser::new(),
            size_parser: BackgroundSizeParser::new(),
        }
    }

    /// Parse background-attachment classes
    pub fn parse_background_attachment_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.attachment_parser.parse_attachment_class(class)
    }

    /// Parse background-clip classes
    pub fn parse_background_clip_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.clip_parser.parse_clip_class(class)
    }

    /// Parse background gradient direction classes
    pub fn parse_background_gradient_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.gradient_parser.parse_gradient_class(class)
    }

    /// Parse background-color classes
    pub fn parse_background_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.color_parser.parse_color_class(class)
    }

    /// Parse background-image classes
    pub fn parse_background_image_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.image_parser.parse_image_class(class)
    }

    /// Parse background-origin classes
    pub fn parse_background_origin_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.origin_parser.parse_origin_class(class)
    }

    /// Parse background-position classes
    pub fn parse_background_position_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.position_parser.parse_position_class(class)
    }

    /// Parse background-repeat classes
    pub fn parse_background_repeat_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.repeat_parser.parse_repeat_class(class)
    }

    /// Parse background-size classes
    pub fn parse_background_size_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.size_parser.parse_size_class(class)
    }

    /// Get color value for background color classes (convenience method)
    pub fn get_color_value(&self, class: &str) -> Option<String> {
        BackgroundColorUtils::get_color_value(class)
    }
}

impl UtilityParser for BackgroundParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try parsing in order of specificity/precedence
        if let Some(properties) = self.parse_background_attachment_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_background_clip_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_background_gradient_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_background_color_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_background_image_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_background_origin_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_background_position_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_background_repeat_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_background_size_class(class) {
            return Some(properties);
        }

        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        let mut patterns = Vec::new();

        patterns.extend(self.attachment_parser.supported_patterns());
        patterns.extend(self.clip_parser.supported_patterns());
        patterns.extend(self.color_parser.supported_patterns().iter().map(|s| s.as_str()));
        patterns.extend(self.gradient_parser.supported_patterns());
        patterns.extend(self.image_parser.supported_patterns());
        patterns.extend(self.origin_parser.supported_patterns());
        patterns.extend(self.position_parser.supported_patterns());
        patterns.extend(self.repeat_parser.supported_patterns());
        patterns.extend(self.size_parser.supported_patterns());

        patterns
    }

    fn get_priority(&self) -> u32 {
        90 // Background utilities have medium-high priority
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Background
    }
}

impl Default for BackgroundParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn background_parser_creation() {
        let parser = BackgroundParser::new();
        // Parser should be created successfully
        assert!(true); // Placeholder - would test actual functionality
    }

    #[test]
    fn utility_parser_implementation() {
        let parser = BackgroundParser::new();

        assert_eq!(parser.get_priority(), 90);
        assert_eq!(parser.get_category(), ParserCategory::Background);

        let patterns = parser.get_supported_patterns();
        assert!(!patterns.is_empty());
        // Should contain various background patterns
        assert!(patterns.contains(&"bg-fixed"));
        assert!(patterns.contains(&"bg-clip-border"));
        assert!(patterns.contains(&"bg-gradient-to-r"));
    }

    #[test]
    fn comprehensive_background_parsing() {
        let parser = BackgroundParser::new();

        // Test that the main dispatcher works
        let result = parser.parse_class("bg-fixed");
        assert!(result.is_some());

        let result = parser.parse_class("bg-clip-border");
        assert!(result.is_some());

        let result = parser.parse_class("bg-gradient-to-r");
        assert!(result.is_some());

        let result = parser.parse_class("bg-white");
        assert!(result.is_some());

        let result = parser.parse_class("bg-invalid");
        assert!(result.is_none());
    }

    #[test]
    fn individual_parser_delegation() {
        let parser = BackgroundParser::new();

        // Test delegation to individual parsers
        let result = parser.parse_background_attachment_class("bg-fixed");
        assert!(result.is_some());

        let result = parser.parse_background_clip_class("bg-clip-border");
        assert!(result.is_some());

        let result = parser.parse_background_gradient_class("bg-gradient-to-r");
        assert!(result.is_some());

        let result = parser.parse_background_color_class("bg-white");
        assert!(result.is_some());
    }
}
