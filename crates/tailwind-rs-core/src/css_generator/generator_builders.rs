//! CSS Generator Builder Methods
//!
//! This module contains the constructor and initialization methods for CssGenerator.

use crate::responsive::Breakpoint;
use std::collections::HashMap;
use super::types::CssGenerationConfig;
use super::parsers::{
    SpacingParser, AnimationParser, InteractiveParser,
    AdvancedSpacingParser, AdvancedColorParser, PositioningParser, TypographyParser,
    FlexboxParser, LayoutParser, ColorParser, EffectsParser, SizingParser,
    AdvancedBorderParser, RingParser, TransitionParser, ShadowParser, SvgParser,
    MarginParser, GroupParser, AdvancedGridParser, ProseParser, DivideParser,
    GradientParser, ObjectFitParser, TransformParser, ArbitraryParser, DataAttributeParser, 
    BackgroundPropertiesParser, TransitionPropertiesParser, FractionalTransformsParser,
    AspectRatioParser, ColumnsParser, BreakControlParser, BoxUtilitiesParser, LayoutUtilitiesParser,
    OverflowParser, OverscrollParser, PositionParser, InsetParser, VisibilityParser, ZIndexParser,
    FlexBasisParser, FlexDirectionParser, FlexWrapParser, FlexParser, FlexGrowParser, FlexShrinkParser,
    OrderParser, GridTemplateColumnsParser, GridColumnParser, GridTemplateRowsParser, GridRowParser,
    GridAutoFlowParser, GridAutoColumnsParser, GridAutoRowsParser, GapParser, JustifyContentParser,
    JustifyItemsParser, JustifySelfParser, AlignContentParser, AlignItemsParser, AlignSelfParser,
    PlaceContentParser, PlaceItemsParser, PlaceSelfParser, BackgroundParser, BorderUtilitiesParser,
    EffectsUtilitiesParser, FilterUtilitiesParser, BackdropFilterUtilitiesParser, AccessibilityParser, TableParser, MaskUtilitiesParser
};
use super::variants::VariantParser;

/// Builder methods for CssGenerator
pub trait CssGeneratorBuilder {
    /// Create a new CSS generator
    fn new() -> Self;
    
    /// Create a new CSS generator with custom configuration
    fn with_config(config: CssGenerationConfig) -> Self;
    
    /// Initialize default breakpoints
    fn initialize_default_breakpoints(&mut self);
    
    /// Initialize custom breakpoints from config
    fn initialize_custom_breakpoints(&mut self);
}

impl CssGeneratorBuilder for super::CssGenerator {
    fn new() -> Self {
        let mut generator = Self {
            rules: HashMap::new(),
            breakpoints: HashMap::new(),
            custom_properties: HashMap::new(),
            config: CssGenerationConfig::default(),
            spacing_parser: SpacingParser::new(),
            advanced_spacing_parser: AdvancedSpacingParser::new(),
            color_parser: ColorParser::new(),
            advanced_color_parser: AdvancedColorParser::new(),
            typography_parser: TypographyParser::new(),
            layout_parser: LayoutParser::new(),
            positioning_parser: PositioningParser::new(),
            flexbox_parser: FlexboxParser::new(),
            effects_parser: EffectsParser::new(),
            sizing_parser: SizingParser::new(),
            advanced_border_parser: AdvancedBorderParser::new(),
            ring_parser: RingParser::new(),
            transition_parser: TransitionParser::new(),
            shadow_parser: ShadowParser::new(),
            svg_parser: SvgParser::new(),
            margin_parser: MarginParser::new(),
            group_parser: GroupParser::new(),
            advanced_grid_parser: AdvancedGridParser::new(),
            animation_parser: AnimationParser::new(),
            interactive_parser: InteractiveParser::new(),
            prose_parser: ProseParser::new(),
            divide_parser: DivideParser::new(),
            gradient_parser: GradientParser::new(),
            object_fit_parser: ObjectFitParser::new(),
            transform_parser: TransformParser::new(),
            arbitrary_parser: ArbitraryParser::new(),
            data_attribute_parser: DataAttributeParser::new(),
            background_properties_parser: BackgroundPropertiesParser::new(),
            transition_properties_parser: TransitionPropertiesParser::new(),
            fractional_transforms_parser: FractionalTransformsParser::new(),
            aspect_ratio_parser: AspectRatioParser::new(),
            columns_parser: ColumnsParser::new(),
            break_control_parser: BreakControlParser::new(),
            box_utilities_parser: BoxUtilitiesParser::new(),
            layout_utilities_parser: LayoutUtilitiesParser::new(),
            overflow_parser: OverflowParser::new(),
            overscroll_parser: OverscrollParser::new(),
            position_parser: PositionParser::new(),
            inset_parser: InsetParser::new(),
            visibility_parser: VisibilityParser::new(),
            z_index_parser: ZIndexParser::new(),
            flex_basis_parser: FlexBasisParser::new(),
            flex_direction_parser: FlexDirectionParser::new(),
            flex_wrap_parser: FlexWrapParser::new(),
            flex_parser: FlexParser::new(),
            flex_grow_parser: FlexGrowParser::new(),
            flex_shrink_parser: FlexShrinkParser::new(),
            order_parser: OrderParser::new(),
            grid_template_columns_parser: GridTemplateColumnsParser::new(),
            grid_column_parser: GridColumnParser::new(),
            grid_template_rows_parser: GridTemplateRowsParser::new(),
            grid_row_parser: GridRowParser::new(),
            grid_auto_flow_parser: GridAutoFlowParser::new(),
            grid_auto_columns_parser: GridAutoColumnsParser::new(),
            grid_auto_rows_parser: GridAutoRowsParser::new(),
            gap_parser: GapParser::new(),
            justify_content_parser: JustifyContentParser::new(),
            justify_items_parser: JustifyItemsParser::new(),
            justify_self_parser: JustifySelfParser::new(),
            align_content_parser: AlignContentParser::new(),
            align_items_parser: AlignItemsParser::new(),
            align_self_parser: AlignSelfParser::new(),
            place_content_parser: PlaceContentParser::new(),
            place_items_parser: PlaceItemsParser::new(),
            place_self_parser: PlaceSelfParser::new(),
            background_parser: BackgroundParser::new(),
            border_utilities_parser: BorderUtilitiesParser::new(),
            effects_utilities_parser: EffectsUtilitiesParser::new(),
            filter_utilities_parser: FilterUtilitiesParser::new(),
            backdrop_filter_utilities_parser: BackdropFilterUtilitiesParser::new(),
            accessibility_parser: AccessibilityParser::new(),
            table_parser: TableParser::new(),
            mask_utilities_parser: MaskUtilitiesParser::new(),
            variant_parser: VariantParser::new(),
        };
        
        generator.initialize_default_breakpoints();
        generator
    }

    fn with_config(config: CssGenerationConfig) -> Self {
        let mut generator = Self {
            rules: HashMap::new(),
            breakpoints: HashMap::new(),
            custom_properties: HashMap::new(),
            config,
            spacing_parser: SpacingParser::new(),
            advanced_spacing_parser: AdvancedSpacingParser::new(),
            color_parser: ColorParser::new(),
            advanced_color_parser: AdvancedColorParser::new(),
            typography_parser: TypographyParser::new(),
            layout_parser: LayoutParser::new(),
            positioning_parser: PositioningParser::new(),
            flexbox_parser: FlexboxParser::new(),
            effects_parser: EffectsParser::new(),
            sizing_parser: SizingParser::new(),
            advanced_border_parser: AdvancedBorderParser::new(),
            ring_parser: RingParser::new(),
            transition_parser: TransitionParser::new(),
            shadow_parser: ShadowParser::new(),
            svg_parser: SvgParser::new(),
            margin_parser: MarginParser::new(),
            group_parser: GroupParser::new(),
            advanced_grid_parser: AdvancedGridParser::new(),
            animation_parser: AnimationParser::new(),
            interactive_parser: InteractiveParser::new(),
            prose_parser: ProseParser::new(),
            divide_parser: DivideParser::new(),
            gradient_parser: GradientParser::new(),
            object_fit_parser: ObjectFitParser::new(),
            transform_parser: TransformParser::new(),
            arbitrary_parser: ArbitraryParser::new(),
            data_attribute_parser: DataAttributeParser::new(),
            background_properties_parser: BackgroundPropertiesParser::new(),
            transition_properties_parser: TransitionPropertiesParser::new(),
            fractional_transforms_parser: FractionalTransformsParser::new(),
            aspect_ratio_parser: AspectRatioParser::new(),
            columns_parser: ColumnsParser::new(),
            break_control_parser: BreakControlParser::new(),
            box_utilities_parser: BoxUtilitiesParser::new(),
            layout_utilities_parser: LayoutUtilitiesParser::new(),
            overflow_parser: OverflowParser::new(),
            overscroll_parser: OverscrollParser::new(),
            position_parser: PositionParser::new(),
            inset_parser: InsetParser::new(),
            visibility_parser: VisibilityParser::new(),
            z_index_parser: ZIndexParser::new(),
            flex_basis_parser: FlexBasisParser::new(),
            flex_direction_parser: FlexDirectionParser::new(),
            flex_wrap_parser: FlexWrapParser::new(),
            flex_parser: FlexParser::new(),
            flex_grow_parser: FlexGrowParser::new(),
            flex_shrink_parser: FlexShrinkParser::new(),
            order_parser: OrderParser::new(),
            grid_template_columns_parser: GridTemplateColumnsParser::new(),
            grid_column_parser: GridColumnParser::new(),
            grid_template_rows_parser: GridTemplateRowsParser::new(),
            grid_row_parser: GridRowParser::new(),
            grid_auto_flow_parser: GridAutoFlowParser::new(),
            grid_auto_columns_parser: GridAutoColumnsParser::new(),
            grid_auto_rows_parser: GridAutoRowsParser::new(),
            gap_parser: GapParser::new(),
            justify_content_parser: JustifyContentParser::new(),
            justify_items_parser: JustifyItemsParser::new(),
            justify_self_parser: JustifySelfParser::new(),
            align_content_parser: AlignContentParser::new(),
            align_items_parser: AlignItemsParser::new(),
            align_self_parser: AlignSelfParser::new(),
            place_content_parser: PlaceContentParser::new(),
            place_items_parser: PlaceItemsParser::new(),
            place_self_parser: PlaceSelfParser::new(),
            background_parser: BackgroundParser::new(),
            border_utilities_parser: BorderUtilitiesParser::new(),
            effects_utilities_parser: EffectsUtilitiesParser::new(),
            filter_utilities_parser: FilterUtilitiesParser::new(),
            backdrop_filter_utilities_parser: BackdropFilterUtilitiesParser::new(),
            accessibility_parser: AccessibilityParser::new(),
            table_parser: TableParser::new(),
            mask_utilities_parser: MaskUtilitiesParser::new(),
            variant_parser: VariantParser::new(),
        };
        
        if generator.config.custom_breakpoints.is_empty() {
            generator.initialize_default_breakpoints();
        } else {
            generator.initialize_custom_breakpoints();
        }
        
        generator
    }
    
    fn initialize_default_breakpoints(&mut self) {
        self.breakpoints.insert(Breakpoint::Sm, "(min-width: 640px)".to_string());
        self.breakpoints.insert(Breakpoint::Md, "(min-width: 768px)".to_string());
        self.breakpoints.insert(Breakpoint::Lg, "(min-width: 1024px)".to_string());
        self.breakpoints.insert(Breakpoint::Xl, "(min-width: 1280px)".to_string());
        self.breakpoints.insert(Breakpoint::Xl2, "(min-width: 1536px)".to_string());
    }
    
    fn initialize_custom_breakpoints(&mut self) {
        self.breakpoints = self.config.custom_breakpoints.clone();
    }
}
