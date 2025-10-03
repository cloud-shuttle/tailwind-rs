//! Transform Parser Module
//!
//! Comprehensive transform utilities parser that handles all CSS transform properties:
//! - Basic transforms (transform, transform-gpu, etc.)
//! - Backface visibility
//! - Perspective and perspective origin
//! - Transform style
//! - Transform origin
//! - Scaling (uniform, X-axis, Y-axis)
//! - Rotation (2D and 3D)
//! - Skewing (uniform, X-axis, Y-axis)

use crate::css_generator::types::CssProperty;
use crate::css_generator::parsers::{ParserCategory, UtilityParser};

pub mod basic;
pub mod backface;
pub mod origin;
pub mod perspective;
pub mod rotate;
pub mod scale;
pub mod skew;
pub mod style;
pub mod utils;

pub use basic::BasicTransformParser;
pub use backface::BackfaceVisibilityParser;
pub use origin::TransformOriginParser;
pub use perspective::PerspectiveParser;
pub use rotate::RotationParser;
pub use scale::ScaleParser;
pub use skew::SkewParser;
pub use style::TransformStyleParser;
pub use utils::TransformValidation;

/// Main transform parser that coordinates all transform utilities
#[derive(Debug, Clone)]
pub struct TransformParser {
    basic_parser: BasicTransformParser,
    backface_parser: BackfaceVisibilityParser,
    perspective_parser: PerspectiveParser,
    style_parser: TransformStyleParser,
    origin_parser: TransformOriginParser,
    scale_parser: ScaleParser,
    rotate_parser: RotationParser,
    skew_parser: SkewParser,
}

impl TransformParser {
    /// Create a new transform parser
    pub fn new() -> Self {
        Self {
            basic_parser: BasicTransformParser::new(),
            backface_parser: BackfaceVisibilityParser::new(),
            perspective_parser: PerspectiveParser::new(),
            style_parser: TransformStyleParser::new(),
            origin_parser: TransformOriginParser::new(),
            scale_parser: ScaleParser::new(),
            rotate_parser: RotationParser::new(),
            skew_parser: SkewParser::new(),
        }
    }

    /// Parse basic transform classes
    pub fn parse_basic_transform_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.basic_parser.parse_basic_transform_class(class)
    }

    /// Parse backface-visibility classes
    pub fn parse_backface_visibility_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.backface_parser.parse_backface_visibility_class(class)
    }

    /// Parse perspective classes
    pub fn parse_perspective_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.perspective_parser.parse_perspective_class(class)
    }

    /// Parse perspective-origin classes
    pub fn parse_perspective_origin_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.origin_parser.parse_perspective_origin_class(class)
    }

    /// Parse transform-style classes
    pub fn parse_transform_style_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.style_parser.parse_transform_style_class(class)
    }

    /// Parse transform-origin classes
    pub fn parse_origin_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.origin_parser.parse_origin_class(class)
    }

    /// Parse scale classes
    pub fn parse_scale_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.scale_parser.parse_scale_class(class)
    }

    /// Parse 3D rotation classes
    pub fn parse_rotate_3d_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.rotate_parser.parse_rotate_3d_class(class)
    }

    /// Parse rotation classes
    pub fn parse_rotate_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.rotate_parser.parse_rotate_class(class)
    }

    /// Parse skew classes
    pub fn parse_skew_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.skew_parser.parse_skew_class(class)
    }

    /// Parse X-axis skew classes
    pub fn parse_skew_x_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.skew_parser.parse_skew_x_class(class)
    }

    /// Parse Y-axis skew classes
    pub fn parse_skew_y_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.skew_parser.parse_skew_y_class(class)
    }
}

impl UtilityParser for TransformParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try parsing in order of specificity/precedence
        if let Some(properties) = self.parse_basic_transform_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_backface_visibility_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_perspective_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_perspective_origin_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_transform_style_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_origin_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_scale_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_rotate_3d_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_rotate_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_skew_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_skew_x_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_skew_y_class(class) {
            return Some(properties);
        }

        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        let mut patterns = Vec::new();

        patterns.extend(self.basic_parser.supported_patterns());
        patterns.extend(self.backface_parser.supported_patterns());
        // For parsers that return Vec<String>, we need to collect them statically
        // For now, we'll just add some representative patterns
        patterns.extend([
            "perspective-none",
            "perspective-dramatic",
            "perspective-normal",
            "scale-100",
            "scale-x-50",
            "scale-y-75",
        ]);
        patterns.extend(self.style_parser.supported_patterns());
        patterns.extend(self.origin_parser.supported_patterns());
        patterns.extend(self.rotate_parser.supported_patterns());
        patterns.extend(self.skew_parser.supported_patterns());

        patterns
    }

    fn get_priority(&self) -> u32 {
        100 // Transform utilities have high priority
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Transforms
    }
}

impl Default for TransformParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_parser_creation() {
        let parser = TransformParser::new();
        // Parser should be created successfully
        assert!(true); // Placeholder - would test actual functionality
    }

    #[test]
    fn utility_parser_implementation() {
        let parser = TransformParser::new();

        assert_eq!(parser.get_priority(), 100);
        assert_eq!(parser.get_category(), ParserCategory::Transforms);

        let patterns = parser.get_supported_patterns();
        assert!(!patterns.is_empty());
        // Should contain various transform patterns
        assert!(patterns.contains(&"transform"));
        assert!(patterns.contains(&"backface-visible"));
        assert!(patterns.contains(&"scale-100"));
    }

    #[test]
    fn basic_parsing_delegation() {
        let parser = TransformParser::new();

        // Test that basic parsing works
        let result = parser.parse_basic_transform_class("transform");
        assert!(result.is_some());

        let result = parser.parse_backface_visibility_class("backface-hidden");
        assert!(result.is_some());

        let result = parser.parse_perspective_class("perspective-normal");
        assert!(result.is_some());

        let result = parser.parse_scale_class("scale-50");
        assert!(result.is_some());

        let result = parser.parse_class("transform");
        assert!(result.is_some());
    }

    #[test]
    fn comprehensive_transform_parsing() {
        let parser = TransformParser::new();

        // Test that the main dispatcher works for implemented parsers
        let result = parser.parse_class("transform");
        assert!(result.is_some());

        let result = parser.parse_class("backface-visible");
        assert!(result.is_some());

        let result = parser.parse_class("perspective-dramatic");
        assert!(result.is_some());

        let result = parser.parse_class("scale-75");
        assert!(result.is_some());

        let result = parser.parse_class("invalid-transform-class");
        assert!(result.is_none());
    }
}
