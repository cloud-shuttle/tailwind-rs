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
