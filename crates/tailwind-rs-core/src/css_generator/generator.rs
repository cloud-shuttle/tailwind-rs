//! Main CSS Generator implementation
//!
//! This module contains the core CssGenerator struct and its main functionality.

use super::parsers::{
    AccentColorParser, AccessibilityParser, AdvancedBorderParser, AdvancedColorParser,
    AdvancedGridParser, AdvancedSpacingParser, AlignContentParser, AlignItemsParser,
    AlignSelfParser, AnimationParser, ArbitraryParser, AspectRatioParser,
    BackdropFilterUtilitiesParser, BackgroundParser, BackgroundPropertiesParser,
    BorderUtilitiesParser, BoxUtilitiesParser, BreakControlParser, ColorParser, ColumnsParser,
    DataAttributeParser, DivideParser, EffectsParser, EffectsUtilitiesParser,
    FilterUtilitiesParser, FlexBasisParser, FlexDirectionParser, FlexGrowParser, FlexParser,
    FlexShrinkParser, FlexWrapParser, FlexboxParser, FractionalTransformsParser, GapParser,
    GradientParser, GridAutoColumnsParser, GridAutoFlowParser, GridAutoRowsParser,
    GridColumnParser, GridRowParser, GridTemplateColumnsParser, GridTemplateRowsParser,
    GroupParser, InsetParser, InteractiveParser, JustifyContentParser, JustifyItemsParser,
    JustifySelfParser, LayoutParser, LayoutUtilitiesParser, MarginParser, MaskUtilitiesParser,
    ObjectFitParser, OrderParser, OverflowParser, OverscrollParser, PlaceContentParser,
    PlaceItemsParser, PlaceSelfParser, PositionParser, PositioningParser, ProseParser, RingParser,
    ShadowParser, SizingParser, SpacingParser, SvgParser, TableParser, TransformParser,
    TransitionParser, TransitionPropertiesParser, TypographyParser, VisibilityParser, ZIndexParser,
};
use super::types::{CssGenerationConfig, CssProperty, CssRule};
use super::variants::VariantParser;
use crate::error::Result;
use crate::responsive::Breakpoint;
use std::collections::HashMap;

/// CSS generator that converts Tailwind classes to CSS rules
#[derive(Debug, Clone)]
pub struct CssGenerator {
    /// Generated CSS rules
    pub rules: HashMap<String, CssRule>,
    /// Responsive breakpoints
    pub breakpoints: HashMap<Breakpoint, String>,
    /// Custom CSS properties
    pub custom_properties: HashMap<String, String>,
    /// Generation configuration
    pub config: CssGenerationConfig,
    /// Spacing parser
    pub spacing_parser: SpacingParser,
    /// Advanced spacing parser
    pub advanced_spacing_parser: AdvancedSpacingParser,
    /// Color parser
    pub color_parser: ColorParser,
    /// Advanced color parser
    pub advanced_color_parser: AdvancedColorParser,
    /// Typography parser
    pub typography_parser: TypographyParser,
    /// Layout parser
    pub layout_parser: LayoutParser,
    /// Positioning parser
    pub positioning_parser: PositioningParser,
    /// Flexbox parser
    pub flexbox_parser: FlexboxParser,
    /// Effects parser
    pub effects_parser: EffectsParser,
    /// Sizing parser
    pub sizing_parser: SizingParser,
    /// Advanced border parser
    pub advanced_border_parser: AdvancedBorderParser,
    /// Ring parser
    pub ring_parser: RingParser,
    /// Transition parser
    pub transition_parser: TransitionParser,
    /// Shadow parser
    pub shadow_parser: ShadowParser,
    /// SVG parser
    pub svg_parser: SvgParser,
    /// Margin parser
    pub margin_parser: MarginParser,
    /// Group parser
    pub group_parser: GroupParser,
    /// Advanced grid parser
    pub advanced_grid_parser: AdvancedGridParser,
    /// Animation parser
    pub animation_parser: AnimationParser,
    /// Interactive parser
    pub interactive_parser: InteractiveParser,
    /// Prose parser
    pub prose_parser: ProseParser,
    /// Divide parser
    pub divide_parser: DivideParser,
    /// Gradient parser
    pub gradient_parser: GradientParser,
    /// Object fit parser
    pub object_fit_parser: ObjectFitParser,
    /// Transform parser
    pub transform_parser: TransformParser,
    /// Arbitrary values parser
    pub arbitrary_parser: ArbitraryParser,
    /// Data attributes parser
    pub data_attribute_parser: DataAttributeParser,
    /// Background properties parser
    pub background_properties_parser: BackgroundPropertiesParser,
    /// Transition properties parser
    pub transition_properties_parser: TransitionPropertiesParser,
    /// Fractional transforms parser
    pub fractional_transforms_parser: FractionalTransformsParser,
    /// Aspect ratio parser
    pub aspect_ratio_parser: AspectRatioParser,
    /// Columns parser
    pub columns_parser: ColumnsParser,
    /// Break control parser
    pub break_control_parser: BreakControlParser,
    /// Box utilities parser
    pub box_utilities_parser: BoxUtilitiesParser,
    /// Layout utilities parser
    pub layout_utilities_parser: LayoutUtilitiesParser,
    /// Overflow parser
    pub overflow_parser: OverflowParser,
    /// Overscroll parser
    pub overscroll_parser: OverscrollParser,
    /// Position parser
    pub position_parser: PositionParser,
    /// Inset parser
    pub inset_parser: InsetParser,
    /// Visibility parser
    pub visibility_parser: VisibilityParser,
    /// Z-index parser
    pub z_index_parser: ZIndexParser,
    /// Flex basis parser
    pub flex_basis_parser: FlexBasisParser,
    /// Flex direction parser
    pub flex_direction_parser: FlexDirectionParser,
    /// Flex wrap parser
    pub flex_wrap_parser: FlexWrapParser,
    /// Flex parser
    pub flex_parser: FlexParser,
    /// Flex grow parser
    pub flex_grow_parser: FlexGrowParser,
    /// Flex shrink parser
    pub flex_shrink_parser: FlexShrinkParser,
    /// Order parser
    pub order_parser: OrderParser,
    /// Grid template columns parser
    pub grid_template_columns_parser: GridTemplateColumnsParser,
    /// Grid column parser
    pub grid_column_parser: GridColumnParser,
    /// Grid template rows parser
    pub grid_template_rows_parser: GridTemplateRowsParser,
    /// Grid row parser
    pub grid_row_parser: GridRowParser,
    /// Grid auto flow parser
    pub grid_auto_flow_parser: GridAutoFlowParser,
    /// Grid auto columns parser
    pub grid_auto_columns_parser: GridAutoColumnsParser,
    /// Grid auto rows parser
    pub grid_auto_rows_parser: GridAutoRowsParser,
    /// Gap parser
    pub gap_parser: GapParser,
    /// Justify content parser
    pub justify_content_parser: JustifyContentParser,
    /// Justify items parser
    pub justify_items_parser: JustifyItemsParser,
    /// Justify self parser
    pub justify_self_parser: JustifySelfParser,
    /// Align content parser
    pub align_content_parser: AlignContentParser,
    /// Align items parser
    pub align_items_parser: AlignItemsParser,
    /// Align self parser
    pub align_self_parser: AlignSelfParser,
    /// Place content parser
    pub place_content_parser: PlaceContentParser,
    /// Place items parser
    pub place_items_parser: PlaceItemsParser,
    /// Place self parser
    pub place_self_parser: PlaceSelfParser,
    /// Background parser
    pub background_parser: BackgroundParser,
    /// Border utilities parser
    pub border_utilities_parser: BorderUtilitiesParser,
    /// Effects utilities parser
    pub effects_utilities_parser: EffectsUtilitiesParser,
    /// Filter utilities parser
    pub filter_utilities_parser: FilterUtilitiesParser,
    /// Backdrop filter utilities parser
    pub backdrop_filter_utilities_parser: BackdropFilterUtilitiesParser,
    /// Accessibility parser
    pub accessibility_parser: AccessibilityParser,
    /// Table parser
    pub table_parser: TableParser,
    /// Mask utilities parser
    pub mask_utilities_parser: MaskUtilitiesParser,
    /// Accent color parser
    pub accent_color_parser: AccentColorParser,
    /// Variant parser
    pub variant_parser: VariantParser,
}

impl Default for CssGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CssGenerator {
    /// Create a new CSS generator
    pub fn new() -> Self {
        use super::generator_builders::CssGeneratorBuilder;
        <Self as CssGeneratorBuilder>::new()
    }

    /// Create a new CSS generator with custom configuration
    pub fn with_config(config: CssGenerationConfig) -> Self {
        use super::generator_builders::CssGeneratorBuilder;
        <Self as CssGeneratorBuilder>::with_config(config)
    }

    /// Add a class to the generator
    pub fn add_class(&mut self, class: &str) -> Result<()> {
        use super::generator_operations::CssGeneratorOperations;
        <Self as CssGeneratorOperations>::add_class(self, class)
    }

    /// Add a CSS selector directly (for non-Tailwind CSS selectors)
    pub fn add_css_selector(&mut self, selector: &str, properties: &str) -> Result<()> {
        use super::generator_operations::CssGeneratorOperations;
        <Self as CssGeneratorOperations>::add_css_selector(self, selector, properties)
    }

    /// Add a responsive class
    pub fn add_responsive_class(&mut self, breakpoint: Breakpoint, class: &str) -> Result<()> {
        use super::generator_operations::CssGeneratorOperations;
        <Self as CssGeneratorOperations>::add_responsive_class(self, breakpoint, class)
    }

    /// Add a custom CSS property
    pub fn add_custom_property(&mut self, name: &str, value: &str) {
        use super::generator_operations::CssGeneratorOperations;
        <Self as CssGeneratorOperations>::add_custom_property(self, name, value)
    }

    /// Get the current configuration
    pub fn config(&self) -> &CssGenerationConfig {
        &self.config
    }

    /// Update the configuration
    pub fn set_config(&mut self, config: CssGenerationConfig) {
        self.config = config;
    }

    /// Get all generated rules
    pub fn rules(&self) -> &HashMap<String, CssRule> {
        &self.rules
    }

    /// Get the number of generated rules
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Generate comprehensive CSS with all utilities
    pub fn generate_comprehensive_css(&mut self, _config: &CssGenerationConfig) -> Result<String> {
        // Add common utility classes
        let common_classes = vec![
            "p-4",
            "m-4",
            "bg-blue-500",
            "text-white",
            "rounded-md",
            "hover:bg-blue-600",
            "focus:outline-none",
            "sm:p-6",
            "md:p-8",
        ];

        for class in common_classes {
            let _ = self.add_class(class);
        }

        Ok(self.generate_css())
    }

    /// Get all rules (for compatibility)
    pub fn get_rules(&self) -> &HashMap<String, CssRule> {
        &self.rules
    }

    /// Remove a rule by selector
    pub fn remove_rule(&mut self, selector: &str) {
        use super::generator_operations::CssGeneratorOperations;
        <Self as CssGeneratorOperations>::remove_rule(self, selector)
    }

    /// Update a rule
    pub fn update_rule(&mut self, selector: &str, rule: CssRule) {
        use super::generator_operations::CssGeneratorOperations;
        <Self as CssGeneratorOperations>::update_rule(self, selector, rule)
    }

    /// Generate CSS from all added classes
    pub fn generate_css(&self) -> String {
        super::css_output::CssOutputGenerator::generate_css(&self.rules, &self.custom_properties)
    }

    /// Generate minified CSS from all added classes
    pub fn generate_minified_css(&self) -> String {
        super::css_output::CssOutputGenerator::generate_minified_css(
            &self.rules,
            &self.custom_properties,
        )
    }

    /// Convert a class name to a CSS rule
    pub fn class_to_css_rule(&self, class: &str) -> Result<CssRule> {
        let (variants, base_class) = self.parse_variants(class);
        let properties = self.class_to_properties(class)?;

        // Build selector with variants
        let mut selector = String::new();
        for variant in &variants {
            let variant_selector = self.variant_parser.get_variant_selector(variant);
            if !variant_selector.is_empty() {
                selector.push_str(&variant_selector);
            }
        }

        // Add the base class
        selector.push_str(&format!(".{}", base_class));

        // Determine media query for responsive and device variants
        let media_query = variants.iter().find_map(|variant| {
            // Try responsive media query first
            if let Some(responsive_query) = self.variant_parser.get_responsive_media_query(variant)
            {
                Some(responsive_query)
            } else {
                // Try device media query
                self.variant_parser.get_device_media_query(variant)
            }
        });

        Ok(CssRule {
            selector,
            properties,
            media_query: media_query.map(|s| s.to_string()),
            specificity: variants.len() as u32 * 10, // Higher specificity for more variants
        })
    }

    /// Parse variants from a class name and return (variants, base_class)
    fn parse_variants(&self, class: &str) -> (Vec<String>, String) {
        self.variant_parser.parse_variants(class)
    }

    /// Convert a class name to CSS properties
    pub fn class_to_properties(&self, class: &str) -> Result<Vec<CssProperty>> {
        use super::generator_parsers::CssGeneratorParsers;
        <Self as CssGeneratorParsers>::class_to_properties(self, class)
    }
}
