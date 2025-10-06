//! CSS Generator Builder Methods
//!
//! This module contains the constructor and initialization methods for CssGenerator.

use super::parsers::{
    AccentColorParser, AccessibilityParser, AdvancedBorderParser, AdvancedColorParser,
    AdvancedGridParser, AdvancedSpacingParser, AlignContentParser, AlignItemsParser,
    AlignSelfParser, AnimationParser, ArbitraryParser, AspectRatioParser,
    BackdropFilterUtilitiesParser, BackgroundPropertiesParser, FieldSizingParser,
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
    ScaleParser, ShadowParser, SizingParser, SpacingParser, SvgParser, TableParser,
    TransitionParser, TransitionPropertiesParser, TypographyParser, VisibilityParser, ZIndexParser,
};
use crate::background::color::BackgroundColorParser;
use super::types::CssGenerationConfig;
use super::variants::VariantParser;
use super::trie::{ParserTrie, ParserType};
use super::color_cache::ColorCache;
use crate::transforms::TransformParser;
use crate::responsive::Breakpoint;
use std::collections::HashMap;

// Legacy builder trait - delegates to core::builders::CssGeneratorBuilder
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
        // Delegate to the new core builder
        use super::core::builders::CssGeneratorBuilder;
        <Self as super::core::builders::CssGeneratorBuilder>::new()
    }

    fn with_config(config: CssGenerationConfig) -> Self {
        // Delegate to the new core builder
        use super::core::builders::CssGeneratorBuilder;
        <Self as super::core::builders::CssGeneratorBuilder>::with_config(config)
    }

    fn initialize_default_breakpoints(&mut self) {
        // Delegate to core implementation if available, otherwise fallback
        if let Some(ref mut breakpoints) = self.breakpoints.as_mut() {
            breakpoints.insert(Breakpoint::Sm, "(min-width: 640px)".to_string());
            breakpoints.insert(Breakpoint::Md, "(min-width: 768px)".to_string());
            breakpoints.insert(Breakpoint::Lg, "(min-width: 1024px)".to_string());
            breakpoints.insert(Breakpoint::Xl, "(min-width: 1280px)".to_string());
            breakpoints.insert(Breakpoint::Xl2, "(min-width: 1536px)".to_string());
        }
    }

    fn initialize_custom_breakpoints(&mut self) {
        // Delegate to core implementation if available, otherwise fallback
        if let Some(ref mut breakpoints) = self.breakpoints.as_mut() {
            if !self.config.custom_breakpoints.is_empty() {
                *breakpoints = self.config.custom_breakpoints.clone();
            } else {
                self.initialize_default_breakpoints();
            }
        }
    }

    fn initialize_parser_trie(&mut self) {
        // In the new architecture, parser initialization is handled by the class_processor
        // This method is kept for backward compatibility but delegates to the new system
        // The actual parser trie initialization happens in core/builders.rs
    }
}
