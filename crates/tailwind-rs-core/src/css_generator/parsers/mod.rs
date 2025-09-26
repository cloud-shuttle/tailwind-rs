//! Parser Module Exports
//! 
//! This module provides the main interface for all utility parsers.

pub mod spacing;
pub mod spacing_advanced;
pub mod color;
pub mod colors_advanced;
pub mod typography;
pub mod layout;
pub mod positioning;
pub mod sizing;
pub mod flexbox;
pub mod grid;
pub mod grid_advanced;
pub mod borders;
pub mod borders_advanced;
pub mod rings;
pub mod shadows;
pub mod svg;
pub mod margins;
pub mod groups;
pub mod effects;
pub mod transforms;
pub mod transitions;
pub mod animations;
pub mod interactive;
pub mod prose;
pub mod divide;
pub mod gradients;
pub mod object_fit;
pub mod arbitrary;
pub mod data_attributes;
pub mod background_properties;
pub mod transition_properties;
pub mod fractional_transforms;

pub use spacing::SpacingParser;
pub use spacing_advanced::AdvancedSpacingParser;
pub use color::ColorParser;
pub use colors_advanced::AdvancedColorParser;
pub use typography::TypographyParser;
pub use layout::LayoutParser;
pub use positioning::PositioningParser;
pub use sizing::SizingParser;
pub use flexbox::FlexboxParser;
pub use grid::GridParser;
pub use grid_advanced::AdvancedGridParser;
pub use borders::BorderParser;
pub use borders_advanced::AdvancedBorderParser;
pub use rings::RingParser;
pub use shadows::ShadowParser;
pub use svg::SvgParser;
pub use margins::MarginParser;
pub use groups::GroupParser;
pub use effects::EffectsParser;
pub use transforms::TransformParser;
pub use transitions::TransitionParser;
pub use animations::AnimationParser;
pub use interactive::InteractiveParser;
pub use prose::ProseParser;
pub use divide::DivideParser;
pub use gradients::GradientParser;
pub use object_fit::ObjectFitParser;
pub use arbitrary::ArbitraryParser;
pub use data_attributes::DataAttributeParser;
pub use background_properties::BackgroundPropertiesParser;
pub use transition_properties::TransitionPropertiesParser;
pub use fractional_transforms::FractionalTransformsParser;

/// Common parser interface for all utility parsers
pub trait UtilityParser {
    /// Parse a class and return CSS properties
    fn parse_class(&self, class: &str) -> Option<Vec<crate::css_generator::types::CssProperty>>;
    
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
    Positioning,
    Flexbox,
    Grid,
    Borders,
    Effects,
    Transforms,
    Animations,
    Interactive,
    Background,
    Transitions,
}
