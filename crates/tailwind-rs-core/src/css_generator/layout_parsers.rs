//! Layout Parser Methods for CssGenerator
//!
//! This module contains layout-related parsing methods for overflow, overscroll, position, inset, visibility, and z-index.

// Removed unused imports
use super::parsers::{
    InsetParser, OverflowParser, OverscrollParser, PositionParser, VisibilityParser, ZIndexParser,
};
use super::types::CssProperty;

/// Layout parser methods for CssGenerator
pub trait LayoutParsers {
    /// Parse overflow classes
    fn parse_overflow_class(&self, class: &str) -> Option<Vec<CssProperty>>;

    /// Parse overscroll classes
    fn parse_overscroll_class(&self, class: &str) -> Option<Vec<CssProperty>>;

    /// Parse position classes
    fn parse_position_class(&self, class: &str) -> Option<Vec<CssProperty>>;

    /// Parse inset classes
    fn parse_inset_class(&self, class: &str) -> Option<Vec<CssProperty>>;

    /// Parse visibility classes
    fn parse_visibility_class(&self, class: &str) -> Option<Vec<CssProperty>>;

    /// Parse z-index classes
    fn parse_z_index_class(&self, class: &str) -> Option<Vec<CssProperty>>;
}

impl LayoutParsers for super::CssGenerator {
    fn parse_overflow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        use super::parsers::UtilityParser;
        let parser = OverflowParser::new();
        parser.parse_class(class)
    }

    fn parse_overscroll_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        use super::parsers::UtilityParser;
        let parser = OverscrollParser::new();
        parser.parse_class(class)
    }

    fn parse_position_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        use super::parsers::UtilityParser;
        let parser = PositionParser::new();
        parser.parse_class(class)
    }

    fn parse_inset_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        use super::parsers::UtilityParser;
        let parser = InsetParser::new();
        parser.parse_class(class)
    }

    fn parse_visibility_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        use super::parsers::UtilityParser;
        let parser = VisibilityParser::new();
        parser.parse_class(class)
    }

    fn parse_z_index_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        use super::parsers::UtilityParser;
        let parser = ZIndexParser::new();
        parser.parse_class(class)
    }
}
