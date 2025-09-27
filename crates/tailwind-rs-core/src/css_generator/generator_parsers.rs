//! CSS Generator Parser Methods
//!
//! This module contains all the parser delegation methods for CssGenerator.

use crate::error::{Result, TailwindError};
use super::types::CssProperty;
use super::parsers::{
    ColorParser, TypographyParser, LayoutParser, FlexboxParser, GridParser,
    BorderParser, EffectsParser, TransformParser, UtilityParser, OverflowParser,
    OverscrollParser, PositionParser, InsetParser, VisibilityParser, ZIndexParser,
    FlexBasisParser, FlexDirectionParser, FlexWrapParser, FlexParser, FlexGrowParser, FlexShrinkParser,
    OrderParser, GridTemplateColumnsParser, GridColumnParser, GridTemplateRowsParser, GridRowParser,
    GridAutoFlowParser, GridAutoColumnsParser, GridAutoRowsParser, GapParser, JustifyContentParser,
    JustifyItemsParser, JustifySelfParser, AlignContentParser, AlignItemsParser, AlignSelfParser,
    PlaceContentParser, PlaceItemsParser, PlaceSelfParser
};

/// Parser methods trait for CssGenerator
pub trait CssGeneratorParsers {
    /// Convert a class name to CSS properties
    fn class_to_properties(&self, class: &str) -> Result<Vec<CssProperty>>;
    
    /// Parse variants from a class name and return (variants, base_class)
    fn parse_variants(&self, class: &str) -> (Vec<String>, String);
    
    /// Convert a class name to a CSS rule
    fn class_to_css_rule(&self, class: &str) -> Result<super::types::CssRule>;
}

impl CssGeneratorParsers for super::CssGenerator {
    fn class_to_properties(&self, class: &str) -> Result<Vec<CssProperty>> {
        // First, parse variants and get the base class
        let (_variants, base_class) = self.parse_variants(class);
        
        // Try to parse the base class using comprehensive patterns
        // Try advanced parsers first (higher priority)
        if let Some(properties) = self.advanced_color_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.advanced_spacing_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.typography_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.positioning_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.flexbox_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.sizing_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.advanced_border_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.ring_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.transition_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.shadow_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.svg_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.margin_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.group_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.advanced_grid_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.prose_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.divide_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.gradient_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.object_fit_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.transform_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.arbitrary_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.data_attribute_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.background_properties_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.background_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.border_utilities_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.effects_utilities_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.filter_utilities_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.backdrop_filter_utilities_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.accessibility_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.table_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.mask_utilities_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.transition_properties_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.fractional_transforms_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.aspect_ratio_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.columns_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.break_control_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.box_utilities_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.layout_utilities_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.place_content_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.place_items_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.place_self_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        // Grid and Flexbox parsers
        if let Some(properties) = self.grid_template_columns_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.grid_column_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.grid_template_rows_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.grid_row_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.grid_auto_flow_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.grid_auto_columns_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.grid_auto_rows_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.gap_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        // Flexbox parsers
        if let Some(properties) = self.flex_basis_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.flex_direction_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.flex_wrap_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.flex_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.flex_grow_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.flex_shrink_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.order_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        // Justify and Align parsers
        if let Some(properties) = self.justify_content_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.justify_items_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.justify_self_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.align_content_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.align_items_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.align_self_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        // Layout parsers
        if let Some(properties) = self.overflow_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.overscroll_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.position_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.inset_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.visibility_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.z_index_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.typography_parser.parse_class(&base_class) {
            return Ok(properties);
        }
        
        // Try basic parsers
        if let Some(properties) = self.parse_spacing_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_color_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_typography_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_layout_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_flexbox_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_grid_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_border_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_effects_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_transform_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_animation_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_interactive_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_sizing_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_background_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_filter_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_transition_class(&base_class) {
            return Ok(properties);
        }
        
        // Try new layout parsers
        if let Some(properties) = self.parse_overflow_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_overscroll_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_position_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_inset_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_visibility_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_z_index_class(&base_class) {
            return Ok(properties);
        }
        
        // Try new flexbox parsers
        if let Some(properties) = self.parse_flex_basis_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_flex_direction_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_flex_wrap_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_flex_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_flex_grow_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_flex_shrink_class(&base_class) {
            return Ok(properties);
        }
        
        // Try new order and grid parsers
        if let Some(properties) = self.parse_order_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_grid_template_columns_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_grid_column_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_grid_template_rows_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_grid_row_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_grid_auto_flow_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_grid_auto_columns_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_grid_auto_rows_class(&base_class) {
            return Ok(properties);
        }
        
        // Try new gap and justify parsers
        if let Some(properties) = self.parse_gap_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_justify_content_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_justify_items_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_justify_self_class(&base_class) {
            return Ok(properties);
        }
        
        // Try new align and place parsers
        if let Some(properties) = self.parse_align_content_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_align_items_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_align_self_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_place_content_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_place_items_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_place_self_class(&base_class) {
            return Ok(properties);
        }
        
        // Fallback to hardcoded classes for backwards compatibility
        match base_class.as_str() {
            // Display utilities
            "block" => Ok(vec![CssProperty { name: "display".to_string(), value: "block".to_string(), important: false }]),
            "inline" => Ok(vec![CssProperty { name: "display".to_string(), value: "inline".to_string(), important: false }]),
            "flex" => Ok(vec![CssProperty { name: "display".to_string(), value: "flex".to_string(), important: false }]),
            "grid" => Ok(vec![CssProperty { name: "display".to_string(), value: "grid".to_string(), important: false }]),
            "hidden" => Ok(vec![CssProperty { name: "display".to_string(), value: "none".to_string(), important: false }]),
            
            _ => Err(TailwindError::class_generation(format!("Unknown class: {}", class))),
        }
    }
    
    fn parse_variants(&self, class: &str) -> (Vec<String>, String) {
        self.variant_parser.parse_variants(class)
    }
    
    fn class_to_css_rule(&self, class: &str) -> Result<super::types::CssRule> {
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
        let media_query = variants.iter()
            .find_map(|variant| {
                // Try responsive media query first
                if let Some(responsive_query) = self.variant_parser.get_responsive_media_query(variant) {
                    Some(responsive_query)
                } else {
                    // Try device media query
                    self.variant_parser.get_device_media_query(variant)
                }
            });
        
        Ok(super::types::CssRule {
            selector,
            properties,
            media_query: media_query.map(|s| s.to_string()),
            specificity: variants.len() as u32 * 10, // Higher specificity for more variants
        })
    }
}

impl super::CssGenerator {
    // Parser methods - delegate to the actual parser modules
    fn parse_spacing_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        self.spacing_parser.parse_class(class)
    }
    
    fn parse_animation_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        self.animation_parser.parse_class(class)
    }
    
    // Parser methods - delegate to the actual parser modules
    fn parse_color_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = ColorParser::new();
        parser.parse_class(class)
    }
    
    fn parse_typography_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = TypographyParser::new();
        parser.parse_class(class)
    }
    
    fn parse_layout_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = LayoutParser::new();
        parser.parse_class(class)
    }
    
    fn parse_flexbox_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = FlexboxParser::new();
        parser.parse_class(class)
    }
    
    fn parse_grid_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = GridParser::new();
        parser.parse_class(class)
    }
    
    fn parse_border_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = BorderParser::new();
        parser.parse_class(class)
    }
    
    fn parse_effects_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = EffectsParser::new();
        parser.parse_class(class)
    }
    
    fn parse_transform_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = TransformParser::new();
        parser.parse_class(class)
    }
    
    fn parse_interactive_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        self.interactive_parser.parse_class(class)
    }
    
    fn parse_sizing_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn parse_background_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn parse_filter_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn parse_transition_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    
    // New layout parser methods
    fn parse_overflow_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = OverflowParser::new();
        parser.parse_class(class)
    }
    
    fn parse_overscroll_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = OverscrollParser::new();
        parser.parse_class(class)
    }
    
    fn parse_position_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = PositionParser::new();
        parser.parse_class(class)
    }
    
    fn parse_inset_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = InsetParser::new();
        parser.parse_class(class)
    }
    
    fn parse_visibility_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = VisibilityParser::new();
        parser.parse_class(class)
    }
    
    fn parse_z_index_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = ZIndexParser::new();
        parser.parse_class(class)
    }
    
    // New flexbox parser methods
    fn parse_flex_basis_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = FlexBasisParser::new();
        parser.parse_class(class)
    }
    
    fn parse_flex_direction_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = FlexDirectionParser::new();
        parser.parse_class(class)
    }
    
    fn parse_flex_wrap_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = FlexWrapParser::new();
        parser.parse_class(class)
    }
    
    fn parse_flex_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = FlexParser::new();
        parser.parse_class(class)
    }
    
    fn parse_flex_grow_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = FlexGrowParser::new();
        parser.parse_class(class)
    }
    
    fn parse_flex_shrink_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = FlexShrinkParser::new();
        parser.parse_class(class)
    }
    
    // New order and grid parser methods
    fn parse_order_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = OrderParser::new();
        parser.parse_class(class)
    }
    
    fn parse_grid_template_columns_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = GridTemplateColumnsParser::new();
        parser.parse_class(class)
    }
    
    fn parse_grid_column_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = GridColumnParser::new();
        parser.parse_class(class)
    }
    
    fn parse_grid_template_rows_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = GridTemplateRowsParser::new();
        parser.parse_class(class)
    }
    
    fn parse_grid_row_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = GridRowParser::new();
        parser.parse_class(class)
    }
    
    fn parse_grid_auto_flow_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = GridAutoFlowParser::new();
        parser.parse_class(class)
    }
    
    fn parse_grid_auto_columns_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = GridAutoColumnsParser::new();
        parser.parse_class(class)
    }
    
    fn parse_grid_auto_rows_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = GridAutoRowsParser::new();
        parser.parse_class(class)
    }
    
    // New gap and justify parser methods
    fn parse_gap_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = GapParser::new();
        parser.parse_class(class)
    }
    
    fn parse_justify_content_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = JustifyContentParser::new();
        parser.parse_class(class)
    }
    
    fn parse_justify_items_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = JustifyItemsParser::new();
        parser.parse_class(class)
    }
    
    fn parse_justify_self_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = JustifySelfParser::new();
        parser.parse_class(class)
    }
    
    // New align and place parser methods
    fn parse_align_content_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = AlignContentParser::new();
        parser.parse_class(class)
    }
    
    fn parse_align_items_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = AlignItemsParser::new();
        parser.parse_class(class)
    }
    
    fn parse_align_self_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = AlignSelfParser::new();
        parser.parse_class(class)
    }
    
    fn parse_place_content_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = PlaceContentParser::new();
        parser.parse_class(class)
    }
    
    fn parse_place_items_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = PlaceItemsParser::new();
        parser.parse_class(class)
    }
    
    fn parse_place_self_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = PlaceSelfParser::new();
        parser.parse_class(class)
    }
}
