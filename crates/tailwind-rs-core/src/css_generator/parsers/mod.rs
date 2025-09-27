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
pub mod border_radius_parser;
pub mod rings;
pub mod shadows;
pub mod svg;
pub mod margins;
pub mod groups;
pub mod effects;
pub mod filter_utilities;
pub mod backdrop_filter_utilities;
pub mod accessibility;
pub mod table;

pub use filter_utilities::FilterUtilitiesParser;
pub use backdrop_filter_utilities::BackdropFilterUtilitiesParser;
pub use accessibility::AccessibilityParser;
pub use table::TableParser;
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
pub mod aspect_ratio;
pub mod columns;
pub mod break_control;
pub mod box_utilities;
pub mod layout_utilities;
pub mod overflow;
pub mod overscroll;
pub mod position;
pub mod inset;
pub mod visibility;
pub mod z_index;
pub mod flex_basis;
pub mod flex_direction;
pub mod flex_wrap;
pub mod flex;
pub mod flex_grow;
pub mod flex_shrink;
pub mod order;
pub mod grid_template_columns;
pub mod grid_column;
pub mod grid_template_rows;
pub mod grid_row;
pub mod grid_auto_flow;
pub mod grid_auto_columns;
pub mod grid_auto_rows;
pub mod gap;
pub mod justify_content;
pub mod justify_items;
pub mod justify_self;
pub mod align_content;
pub mod align_items;
pub mod align_self;
pub mod place_content;
pub mod place_items;
pub mod place_self;
pub mod background;
pub mod border_utilities;
pub mod effects_utilities;
pub mod mask_utilities;

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
pub use aspect_ratio::AspectRatioParser;
pub use columns::ColumnsParser;
pub use break_control::BreakControlParser;
pub use box_utilities::BoxUtilitiesParser;
pub use layout_utilities::LayoutUtilitiesParser;
pub use overflow::OverflowParser;
pub use overscroll::OverscrollParser;
pub use position::PositionParser;
pub use inset::InsetParser;
pub use visibility::VisibilityParser;
pub use z_index::ZIndexParser;
pub use flex_basis::FlexBasisParser;
pub use flex_direction::FlexDirectionParser;
pub use flex_wrap::FlexWrapParser;
pub use flex::FlexParser;
pub use flex_grow::FlexGrowParser;
pub use flex_shrink::FlexShrinkParser;
pub use order::OrderParser;
pub use grid_template_columns::GridTemplateColumnsParser;
pub use grid_column::GridColumnParser;
pub use grid_template_rows::GridTemplateRowsParser;
pub use grid_row::GridRowParser;
pub use grid_auto_flow::GridAutoFlowParser;
pub use grid_auto_columns::GridAutoColumnsParser;
pub use grid_auto_rows::GridAutoRowsParser;
pub use gap::GapParser;
pub use justify_content::JustifyContentParser;
pub use justify_items::JustifyItemsParser;
pub use justify_self::JustifySelfParser;
pub use align_content::AlignContentParser;
pub use align_items::AlignItemsParser;
pub use align_self::AlignSelfParser;
pub use place_content::PlaceContentParser;
pub use place_items::PlaceItemsParser;
pub use place_self::PlaceSelfParser;
pub use background::BackgroundParser;
pub use border_utilities::BorderUtilitiesParser;
pub use effects_utilities::EffectsUtilitiesParser;
pub use mask_utilities::MaskUtilitiesParser;

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
    Accessibility,
}
