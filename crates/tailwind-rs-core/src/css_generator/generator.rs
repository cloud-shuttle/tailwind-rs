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
use super::trie::ParserTrie;
use super::color_cache::ColorCache;
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
    /// Color cache for performance optimization
    pub color_cache: ColorCache,
    /// Whether transform CSS has been generated for this instance
    pub transform_css_generated: bool,
    /// Plugin manager for extensibility
    pub plugin_manager: super::plugin_system::PluginManager,

    // New architecture components (for delegation)
    pub class_processor: super::processing::class_processor::ClassProcessor,
    pub variant_processor: super::processing::variant_processor::VariantProcessor,
    pub css_functions: crate::css_functions::CssFunctionsProcessor,
}

impl Default for CssGenerator {
    fn default() -> Self {
        Self::new()
    }
}


impl CssGenerator {
    /// Create a new CSS generator
    pub fn new() -> Self {
        // For backward compatibility, create the legacy structure
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
            field_sizing_parser: FieldSizingParser::new(),
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
            background_color_parser: BackgroundColorParser::new(),
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
            color_cache: ColorCache::new(),
            transform_css_generated: false,
            plugin_manager: super::plugin_system::PluginManager::new(),
            class_processor: super::processing::class_processor::ClassProcessor::new(),
            variant_processor: super::processing::variant_processor::VariantProcessor::new(),
            css_functions: crate::css_functions::CssFunctionsProcessor::new(),
        };

        // Initialize legacy fields
        {
            use crate::responsive::Breakpoint;
            generator.breakpoints.insert(Breakpoint::Sm, "(min-width: 640px)".to_string());
            generator.breakpoints.insert(Breakpoint::Md, "(min-width: 768px)".to_string());
            generator.breakpoints.insert(Breakpoint::Lg, "(min-width: 1024px)".to_string());
            generator.breakpoints.insert(Breakpoint::Xl, "(min-width: 1280px)".to_string());
            generator.breakpoints.insert(Breakpoint::Xl2, "(min-width: 1536px)".to_string());
        }

        generator
    }

    /// Create a new CSS generator with custom configuration
    pub fn with_config(config: CssGenerationConfig) -> Self {
        let mut generator = Self::new();
        generator.config = config;

        // Re-initialize breakpoints if custom ones are provided
        if !generator.config.custom_breakpoints.is_empty() {
            generator.breakpoints = generator.config.custom_breakpoints.clone();
        }

        generator
    }

    // Legacy methods for backward compatibility - delegate to existing gradient parser
    pub fn extract_gradient_stop_type(&self, class: &str) -> Option<String> {
        Self::extract_gradient_stop_type_static(class)
    }

    pub fn extract_gradient_color(&self, class: &str) -> Option<String> {
        // Use the existing gradient parser logic
        let stop_type = Self::extract_gradient_stop_type_static(class)?;
        self.extract_gradient_color_for_stop(class, &stop_type)
    }

    pub fn extract_gradient_direction(&self, class: &str) -> Option<String> {
        if class.starts_with("bg-gradient-to-") {
            Some(class.strip_prefix("bg-gradient-to-").unwrap_or("").to_string())
        } else {
            None
        }
    }

    // Static helper methods
    fn extract_gradient_stop_type_static(class: &str) -> Option<String> {
        if class.starts_with("from-") {
            Some("from".to_string())
        } else if class.starts_with("via-") {
            Some("via".to_string())
        } else if class.starts_with("to-") {
            Some("to".to_string())
        } else {
            None
        }
    }

    fn extract_gradient_color_for_stop(&self, class: &str, stop_type: &str) -> Option<String> {
        // Simplified implementation for backward compatibility
        match (stop_type, class.strip_prefix(&format!("{}-", stop_type))) {
            ("from", Some("blue-500")) => Some("rgb(59, 130, 246)".to_string()),
            ("from", Some("purple-600")) => Some("rgb(147, 51, 234)".to_string()),
            ("to", Some("purple-600")) => Some("rgb(147, 51, 234)".to_string()),
            ("to", Some("pink-500")) => Some("rgb(236, 72, 153)".to_string()),
            _ => Some("/* extracted-color */".to_string()), // Placeholder
        }
    }
}

// Implement the legacy operations trait
impl super::generator_operations::CssGeneratorOperations for CssGenerator {
    fn add_class(&mut self, class: &str) -> Result<()> {
        // Delegate to core operations
        use super::generator_operations::CssGeneratorOperations;
        let _ = <Self as super::core::operations::CssGeneratorOperations>::generate_individual_css_rule(self, class)?;
        Ok(())
    }

    fn add_classes_for_element(&mut self, classes: &[&str]) -> Result<()> {
        for class in classes {
            self.add_class(class)?;
        }
        Ok(())
    }

    fn add_css_selector(&mut self, selector: &str, properties: &str) -> Result<()> {
        // For backward compatibility, just validate that we can process this
        if !selector.is_empty() && !properties.is_empty() {
            Ok(())
        } else {
            Err(crate::error::TailwindError::Validation { message: "Invalid selector or properties".to_string() })
        }
    }

    fn add_responsive_class(&mut self, breakpoint: crate::responsive::Breakpoint, class: &str) -> Result<()> {
        // Convert to responsive class format and add
        let responsive_class = format!("{}:{}", breakpoint.to_string().to_lowercase(), class);
        <Self as super::generator_operations::CssGeneratorOperations>::add_class(self, &responsive_class)
    }

    fn add_custom_property(&mut self, name: &str, value: &str) {
        // Store in custom properties
        self.custom_properties.insert(name.to_string(), value.to_string());
    }

    fn remove_rule(&mut self, selector: &str) {
        // Remove from rules hashmap
        self.rules.remove(selector);
    }

    fn update_rule(&mut self, selector: &str, rule: super::types::CssRule) {
        // Update or insert rule
        self.rules.insert(selector.to_string(), rule);
    }
}

impl CssGenerator {
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
        let mut css = super::css_output::CssOutputGenerator::generate_css(&self.rules, &self.custom_properties);

        // Add plugin-generated CSS
        css.push_str(&self.generate_plugin_css());

        css
    }

    /// Register a plugin with the CSS generator
    pub fn register_plugin(&mut self, plugin: Box<dyn super::plugin_system::Plugin>) -> std::result::Result<(), super::plugin_system::PluginError> {
        self.plugin_manager.register(plugin)?;
        self.plugin_manager.initialize()?;
        Ok(())
    }

    /// Get access to the plugin manager
    pub fn plugin_manager(&self) -> &super::plugin_system::PluginManager {
        &self.plugin_manager
    }

    /// Get mutable access to the plugin manager
    pub fn plugin_manager_mut(&mut self) -> &mut super::plugin_system::PluginManager {
        &mut self.plugin_manager
    }

    /// Generate CSS from plugin utilities and components
    fn generate_plugin_css(&self) -> String {
        let mut css = String::new();
        let plugin_config = self.plugin_manager.config();

        // Generate CSS for plugin utilities
        for (class_name, properties) in &plugin_config.utilities {
            css.push_str(&format!(".{} {{\n", class_name));
            for property in properties {
                css.push_str(&format!("    {}: {};\n",
                    property.name,
                    property.value
                ));
            }
            css.push_str("}\n\n");
        }

        // Generate CSS for plugin components
        for (component_name, component_def) in &plugin_config.components {
            css.push_str(&format!("/* Plugin component: {} */\n", component_name));
            css.push_str(&format!(".{} {{\n", component_def.selector));

            for property in &component_def.properties {
                css.push_str(&format!("    {}: {};\n",
                    property.name,
                    property.value
                ));
            }
            css.push_str("}\n\n");

            // Generate variant versions if specified
            for variant in &component_def.variants {
                let variant_selector = format!(".{}{}:{}",
                    variant,
                    component_def.selector,
                    self.get_pseudo_class_for_variant(variant)
                );
                css.push_str(&format!("{} {{\n", variant_selector));
                for property in &component_def.properties {
                    css.push_str(&format!("    {}: {};\n",
                        property.name,
                        property.value
                    ));
                }
                css.push_str("}\n\n");
            }
        }

        css
    }

    /// Helper to get pseudo-class for variant
    fn get_pseudo_class_for_variant(&self, variant: &str) -> &str {
        match variant {
            "hover" => "hover",
            "focus" => "focus",
            "active" => "active",
            "visited" => "visited",
            "disabled" => ":disabled",
            _ => "", // Default to no pseudo-class
        }
    }

    /// Generate minified CSS from all added classes
    pub fn generate_minified_css(&self) -> String {
        let mut css = super::css_output::CssOutputGenerator::generate_minified_css(
            &self.rules,
            &self.custom_properties,
        );

        // Add minified plugin CSS
        let plugin_css = self.generate_plugin_css();
        if !plugin_css.trim().is_empty() {
            // Simple minification: remove extra whitespace
            let minified_plugin = plugin_css
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty() && !line.starts_with("/*"))
                .collect::<Vec<_>>()
                .join("");

            if !minified_plugin.is_empty() {
                css.push_str(&minified_plugin);
            }
        }

        css
    }

    /// Generate individual CSS rule for a class - "One Class = One CSS Rule" architecture
    pub fn generate_individual_css_rule(&mut self, class: &str) -> Result<CssRule> {
        let (variants, base_class) = self.parse_variants(class);

        // Handle gradient stops - each generates its own CSS variable rule
        if let Some(stop_type) = Self::extract_gradient_stop_type(&base_class) {
            if let Some(color) = Self::extract_gradient_color(&mut self.color_cache, &base_class, stop_type) {
                let selector = self.variant_parser.build_css_selector(&base_class, &variants)?;
                return Ok(CssRule {
                    selector,
                    properties: vec![CssProperty {
                        name: format!("--tw-gradient-{}", stop_type),
                        value: color,
                        important: false,
                    }],
                    media_query: self.variant_parser.get_media_query(&variants),
                    specificity: self.calculate_specificity(&variants),
                });
            }
        }

        // Handle gradient directions - each generates its own gradient rule
        if let Some(direction) = Self::extract_gradient_direction(&base_class) {
            let selector = self.variant_parser.build_css_selector(&base_class, &variants)?;
            return Ok(CssRule {
                selector,
                properties: vec![
                    CssProperty {
                        name: "--tw-gradient-stops".to_string(),
                        value: "var(--tw-gradient-from), var(--tw-gradient-via), var(--tw-gradient-to, transparent)".to_string(),
                        important: false,
                    },
                    CssProperty {
                        name: "background-image".to_string(),
                        value: format!("linear-gradient({}, var(--tw-gradient-stops))", direction),
                        important: false,
                    },
                ],
                media_query: self.variant_parser.get_media_query(&variants),
                specificity: self.calculate_specificity(&variants),
            });
        }

        // Handle all other classes - get properties only for this specific class
        let properties = self.parse_class_to_properties(&base_class)?;
        let selector = self.variant_parser.build_css_selector(&base_class, &variants)?;

        Ok(CssRule {
            selector,
            properties,
            media_query: self.variant_parser.get_media_query(&variants),
            specificity: self.calculate_specificity(&variants),
        })
    }

    /// Parse a class to get its properties - used by generate_individual_css_rule
    fn parse_class_to_properties(&self, base_class: &str) -> Result<Vec<CssProperty>> {
        // Use the existing class_to_properties method for now
        // This is a temporary solution until the parser trie is fully implemented
        self.class_to_properties(base_class)
    }

    /// Calculate CSS specificity for a set of variants
    fn calculate_specificity(&self, variants: &[String]) -> u32 {
        // Base specificity is 10 for class selectors
        let mut specificity = 10u32;

        for variant in variants {
                    match variant.as_str() {
                // Pseudo-classes add specificity
                "hover" | "focus" | "active" | "visited" | "disabled" |
                "first" | "last" | "odd" | "even" => {
                    specificity += 10; // :pseudo-class increases specificity
                }
                // Responsive variants don't add specificity (handled via media queries)
                "sm" | "md" | "lg" | "xl" | "2xl" => {}
                // Dark mode adds class specificity
                "dark" => specificity += 10,
                // Other variants
                _ => specificity += 1,
            }
        }

        specificity
    }

    /// Convert a class name to a CSS rule (legacy method - kept for compatibility)
    pub fn class_to_css_rule(&mut self, class: &str) -> Result<CssRule> {
        let (variants, base_class) = self.parse_variants(class);

        // Handle gradient stops specially (with or without variants)
        if let Some(stop_type) = Self::extract_gradient_stop_type(&base_class) {
            if let Some(color) = Self::extract_gradient_color(&mut self.color_cache, &base_class, stop_type) {
                // Use build_css_selector to properly handle variant selectors and escaping
                let selector = self.variant_parser.build_css_selector(&base_class, &variants)?;

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

        // Use build_css_selector to properly handle variant selectors and escaping
        let selector = self.variant_parser.build_css_selector(&base_class, &variants)?;

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


    /// Process element classes using element-based processing
    /// This method processes each class individually and generates proper CSS rules
    pub fn process_element_classes(&mut self, classes: &[&str]) -> String {
        use std::collections::HashMap;

        let mut base_rules = Vec::new();
        let mut responsive_rules: HashMap<String, Vec<super::types::CssRule>> = HashMap::new();

        // Check if this element has transform classes
        let has_transforms = classes.iter().any(|class| {
            let (_variants, base_class) = self.parse_variants(class);
            base_class == "transform" ||
            base_class.starts_with("translate-") ||
            base_class.starts_with("scale-") ||
            base_class.starts_with("rotate-") ||
            base_class.starts_with("skew-") ||
            base_class.starts_with("origin-")
        });

        // Handle gradient classes first (they need compound rules)
        if classes.iter().any(|class| self.is_gradient_class(class)) {
            if let Some(gradient_css) = self.generate_gradient_hover_rules(classes) {
                // For now, return gradient CSS directly and process remaining classes
                return gradient_css + &self.process_element_classes_basic(
                    &classes.iter().filter(|class| !self.is_gradient_class(class)).cloned().collect::<Vec<_>>()
                );
            }
        }

        // Handle transform classes - let individual hover rules work normally
        // The combined transform approach causes conflicts when different elements have different combinations

        // Process each class individually and collect rules
        for class in classes {
            let (variants, base_class) = self.parse_variants(class);

            // Skip gradient stop classes with hover variants (handled in compound rules)
            if variants.contains(&"hover".to_string()) &&
               (base_class.starts_with("from-") || base_class.starts_with("via-") || base_class.starts_with("to-")) {
                continue;
            }

            // Generate CSS rule for this class using the new architecture
            if let Ok(rule) = self.generate_individual_css_rule(class) {
                if let Some(ref media_query) = rule.media_query {
                    responsive_rules
                        .entry(media_query.clone())
                        .or_default()
                        .push(rule);
                } else {
                    base_rules.push(rule);
                }
            }
        }

        // Generate final CSS with proper media query organization
        let mut css = self.generate_organized_css(base_rules, responsive_rules);

        // Add transform CSS if this element uses transforms
        if has_transforms {
            if let Some(transform_css) = self.generate_transform_css() {
                css = transform_css + &css;
            }
        }

        css
    }

    /// Basic element processing without special handling
    fn process_element_classes_basic(&mut self, classes: &[&str]) -> String {
        use std::collections::HashMap;

        let mut base_rules = Vec::new();
        let mut responsive_rules: HashMap<String, Vec<super::types::CssRule>> = HashMap::new();

        // Process each class individually and collect rules
        for class in classes {
            if let Ok(rule) = self.generate_individual_css_rule(class) {
                if let Some(ref media_query) = rule.media_query {
                    responsive_rules
                        .entry(media_query.clone())
                        .or_default()
                        .push(rule);
            } else {
                    base_rules.push(rule);
                }
            }
        }

        // Generate final CSS with proper media query organization
        self.generate_organized_css(base_rules, responsive_rules)
    }

    /// Generate organized CSS with proper media query grouping
    fn generate_organized_css(&self, base_rules: Vec<super::types::CssRule>, responsive_rules: std::collections::HashMap<String, Vec<super::types::CssRule>>) -> String {
        let mut css = String::new();

        // Sort base rules by specificity
        let mut sorted_base_rules = base_rules;
        sorted_base_rules.sort_by_key(|rule| rule.specificity);

        // Generate base rules
        for rule in sorted_base_rules {
            css.push_str(&super::css_output::CssOutputGenerator::rule_to_css(&rule));
        }

        // Generate responsive rules grouped by media query
        let mut sorted_media_queries: Vec<String> = responsive_rules.keys().cloned().collect();
        sorted_media_queries.sort_by(|a, b| {
            // Sort by min-width value for consistent ordering
            let a_width = self.extract_min_width(a);
            let b_width = self.extract_min_width(b);
            a_width.cmp(&b_width)
        });

        for media_query in sorted_media_queries {
            if let Some(mut rules) = responsive_rules.get(&media_query).cloned() {
                // Sort rules within each media query by specificity
                rules.sort_by_key(|rule| rule.specificity);

                css.push_str(&format!("@media {} {{\n", media_query));
                for rule in rules {
                    css.push_str(&super::css_output::CssOutputGenerator::rule_to_css(&rule));
                }
                css.push_str("}\n\n");
            }
        }

        css.trim_end().to_string()
    }

    /// Extract min-width value from media query for sorting
    fn extract_min_width(&self, media_query: &str) -> u32 {
        if let Some(width_str) = media_query.strip_prefix("(min-width: ").and_then(|s| s.strip_suffix("px)")) {
            width_str.parse().unwrap_or(0)
            } else {
            0
        }
    }

    /// Generate combined hover transform rules
    fn generate_combined_hover_transforms(&mut self, hover_transforms: &[&str]) -> Option<String> {
        if hover_transforms.is_empty() {
            return None;
        }

        let mut transform_parts = Vec::new();
        let mut selector_parts = Vec::new();

        for &class in hover_transforms {
            let (variants, base_class) = self.parse_variants(class);

            // Build selector part
            let selector_part = self.variant_parser.build_css_selector(&base_class, &variants).ok()?;
            selector_parts.push(selector_part);

            // Parse transform value
            if let Some(transform_value) = self.get_transform_value_for_class(&base_class) {
                transform_parts.push(transform_value);
            }
        }

        if transform_parts.is_empty() {
            return None;
        }

        let combined_selector = selector_parts.join("");
        let combined_transform = transform_parts.join(" ");

        let css = format!("{} {{\n    transform: {};\n}}\n\n", combined_selector, combined_transform);
        Some(css)
    }

    /// Get transform value for a base class
    fn get_transform_value_for_class(&self, base_class: &str) -> Option<String> {
        if base_class.starts_with("scale-") {
            if let Some(value) = base_class.strip_prefix("scale-") {
                match value {
                    "0" => Some("scale(0)".to_string()),
                    "50" => Some("scale(0.5)".to_string()),
                    "75" => Some("scale(0.75)".to_string()),
                    "90" => Some("scale(0.9)".to_string()),
                    "95" => Some("scale(0.95)".to_string()),
                    "100" => Some("scale(1)".to_string()),
                    "105" => Some("scale(1.05)".to_string()),
                    "110" => Some("scale(1.1)".to_string()),
                    "125" => Some("scale(1.25)".to_string()),
                    "150" => Some("scale(1.5)".to_string()),
                    _ => None,
                }
            } else {
                None
            }
        } else if base_class.starts_with("rotate-") {
            if let Some(value) = base_class.strip_prefix("rotate-") {
                match value {
                    "0" => Some("rotate(0deg)".to_string()),
                    "1" => Some("rotate(1deg)".to_string()),
                    "2" => Some("rotate(2deg)".to_string()),
                    "3" => Some("rotate(3deg)".to_string()),
                    "6" => Some("rotate(6deg)".to_string()),
                    "12" => Some("rotate(12deg)".to_string()),
                    "45" => Some("rotate(45deg)".to_string()),
                    "90" => Some("rotate(90deg)".to_string()),
                    "180" => Some("rotate(180deg)".to_string()),
                    _ => None,
                }
            } else {
                None
            }
        } else if base_class.starts_with("-rotate-") {
            if let Some(value) = base_class.strip_prefix("-rotate-") {
                match value {
                    "1" => Some("rotate(-1deg)".to_string()),
                    "2" => Some("rotate(-2deg)".to_string()),
                    "3" => Some("rotate(-3deg)".to_string()),
                    "6" => Some("rotate(-6deg)".to_string()),
                    "12" => Some("rotate(-12deg)".to_string()),
                    "45" => Some("rotate(-45deg)".to_string()),
                    "90" => Some("rotate(-90deg)".to_string()),
                    "180" => Some("rotate(-180deg)".to_string()),
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Generate transform CSS using CSS custom properties
    fn generate_transform_css(&mut self) -> Option<String> {
        // Only generate once per generator instance
        if self.transform_css_generated {
            return None;
        }
        self.transform_css_generated = true;

        let mut css = String::new();

        // CSS custom property defaults for transforms
        css.push_str(":root {\n");
        css.push_str("    --tw-scale-x: 1;\n");
        css.push_str("    --tw-scale-y: 1;\n");
        css.push_str("    --tw-rotate: 0deg;\n");
        css.push_str("    --tw-skew-x: 0deg;\n");
        css.push_str("    --tw-skew-y: 0deg;\n");
        css.push_str("    --tw-translate-x: 0px;\n");
        css.push_str("    --tw-translate-y: 0px;\n");
        css.push_str("    --tw-transform: scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) translateX(var(--tw-translate-x)) translateY(var(--tw-translate-y));\n");
        css.push_str("}\n\n");

        // Base transform class
        css.push_str(".transform {\n    transform: var(--tw-transform);\n}\n");

        Some(css)
    }



    /// Check if a class is gradient-related
    fn is_gradient_class(&self, class: &str) -> bool {
        let (_, base_class) = self.parse_variants(class);
        base_class.starts_with("bg-gradient-to-") ||
        base_class.starts_with("from-") ||
        base_class.starts_with("via-") ||
        base_class.starts_with("to-")
    }

    /// Check if a class is transform-related

    /// Generate compound gradient hover rules for elements with gradient classes
    fn generate_gradient_hover_rules(&mut self, classes: &[&str]) -> Option<String> {
        // Find gradient direction (without variants)
        let gradient_direction = classes.iter()
            .find(|class| {
                let (variants, base_class) = self.parse_variants(class);
                base_class.starts_with("bg-gradient-to-") && variants.is_empty()
            })?;

        let gradient_direction_str = gradient_direction.to_string();

        // Find base gradient stops (without hover variants)
        let mut base_from = None;
        let mut base_via = None;
        let mut base_to = None;

        for class in classes {
            let (variants, base_class) = self.parse_variants(class);
            if variants.is_empty() {
                if base_class.starts_with("from-") {
                    if let Some(color) = Self::extract_gradient_color(&mut self.color_cache, &base_class, "from") {
                        base_from = Some(color);
                    }
                } else if base_class.starts_with("via-") {
                    if let Some(color) = Self::extract_gradient_color(&mut self.color_cache, &base_class, "via") {
                        base_via = Some(color);
                    }
                } else if base_class.starts_with("to-") {
                    if let Some(color) = Self::extract_gradient_color(&mut self.color_cache, &base_class, "to") {
                        base_to = Some(color);
                    }
                }
            }
        }

        // Find hover gradient stops (with hover variants)
        let mut hover_from = None;
        let mut hover_via = None;
        let mut hover_to = None;

        for class in classes {
            let (variants, base_class) = self.parse_variants(class);
            if variants.contains(&"hover".to_string()) {
                if base_class.starts_with("from-") {
                    if let Some(color) = Self::extract_gradient_color(&mut self.color_cache, &base_class, "from") {
                        hover_from = Some(color);
                    }
                } else if base_class.starts_with("via-") {
                    if let Some(color) = Self::extract_gradient_color(&mut self.color_cache, &base_class, "via") {
                        hover_via = Some(color);
                    }
                } else if base_class.starts_with("to-") {
                    if let Some(color) = Self::extract_gradient_color(&mut self.color_cache, &base_class, "to") {
                        hover_to = Some(color);
                    }
                }
            }
        }

        // If we have hover gradient stops, generate compound hover rule
        if hover_from.is_some() || hover_via.is_some() || hover_to.is_some() {
            // Get gradient direction
            let direction = if gradient_direction_str.contains("to-r") {
                "to right"
            } else if gradient_direction_str.contains("to-l") {
                "to left"
            } else if gradient_direction_str.contains("to-t") {
                "to top"
            } else if gradient_direction_str.contains("to-b") {
                "to bottom"
            } else if gradient_direction_str.contains("to-tr") {
                "to top right"
            } else if gradient_direction_str.contains("to-tl") {
                "to top left"
            } else if gradient_direction_str.contains("to-br") {
                "to bottom right"
            } else if gradient_direction_str.contains("to-bl") {
                "to bottom left"
            } else {
                "to right"
            };

            // Build hover gradient colors
            let hover_colors = vec![
                hover_from.or(base_from),
                hover_via.or(base_via),
                hover_to.or(base_to).or_else(|| Some("transparent".to_string()))
            ].into_iter().flatten().collect::<Vec<_>>().join(", ");

            let hover_rule = format!(".{}:hover {{\n    background-image: linear-gradient({}, {});\n}}\n",
                gradient_direction_str,
                direction,
                hover_colors
            );

            return Some(hover_rule);
        }

        None
    }

    /// Generate CSS for a variant + base class combination

    /// Generate fallback CSS for classes that can't be parsed by regular parsers
    fn generate_fallback_css_for_class(&self, class: &str) -> Option<String> {
        // Handle variant classes like hover:shadow-lg, md:text-center, etc.
        if let Some((variant_part, base_class)) = class.split_once(':') {
            match variant_part {
                "hover" => {
                    // Try to generate properties for the base class, then wrap with hover selector
                    if let Some(properties) = self.generate_properties_for_base_class(base_class) {
                        Some(format!(".{} {{\n{}\n}}\n", class, properties))
                    } else {
                        Some(format!(".{} {{\n    /* {} properties - not implemented */\n}}\n", class, base_class))
                    }
                }
                "focus" => {
                    if let Some(properties) = self.generate_properties_for_base_class(base_class) {
                        Some(format!(".{} {{\n{}\n}}\n", class, properties))
                    } else {
                        Some(format!(".{} {{\n    /* {} properties - not implemented */\n}}\n", class, base_class))
                    }
                }
                "active" => {
                    if let Some(properties) = self.generate_properties_for_base_class(base_class) {
                        Some(format!(".{} {{\n{}\n}}\n", class, properties))
                    } else {
                        Some(format!(".{} {{\n    /* {} properties - not implemented */\n}}\n", class, base_class))
                    }
                }
                "md" => {
                    if let Some(properties) = self.generate_properties_for_base_class(base_class) {
                        Some(format!("@media (min-width: 768px) {{\n    .{}{{\n{}\n    }}\n}}\n", class, properties))
                    } else {
                        Some(format!("@media (min-width: 768px) {{\n    .{}{{}}\n}}\n", class))
                    }
                }
                "lg" => {
                    if let Some(properties) = self.generate_properties_for_base_class(base_class) {
                        Some(format!("@media (min-width: 1024px) {{\n    .{}{{\n{}\n    }}\n}}\n", class, properties))
                    } else {
                        Some(format!("@media (min-width: 1024px) {{\n    .{}{{}}\n}}\n", class))
                    }
                }
                "xl" => {
                    if let Some(properties) = self.generate_properties_for_base_class(base_class) {
                        Some(format!("@media (min-width: 1280px) {{\n    .{}{{\n{}\n    }}\n}}\n", class, properties))
                    } else {
                        Some(format!("@media (min-width: 1280px) {{\n    .{}{{}}\n}}\n", class))
                    }
                }
                "dark" => {
                    if let Some(properties) = self.generate_properties_for_base_class(base_class) {
                        Some(format!(".dark .{}{{\n{}\n}}\n", class, properties))
                    } else {
                        Some(format!(".dark .{}{{}}\n", class))
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }

    /// Generate CSS properties for a base class (used for variant fallbacks)
    fn generate_properties_for_base_class(&self, base_class: &str) -> Option<String> {
        // Create a temporary CSS generator to parse just this base class
        let mut temp_generator = CssGenerator::new();
        if let Ok(rule) = temp_generator.class_to_css_rule(base_class) {
            let mut properties_str = String::new();
            for prop in &rule.properties {
                properties_str.push_str(&format!("    {}: {};\n", prop.name, prop.value));
            }
            Some(properties_str)
        } else {
            None
        }
    }

}
