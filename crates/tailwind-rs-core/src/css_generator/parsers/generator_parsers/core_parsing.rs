//! Core parsing logic for CssGenerator
//! 
//! This module contains the main class_to_properties method and related core parsing logic.

use crate::css_generator::types::CssProperty;
use crate::error::{Result, TailwindError};

/// Core parsing implementation for CssGenerator
impl crate::css_generator::core::CssGenerator {
    /// Convert a class name to CSS properties
    /// 
    /// This is the main parsing method that tries all available parsers
    /// in order of priority to find matching CSS properties for a class.
    pub fn class_to_properties(&self, class: &str) -> Result<Vec<CssProperty>> {
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

        // Try the new transform parsers
        if let Some(properties) = self.basic_transforms_parser.parse_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.scale_parser.parse_class(&base_class) {
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

        if let Some(properties) = self
            .backdrop_filter_utilities_parser
            .parse_class(&base_class)
        {
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

        // Continue with specialized parsers...
        self.try_specialized_parsers(&base_class)
    }

    /// Try specialized parsers for the base class
    fn try_specialized_parsers(&self, base_class: &str) -> Result<Vec<CssProperty>> {
        // Grid and Flexbox parsers
        if let Some(properties) = self.grid_template_columns_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.grid_column_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.grid_template_rows_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.grid_row_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.grid_auto_flow_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.grid_auto_columns_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.grid_auto_rows_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.gap_parser.parse_class(base_class) {
            return Ok(properties);
        }

        // Flexbox parsers
        if let Some(properties) = self.flex_basis_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.flex_direction_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.flex_wrap_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.flex_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.flex_grow_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.flex_shrink_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.order_parser.parse_class(base_class) {
            return Ok(properties);
        }

        // Justify and Align parsers
        if let Some(properties) = self.justify_content_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.justify_items_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.justify_self_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.align_content_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.align_items_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.align_self_parser.parse_class(base_class) {
            return Ok(properties);
        }

        // Layout parsers
        if let Some(properties) = self.overflow_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.overscroll_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.position_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.inset_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.visibility_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.z_index_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.typography_parser.parse_class(base_class) {
            return Ok(properties);
        }

        // Try basic parsers
        self.try_basic_parsers(base_class)
    }

    /// Try basic parsers for the base class
    fn try_basic_parsers(&self, base_class: &str) -> Result<Vec<CssProperty>> {
        if let Some(properties) = self.parse_spacing_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_color_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_typography_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_layout_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_flexbox_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_grid_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_border_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_effects_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_transform_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_animation_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_interactive_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.accent_color_parser.parse_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_sizing_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_background_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_filter_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_transition_class(base_class) {
            return Ok(properties);
        }

        // Try new layout parsers
        if let Some(properties) = self.parse_overflow_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_overscroll_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_position_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_inset_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_visibility_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_z_index_class(base_class) {
            return Ok(properties);
        }

        // Try new flexbox parsers
        if let Some(properties) = self.parse_flex_basis_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_flex_direction_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_flex_wrap_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_flex_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_flex_grow_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_flex_shrink_class(base_class) {
            return Ok(properties);
        }

        // Try new order and grid parsers
        if let Some(properties) = self.parse_order_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_grid_template_columns_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_grid_column_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_grid_template_rows_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_grid_row_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_grid_auto_flow_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_grid_auto_columns_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_grid_auto_rows_class(base_class) {
            return Ok(properties);
        }

        // Try new gap and justify parsers
        if let Some(properties) = self.parse_gap_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_justify_content_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_justify_items_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_justify_self_class(base_class) {
            return Ok(properties);
        }

        // Try new align and place parsers
        if let Some(properties) = self.parse_align_content_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_align_items_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_align_self_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_place_content_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_place_items_class(base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_place_self_class(base_class) {
            return Ok(properties);
        }

        // Fallback to hardcoded classes for backwards compatibility
        self.try_fallback_classes(base_class)
    }

    /// Try fallback hardcoded classes
    fn try_fallback_classes(&self, base_class: &str) -> Result<Vec<CssProperty>> {
        match base_class {
            // Display utilities
            "block" => Ok(vec![CssProperty {
                name: "display".to_string(),
                value: "block".to_string(),
                important: false,
            }]),
            "inline" => Ok(vec![CssProperty {
                name: "display".to_string(),
                value: "inline".to_string(),
                important: false,
            }]),
            "flex" => Ok(vec![CssProperty {
                name: "display".to_string(),
                value: "flex".to_string(),
                important: false,
            }]),
            "grid" => Ok(vec![CssProperty {
                name: "display".to_string(),
                value: "grid".to_string(),
                important: false,
            }]),
            "hidden" => Ok(vec![CssProperty {
                name: "display".to_string(),
                value: "none".to_string(),
                important: false,
            }]),

            _ => Err(TailwindError::class_generation(format!(
                "Unknown class: {}",
                base_class
            ))),
        }
    }
}
