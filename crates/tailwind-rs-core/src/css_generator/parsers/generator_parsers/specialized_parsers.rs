//! Specialized parser methods for CssGenerator
//! 
//! This module contains all the specialized parser methods that delegate
//! to specific parser modules.

use crate::css_generator::types::CssProperty;
use crate::css_generator::parsers::{
    AlignContentParser, AlignItemsParser, AlignSelfParser, BorderParser, ColorParser,
    EffectsParser, FlexBasisParser, FlexDirectionParser, FlexGrowParser, FlexParser,
    FlexShrinkParser, FlexWrapParser, FlexboxParser, GapParser, GridAutoColumnsParser,
    GridAutoFlowParser, GridAutoRowsParser, GridColumnParser, GridParser, GridRowParser,
    GridTemplateColumnsParser, GridTemplateRowsParser, InsetParser, JustifyContentParser,
    JustifyItemsParser, JustifySelfParser, LayoutParser, OrderParser, OverflowParser,
    OverscrollParser, PlaceContentParser, PlaceItemsParser, PlaceSelfParser, PositionParser,
    TransformParser, TypographyParser, UtilityParser, VisibilityParser, ZIndexParser,
};

/// Specialized parser methods for CssGenerator
impl crate::css_generator::CssGenerator {
    // Basic parser methods - delegate to the actual parser modules
    pub fn parse_spacing_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.spacing_parser.parse_class(class)
    }

    pub fn parse_animation_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.animation_parser.parse_class(class)
    }

    pub fn parse_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = ColorParser::new();
        parser.parse_class(class)
    }

    pub fn parse_typography_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = TypographyParser::new();
        parser.parse_class(class)
    }

    pub fn parse_layout_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = LayoutParser::new();
        parser.parse_class(class)
    }

    pub fn parse_flexbox_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = FlexboxParser::new();
        parser.parse_class(class)
    }

    pub fn parse_grid_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = GridParser::new();
        parser.parse_class(class)
    }

    pub fn parse_border_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = BorderParser::new();
        parser.parse_class(class)
    }

    pub fn parse_effects_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = EffectsParser::new();
        parser.parse_class(class)
    }

    pub fn parse_transform_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = TransformParser::new();
        parser.parse_class(class)
    }

    pub fn parse_interactive_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.interactive_parser.parse_class(class)
    }

    pub fn parse_sizing_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }

    pub fn parse_background_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }

    pub fn parse_filter_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }

    pub fn parse_transition_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }

    // New layout parser methods
    pub fn parse_overflow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = OverflowParser::new();
        parser.parse_class(class)
    }

    pub fn parse_overscroll_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = OverscrollParser::new();
        parser.parse_class(class)
    }

    pub fn parse_position_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = PositionParser::new();
        parser.parse_class(class)
    }

    pub fn parse_inset_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = InsetParser::new();
        parser.parse_class(class)
    }

    pub fn parse_visibility_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = VisibilityParser::new();
        parser.parse_class(class)
    }

    pub fn parse_z_index_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = ZIndexParser::new();
        parser.parse_class(class)
    }

    // New flexbox parser methods
    pub fn parse_flex_basis_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = FlexBasisParser::new();
        parser.parse_class(class)
    }

    pub fn parse_flex_direction_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = FlexDirectionParser::new();
        parser.parse_class(class)
    }

    pub fn parse_flex_wrap_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = FlexWrapParser::new();
        parser.parse_class(class)
    }

    pub fn parse_flex_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = FlexParser::new();
        parser.parse_class(class)
    }

    pub fn parse_flex_grow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = FlexGrowParser::new();
        parser.parse_class(class)
    }

    pub fn parse_flex_shrink_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = FlexShrinkParser::new();
        parser.parse_class(class)
    }

    // New order and grid parser methods
    pub fn parse_order_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = OrderParser::new();
        parser.parse_class(class)
    }

    pub fn parse_grid_template_columns_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = GridTemplateColumnsParser::new();
        parser.parse_class(class)
    }

    pub fn parse_grid_column_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = GridColumnParser::new();
        parser.parse_class(class)
    }

    pub fn parse_grid_template_rows_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = GridTemplateRowsParser::new();
        parser.parse_class(class)
    }

    pub fn parse_grid_row_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = GridRowParser::new();
        parser.parse_class(class)
    }

    pub fn parse_grid_auto_flow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = GridAutoFlowParser::new();
        parser.parse_class(class)
    }

    pub fn parse_grid_auto_columns_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = GridAutoColumnsParser::new();
        parser.parse_class(class)
    }

    pub fn parse_grid_auto_rows_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = GridAutoRowsParser::new();
        parser.parse_class(class)
    }

    // New gap and justify parser methods
    pub fn parse_gap_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = GapParser::new();
        parser.parse_class(class)
    }

    pub fn parse_justify_content_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = JustifyContentParser::new();
        parser.parse_class(class)
    }

    pub fn parse_justify_items_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = JustifyItemsParser::new();
        parser.parse_class(class)
    }

    pub fn parse_justify_self_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = JustifySelfParser::new();
        parser.parse_class(class)
    }

    // New align and place parser methods
    pub fn parse_align_content_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = AlignContentParser::new();
        parser.parse_class(class)
    }

    pub fn parse_align_items_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = AlignItemsParser::new();
        parser.parse_class(class)
    }

    pub fn parse_align_self_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = AlignSelfParser::new();
        parser.parse_class(class)
    }

    pub fn parse_place_content_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = PlaceContentParser::new();
        parser.parse_class(class)
    }

    pub fn parse_place_items_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = PlaceItemsParser::new();
        parser.parse_class(class)
    }

    pub fn parse_place_self_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = PlaceSelfParser::new();
        parser.parse_class(class)
    }
}
