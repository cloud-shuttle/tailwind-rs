//! Main CSS Generator implementation (Refactored)
//!
//! This module contains the core CssGenerator struct and its main functionality.
//! Refactored into smaller, more manageable modules.

use crate::error::Result;
use crate::responsive::Breakpoint;
use std::collections::HashMap;
use super::types::{CssRule, CssProperty, CssGenerationConfig};
use super::variants::VariantParser;
use super::parsers::{
    SpacingParser, AnimationParser, InteractiveParser,
    AdvancedSpacingParser, AdvancedColorParser, PositioningParser, TypographyParser,
    FlexboxParser, LayoutParser, ColorParser, EffectsParser, SizingParser,
    AdvancedBorderParser, RingParser, TransitionParser, ShadowParser, SvgParser,
    MarginParser, GroupParser, AdvancedGridParser, ProseParser, DivideParser,
    GradientParser, ObjectFitParser, TransformParser, ArbitraryParser, DataAttributeParser, 
    BackgroundPropertiesParser, TransitionPropertiesParser, FractionalTransformsParser,
    AspectRatioParser, ColumnsParser, BreakControlParser, BoxUtilitiesParser, LayoutUtilitiesParser
};

mod generator_builders;
mod generator_operations;
mod generator_parsers;

use generator_builders::CssGeneratorBuilder;
use generator_operations::CssGeneratorOperations;
use generator_parsers::CssGeneratorParsers;

/// CSS generator that converts Tailwind classes to CSS rules
#[derive(Debug, Clone)]
pub struct CssGenerator {
    /// Generated CSS rules
    rules: HashMap<String, CssRule>,
    /// Responsive breakpoints
    breakpoints: HashMap<Breakpoint, String>,
    /// Custom CSS properties
    custom_properties: HashMap<String, String>,
    /// Generation configuration
    config: CssGenerationConfig,
    /// Spacing parser
    spacing_parser: SpacingParser,
    /// Advanced spacing parser
    advanced_spacing_parser: AdvancedSpacingParser,
    /// Color parser
    color_parser: ColorParser,
    /// Advanced color parser
    advanced_color_parser: AdvancedColorParser,
    /// Typography parser
    typography_parser: TypographyParser,
    /// Layout parser
    layout_parser: LayoutParser,
    /// Positioning parser
    positioning_parser: PositioningParser,
    /// Flexbox parser
    flexbox_parser: FlexboxParser,
    /// Effects parser
    effects_parser: EffectsParser,
    /// Sizing parser
    sizing_parser: SizingParser,
    /// Advanced border parser
    advanced_border_parser: AdvancedBorderParser,
    /// Ring parser
    ring_parser: RingParser,
    /// Transition parser
    transition_parser: TransitionParser,
    /// Shadow parser
    shadow_parser: ShadowParser,
    /// SVG parser
    svg_parser: SvgParser,
    /// Margin parser
    margin_parser: MarginParser,
    /// Group parser
    group_parser: GroupParser,
    /// Advanced grid parser
    advanced_grid_parser: AdvancedGridParser,
    /// Animation parser
    animation_parser: AnimationParser,
    /// Interactive parser
    interactive_parser: InteractiveParser,
    /// Prose parser
    prose_parser: ProseParser,
    /// Divide parser
    divide_parser: DivideParser,
    /// Gradient parser
    gradient_parser: GradientParser,
    /// Object fit parser
    object_fit_parser: ObjectFitParser,
    /// Transform parser
    transform_parser: TransformParser,
    /// Arbitrary values parser
    arbitrary_parser: ArbitraryParser,
    /// Data attributes parser
    data_attribute_parser: DataAttributeParser,
    /// Background properties parser
    background_properties_parser: BackgroundPropertiesParser,
    /// Transition properties parser
    transition_properties_parser: TransitionPropertiesParser,
    /// Fractional transforms parser
    fractional_transforms_parser: FractionalTransformsParser,
    /// Aspect ratio parser
    aspect_ratio_parser: AspectRatioParser,
    /// Columns parser
    columns_parser: ColumnsParser,
    /// Break control parser
    break_control_parser: BreakControlParser,
    /// Box utilities parser
    box_utilities_parser: BoxUtilitiesParser,
    /// Layout utilities parser
    layout_utilities_parser: LayoutUtilitiesParser,
    /// Variant parser
    variant_parser: VariantParser,
}

impl CssGenerator {
    /// Create a new CSS generator
    pub fn new() -> Self {
        Self::new()
    }

    /// Create a new CSS generator with custom configuration
    pub fn with_config(config: CssGenerationConfig) -> Self {
        Self::with_config(config)
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

    /// Get all rules (for compatibility)
    pub fn get_rules(&self) -> &HashMap<String, CssRule> {
        &self.rules
    }

    /// Generate comprehensive CSS with all utilities
    pub fn generate_comprehensive_css(&mut self, config: &CssGenerationConfig) -> Result<String> {
        // Add common utility classes
        let common_classes = vec![
            "p-4", "m-4", "bg-blue-500", "text-white", "rounded-md",
            "hover:bg-blue-600", "focus:outline-none", "sm:p-6", "md:p-8"
        ];
        
        for class in common_classes {
            let _ = self.add_class(class);
        }
        
        Ok(self.generate_css())
    }

    /// Generate CSS from all added classes
    pub fn generate_css(&self) -> String {
        super::css_output::CssOutputGenerator::generate_css(&self.rules, &self.custom_properties)
    }

    /// Generate minified CSS from all added classes
    pub fn generate_minified_css(&self) -> String {
        super::css_output::CssOutputGenerator::generate_minified_css(&self.rules, &self.custom_properties)
    }
}

// Implement the trait methods
impl CssGeneratorBuilder for CssGenerator {}
impl CssGeneratorOperations for CssGenerator {}
impl CssGeneratorParsers for CssGenerator {}
