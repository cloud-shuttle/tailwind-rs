//! Enhanced Variants Parser Module
//!
//! This module contains the core parsing logic for the enhanced variant system,
//! including variant combination parsing, selector generation, and complex
//! variant interaction handling.

use crate::error::{Result, TailwindError};
use crate::responsive::Breakpoint;
use super::types::*;
use super::definitions::*;
use std::collections::HashMap;

/// Enhanced variant parser for complex class combinations
#[derive(Debug, Clone)]
pub struct EnhancedVariantParser {
    /// Variant definitions
    variants: HashMap<String, VariantDefinition>,
    /// Responsive breakpoints
    breakpoints: HashMap<String, Breakpoint>,
    /// Custom variant definitions
    custom_variants: HashMap<String, CustomVariant>,
}

impl EnhancedVariantParser {
    /// Create a new enhanced variant parser with default definitions
    pub fn new() -> Self {
        Self {
            variants: VariantDefinitions::standard_definitions(),
            breakpoints: VariantDefinitions::get_responsive_breakpoints(),
            custom_variants: HashMap::new(),
        }
    }

    /// Parse a class with potential variants
    pub fn parse_class(&self, class: &str) -> Result<VariantParseResult> {
        // Split the class into variants and base class
        let (variants_part, base_class) = self.split_class_variants(class)?;

        // Parse the variant combination
        let combination = self.parse_variant_combination(&variants_part)?;

        // Generate the CSS selector
        let css_selector = self.generate_css_selector(&combination, &base_class)?;

        // Get media query if applicable
        let media_query = self.get_media_query(&combination);

        Ok(VariantParseResult::success(
            class.to_string(),
            base_class,
            combination,
            css_selector,
            media_query,
        ))
    }

    /// Split a class into variants and base class
    fn split_class_variants(&self, class: &str) -> Result<(String, String)> {
        // Find the last colon that's not inside brackets or parentheses
        let mut variants = Vec::new();
        let mut current_variant = String::new();
        let mut paren_depth = 0;
        let mut bracket_depth = 0;

        for (i, ch) in class.char_indices() {
            match ch {
                '(' => paren_depth += 1,
                ')' => paren_depth -= 1,
                '[' => bracket_depth += 1,
                ']' => bracket_depth -= 1,
                ':' if paren_depth == 0 && bracket_depth == 0 => {
                    // This is a variant separator
                    if !current_variant.is_empty() {
                        variants.push(current_variant);
                        current_variant = String::new();
                    }
                }
                _ => current_variant.push(ch),
            }
        }

        // Add the last part
        if !current_variant.is_empty() {
            variants.push(current_variant);
        }

        if variants.is_empty() {
            return Ok((String::new(), class.to_string()));
        }

        // The last part is the base class
        let base_class = variants.pop().unwrap();
        let variants_part = variants.join(":");

        Ok((variants_part, base_class))
    }

    /// Parse a variant combination string
    fn parse_variant_combination(&self, variants_str: &str) -> Result<VariantCombination> {
        if variants_str.is_empty() {
            return Ok(VariantCombination::new());
        }

        let variant_names = variants_str.split(':').collect::<Vec<_>>();
        let mut variants = Vec::new();
        let mut valid = true;
        let mut error_message = None;

        for name in variant_names {
            match self.parse_single_variant(name) {
                Ok(variant) => variants.push(variant),
                Err(err) => {
                    valid = false;
                    error_message = Some(err.to_string());
                    break;
                }
            }
        }

        // Validate the combination
        if valid {
            if let Err(combination_error) = VariantValidators::is_valid_combination(&variants) {
                valid = false;
                error_message = Some(combination_error);
            }
        }

        let mut combination = VariantCombination {
            variants,
            specificity: 0,
            valid,
            error_message,
        };

        if valid {
            combination = combination.calculate_specificity();
        }

        Ok(combination)
    }

    /// Parse a single variant
    fn parse_single_variant(&self, name: &str) -> Result<ParsedVariant> {
        // Check standard variants first
        if let Some(definition) = self.variants.get(name) {
            return Ok(ParsedVariant::new(name.to_string(), definition.variant_type.clone()));
        }

        // Check custom variants
        if let Some(custom) = self.custom_variants.get(name) {
            return Ok(ParsedVariant::new(
                name.to_string(),
                VariantType::Custom(custom.name.clone()),
            ));
        }

        // Check responsive breakpoints
        if let Some(breakpoint) = self.breakpoints.get(name) {
            return Ok(ParsedVariant::new(
                name.to_string(),
                VariantType::Responsive,
            ).with_parameter("breakpoint".to_string(), format!("{:?}", breakpoint)));
        }

        Err(TailwindError::ClassGeneration { message: format!("Unknown variant: {}", name) })
    }

    /// Generate CSS selector for a variant combination
    fn generate_css_selector(&self, combination: &VariantCombination, base_class: &str) -> Result<String> {
        if !combination.valid {
            return Err(TailwindError::ClassGeneration { message: combination.error_message.clone().unwrap_or_else(|| "Invalid combination".to_string()) });
        }

        let mut selector_parts = Vec::new();

        // Add variant selectors in specificity order (highest first)
        let mut sorted_variants = combination.variants.clone();
        sorted_variants.sort_by(|a, b| {
            let a_priority = a.variant_type.priority();
            let b_priority = b.variant_type.priority();
            b_priority.cmp(&a_priority) // Higher specificity first
        });

        for variant in &sorted_variants {
            if let Some(definition) = self.variants.get(&variant.name) {
                selector_parts.push(definition.selector_pattern.clone());
            } else if let Some(custom) = self.custom_variants.get(&variant.name) {
                selector_parts.push(custom.selector.clone());
            }
        }

        // Add the base class
        selector_parts.push(format!(".{}", base_class));

        // Join with appropriate spacing
        let selector = self.combine_selector_parts(&selector_parts);

        Ok(selector)
    }

    /// Combine selector parts with proper spacing
    fn combine_selector_parts(&self, parts: &[String]) -> String {
        let mut result = String::new();
        let mut first = true;

        for part in parts {
            if first {
                result.push_str(part);
                first = false;
            } else {
                // Add space if the previous part doesn't end with a space and this part starts with a class or pseudo-class
                if !result.ends_with(' ') && (part.starts_with('.') || part.starts_with(':')) {
                    result.push(' ');
                }
                result.push_str(part);
            }
        }

        result
    }

    /// Get media query for a variant combination
    fn get_media_query(&self, combination: &VariantCombination) -> Option<String> {
        // Find the first responsive variant (they should be mutually exclusive)
        for variant in &combination.variants {
            if variant.variant_type == VariantType::Responsive {
                if let Some(definition) = self.variants.get(&variant.name) {
                    return definition.media_query.clone();
                }
            }
        }

        // Check for other media query variants
        for variant in &combination.variants {
            if let Some(definition) = self.variants.get(&variant.name) {
                if let Some(mq) = &definition.media_query {
                    return Some(mq.clone());
                }
            }
        }

        None
    }

    /// Add a custom variant
    pub fn add_custom_variant(&mut self, name: String, selector: String) -> Result<()> {
        if !VariantValidators::is_valid_variant_name(&name) {
            return Err(TailwindError::Config { message: format!("Invalid variant name: {}", name) });
        }

        let custom = CustomVariant::new(name.clone(), selector);
        self.custom_variants.insert(name, custom);
        Ok(())
    }

    /// Remove a custom variant
    pub fn remove_custom_variant(&mut self, name: &str) -> bool {
        self.custom_variants.remove(name).is_some()
    }

    /// Get all available variant names
    pub fn get_variant_names(&self) -> Vec<String> {
        let mut names = self.variants.keys().cloned().collect::<Vec<_>>();
        names.extend(self.custom_variants.keys().cloned());
        names
    }

    /// Check if a variant is supported
    pub fn supports_variant(&self, name: &str) -> bool {
        self.variants.contains_key(name) || self.custom_variants.contains_key(name)
    }

    /// Get variant definition
    pub fn get_variant_definition(&self, name: &str) -> Option<&VariantDefinition> {
        self.variants.get(name)
    }

    /// Get custom variant definition
    pub fn get_custom_variant(&self, name: &str) -> Option<&CustomVariant> {
        self.custom_variants.get(name)
    }
}

impl Default for EnhancedVariantParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_creation() {
        let parser = EnhancedVariantParser::new();
        assert!(!parser.variants.is_empty());
        assert!(!parser.breakpoints.is_empty());
    }

    #[test]
    fn class_splitting() {
        let parser = EnhancedVariantParser::new();

        // Simple class without variants
        let (variants, base) = parser.split_class_variants("bg-blue-500").unwrap();
        assert_eq!(variants, "");
        assert_eq!(base, "bg-blue-500");

        // Single variant
        let (variants, base) = parser.split_class_variants("hover:bg-blue-500").unwrap();
        assert_eq!(variants, "hover");
        assert_eq!(base, "bg-blue-500");

        // Multiple variants
        let (variants, base) = parser.split_class_variants("sm:hover:bg-blue-500").unwrap();
        assert_eq!(variants, "sm:hover");
        assert_eq!(base, "bg-blue-500");

        // Variants with parameters (should not split inside brackets)
        let (variants, base) = parser.split_class_variants("hover:bg-blue-500[custom]").unwrap();
        assert_eq!(variants, "hover");
        assert_eq!(base, "bg-blue-500[custom]");
    }

    #[test]
    fn single_variant_parsing() {
        let parser = EnhancedVariantParser::new();

        // State variant
        let variant = parser.parse_single_variant("hover").unwrap();
        assert_eq!(variant.name, "hover");
        assert_eq!(variant.variant_type, VariantType::State);

        // Responsive variant
        let variant = parser.parse_single_variant("sm").unwrap();
        assert_eq!(variant.name, "sm");
        assert_eq!(variant.variant_type, VariantType::Responsive);

        // Unknown variant
        assert!(parser.parse_single_variant("unknown").is_err());
    }

    #[test]
    fn variant_combination_parsing() {
        let parser = EnhancedVariantParser::new();

        // Valid combination
        let combo = parser.parse_variant_combination("hover:sm").unwrap();
        assert_eq!(combo.variants.len(), 2);
        assert!(combo.valid);
        assert_eq!(combo.specificity, 180); // 80 + 100

        // Invalid combination (print + screen)
        let combo = parser.parse_variant_combination("print:screen").unwrap();
        assert!(!combo.valid);
        assert!(combo.error_message.is_some());
    }

    #[test]
    fn css_selector_generation() {
        let parser = EnhancedVariantParser::new();

        // Single variant
        let combo = parser.parse_variant_combination("hover").unwrap();
        let selector = parser.generate_css_selector(&combo, "bg-blue-500").unwrap();
        assert_eq!(selector, ":hover .bg-blue-500");

        // Multiple variants
        let combo = parser.parse_variant_combination("hover:focus").unwrap();
        let selector = parser.generate_css_selector(&combo, "bg-red-500").unwrap();
        assert!(selector.contains(":hover"));
        assert!(selector.contains(":focus"));
        assert!(selector.contains(".bg-red-500"));
    }

    #[test]
    fn media_query_extraction() {
        let parser = EnhancedVariantParser::new();

        // Responsive variant
        let combo = parser.parse_variant_combination("sm").unwrap();
        let mq = parser.get_media_query(&combo);
        assert!(mq.is_some());
        assert!(mq.unwrap().contains("640px"));

        // Non-responsive variant
        let combo = parser.parse_variant_combination("hover").unwrap();
        let mq = parser.get_media_query(&combo);
        assert!(mq.is_none());
    }

    #[test]
    fn full_class_parsing() {
        let parser = EnhancedVariantParser::new();

        // Simple class
        let result = parser.parse_class("bg-blue-500").unwrap();
        assert_eq!(result.base_class, "bg-blue-500");
        assert!(result.combination.variants.is_empty());
        assert!(result.success);

        // Class with variants
        let result = parser.parse_class("sm:hover:bg-blue-500").unwrap();
        assert_eq!(result.base_class, "bg-blue-500");
        assert_eq!(result.combination.variants.len(), 2);
        assert!(result.success);
        assert!(result.media_query.is_some());
    }

    #[test]
    fn custom_variant_management() {
        let mut parser = EnhancedVariantParser::new();

        // Add custom variant
        parser.add_custom_variant("custom".to_string(), ":custom".to_string()).unwrap();
        assert!(parser.supports_variant("custom"));

        // Parse with custom variant
        let result = parser.parse_class("custom:bg-blue-500").unwrap();
        assert!(result.success);

        // Remove custom variant
        assert!(parser.remove_custom_variant("custom"));
        assert!(!parser.supports_variant("custom"));
    }

    #[test]
    fn invalid_custom_variant_name() {
        let mut parser = EnhancedVariantParser::new();

        // Invalid name
        assert!(parser.add_custom_variant("Invalid Name".to_string(), ":test".to_string()).is_err());
        assert!(parser.add_custom_variant("-invalid".to_string(), ":test".to_string()).is_err());
        assert!(parser.add_custom_variant("invalid-".to_string(), ":test".to_string()).is_err());
    }

    #[test]
    fn selector_part_combination() {
        let parser = EnhancedVariantParser::new();

        // Test various combinations
        assert_eq!(parser.combine_selector_parts(&[":hover".to_string(), ".bg-blue".to_string()]), ":hover .bg-blue");
        assert_eq!(parser.combine_selector_parts(&[".dark".to_string(), ":hover".to_string(), ".bg-red".to_string()]), ".dark :hover .bg-red");
    }

    #[test]
    fn complex_variant_interactions() {
        let parser = EnhancedVariantParser::new();

        // Test that responsive variants work with state variants
        let result = parser.parse_class("sm:hover:focus:bg-gradient-to-r").unwrap();
        assert!(result.success);
        assert_eq!(result.combination.variants.len(), 3);
        assert!(result.media_query.is_some());
    }
}
