//! CSS Generator Builder Methods
//!
//! This module contains the constructor and initialization methods for CssGenerator.

use super::parsers::{
    AccentColorParser, AccessibilityParser, AdvancedBorderParser, AdvancedColorParser,
    AdvancedGridParser, AdvancedSpacingParser, AlignContentParser, AlignItemsParser,
    AlignSelfParser, AnimationParser, ArbitraryParser, AspectRatioParser,
    BackdropFilterUtilitiesParser, BackgroundParser, BackgroundPropertiesParser,
    BasicTransformsParser, BorderRadiusParser, BorderUtilitiesParser, BoxUtilitiesParser, BreakControlParser, ColorParser, ColumnsParser,
    OutlineParser,
    DataAttributeParser, DivideParser, EffectsParser, EffectsUtilitiesParser,
    FilterUtilitiesParser, FlexBasisParser, FlexDirectionParser, FlexGrowParser, FlexParser,
    FlexShrinkParser, FlexWrapParser, FlexboxParser, FractionalTransformsParser, GapParser,
    GradientParser, GridAutoColumnsParser, GridAutoFlowParser, GridAutoRowsParser,
    GridColumnParser, GridRowParser, GridTemplateColumnsParser, GridTemplateRowsParser,
    GroupParser, InsetParser, InteractiveParser, JustifyContentParser, JustifyItemsParser,
    JustifySelfParser, LayoutParser, LayoutUtilitiesParser, MarginParser, MaskUtilitiesParser,
    ObjectFitParser, OrderParser, OverflowParser, OverscrollParser, PlaceContentParser,
    PlaceItemsParser, PlaceSelfParser, PositionParser, PositioningParser, ProseParser, RingParser,
    ScaleParser, ShadowParser, SizingParser, SpacingParser, SvgParser, TableParser, TransformParser,
    TransitionParser, TransitionPropertiesParser, TypographyParser, VisibilityParser, ZIndexParser,
};
use super::types::CssGenerationConfig;
use super::variants::VariantParser;
use super::trie::{ParserTrie, ParserType};
use crate::responsive::Breakpoint;
use std::collections::HashMap;

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

    /// Initialize the parser trie with all parser mappings
    fn initialize_parser_trie(&mut self);
}

impl CssGeneratorBuilder for super::CssGenerator {
    fn new() -> Self {
        let mut generator = Self {
            rules: HashMap::new(),
            breakpoints: HashMap::new(),
            custom_properties: HashMap::new(),
            config: CssGenerationConfig::default(),
            gradient_context: super::core::GradientContext::default(),
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
            outline_parser: OutlineParser::new(),
            ring_parser: RingParser::new(),
            transition_parser: TransitionParser::new(),
            shadow_parser: ShadowParser::new(),
            border_radius_parser: BorderRadiusParser::new(),
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
            basic_transforms_parser: BasicTransformsParser::new(),
            scale_parser: ScaleParser::new(),
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
            accent_color_parser: AccentColorParser::new(),
            variant_parser: VariantParser::new(),
            parser_trie: ParserTrie::new(),
        };

        // Initialize the parser trie for fast lookups
        generator.initialize_parser_trie();
        generator.initialize_default_breakpoints();
        generator
    }

    fn with_config(config: CssGenerationConfig) -> Self {
        let mut generator = Self {
            rules: HashMap::new(),
            breakpoints: HashMap::new(),
            custom_properties: HashMap::new(),
            config,
            gradient_context: super::core::GradientContext::default(),
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
            outline_parser: OutlineParser::new(),
            ring_parser: RingParser::new(),
            transition_parser: TransitionParser::new(),
            shadow_parser: ShadowParser::new(),
            border_radius_parser: BorderRadiusParser::new(),
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
            basic_transforms_parser: BasicTransformsParser::new(),
            scale_parser: ScaleParser::new(),
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
            accent_color_parser: AccentColorParser::new(),
            variant_parser: VariantParser::new(),
            parser_trie: ParserTrie::new(),
        };

        // Initialize the parser trie for fast lookups
        generator.initialize_parser_trie();

        if generator.config.custom_breakpoints.is_empty() {
            generator.initialize_default_breakpoints();
        } else {
            generator.initialize_custom_breakpoints();
        }

        generator
    }

    fn initialize_default_breakpoints(&mut self) {
        self.breakpoints
            .insert(Breakpoint::Sm, "(min-width: 640px)".to_string());
        self.breakpoints
            .insert(Breakpoint::Md, "(min-width: 768px)".to_string());
        self.breakpoints
            .insert(Breakpoint::Lg, "(min-width: 1024px)".to_string());
        self.breakpoints
            .insert(Breakpoint::Xl, "(min-width: 1280px)".to_string());
        self.breakpoints
            .insert(Breakpoint::Xl2, "(min-width: 1536px)".to_string());
    }

    fn initialize_custom_breakpoints(&mut self) {
        self.breakpoints = self.config.custom_breakpoints.clone();
    }

    fn initialize_parser_trie(&mut self) {
        // Populate the trie with parser mappings for fast lookups
        // Order follows the priority order from generator_parsers.rs

        // Arbitrary parser (highest priority)
        self.parser_trie.insert("[", ParserType::Arbitrary(self.arbitrary_parser.clone()));

        // Basic transforms parser
        self.parser_trie.insert("translate-x-", ParserType::BasicTransforms(self.basic_transforms_parser.clone()));
        self.parser_trie.insert("translate-y-", ParserType::BasicTransforms(self.basic_transforms_parser.clone()));

        // Scale parser
        self.parser_trie.insert("scale-", ParserType::Transform(self.transform_parser.clone()));
        self.parser_trie.insert("scale-x-", ParserType::Scale(self.scale_parser.clone()));
        self.parser_trie.insert("scale-y-", ParserType::Scale(self.scale_parser.clone()));

        // Mask utilities parser
        self.parser_trie.insert("mask-", ParserType::MaskUtilities(self.mask_utilities_parser.clone()));

        // Transform parser (including 3D transforms)
        self.parser_trie.insert("rotate-", ParserType::Transform(self.transform_parser.clone()));
        self.parser_trie.insert("rotate-x-", ParserType::Transform(self.transform_parser.clone()));
        self.parser_trie.insert("rotate-y-", ParserType::Transform(self.transform_parser.clone()));
        self.parser_trie.insert("rotate-z-", ParserType::Transform(self.transform_parser.clone()));
        self.parser_trie.insert("skew-x-", ParserType::Transform(self.transform_parser.clone()));
        self.parser_trie.insert("skew-y-", ParserType::Transform(self.transform_parser.clone()));
        self.parser_trie.insert("perspective-", ParserType::Transform(self.transform_parser.clone()));
        self.parser_trie.insert("transform-style-", ParserType::Transform(self.transform_parser.clone()));
        self.parser_trie.insert("transform", ParserType::Transform(self.transform_parser.clone()));
        self.parser_trie.insert("scale-", ParserType::Transform(self.transform_parser.clone()));
        self.parser_trie.insert("rotate-", ParserType::Transform(self.transform_parser.clone()));
        self.parser_trie.insert("-rotate-", ParserType::Transform(self.transform_parser.clone()));

        // Gradient parser
        self.parser_trie.insert("bg-gradient-", ParserType::Background(self.background_parser.clone()));
        self.parser_trie.insert("bg-conic", ParserType::Gradient(self.gradient_parser.clone()));
        self.parser_trie.insert("bg-radial", ParserType::Gradient(self.gradient_parser.clone()));
        self.parser_trie.insert("from-", ParserType::Gradient(self.gradient_parser.clone()));
        self.parser_trie.insert("to-", ParserType::Gradient(self.gradient_parser.clone()));
        self.parser_trie.insert("via-", ParserType::Gradient(self.gradient_parser.clone()));

        // Effects parser
        self.parser_trie.insert("opacity-", ParserType::Effects(self.effects_parser.clone()));
        self.parser_trie.insert("backdrop-blur-", ParserType::Effects(self.effects_parser.clone()));
        self.parser_trie.insert("backdrop-opacity-", ParserType::Effects(self.effects_parser.clone()));

        // Typography parser (register first for text- classes)
        self.parser_trie.insert("font-", ParserType::Typography(self.typography_parser.clone()));
        self.parser_trie.insert("text-", ParserType::Typography(self.typography_parser.clone()));
        self.parser_trie.insert("leading-", ParserType::Typography(self.typography_parser.clone()));
        self.parser_trie.insert("italic", ParserType::Typography(self.typography_parser.clone()));
        self.parser_trie.insert("tracking-", ParserType::Typography(self.typography_parser.clone()));

        // Color parser (border colors only - text colors handled by Typography)
        self.parser_trie.insert("border-", ParserType::Color(self.color_parser.clone()));

        // Background parser
        self.parser_trie.insert("bg-", ParserType::Background(self.background_parser.clone()));

        // Background properties parser
        self.parser_trie.insert("bg-size-", ParserType::BackgroundProperties(self.background_properties_parser.clone()));
        self.parser_trie.insert("bg-position-", ParserType::BackgroundProperties(self.background_properties_parser.clone()));
        self.parser_trie.insert("bg-repeat-", ParserType::BackgroundProperties(self.background_properties_parser.clone()));
        self.parser_trie.insert("bg-origin-", ParserType::BackgroundProperties(self.background_properties_parser.clone()));
        self.parser_trie.insert("bg-clip-", ParserType::BackgroundProperties(self.background_properties_parser.clone()));

        // Layout parser
        self.parser_trie.insert("display", ParserType::Layout(self.layout_parser.clone()));
        self.parser_trie.insert("flex", ParserType::Layout(self.layout_parser.clone()));
        self.parser_trie.insert("grid", ParserType::Layout(self.layout_parser.clone()));
        self.parser_trie.insert("block", ParserType::Layout(self.layout_parser.clone()));
        self.parser_trie.insert("inline", ParserType::Layout(self.layout_parser.clone()));
        self.parser_trie.insert("container", ParserType::Layout(self.layout_parser.clone()));

        // Flexbox parser
        self.parser_trie.insert("justify-", ParserType::Flexbox(self.flexbox_parser.clone()));
        self.parser_trie.insert("items-", ParserType::Flexbox(self.flexbox_parser.clone()));
        self.parser_trie.insert("flex-wrap", ParserType::Flexbox(self.flexbox_parser.clone()));

        // Sizing parser
        self.parser_trie.insert("w-", ParserType::Sizing(self.sizing_parser.clone()));
        self.parser_trie.insert("h-", ParserType::Sizing(self.sizing_parser.clone()));
        self.parser_trie.insert("min-w-", ParserType::Sizing(self.sizing_parser.clone()));
        self.parser_trie.insert("max-w-", ParserType::Sizing(self.sizing_parser.clone()));
        self.parser_trie.insert("min-h-", ParserType::Sizing(self.sizing_parser.clone()));
        self.parser_trie.insert("max-h-", ParserType::Sizing(self.sizing_parser.clone()));

        // Spacing parser
        self.parser_trie.insert("p-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("px-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("py-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("pt-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("pr-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("pb-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("pl-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("ps-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("pe-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("m-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("mx-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("my-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("mt-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("mr-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("mb-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("ml-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("ms-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("me-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("gap-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("gap-x-", ParserType::Spacing(self.spacing_parser.clone()));
        self.parser_trie.insert("gap-y-", ParserType::Spacing(self.spacing_parser.clone()));

        // Positioning parser
        self.parser_trie.insert("top-", ParserType::Positioning(self.positioning_parser.clone()));
        self.parser_trie.insert("left-", ParserType::Positioning(self.positioning_parser.clone()));
        self.parser_trie.insert("right-", ParserType::Positioning(self.positioning_parser.clone()));
        self.parser_trie.insert("bottom-", ParserType::Positioning(self.positioning_parser.clone()));
        self.parser_trie.insert("absolute", ParserType::Positioning(self.positioning_parser.clone()));
        self.parser_trie.insert("relative", ParserType::Positioning(self.positioning_parser.clone()));
        self.parser_trie.insert("fixed", ParserType::Positioning(self.positioning_parser.clone()));
        self.parser_trie.insert("sticky", ParserType::Positioning(self.positioning_parser.clone()));

        // Advanced border parser
        self.parser_trie.insert("border-", ParserType::AdvancedBorder(self.advanced_border_parser.clone()));

        // Shadow parser
        self.parser_trie.insert("shadow-", ParserType::Shadow(self.shadow_parser.clone()));
        self.parser_trie.insert("drop-shadow-", ParserType::Effects(self.effects_parser.clone()));

        // Border radius parser
        self.parser_trie.insert("rounded", ParserType::BorderRadius(self.border_radius_parser.clone()));

        // Effects parser (registered after Shadow parser to handle shadow colors)
        self.parser_trie.insert("shadow-", ParserType::Effects(self.effects_parser.clone()));

        // Transition parser
        self.parser_trie.insert("transition", ParserType::Transition(self.transition_parser.clone()));
        self.parser_trie.insert("transition-all", ParserType::Transition(self.transition_parser.clone()));
        self.parser_trie.insert("transition-", ParserType::Transition(self.transition_parser.clone()));
        self.parser_trie.insert("duration-", ParserType::Transition(self.transition_parser.clone()));
        self.parser_trie.insert("delay-", ParserType::Transition(self.transition_parser.clone()));
        self.parser_trie.insert("ease-", ParserType::Transition(self.transition_parser.clone()));

        // Ring parser
        self.parser_trie.insert("ring-", ParserType::Ring(self.ring_parser.clone()));

        // Interactive parser
        self.parser_trie.insert("hover:", ParserType::Interactive(self.interactive_parser.clone()));
        self.parser_trie.insert("focus:", ParserType::Interactive(self.interactive_parser.clone()));
        self.parser_trie.insert("active:", ParserType::Interactive(self.interactive_parser.clone()));

        // Animation parser
        self.parser_trie.insert("animate-", ParserType::Animation(self.animation_parser.clone()));

        // Advanced grid parser
        self.parser_trie.insert("grid-", ParserType::AdvancedGrid(self.advanced_grid_parser.clone()));

        // SVG parser
        self.parser_trie.insert("fill-", ParserType::Svg(self.svg_parser.clone()));
        self.parser_trie.insert("stroke-", ParserType::Svg(self.svg_parser.clone()));

        // Prose parser
        self.parser_trie.insert("prose", ParserType::Prose(self.prose_parser.clone()));

        // Divide parser
        self.parser_trie.insert("divide-", ParserType::Divide(self.divide_parser.clone()));

        // Object fit parser
        self.parser_trie.insert("object-", ParserType::ObjectFit(self.object_fit_parser.clone()));

        // Accent color parser
        self.parser_trie.insert("accent-", ParserType::AccentColor(self.accent_color_parser.clone()));

        // Data attribute parser
        self.parser_trie.insert("data-", ParserType::DataAttribute(self.data_attribute_parser.clone()));

        // Advanced color parser
        self.parser_trie.insert("text-opacity-", ParserType::AdvancedColor(self.advanced_color_parser.clone()));
        self.parser_trie.insert("bg-opacity-", ParserType::AdvancedColor(self.advanced_color_parser.clone()));

        // Advanced spacing parser
        self.parser_trie.insert("space-", ParserType::AdvancedSpacing(self.advanced_spacing_parser.clone()));

        // Transition properties parser
        self.parser_trie.insert("transition-", ParserType::TransitionProperties(self.transition_properties_parser.clone()));

        // Fractional transforms parser
        // Note: FractionalTransformsParser only handles translate fractions, not scale

        // Aspect ratio parser
        self.parser_trie.insert("aspect-", ParserType::AspectRatio(self.aspect_ratio_parser.clone()));

        // Columns parser
        self.parser_trie.insert("columns-", ParserType::Columns(self.columns_parser.clone()));

        // Break control parser
        self.parser_trie.insert("break-", ParserType::BreakControl(self.break_control_parser.clone()));

        // Box utilities parser
        self.parser_trie.insert("box-", ParserType::BoxUtilities(self.box_utilities_parser.clone()));

        // Layout utilities parser
        self.parser_trie.insert("clear", ParserType::LayoutUtilities(self.layout_utilities_parser.clone()));
        self.parser_trie.insert("float-", ParserType::LayoutUtilities(self.layout_utilities_parser.clone()));

        // Overflow parser
        self.parser_trie.insert("overflow-", ParserType::Overflow(self.overflow_parser.clone()));

        // Overscroll parser
        self.parser_trie.insert("overscroll-", ParserType::Overscroll(self.overscroll_parser.clone()));

        // Position parser
        self.parser_trie.insert("static", ParserType::Position(self.position_parser.clone()));

        // Inset parser
        self.parser_trie.insert("inset-", ParserType::Inset(self.inset_parser.clone()));

        // Visibility parser
        self.parser_trie.insert("visible", ParserType::Visibility(self.visibility_parser.clone()));
        self.parser_trie.insert("invisible", ParserType::Visibility(self.visibility_parser.clone()));

        // Z-index parser
        self.parser_trie.insert("z-", ParserType::ZIndex(self.z_index_parser.clone()));

        // Flex parsers
        self.parser_trie.insert("flex-", ParserType::Flex(self.flex_parser.clone()));
        self.parser_trie.insert("flex-grow-", ParserType::FlexGrow(self.flex_grow_parser.clone()));
        self.parser_trie.insert("flex-shrink-", ParserType::FlexShrink(self.flex_shrink_parser.clone()));
        self.parser_trie.insert("order-", ParserType::Order(self.order_parser.clone()));

        // Grid parsers
        self.parser_trie.insert("grid-cols-", ParserType::GridTemplateColumns(self.grid_template_columns_parser.clone()));
        self.parser_trie.insert("grid-rows-", ParserType::GridTemplateRows(self.grid_template_rows_parser.clone()));
        self.parser_trie.insert("col-", ParserType::GridColumn(self.grid_column_parser.clone()));
        self.parser_trie.insert("row-", ParserType::GridRow(self.grid_row_parser.clone()));
        self.parser_trie.insert("grid-flow-", ParserType::GridAutoFlow(self.grid_auto_flow_parser.clone()));
        self.parser_trie.insert("auto-cols-", ParserType::GridAutoColumns(self.grid_auto_columns_parser.clone()));
        self.parser_trie.insert("auto-rows-", ParserType::GridAutoRows(self.grid_auto_rows_parser.clone()));

        // Alignment parsers
        self.parser_trie.insert("justify-", ParserType::JustifyContent(self.justify_content_parser.clone()));
        self.parser_trie.insert("content-", ParserType::JustifyContent(self.justify_content_parser.clone()));
        self.parser_trie.insert("items-", ParserType::AlignItems(self.align_items_parser.clone()));
        self.parser_trie.insert("self-", ParserType::AlignSelf(self.align_self_parser.clone()));
        self.parser_trie.insert("place-", ParserType::PlaceContent(self.place_content_parser.clone()));

        // Margin parser
        self.parser_trie.insert("m-", ParserType::Margin(self.margin_parser.clone()));

        // Group parser
        self.parser_trie.insert("group", ParserType::Group(self.group_parser.clone()));

        // Filter utilities parsers
        self.parser_trie.insert("blur-", ParserType::FilterUtilities(self.filter_utilities_parser.clone()));
        self.parser_trie.insert("brightness-", ParserType::FilterUtilities(self.filter_utilities_parser.clone()));
        self.parser_trie.insert("contrast-", ParserType::FilterUtilities(self.filter_utilities_parser.clone()));
        self.parser_trie.insert("grayscale", ParserType::FilterUtilities(self.filter_utilities_parser.clone()));
        self.parser_trie.insert("invert", ParserType::FilterUtilities(self.filter_utilities_parser.clone()));
        self.parser_trie.insert("sepia", ParserType::FilterUtilities(self.filter_utilities_parser.clone()));

        // Backdrop filter utilities parser
        self.parser_trie.insert("backdrop-", ParserType::BackdropFilterUtilities(self.backdrop_filter_utilities_parser.clone()));

        // Effects utilities parser
        self.parser_trie.insert("backdrop-", ParserType::EffectsUtilities(self.effects_utilities_parser.clone()));

        // Border utilities parser
        self.parser_trie.insert("border", ParserType::BorderUtilities(self.border_utilities_parser.clone()));
        self.parser_trie.insert("border-", ParserType::BorderUtilities(self.border_utilities_parser.clone()));

        // Outline parser
        self.parser_trie.insert("outline-", ParserType::Outline(self.outline_parser.clone()));

        // Accessibility parser
        self.parser_trie.insert("sr-", ParserType::Accessibility(self.accessibility_parser.clone()));
    }
}
