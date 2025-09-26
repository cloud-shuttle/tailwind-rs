//! Enhanced variant system for Tailwind-RS Core
//!
//! This module provides advanced variant parsing and combination capabilities,
//! supporting complex variant combinations, modern CSS features, and
//! comprehensive selector generation.

use crate::error::{Result, TailwindError};
use crate::responsive::Breakpoint;
use serde::{Deserialize, Serialize};
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

/// Variant definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariantDefinition {
    /// Variant name
    pub name: String,
    /// Variant type
    pub variant_type: VariantType,
    /// CSS selector pattern
    pub selector_pattern: String,
    /// Media query (for responsive variants)
    pub media_query: Option<String>,
    /// Specificity weight
    pub specificity: u32,
    /// Whether this variant can be combined with others
    pub combinable: bool,
    /// Dependencies (other variants that must be present)
    pub dependencies: Vec<String>,
}

/// Variant type classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VariantType {
    /// State variants (hover, focus, etc.)
    State,
    /// Responsive variants (sm, md, lg, etc.)
    Responsive,
    /// Dark mode variants
    DarkMode,
    /// Group variants (group-hover, group-focus, etc.)
    Group,
    /// Peer variants (peer-hover, peer-focus, etc.)
    Peer,
    /// Pseudo-element variants (before, after, etc.)
    PseudoElement,
    /// Container query variants
    Container,
    /// Cascade layer variants
    Layer,
    /// Custom variants
    Custom,
}

/// Custom variant definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomVariant {
    /// Variant name
    pub name: String,
    /// CSS selector pattern
    pub selector_pattern: String,
    /// Media query
    pub media_query: Option<String>,
    /// Specificity weight
    pub specificity: u32,
    /// Whether this variant can be combined with others
    pub combinable: bool,
}

/// Parsed variant combination
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariantCombination {
    /// All variants in the combination
    pub variants: Vec<ParsedVariant>,
    /// Base class name
    pub base_class: String,
    /// Generated CSS selector
    pub selector: String,
    /// Media query (if any)
    pub media_query: Option<String>,
    /// Total specificity
    pub specificity: u32,
    /// Whether this combination is valid
    pub is_valid: bool,
    /// Validation errors
    pub errors: Vec<String>,
}

/// Parsed individual variant
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedVariant {
    /// Variant name
    pub name: String,
    /// Variant type
    pub variant_type: VariantType,
    /// CSS selector fragment
    pub selector_fragment: String,
    /// Specificity weight
    pub specificity: u32,
    /// Position in the combination
    pub position: usize,
}

/// Enhanced variant parsing result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariantParseResult {
    /// Parsed variant combination
    pub combination: VariantCombination,
    /// Processing metadata
    pub metadata: VariantMetadata,
}

/// Variant processing metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariantMetadata {
    /// Processing time
    pub processing_time: std::time::Duration,
    /// Number of variants parsed
    pub variant_count: usize,
    /// Number of combinations generated
    pub combination_count: usize,
    /// Memory usage
    pub memory_usage: usize,
    /// Cache hits
    pub cache_hits: usize,
    /// Cache misses
    pub cache_misses: usize,
}

impl EnhancedVariantParser {
    /// Create a new enhanced variant parser
    pub fn new() -> Self {
        let mut parser = Self {
            variants: HashMap::new(),
            breakpoints: HashMap::new(),
            custom_variants: HashMap::new(),
        };

        parser.initialize_default_variants();
        parser.initialize_breakpoints();

        parser
    }

    /// Initialize default variant definitions
    fn initialize_default_variants(&mut self) {
        // State variants
        self.add_variant(VariantDefinition {
            name: "hover".to_string(),
            variant_type: VariantType::State,
            selector_pattern: ":hover".to_string(),
            media_query: None,
            specificity: 10,
            combinable: true,
            dependencies: Vec::new(),
        });

        self.add_variant(VariantDefinition {
            name: "focus".to_string(),
            variant_type: VariantType::State,
            selector_pattern: ":focus".to_string(),
            media_query: None,
            specificity: 10,
            combinable: true,
            dependencies: Vec::new(),
        });

        self.add_variant(VariantDefinition {
            name: "active".to_string(),
            variant_type: VariantType::State,
            selector_pattern: ":active".to_string(),
            media_query: None,
            specificity: 10,
            combinable: true,
            dependencies: Vec::new(),
        });

        self.add_variant(VariantDefinition {
            name: "visited".to_string(),
            variant_type: VariantType::State,
            selector_pattern: ":visited".to_string(),
            media_query: None,
            specificity: 10,
            combinable: true,
            dependencies: Vec::new(),
        });

        self.add_variant(VariantDefinition {
            name: "disabled".to_string(),
            variant_type: VariantType::State,
            selector_pattern: ":disabled".to_string(),
            media_query: None,
            specificity: 10,
            combinable: true,
            dependencies: Vec::new(),
        });

        // Dark mode variants
        self.add_variant(VariantDefinition {
            name: "dark".to_string(),
            variant_type: VariantType::DarkMode,
            selector_pattern: ".dark".to_string(),
            media_query: None,
            specificity: 20,
            combinable: true,
            dependencies: Vec::new(),
        });

        // Group variants
        self.add_variant(VariantDefinition {
            name: "group-hover".to_string(),
            variant_type: VariantType::Group,
            selector_pattern: ".group:hover".to_string(),
            media_query: None,
            specificity: 15,
            combinable: true,
            dependencies: Vec::new(),
        });

        self.add_variant(VariantDefinition {
            name: "group-focus".to_string(),
            variant_type: VariantType::Group,
            selector_pattern: ".group:focus".to_string(),
            media_query: None,
            specificity: 15,
            combinable: true,
            dependencies: Vec::new(),
        });

        // Peer variants
        self.add_variant(VariantDefinition {
            name: "peer-hover".to_string(),
            variant_type: VariantType::Peer,
            selector_pattern: ".peer:hover".to_string(),
            media_query: None,
            specificity: 15,
            combinable: true,
            dependencies: Vec::new(),
        });

        self.add_variant(VariantDefinition {
            name: "peer-focus".to_string(),
            variant_type: VariantType::Peer,
            selector_pattern: ".peer:focus".to_string(),
            media_query: None,
            specificity: 15,
            combinable: true,
            dependencies: Vec::new(),
        });

        // Pseudo-element variants
        self.add_variant(VariantDefinition {
            name: "before".to_string(),
            variant_type: VariantType::PseudoElement,
            selector_pattern: "::before".to_string(),
            media_query: None,
            specificity: 10,
            combinable: true,
            dependencies: Vec::new(),
        });

        self.add_variant(VariantDefinition {
            name: "after".to_string(),
            variant_type: VariantType::PseudoElement,
            selector_pattern: "::after".to_string(),
            media_query: None,
            specificity: 10,
            combinable: true,
            dependencies: Vec::new(),
        });

        // Container query variants
        self.add_variant(VariantDefinition {
            name: "container".to_string(),
            variant_type: VariantType::Container,
            selector_pattern: "@container".to_string(),
            media_query: None,
            specificity: 25,
            combinable: true,
            dependencies: Vec::new(),
        });

        // Cascade layer variants
        self.add_variant(VariantDefinition {
            name: "layer".to_string(),
            variant_type: VariantType::Layer,
            selector_pattern: "@layer".to_string(),
            media_query: None,
            specificity: 30,
            combinable: true,
            dependencies: Vec::new(),
        });

        // Responsive variants
        self.add_variant(VariantDefinition {
            name: "sm".to_string(),
            variant_type: VariantType::Responsive,
            selector_pattern: "".to_string(),
            media_query: Some("(min-width: 640px)".to_string()),
            specificity: 20,
            combinable: false,
            dependencies: Vec::new(),
        });

        self.add_variant(VariantDefinition {
            name: "md".to_string(),
            variant_type: VariantType::Responsive,
            selector_pattern: "".to_string(),
            media_query: Some("(min-width: 768px)".to_string()),
            specificity: 20,
            combinable: false,
            dependencies: Vec::new(),
        });

        self.add_variant(VariantDefinition {
            name: "lg".to_string(),
            variant_type: VariantType::Responsive,
            selector_pattern: "".to_string(),
            media_query: Some("(min-width: 1024px)".to_string()),
            specificity: 20,
            combinable: false,
            dependencies: Vec::new(),
        });

        self.add_variant(VariantDefinition {
            name: "xl".to_string(),
            variant_type: VariantType::Responsive,
            selector_pattern: "".to_string(),
            media_query: Some("(min-width: 1280px)".to_string()),
            specificity: 20,
            combinable: false,
            dependencies: Vec::new(),
        });

        self.add_variant(VariantDefinition {
            name: "2xl".to_string(),
            variant_type: VariantType::Responsive,
            selector_pattern: "".to_string(),
            media_query: Some("(min-width: 1536px)".to_string()),
            specificity: 20,
            combinable: false,
            dependencies: Vec::new(),
        });
    }

    /// Initialize responsive breakpoints
    fn initialize_breakpoints(&mut self) {
        self.breakpoints.insert("sm".to_string(), Breakpoint::Sm);
        self.breakpoints.insert("md".to_string(), Breakpoint::Md);
        self.breakpoints.insert("lg".to_string(), Breakpoint::Lg);
        self.breakpoints.insert("xl".to_string(), Breakpoint::Xl);
        self.breakpoints.insert("2xl".to_string(), Breakpoint::Xl);
    }

    /// Add a variant definition
    pub fn add_variant(&mut self, variant: VariantDefinition) {
        self.variants.insert(variant.name.clone(), variant);
    }

    /// Add a custom variant
    pub fn add_custom_variant(&mut self, variant: CustomVariant) {
        self.custom_variants.insert(variant.name.clone(), variant);
    }

    /// Parse a class string with enhanced variant support
    pub fn parse_class(&self, class: &str) -> Result<VariantParseResult> {
        let start_time = std::time::Instant::now();

        // Parse variants and base class
        let (variants, base_class) = self.parse_variants_advanced(class);

        // Generate variant combination
        let variant_count = variants.len();
        let combination = self.generate_variant_combination(variants, base_class)?;

        let processing_time = start_time.elapsed();

        Ok(VariantParseResult {
            combination,
            metadata: VariantMetadata {
                processing_time,
                variant_count,
                combination_count: 1,
                memory_usage: 0, // Placeholder
                cache_hits: 0,   // Placeholder
                cache_misses: 1, // Placeholder
            },
        })
    }

    /// Advanced variant parsing supporting multiple variants
    fn parse_variants_advanced(&self, class: &str) -> (Vec<String>, String) {
        let mut variants = Vec::new();
        let mut remaining = class.to_string();

        // Parse variants in order of specificity (most specific first)
        let variant_patterns = [
            // Dark mode variants
            ("dark:", "dark"),
            // Group variants
            ("group-hover:", "group-hover"),
            ("group-focus:", "group-focus"),
            ("group-active:", "group-active"),
            ("group-disabled:", "group-disabled"),
            // Peer variants
            ("peer-hover:", "peer-hover"),
            ("peer-focus:", "peer-focus"),
            ("peer-active:", "peer-active"),
            ("peer-disabled:", "peer-disabled"),
            // State variants
            ("hover:", "hover"),
            ("focus:", "focus"),
            ("active:", "active"),
            ("visited:", "visited"),
            ("disabled:", "disabled"),
            // Pseudo-element variants
            ("before:", "before"),
            ("after:", "after"),
            // Container variants
            ("container:", "container"),
            // Layer variants
            ("layer:", "layer"),
            // Responsive variants
            ("sm:", "sm"),
            ("md:", "md"),
            ("lg:", "lg"),
            ("xl:", "xl"),
            ("2xl:", "2xl"),
        ];

        // Parse multiple variants
        loop {
            let mut found = false;
            for (prefix, variant) in &variant_patterns {
                if remaining.starts_with(prefix) {
                    variants.push(variant.to_string());
                    remaining = remaining
                        .strip_prefix(prefix)
                        .unwrap_or(&remaining)
                        .to_string();
                    found = true;
                    break;
                }
            }

            if !found {
                break;
            }
        }

        (variants, remaining)
    }

    /// Generate variant combination
    fn generate_variant_combination(
        &self,
        variants: Vec<String>,
        base_class: String,
    ) -> Result<VariantCombination> {
        let mut parsed_variants = Vec::new();
        let mut selector_parts = Vec::new();
        let mut media_query = None;
        let mut total_specificity = 10; // Base specificity
        let mut errors = Vec::new();

        // Parse each variant
        for (i, variant_name) in variants.iter().enumerate() {
            if let Some(variant_def) = self.variants.get(variant_name) {
                let parsed_variant = ParsedVariant {
                    name: variant_name.clone(),
                    variant_type: variant_def.variant_type.clone(),
                    selector_fragment: variant_def.selector_pattern.clone(),
                    specificity: variant_def.specificity,
                    position: i,
                };

                parsed_variants.push(parsed_variant.clone());
                selector_parts.push(variant_def.selector_pattern.clone());
                total_specificity += variant_def.specificity;

                // Handle media queries for responsive variants
                if let Some(mq) = &variant_def.media_query {
                    media_query = Some(mq.clone());
                }
            } else if let Some(custom_variant) = self.custom_variants.get(variant_name) {
                let parsed_variant = ParsedVariant {
                    name: variant_name.clone(),
                    variant_type: VariantType::Custom,
                    selector_fragment: custom_variant.selector_pattern.clone(),
                    specificity: custom_variant.specificity,
                    position: i,
                };

                parsed_variants.push(parsed_variant.clone());
                selector_parts.push(custom_variant.selector_pattern.clone());
                total_specificity += custom_variant.specificity;

                if let Some(mq) = &custom_variant.media_query {
                    media_query = Some(mq.clone());
                }
            } else {
                errors.push(format!("Unknown variant: {}", variant_name));
            }
        }

        // Generate final selector
        let selector = self.generate_selector(selector_parts, &base_class);

        // Validate combination
        let is_valid = self.validate_combination(&parsed_variants, &errors);

        Ok(VariantCombination {
            variants: parsed_variants,
            base_class,
            selector,
            media_query,
            specificity: total_specificity,
            is_valid,
            errors,
        })
    }

    /// Generate CSS selector from parts
    fn generate_selector(&self, parts: Vec<String>, base_class: &str) -> String {
        let mut selector = String::new();

        // Add variant parts
        for part in parts {
            if part.starts_with('.') {
                selector.push_str(&part);
                selector.push(' ');
            } else if part.starts_with(':') {
                selector.push_str(&part);
            } else if part.starts_with('@') {
                // Handle at-rules like @container, @layer
                selector.push_str(&part);
                selector.push(' ');
            }
        }

        // Add base class
        selector.push_str(&format!(".{}", base_class));

        selector
    }

    /// Validate variant combination
    fn validate_combination(&self, variants: &[ParsedVariant], errors: &[String]) -> bool {
        if !errors.is_empty() {
            return false;
        }

        // Check for conflicting variants
        let mut variant_types = std::collections::HashSet::new();
        for variant in variants {
            if !variant_types.insert(&variant.variant_type) {
                // Check if this variant type allows multiple instances
                match variant.variant_type {
                    VariantType::State => {
                        // Multiple state variants are allowed
                    }
                    VariantType::Responsive => {
                        // Only one responsive variant allowed
                        return false;
                    }
                    VariantType::DarkMode => {
                        // Only one dark mode variant allowed
                        return false;
                    }
                    _ => {
                        // Other types may have specific rules
                    }
                }
            }
        }

        true
    }

    /// Get variant definition
    pub fn get_variant(&self, name: &str) -> Option<&VariantDefinition> {
        self.variants.get(name)
    }

    /// Get all variants
    pub fn get_all_variants(&self) -> &HashMap<String, VariantDefinition> {
        &self.variants
    }

    /// Get custom variants
    pub fn get_custom_variants(&self) -> &HashMap<String, CustomVariant> {
        &self.custom_variants
    }

    /// Remove variant
    pub fn remove_variant(&mut self, name: &str) -> Option<VariantDefinition> {
        self.variants.remove(name)
    }

    /// Remove custom variant
    pub fn remove_custom_variant(&mut self, name: &str) -> Option<CustomVariant> {
        self.custom_variants.remove(name)
    }

    /// Clear all variants
    pub fn clear_variants(&mut self) {
        self.variants.clear();
        self.custom_variants.clear();
    }

    /// Reset to default variants
    pub fn reset_to_defaults(&mut self) {
        self.clear_variants();
        self.initialize_default_variants();
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
    fn test_enhanced_variant_parser_creation() {
        let parser = EnhancedVariantParser::new();
        assert!(!parser.variants.is_empty());
        assert!(!parser.breakpoints.is_empty());
    }

    #[test]
    fn test_variant_definition_creation() {
        let variant = VariantDefinition {
            name: "hover".to_string(),
            variant_type: VariantType::State,
            selector_pattern: ":hover".to_string(),
            media_query: None,
            specificity: 10,
            combinable: true,
            dependencies: Vec::new(),
        };

        assert_eq!(variant.name, "hover");
        assert_eq!(variant.variant_type, VariantType::State);
        assert_eq!(variant.specificity, 10);
    }

    #[test]
    fn test_parse_simple_class() {
        let parser = EnhancedVariantParser::new();
        let result = parser.parse_class("p-4");

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.combination.base_class, "p-4");
        assert!(result.combination.variants.is_empty());
        assert!(result.combination.is_valid);
    }

    #[test]
    fn test_parse_single_variant() {
        let parser = EnhancedVariantParser::new();
        let result = parser.parse_class("hover:bg-blue-500");

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.combination.base_class, "bg-blue-500");
        assert_eq!(result.combination.variants.len(), 1);
        assert_eq!(result.combination.variants[0].name, "hover");
        assert!(result.combination.is_valid);
    }

    #[test]
    fn test_parse_multiple_variants() {
        let parser = EnhancedVariantParser::new();
        let result = parser.parse_class("dark:hover:bg-blue-500");

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.combination.base_class, "bg-blue-500");
        assert_eq!(result.combination.variants.len(), 2);
        assert!(result.combination.is_valid);
    }

    #[test]
    fn test_parse_responsive_variant() {
        let parser = EnhancedVariantParser::new();
        let result = parser.parse_class("md:p-4");

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.combination.base_class, "p-4");
        assert_eq!(result.combination.variants.len(), 1);
        assert_eq!(result.combination.variants[0].name, "md");
        assert!(result.combination.is_valid);
    }

    #[test]
    fn test_parse_complex_combination() {
        let parser = EnhancedVariantParser::new();
        let result = parser.parse_class("dark:group-hover:focus:bg-blue-500");

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.combination.base_class, "bg-blue-500");
        assert_eq!(result.combination.variants.len(), 3);
        assert!(result.combination.is_valid);
    }

    #[test]
    fn test_custom_variant_addition() {
        let mut parser = EnhancedVariantParser::new();
        let custom_variant = CustomVariant {
            name: "custom".to_string(),
            selector_pattern: ".custom".to_string(),
            media_query: None,
            specificity: 15,
            combinable: true,
        };

        parser.add_custom_variant(custom_variant);
        assert!(parser.custom_variants.contains_key("custom"));
    }

    #[test]
    fn test_variant_removal() {
        let mut parser = EnhancedVariantParser::new();
        let removed = parser.remove_variant("hover");
        assert!(removed.is_some());
        assert!(!parser.variants.contains_key("hover"));
    }

    #[test]
    fn test_parser_reset() {
        let mut parser = EnhancedVariantParser::new();
        parser.add_custom_variant(CustomVariant {
            name: "test".to_string(),
            selector_pattern: ".test".to_string(),
            media_query: None,
            specificity: 10,
            combinable: true,
        });

        assert!(parser.custom_variants.contains_key("test"));

        parser.reset_to_defaults();
        assert!(!parser.custom_variants.contains_key("test"));
        assert!(parser.variants.contains_key("hover"));
    }
}
