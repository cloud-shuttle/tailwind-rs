//! Core Parser Methods for CssGenerator
//!
//! This module contains the core parsing methods for spacing, animation, color, typography, layout, borders, effects, and transforms.

// Removed unused imports
use super::types::CssProperty;
use super::parsers::{
    ColorParser, TypographyParser, LayoutParser, BorderParser, EffectsParser, 
    TransformParser, UtilityParser, InteractiveParser
};

/// Core parser methods for CssGenerator
pub trait CoreParsers {
    /// Parse spacing classes (padding, margin, etc.)
    fn parse_spacing_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse animation classes
    fn parse_animation_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse color classes
    fn parse_color_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse typography classes
    fn parse_typography_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse layout classes
    fn parse_layout_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse border classes
    fn parse_border_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse effects classes
    fn parse_effects_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse transform classes
    fn parse_transform_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse interactive classes
    fn parse_interactive_class(&self, class: &str) -> Option<Vec<CssProperty>>;
}

impl CoreParsers for super::CssGenerator {
    fn parse_spacing_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        self.spacing_parser.parse_class(class)
    }
    
    fn parse_animation_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        self.animation_parser.parse_class(class)
    }
    
    fn parse_color_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = ColorParser::new();
        parser.parse_class(class)
    }
    
    fn parse_typography_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = TypographyParser::new();
        parser.parse_class(class)
    }
    
    fn parse_layout_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = LayoutParser::new();
        parser.parse_class(class)
    }
    
    fn parse_border_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = BorderParser::new();
        parser.parse_class(class)
    }
    
    fn parse_effects_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = EffectsParser::new();
        parser.parse_class(class)
    }
    
    fn parse_transform_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = TransformParser::new();
        parser.parse_class(class)
    }
    
    fn parse_interactive_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = InteractiveParser::new();
        parser.parse_class(class)
    }
}
