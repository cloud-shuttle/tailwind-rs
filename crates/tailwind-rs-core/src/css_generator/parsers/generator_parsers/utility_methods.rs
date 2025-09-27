//! Utility methods for CssGenerator
//! 
//! This module contains utility methods for variant parsing and CSS rule generation.

use crate::css_generator::types::{CssProperty, CssRule};
use crate::error::Result;

/// Utility methods for CssGenerator
impl crate::css_generator::CssGenerator {
    /// Parse variants from a class name and return (variants, base_class)
    pub fn parse_variants(&self, class: &str) -> (Vec<String>, String) {
        self.variant_parser.parse_variants(class)
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
}
