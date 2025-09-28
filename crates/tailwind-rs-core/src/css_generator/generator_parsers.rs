//! CSS Generator Parser Methods
//!
//! This module contains all the parser delegation methods for CssGenerator.

use super::types::CssProperty;
use crate::error::{Result, TailwindError};

/// Parser methods trait for CssGenerator
pub trait CssGeneratorParsers {
    /// Convert a class name to CSS properties
    fn class_to_properties(&self, class: &str) -> Result<Vec<CssProperty>>;

    /// Parse variants from a class name and return (variants, base_class)
    fn parse_variants(&self, class: &str) -> (Vec<String>, String);

    /// Convert a class name to a CSS rule
    fn class_to_css_rule(&self, class: &str) -> Result<super::types::CssRule>;
}

impl CssGeneratorParsers for super::core::CssGenerator {
    fn class_to_properties(&self, class: &str) -> Result<Vec<CssProperty>> {
        // Simple fallback implementation
        match class {
            "block" => Ok(vec![CssProperty {
                name: "display".to_string(),
                value: "block".to_string(),
                important: false,
            }]),
            _ => Err(TailwindError::class_generation(format!(
                "Unknown class: {}",
                class
            ))),
        }
    }

    fn parse_variants(&self, class: &str) -> (Vec<String>, String) {
        // Simple variant parsing
        let mut variants = Vec::new();
        let mut remaining = class.to_string();

        // Parse responsive variants
        for prefix in ["2xl:", "xl:", "lg:", "md:", "sm:"] {
            if remaining.starts_with(prefix) {
                variants.push(remaining[..prefix.len()].to_string());
                remaining = remaining[prefix.len()..].to_string();
                break;
            }
        }

        (variants, remaining)
    }

    fn class_to_css_rule(&self, class: &str) -> Result<super::types::CssRule> {
        let (variants, base_class) = self.parse_variants(class);
        let properties = self.class_to_properties(class)?;

        // Build selector with variants
        let mut selector = String::new();
        for variant in &variants {
            match variant.as_str() {
                "dark" => selector.push_str(".dark "),
                "hover" => selector.push_str(":hover"),
                "focus" => selector.push_str(":focus"),
                _ => {} // Other variants
            }
        }

        // Add the base class
        selector.push_str(&format!(".{}", base_class));

        // Determine media query for responsive variants
        let media_query = variants.iter().find_map(|variant| {
            if variant.starts_with("sm:") {
                Some("@media (min-width: 640px)".to_string())
            } else if variant.starts_with("md:") {
                Some("@media (min-width: 768px)".to_string())
            } else if variant.starts_with("lg:") {
                Some("@media (min-width: 1024px)".to_string())
            } else if variant.starts_with("xl:") {
                Some("@media (min-width: 1280px)".to_string())
            } else if variant.starts_with("2xl:") {
                Some("@media (min-width: 1536px)".to_string())
            } else {
                None
            }
        });

        Ok(super::types::CssRule {
            selector,
            properties,
            media_query,
            specificity: variants.len() as u32 * 10,
        })
    }
}

impl CssGeneratorParsers for super::CssGenerator {
    fn class_to_properties(&self, class: &str) -> Result<Vec<CssProperty>> {
        use super::parsers::UtilityParser;

        // Try each parser in order of priority/specificity
        // Start with most specific parsers first

        // Try arbitrary parser first (highest priority)
        if let Some(properties) = self.arbitrary_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try basic transforms parser (translate-x-*, translate-y-*)
        if let Some(properties) = self.basic_transforms_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try scale parser (scale-x-*, scale-y-*)
        if let Some(properties) = self.scale_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try mask utilities parser
        if let Some(properties) = self.mask_utilities_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try transform parser
        if let Some(properties) = self.transform_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try gradient parser
        if let Some(properties) = self.gradient_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try effects parser (backdrop-blur, etc.)
        if let Some(properties) = self.effects_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try color parser
        if let Some(properties) = self.color_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try background parser (gradients, colors, etc.)
        if let Some(properties) = self.background_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try background properties parser (size, position, repeat, etc.)
        if let Some(properties) = self.background_properties_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try typography parser
        if let Some(properties) = self.typography_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try layout parser
        if let Some(properties) = self.layout_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try flexbox parser
        if let Some(properties) = self.flexbox_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try sizing parser
        if let Some(properties) = self.sizing_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try spacing parser
        if let Some(properties) = self.spacing_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try positioning parser
        if let Some(properties) = self.positioning_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try border parser
        if let Some(properties) = self.advanced_border_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try shadow parser
        if let Some(properties) = self.shadow_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try transition parser
        if let Some(properties) = self.transition_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try ring parser
        if let Some(properties) = self.ring_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try interactive parser
        if let Some(properties) = self.interactive_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try animation parser
        if let Some(properties) = self.animation_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try advanced grid parser
        if let Some(properties) = self.advanced_grid_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try SVG parser
        if let Some(properties) = self.svg_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try prose parser
        if let Some(properties) = self.prose_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try divide parser
        if let Some(properties) = self.divide_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try object fit parser
        if let Some(properties) = self.object_fit_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try accessibility parser
        if let Some(properties) = self.accent_color_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try data attribute parser
        if let Some(properties) = self.data_attribute_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try advanced color parser
        if let Some(properties) = self.advanced_color_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try advanced spacing parser
        if let Some(properties) = self.advanced_spacing_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try transition properties parser
        if let Some(properties) = self.transition_properties_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try fractional transforms parser
        if let Some(properties) = self.fractional_transforms_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try aspect ratio parser
        if let Some(properties) = self.aspect_ratio_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try columns parser
        if let Some(properties) = self.columns_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try break control parser
        if let Some(properties) = self.break_control_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try box utilities parser
        if let Some(properties) = self.box_utilities_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try layout utilities parser
        if let Some(properties) = self.layout_utilities_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try overflow parser
        if let Some(properties) = self.overflow_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try overscroll parser
        if let Some(properties) = self.overscroll_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try position parser
        if let Some(properties) = self.position_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try inset parser
        if let Some(properties) = self.inset_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try visibility parser
        if let Some(properties) = self.visibility_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try z-index parser
        if let Some(properties) = self.z_index_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try flex basis parser
        if let Some(properties) = self.flex_basis_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try flex direction parser
        if let Some(properties) = self.flex_direction_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try flex wrap parser
        if let Some(properties) = self.flex_wrap_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try flex parser
        if let Some(properties) = self.flex_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try flex grow parser
        if let Some(properties) = self.flex_grow_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try flex shrink parser
        if let Some(properties) = self.flex_shrink_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try order parser
        if let Some(properties) = self.order_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try grid template columns parser
        if let Some(properties) = self.grid_template_columns_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try grid template rows parser
        if let Some(properties) = self.grid_template_rows_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try grid column parser
        if let Some(properties) = self.grid_column_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try grid row parser
        if let Some(properties) = self.grid_row_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try grid auto columns parser
        if let Some(properties) = self.grid_auto_columns_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try grid auto flow parser
        if let Some(properties) = self.grid_auto_flow_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try grid auto rows parser
        if let Some(properties) = self.grid_auto_rows_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try gap parser
        if let Some(properties) = self.gap_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try place content parser
        if let Some(properties) = self.place_content_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try place items parser
        if let Some(properties) = self.place_items_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try place self parser
        if let Some(properties) = self.place_self_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try justify content parser
        if let Some(properties) = self.justify_content_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try justify items parser
        if let Some(properties) = self.justify_items_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try justify self parser
        if let Some(properties) = self.justify_self_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try align content parser
        if let Some(properties) = self.align_content_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try align items parser
        if let Some(properties) = self.align_items_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try align self parser
        if let Some(properties) = self.align_self_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try margin parser
        if let Some(properties) = self.margin_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try group parser
        if let Some(properties) = self.group_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try filter utilities parser
        if let Some(properties) = self.filter_utilities_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try backdrop filter utilities parser
        if let Some(properties) = self.backdrop_filter_utilities_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try effects utilities parser
        if let Some(properties) = self.effects_utilities_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try border utilities parser
        if let Some(properties) = self.border_utilities_parser.parse_class(class) {
            return Ok(properties);
        }

        // Try accessibility parser
        if let Some(properties) = self.accessibility_parser.parse_class(class) {
            return Ok(properties);
        }

        // If no parser matched, return an error
        Err(TailwindError::class_generation(format!(
            "Unknown class: {}",
            class
        )))
    }

    fn parse_variants(&self, class: &str) -> (Vec<String>, String) {
        // Simple variant parsing
        let mut variants = Vec::new();
        let mut remaining = class.to_string();

        // Parse responsive variants
        for prefix in ["2xl:", "xl:", "lg:", "md:", "sm:"] {
            if remaining.starts_with(prefix) {
                variants.push(remaining[..prefix.len()].to_string());
                remaining = remaining[prefix.len()..].to_string();
                break;
            }
        }

        (variants, remaining)
    }

    fn class_to_css_rule(&self, class: &str) -> Result<super::types::CssRule> {
        let (variants, base_class) = self.parse_variants(class);
        let properties = self.class_to_properties(class)?;

        // Build selector with variants
        let mut selector = String::new();
        for variant in &variants {
            match variant.as_str() {
                "dark" => selector.push_str(".dark "),
                "hover" => selector.push_str(":hover"),
                "focus" => selector.push_str(":focus"),
                _ => {} // Other variants
            }
        }

        // Add the base class
        selector.push_str(&format!(".{}", base_class));

        // Determine media query for responsive variants
        let media_query = variants.iter().find_map(|variant| {
            if variant.starts_with("sm:") {
                Some("@media (min-width: 640px)".to_string())
            } else if variant.starts_with("md:") {
                Some("@media (min-width: 768px)".to_string())
            } else if variant.starts_with("lg:") {
                Some("@media (min-width: 1024px)".to_string())
            } else if variant.starts_with("xl:") {
                Some("@media (min-width: 1280px)".to_string())
            } else if variant.starts_with("2xl:") {
                Some("@media (min-width: 1536px)".to_string())
            } else {
                None
            }
        });

        Ok(super::types::CssRule {
            selector,
            properties,
            media_query,
            specificity: variants.len() as u32 * 10,
        })
    }
}
