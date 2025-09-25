//! Parser Module Exports
//! 
//! This module provides the main interface for all utility parsers.

pub mod spacing;
pub mod color;
pub mod typography;
pub mod layout;
pub mod flexbox;
pub mod grid;
pub mod borders;
pub mod effects;
pub mod transforms;
pub mod animations;

pub use spacing::SpacingParser;
pub use color::ColorParser;
pub use typography::TypographyParser;
pub use layout::LayoutParser;
pub use flexbox::FlexboxParser;
pub use grid::GridParser;
pub use borders::BorderParser;
pub use effects::EffectsParser;
pub use transforms::TransformParser;
pub use animations::AnimationParser;

/// Common parser interface for all utility parsers
pub trait UtilityParser {
    /// Parse a class and return CSS properties
    fn parse_class(&self, class: &str) -> Option<Vec<crate::css_generator::core::CssProperty>>;
    
    /// Get supported patterns for this parser
    fn get_supported_patterns(&self) -> Vec<&'static str>;
    
    /// Get parser priority (higher = more specific)
    fn get_priority(&self) -> u32;
    
    /// Get parser category
    fn get_category(&self) -> ParserCategory;
}

/// Parser categories for organization
#[derive(Debug, Clone, PartialEq)]
pub enum ParserCategory {
    Spacing,
    Color,
    Typography,
    Layout,
    Flexbox,
    Grid,
    Borders,
    Effects,
    Transforms,
    Animations,
}
