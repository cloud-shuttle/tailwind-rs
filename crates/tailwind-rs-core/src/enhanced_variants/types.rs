//! Enhanced Variants Types Module
//!
//! This module defines the core types and data structures used throughout
//! the enhanced variant system, including variant definitions, types, and
//! parsing results.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Variant type classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VariantType {
    /// State variants (hover, focus, active, etc.)
    State,
    /// Responsive variants (sm, md, lg, xl, 2xl)
    Responsive,
    /// Dark mode variants
    DarkMode,
    /// Focus-within variants
    FocusWithin,
    /// Motion-safe variants
    MotionSafe,
    /// Motion-reduce variants
    MotionReduce,
    /// Contrast variants
    Contrast,
    /// Reduced motion variants
    ReducedMotion,
    /// Orientation variants (portrait, landscape)
    Orientation,
    /// Print variants
    Print,
    /// Screen variants (opposite of print)
    Screen,
    /// Custom user-defined variants
    Custom(String),
}

impl VariantType {
    /// Get the priority weight for this variant type
    pub fn priority(&self) -> u32 {
        match self {
            VariantType::Responsive => 100,
            VariantType::State => 80,
            VariantType::DarkMode => 60,
            VariantType::FocusWithin => 50,
            VariantType::MotionSafe | VariantType::MotionReduce => 40,
            VariantType::Contrast | VariantType::ReducedMotion => 30,
            VariantType::Orientation => 20,
            VariantType::Print | VariantType::Screen => 10,
            VariantType::Custom(_) => 5,
        }
    }

    /// Check if this variant can be combined with others
    pub fn is_combinable(&self) -> bool {
        match self {
            VariantType::Print | VariantType::Screen => false,
            _ => true,
        }
    }

    /// Get the CSS selector prefix for this variant type
    pub fn selector_prefix(&self) -> &'static str {
        match self {
            VariantType::State => ":",
            VariantType::Responsive => "",
            VariantType::DarkMode => ".dark ",
            VariantType::FocusWithin => ":focus-within ",
            VariantType::MotionSafe => "@media (prefers-reduced-motion: no-preference) { ",
            VariantType::MotionReduce => "@media (prefers-reduced-motion: reduce) { ",
            VariantType::Contrast => "@media (prefers-contrast: high) { ",
            VariantType::ReducedMotion => "@media (prefers-reduced-motion: reduce) { ",
            VariantType::Orientation => "",
            VariantType::Print => "@media print { ",
            VariantType::Screen => "@media screen { ",
            VariantType::Custom(_) => "",
        }
    }
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

impl VariantDefinition {
    /// Create a new variant definition
    pub fn new(
        name: String,
        variant_type: VariantType,
        selector_pattern: String,
    ) -> Self {
        Self {
            specificity: variant_type.priority(),
            combinable: variant_type.is_combinable(),
            media_query: None,
            dependencies: Vec::new(),
            name,
            variant_type,
            selector_pattern,
        }
    }

    /// Set the media query for this variant
    pub fn with_media_query(mut self, media_query: String) -> Self {
        self.media_query = Some(media_query);
        self
    }

    /// Set the specificity for this variant
    pub fn with_specificity(mut self, specificity: u32) -> Self {
        self.specificity = specificity;
        self
    }

    /// Set combinability for this variant
    pub fn with_combinable(mut self, combinable: bool) -> Self {
        self.combinable = combinable;
        self
    }

    /// Add a dependency for this variant
    pub fn with_dependency(mut self, dependency: String) -> Self {
        self.dependencies.push(dependency);
        self
    }

    /// Check if this variant can be applied given the current context
    pub fn can_apply(&self, active_variants: &[String]) -> bool {
        self.dependencies.iter().all(|dep| active_variants.contains(dep))
    }
}

/// Custom variant definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomVariant {
    /// Custom variant name
    pub name: String,
    /// CSS selector
    pub selector: String,
    /// Media query (optional)
    pub media_query: Option<String>,
    /// Specificity weight
    pub specificity: u32,
    /// Whether this variant can be combined
    pub combinable: bool,
}

impl CustomVariant {
    /// Create a new custom variant
    pub fn new(name: String, selector: String) -> Self {
        Self {
            specificity: 1,
            combinable: true,
            media_query: None,
            name,
            selector,
        }
    }

    /// Set the media query
    pub fn with_media_query(mut self, media_query: String) -> Self {
        self.media_query = Some(media_query);
        self
    }

    /// Set the specificity
    pub fn with_specificity(mut self, specificity: u32) -> Self {
        self.specificity = specificity;
        self
    }

    /// Set combinability
    pub fn with_combinable(mut self, combinable: bool) -> Self {
        self.combinable = combinable;
        self
    }
}

/// Parsed individual variant
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedVariant {
    /// Variant name
    pub name: String,
    /// Variant type
    pub variant_type: VariantType,
    /// Additional parameters (e.g., for responsive breakpoints)
    pub parameters: HashMap<String, String>,
    /// Whether this variant was found in the class
    pub matched: bool,
}

impl ParsedVariant {
    /// Create a new parsed variant
    pub fn new(name: String, variant_type: VariantType) -> Self {
        Self {
            parameters: HashMap::new(),
            matched: true,
            name,
            variant_type,
        }
    }

    /// Add a parameter to this variant
    pub fn with_parameter(mut self, key: String, value: String) -> Self {
        self.parameters.insert(key, value);
        self
    }

    /// Mark this variant as not matched
    pub fn unmatched(mut self) -> Self {
        self.matched = false;
        self
    }
}

/// Parsed variant combination
#[derive(Debug, Clone, PartialEq)]
pub struct VariantCombination {
    /// List of variants in this combination
    pub variants: Vec<ParsedVariant>,
    /// Combined specificity
    pub specificity: u32,
    /// Whether this combination is valid
    pub valid: bool,
    /// Error message if invalid
    pub error_message: Option<String>,
}

impl VariantCombination {
    /// Create a new variant combination
    pub fn new() -> Self {
        Self {
            variants: Vec::new(),
            specificity: 0,
            valid: true,
            error_message: None,
        }
    }

    /// Add a variant to this combination
    pub fn with_variant(mut self, variant: ParsedVariant) -> Self {
        self.variants.push(variant);
        self
    }

    /// Mark this combination as invalid
    pub fn invalid(mut self, message: String) -> Self {
        self.valid = false;
        self.error_message = Some(message);
        self
    }

    /// Calculate and set the combined specificity
    pub fn calculate_specificity(mut self) -> Self {
        self.specificity = self.variants.iter()
            .filter_map(|v| {
                if v.matched {
                    Some(match v.variant_type {
                        VariantType::Responsive => 100,
                        VariantType::State => 80,
                        VariantType::DarkMode => 60,
                        _ => 10,
                    })
                } else {
                    None
                }
            })
            .sum();
        self
    }
}

impl Default for VariantCombination {
    fn default() -> Self {
        Self::new()
    }
}

/// Enhanced variant parsing result
#[derive(Debug, Clone, PartialEq)]
pub struct VariantParseResult {
    /// The original class name
    pub original_class: String,
    /// The base class without variants
    pub base_class: String,
    /// The parsed variant combination
    pub combination: VariantCombination,
    /// CSS selector that should be generated
    pub css_selector: String,
    /// Media query if applicable
    pub media_query: Option<String>,
    /// Whether parsing was successful
    pub success: bool,
}

impl VariantParseResult {
    /// Create a successful parse result
    pub fn success(
        original_class: String,
        base_class: String,
        combination: VariantCombination,
        css_selector: String,
        media_query: Option<String>,
    ) -> Self {
        Self {
            original_class,
            base_class,
            combination,
            css_selector,
            media_query,
            success: true,
        }
    }

    /// Create a failed parse result
    pub fn failure(original_class: String, error_message: String) -> Self {
        Self {
            original_class,
            base_class: String::new(),
            combination: VariantCombination::new().invalid(error_message),
            css_selector: String::new(),
            media_query: None,
            success: false,
        }
    }
}

/// Variant processing metadata
#[derive(Debug, Clone)]
pub struct VariantMetadata {
    /// Total number of variants processed
    pub total_variants: usize,
    /// Number of successful combinations
    pub successful_combinations: usize,
    /// Number of failed combinations
    pub failed_combinations: usize,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
}

impl Default for VariantMetadata {
    fn default() -> Self {
        Self {
            total_variants: 0,
            successful_combinations: 0,
            failed_combinations: 0,
            processing_time_ms: 0,
            cache_hit_rate: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_type_priorities() {
        assert_eq!(VariantType::Responsive.priority(), 100);
        assert_eq!(VariantType::State.priority(), 80);
        assert_eq!(VariantType::DarkMode.priority(), 60);
        assert_eq!(VariantType::Print.priority(), 10);
    }

    #[test]
    fn variant_type_combinability() {
        assert!(VariantType::Responsive.is_combinable());
        assert!(VariantType::State.is_combinable());
        assert!(!VariantType::Print.is_combinable());
        assert!(!VariantType::Screen.is_combinable());
    }

    #[test]
    fn variant_definition_creation() {
        let def = VariantDefinition::new(
            "hover".to_string(),
            VariantType::State,
            ":hover".to_string(),
        );

        assert_eq!(def.name, "hover");
        assert_eq!(def.variant_type, VariantType::State);
        assert_eq!(def.selector_pattern, ":hover");
        assert_eq!(def.specificity, 80);
        assert!(def.combinable);
    }

    #[test]
    fn variant_definition_with_options() {
        let def = VariantDefinition::new("sm".to_string(), VariantType::Responsive, ".sm".to_string())
            .with_media_query("(min-width: 640px)".to_string())
            .with_specificity(150)
            .with_combinable(true)
            .with_dependency("mobile-first".to_string());

        assert_eq!(def.media_query, Some("(min-width: 640px)".to_string()));
        assert_eq!(def.specificity, 150);
        assert!(def.combinable);
        assert_eq!(def.dependencies, vec!["mobile-first"]);
    }

    #[test]
    fn custom_variant_creation() {
        let custom = CustomVariant::new("custom".to_string(), ":custom".to_string())
            .with_specificity(50)
            .with_media_query("(custom)".to_string())
            .with_combinable(false);

        assert_eq!(custom.name, "custom");
        assert_eq!(custom.selector, ":custom");
        assert_eq!(custom.specificity, 50);
        assert_eq!(custom.media_query, Some("(custom)".to_string()));
        assert!(!custom.combinable);
    }

    #[test]
    fn parsed_variant_operations() {
        let variant = ParsedVariant::new("hover".to_string(), VariantType::State)
            .with_parameter("modifier".to_string(), "strong".to_string())
            .unmatched();

        assert_eq!(variant.name, "hover");
        assert_eq!(variant.variant_type, VariantType::State);
        assert_eq!(variant.parameters.get("modifier"), Some(&"strong".to_string()));
        assert!(!variant.matched);
    }

    #[test]
    fn variant_combination_operations() {
        let mut combo = VariantCombination::new()
            .with_variant(ParsedVariant::new("hover".to_string(), VariantType::State))
            .with_variant(ParsedVariant::new("sm".to_string(), VariantType::Responsive))
            .calculate_specificity();

        assert_eq!(combo.variants.len(), 2);
        assert_eq!(combo.specificity, 180); // 80 + 100
        assert!(combo.valid);
    }

    #[test]
    fn variant_combination_error_handling() {
        let combo = VariantCombination::new().invalid("Invalid combination".to_string());

        assert!(!combo.valid);
        assert_eq!(combo.error_message, Some("Invalid combination".to_string()));
    }

    #[test]
    fn variant_parse_result_success() {
        let result = VariantParseResult::success(
            "sm:hover:bg-blue-500".to_string(),
            "bg-blue-500".to_string(),
            VariantCombination::new(),
            ".sm:hover.bg-blue-500".to_string(),
            Some("(min-width: 640px)".to_string()),
        );

        assert!(result.success);
        assert_eq!(result.original_class, "sm:hover:bg-blue-500");
        assert_eq!(result.base_class, "bg-blue-500");
        assert_eq!(result.css_selector, ".sm:hover.bg-blue-500");
        assert_eq!(result.media_query, Some("(min-width: 640px)".to_string()));
    }

    #[test]
    fn variant_parse_result_failure() {
        let result = VariantParseResult::failure(
            "invalid:class".to_string(),
            "Unknown variant 'invalid'".to_string(),
        );

        assert!(!result.success);
        assert_eq!(result.original_class, "invalid:class");
        assert!(!result.combination.valid);
        assert_eq!(result.combination.error_message, Some("Unknown variant 'invalid'".to_string()));
    }

    #[test]
    fn variant_definition_dependency_checking() {
        let def = VariantDefinition::new("complex".to_string(), VariantType::State, ":complex".to_string())
            .with_dependency("hover".to_string())
            .with_dependency("focus".to_string());

        assert!(def.can_apply(&["hover".to_string(), "focus".to_string()]));
        assert!(!def.can_apply(&["hover".to_string()]));
        assert!(!def.can_apply(&[]));
    }

    #[test]
    fn variant_metadata_default() {
        let meta = VariantMetadata::default();

        assert_eq!(meta.total_variants, 0);
        assert_eq!(meta.successful_combinations, 0);
        assert_eq!(meta.failed_combinations, 0);
        assert_eq!(meta.processing_time_ms, 0);
        assert_eq!(meta.cache_hit_rate, 0.0);
    }
}
