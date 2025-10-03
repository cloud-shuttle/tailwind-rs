//! Main CSS Generator implementation
//!
//! This module contains the core CssGenerator struct and its main functionality.

use super::parsers::{
    AccentColorParser, AccessibilityParser, AdvancedBorderParser, AdvancedColorParser,
    AdvancedGridParser, AdvancedSpacingParser, AlignContentParser, AlignItemsParser,
    AlignSelfParser, AnimationParser, ArbitraryParser, AspectRatioParser,
    BackdropFilterUtilitiesParser, BackgroundColorParser, BackgroundPropertiesParser, FieldSizingParser,
    BorderRadiusParser, BorderUtilitiesParser, BoxUtilitiesParser, BreakControlParser, ColorParser, ColumnsParser,
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
    ShadowParser, SizingParser, SpacingParser, SvgParser, TableParser,
    BasicTransformsParser, ScaleParser, TransitionParser, TransitionPropertiesParser, TypographyParser, VisibilityParser, ZIndexParser,
};
use super::types::{CssGenerationConfig, CssProperty, CssRule};
use super::variants::VariantParser;
use super::trie::{ParserTrie, ParserType};
use super::element_context::ElementContext;
use crate::transforms::TransformParser;
use crate::error::Result;
use crate::responsive::Breakpoint;
use std::collections::HashMap;

/// CSS generator that converts Tailwind classes to CSS rules
#[derive(Debug)]
pub struct CssGenerator {
    /// Generated CSS rules
    pub rules: HashMap<String, CssRule>,
    /// Responsive breakpoints
    pub breakpoints: HashMap<Breakpoint, String>,
    /// Custom CSS properties
    pub custom_properties: HashMap<String, String>,
    /// Generation configuration
    pub config: CssGenerationConfig,
    /// Context for element state (gradients, shadows, transforms, etc.)
    pub element_context: ElementContext,
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
    /// Field sizing parser
    pub field_sizing_parser: FieldSizingParser,
    /// Advanced border parser
    pub advanced_border_parser: AdvancedBorderParser,
    /// Outline parser
    pub outline_parser: OutlineParser,
    /// Ring parser
    pub ring_parser: RingParser,
    /// Transition parser
    pub transition_parser: TransitionParser,
    /// Shadow parser
    pub shadow_parser: ShadowParser,
    /// Border radius parser
    pub border_radius_parser: BorderRadiusParser,
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
    /// Basic transforms parser
    pub basic_transforms_parser: BasicTransformsParser,
    /// Scale parser
    pub scale_parser: ScaleParser,
    /// Arbitrary values parser
    pub arbitrary_parser: ArbitraryParser,
    /// Data attributes parser
    pub data_attribute_parser: DataAttributeParser,
    /// Background color parser
    pub background_color_parser: BackgroundColorParser,
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
    /// Parser trie for fast lookups
    pub parser_trie: ParserTrie,
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

    /// Add multiple classes for an element (useful for gradient combinations)
    pub fn add_classes_for_element(&mut self, classes: &[&str]) -> Result<()> {
        use super::generator_operations::CssGeneratorOperations;
        <Self as CssGeneratorOperations>::add_classes_for_element(self, classes)
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

        // Handle gradient stops specially (with or without variants)
        if let Some(stop_type) = Self::extract_gradient_stop_type(&base_class) {
            if let Some(color) = Self::extract_gradient_color(&base_class, stop_type) {
                // Build selector with variants - use Tailwind's format: .escaped-class-name:modifiers
                let escaped_class = class.replace(":", "\\:");
                let mut selector = format!(".{}", escaped_class);

                // Add variant modifiers
                for variant in &variants {
                    match variant.as_str() {
                        "hover" => selector.push_str(":hover"),
                        "focus" => selector.push_str(":focus"),
                        "active" => selector.push_str(":active"),
                        "visited" => selector.push_str(":visited"),
                        "disabled" => selector.push_str(":disabled"),
                        "first" => selector.push_str(":first-child"),
                        "last" => selector.push_str(":last-child"),
                        "odd" => selector.push_str(":nth-child(odd)"),
                        "even" => selector.push_str(":nth-child(even)"),
                        _ => {} // Other variants handled via media queries or class selectors
                    }
                }

                let properties = vec![super::types::CssProperty {
                    name: format!("--tw-gradient-{}", stop_type),
                    value: color,
                    important: false,
                }];

                return Ok(CssRule {
                    selector,
                    properties,
                    media_query: self.variant_parser.get_variant_media_query(&variants).map(|s| s.to_string()),
                    specificity: variants.len() as u32 * 10 + 10,
                });
            }
        }

        let properties = self.class_to_properties(&base_class)?;

        // Build selector with variants - use Tailwind's format: .escaped-class-name:modifiers
        let escaped_class = class.replace(":", "\\:");
        let mut selector = format!(".{}", escaped_class);

        // Add variant modifiers
        for variant in &variants {
            match variant.as_str() {
                "hover" => selector.push_str(":hover"),
                "focus" => selector.push_str(":focus"),
                "active" => selector.push_str(":active"),
                "visited" => selector.push_str(":visited"),
                "disabled" => selector.push_str(":disabled"),
                "first" => selector.push_str(":first-child"),
                "last" => selector.push_str(":last-child"),
                "odd" => selector.push_str(":nth-child(odd)"),
                "even" => selector.push_str(":nth-child(even)"),
                _ => {} // Other variants handled via media queries or class selectors
            }
        }

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

    /// Add a gradient stop to the current gradient context
    pub fn add_gradient_stop(&mut self, stop_type: &str, color: String) {
        match stop_type {
            "from" => self.element_context.gradients.from_color = Some(color),
            "via" => self.element_context.gradients.via_color = Some(color),
            "to" => self.element_context.gradients.to_color = Some(color),
            _ => {}
        }
    }

    /// Generate gradient CSS using current context and direction
    pub fn generate_gradient_css(&mut self, direction: &str) -> Option<String> {
        self.element_context.gradients.direction = Some(direction.to_string());

        let mut colors = Vec::new();

        // Add colors in order: from, via, to
        if let Some(from) = &self.element_context.gradients.from_color {
            colors.push(from.clone());
        }
        if let Some(via) = &self.element_context.gradients.via_color {
            colors.push(via.clone());
        }
        if let Some(to) = &self.element_context.gradients.to_color {
            colors.push(to.clone());
        }

        // If no colors collected, return None (let fallback handle it)
        if colors.is_empty() {
            return None;
        }

        // Generate the gradient CSS
        let gradient_css = format!("linear-gradient({}, {})", direction, colors.join(", "));

        // Reset context for next gradient
        self.element_context = ElementContext::default();

        Some(gradient_css)
    }

    /// Clear element context (useful for resetting between elements)
    pub fn clear_element_context(&mut self) {
        self.element_context = ElementContext::default();
    }

    /// Process element classes using element-based processing (new architecture)
    /// This method handles complex class combinations including gradients, shadows, transforms, filters, animations, and arbitrary values
    pub fn process_element_classes(&mut self, classes: &[&str]) -> String {
        // Reset element context for new element
        self.element_context = ElementContext::default();

        // First pass: collect stateful information from all classes
        for class in classes {
            self.element_context.update_from_class(class);
        }

        // Generate variant-aware CSS rules for all stateful classes
        let mut css_output = String::new();

        // Generate CSS for gradients
        if self.element_context.gradients.has_gradient() {
            let gradient_rules = self.element_context.generate_variant_css("bg-gradient-to-r");
            for rule in gradient_rules {
                css_output.push_str(&super::css_output::CssOutputGenerator::rule_to_css(&rule));
            }
        }

        // Generate CSS for shadows
        if self.element_context.shadows.has_shadow() {
            let shadow_rules = self.element_context.generate_variant_css("shadow-lg");
            for rule in shadow_rules {
                css_output.push_str(&super::css_output::CssOutputGenerator::rule_to_css(&rule));
            }
        }

        // Generate CSS for transforms
        if self.element_context.transforms.has_transform() {
            let transform_rules = self.element_context.generate_variant_css("scale-110");
            for rule in transform_rules {
                css_output.push_str(&super::css_output::CssOutputGenerator::rule_to_css(&rule));
            }
        }

        // Generate CSS for filters
        let filter_properties = self.element_context.filters.to_css_properties();
        if !filter_properties.is_empty() {
            let filter_rules = self.element_context.generate_variant_css("blur-md");
            for rule in filter_rules {
                css_output.push_str(&super::css_output::CssOutputGenerator::rule_to_css(&rule));
            }
        }

        // Generate CSS for animations
        let animation_properties = self.element_context.animations.to_css_properties();
        if !animation_properties.is_empty() {
            let animation_rules = self.element_context.generate_variant_css("animate-spin");
            for rule in animation_rules {
                css_output.push_str(&super::css_output::CssOutputGenerator::rule_to_css(&rule));
            }
        }

        // Generate CSS for arbitrary values
        let arbitrary_properties = self.element_context.arbitrary_values.to_css_properties();
        if !arbitrary_properties.is_empty() {
            // For arbitrary values, we need to create a representative class
            let arbitrary_rules = self.element_context.generate_variant_css("w-[100px]");
            for rule in arbitrary_rules {
                css_output.push_str(&super::css_output::CssOutputGenerator::rule_to_css(&rule));
            }
        }

        // Second pass: handle non-stateful classes using existing logic
        let mut non_stateful_css = String::new();
        for class in classes {
            // Skip classes that were handled by element context
            let base_class = if class.contains(':') {
                class.split(':').last().unwrap_or(class)
            } else {
                class
            };

            let is_stateful = base_class.starts_with("bg-gradient-") ||
                             base_class.starts_with("from-") ||
                             base_class.starts_with("via-") ||
                             base_class.starts_with("to-") ||
                             base_class.starts_with("shadow-") ||
                             base_class.starts_with("scale-") ||
                             base_class.starts_with("rotate-") ||
                             base_class.starts_with("translate-") ||
                             base_class.starts_with("skew-") ||
                             base_class.starts_with("blur-") ||
                             base_class.starts_with("brightness-") ||
                             base_class.starts_with("contrast-") ||
                             base_class.starts_with("grayscale") ||
                             base_class.starts_with("hue-rotate-") ||
                             base_class.starts_with("invert") ||
                             base_class.starts_with("saturate-") ||
                             base_class.starts_with("sepia") ||
                             base_class.starts_with("drop-shadow") ||
                             base_class.starts_with("animate-") ||
                             base_class.starts_with("duration-") ||
                             base_class.starts_with("delay-") ||
                             base_class.starts_with("ease-") ||
                             base_class.contains('[');

            if !is_stateful {
                // Use existing class-to-css logic for non-stateful classes
                if let Ok(rule) = self.class_to_css_rule(class) {
                    non_stateful_css.push_str(&super::css_output::CssOutputGenerator::rule_to_css(&rule));
                    non_stateful_css.push('\n');
                }
            }
        }

        // Combine element-based CSS with traditional CSS
        css_output.push_str(&non_stateful_css);
        css_output.trim_end().to_string()
    }

}
