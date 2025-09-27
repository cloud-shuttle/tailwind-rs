//! Flexbox Parser Methods for CssGenerator
//!
//! This module contains flexbox-related parsing methods for flex basis, direction, wrap, flex, grow, shrink, and order.

// Removed unused imports
use super::types::CssProperty;
use super::parsers::{
    FlexBasisParser, FlexDirectionParser, FlexWrapParser, FlexParser, 
    FlexGrowParser, FlexShrinkParser, OrderParser
};

/// Flexbox parser methods for CssGenerator
pub trait FlexboxParsers {
    /// Parse flex basis classes
    fn parse_flex_basis_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse flex direction classes
    fn parse_flex_direction_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse flex wrap classes
    fn parse_flex_wrap_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse flex classes
    fn parse_flex_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse flex grow classes
    fn parse_flex_grow_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse flex shrink classes
    fn parse_flex_shrink_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse order classes
    fn parse_order_class(&self, class: &str) -> Option<Vec<CssProperty>>;
}

impl FlexboxParsers for super::CssGenerator {
    fn parse_flex_basis_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = FlexBasisParser::new();
        parser.parse_class(class)
    }
    
    fn parse_flex_direction_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = FlexDirectionParser::new();
        parser.parse_class(class)
    }
    
    fn parse_flex_wrap_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = FlexWrapParser::new();
        parser.parse_class(class)
    }
    
    fn parse_flex_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = FlexParser::new();
        parser.parse_class(class)
    }
    
    fn parse_flex_grow_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = FlexGrowParser::new();
        parser.parse_class(class)
    }
    
    fn parse_flex_shrink_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = FlexShrinkParser::new();
        parser.parse_class(class)
    }
    
    fn parse_order_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = OrderParser::new();
        parser.parse_class(class)
    }
}
