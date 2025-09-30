//! Parser Module Exports
//!
//! This module provides the main interface for all utility parsers.

pub mod accessibility;
// pub mod generator_parsers; // Temporarily disabled
pub mod backdrop_filter_utilities;
pub mod border_radius_parser;
pub mod borders;
pub mod borders_advanced;
pub mod color;
pub mod colors_advanced;
pub mod effects_modules;
pub mod filter_utilities_modules;
pub mod flexbox;
pub mod gap_parser;
pub mod grid;
pub mod grid_advanced;
pub mod groups;
pub mod layout;
pub mod margin_parser;
pub mod margins;
pub mod padding_parser;
pub mod positioning;
pub mod rings;
pub mod shadows;
pub mod sizing;
pub mod spacing;
pub mod spacing_advanced;
pub mod svg;
pub mod table;
pub mod typography;

pub use accessibility::AccessibilityParser;
pub use backdrop_filter_utilities::BackdropFilterUtilitiesParser;
pub use filter_utilities_modules::FilterUtilitiesParser;
pub use table::TableParser;
pub mod accent_color;
pub mod align_content;
pub mod align_items;
pub mod align_self;
pub mod animations;
pub mod arbitrary;
pub mod aspect_ratio;
pub mod basic_transforms;
pub mod scale_parser;
pub mod background_properties;
pub mod border_utilities;
pub mod box_utilities;
pub mod break_control;
pub mod columns;
pub mod data_attributes;
pub mod divide;
pub mod effects_utilities_modules;
pub mod flex;
pub mod flex_basis;
pub mod flex_direction;
pub mod flex_grow;
pub mod flex_shrink;
pub mod flex_wrap;
pub mod fractional_transforms;
pub mod gap;
pub mod gradients;
pub mod grid_auto_columns;
pub mod grid_auto_flow;
pub mod grid_auto_rows;
pub mod grid_column;
pub mod grid_row;
pub mod grid_template_columns;
pub mod grid_template_rows;
pub mod inset_utilities;
pub mod interactive;
pub mod justify_content;
pub mod justify_items;
pub mod justify_self;
pub mod layout_utilities;
pub mod mask_image_parser;
pub mod mask_properties_parser;
pub mod mask_utilities;
pub mod object_fit;
pub mod order;
pub mod overflow;
pub mod overscroll;
pub mod place_content;
pub mod place_items;
pub mod place_self;
pub mod position;
pub mod prose;
pub mod transition_properties;
pub mod transitions;
pub mod visibility;
pub mod z_index;

pub use accent_color::AccentColorParser;
pub use align_content::AlignContentParser;
pub use align_items::AlignItemsParser;
pub use align_self::AlignSelfParser;
pub use animations::AnimationParser;
pub use arbitrary::ArbitraryParser;
pub use aspect_ratio::AspectRatioParser;
pub use background_properties::BackgroundPropertiesParser;
pub use border_utilities::BorderUtilitiesParser;
pub use border_utilities::OutlineParser;
pub use borders::BorderParser;
pub use borders_advanced::AdvancedBorderParser;
pub use box_utilities::BoxUtilitiesParser;
pub use break_control::BreakControlParser;
pub use color::ColorParser;
pub use colors_advanced::AdvancedColorParser;
pub use columns::ColumnsParser;
pub use data_attributes::DataAttributeParser;
pub use divide::DivideParser;
pub use effects_modules::EffectsParser;
pub use effects_utilities_modules::EffectsParser as EffectsUtilitiesParser;
pub use flex::FlexParser;
pub use flex_basis::FlexBasisParser;
pub use flex_direction::FlexDirectionParser;
pub use flex_grow::FlexGrowParser;
pub use flex_shrink::FlexShrinkParser;
pub use flex_wrap::FlexWrapParser;
pub use flexbox::FlexboxParser;
pub use fractional_transforms::FractionalTransformsParser;
pub use gap::GapParser;
pub use gradients::GradientParser;
pub use grid::GridParser;
pub use grid_advanced::AdvancedGridParser;
pub use grid_auto_columns::GridAutoColumnsParser;
pub use grid_auto_flow::GridAutoFlowParser;
pub use grid_auto_rows::GridAutoRowsParser;
pub use grid_column::GridColumnParser;
pub use grid_row::GridRowParser;
pub use grid_template_columns::GridTemplateColumnsParser;
pub use grid_template_rows::GridTemplateRowsParser;
pub use groups::GroupParser;
pub use inset_utilities::InsetParser;
pub use interactive::InteractiveParser;
pub use justify_content::JustifyContentParser;
pub use justify_items::JustifyItemsParser;
pub use justify_self::JustifySelfParser;
pub use layout::LayoutParser;
pub use layout_utilities::LayoutUtilitiesParser;
pub use margins::MarginParser;
pub use mask_utilities::MaskUtilitiesParser;
pub use object_fit::ObjectFitParser;
pub use order::OrderParser;
pub use overflow::OverflowParser;
pub use overscroll::OverscrollParser;
pub use place_content::PlaceContentParser;
pub use place_items::PlaceItemsParser;
pub use place_self::PlaceSelfParser;
pub use position::PositionParser;
pub use positioning::PositioningParser;
pub use prose::ProseParser;
pub use rings::RingParser;
pub use shadows::ShadowParser;
pub use border_radius_parser::BorderRadiusParser;
pub use sizing::SizingParser;
pub use spacing::SpacingParser;
pub use spacing_advanced::AdvancedSpacingParser;
pub use svg::SvgParser;
pub use basic_transforms::BasicTransformsParser;
pub use scale_parser::ScaleParser;
pub use transition_properties::TransitionPropertiesParser;
pub use transitions::TransitionParser;
pub use typography::TypographyParser;
pub use visibility::VisibilityParser;
pub use z_index::ZIndexParser;

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
