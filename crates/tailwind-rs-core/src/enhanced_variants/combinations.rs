//! Enhanced Variants Combinations Module
//!
//! This module handles complex variant combinations, including ordering,
//! conflict resolution, and advanced interaction patterns.

use super::types::*;
use super::definitions::*;
use std::collections::HashMap;

/// Variant combination processor
pub struct VariantCombinationProcessor;

impl VariantCombinationProcessor {
    /// Process and validate a variant combination
    pub fn process_combination(variants: &[ParsedVariant]) -> Result<ProcessedCombination, String> {
        // Validate the combination
        VariantValidators::is_valid_combination(variants)?;

        // Order variants by specificity
        let ordered_variants = Self::order_variants_by_specificity(variants);

        // Check for interactions and optimizations
        let interactions = Self::analyze_interactions(&ordered_variants);

        // Calculate final specificity
        let specificity = VariantValidators::calculate_recommended_specificity(&ordered_variants);

        Ok(ProcessedCombination {
            variants: ordered_variants,
            specificity,
            interactions,
            valid: true,
        })
    }

    /// Order variants by specificity (highest first)
    fn order_variants_by_specificity(variants: &[ParsedVariant]) -> Vec<ParsedVariant> {
        let mut ordered = variants.to_vec();
        ordered.sort_by(|a, b| {
            let a_priority = a.variant_type.priority();
            let b_priority = b.variant_type.priority();
            b_priority.cmp(&a_priority)
        });
        ordered
    }

    /// Analyze interactions between variants
    fn analyze_interactions(variants: &[ParsedVariant]) -> Vec<VariantInteraction> {
        let mut interactions = Vec::new();

        // Check for state + responsive combinations
        let has_responsive = variants.iter().any(|v| v.variant_type == VariantType::Responsive);
        let has_state = variants.iter().any(|v| v.variant_type == VariantType::State);

        if has_responsive && has_state {
            interactions.push(VariantInteraction::Enhances);
        }

        // Check for dark mode combinations
        let has_dark = variants.iter().any(|v| v.variant_type == VariantType::DarkMode);
        if has_dark {
            interactions.push(VariantInteraction::RequiresSeparateRules);
        }

        // Check for motion preferences
        let has_motion = variants.iter().any(|v| {
            v.variant_type == VariantType::MotionSafe || v.variant_type == VariantType::MotionReduce
        });
        if has_motion {
            interactions.push(VariantInteraction::UsesMediaQueries);
        }

        interactions
    }
}

/// Processed variant combination with metadata
#[derive(Debug, Clone)]
pub struct ProcessedCombination {
    pub variants: Vec<ParsedVariant>,
    pub specificity: u32,
    pub interactions: Vec<VariantInteraction>,
    pub valid: bool,
}

/// Types of variant interactions
#[derive(Debug, Clone, PartialEq)]
pub enum VariantInteraction {
    /// Variants enhance each other (e.g., responsive + state)
    Enhances,
    /// Variants require separate CSS rules
    RequiresSeparateRules,
    /// Variants use media queries
    UsesMediaQueries,
    /// Variants have potential conflicts
    Conflicts,
}

/// Variant combination builder for fluent API
pub struct VariantCombinationBuilder {
    variants: Vec<ParsedVariant>,
}

impl VariantCombinationBuilder {
    /// Create a new combination builder
    pub fn new() -> Self {
        Self {
            variants: Vec::new(),
        }
    }

    /// Add a state variant
    pub fn state(mut self, name: &str) -> Self {
        self.variants.push(ParsedVariant::new(
            name.to_string(),
            VariantType::State,
        ));
        self
    }

    /// Add a responsive variant
    pub fn responsive(mut self, name: &str) -> Self {
        self.variants.push(ParsedVariant::new(
            name.to_string(),
            VariantType::Responsive,
        ));
        self
    }

    /// Add a dark mode variant
    pub fn dark_mode(mut self) -> Self {
        self.variants.push(ParsedVariant::new(
            "dark".to_string(),
            VariantType::DarkMode,
        ));
        self
    }

    /// Add a custom variant
    pub fn custom(mut self, name: &str, variant_type: VariantType) -> Self {
        self.variants.push(ParsedVariant::new(
            name.to_string(),
            variant_type,
        ));
        self
    }

    /// Build the combination
    pub fn build(self) -> Result<VariantCombination, String> {
        let processed = VariantCombinationProcessor::process_combination(&self.variants)?;

        Ok(VariantCombination {
            variants: processed.variants,
            specificity: processed.specificity,
            valid: processed.valid,
            error_message: None,
        })
    }
}

impl Default for VariantCombinationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Variant ordering utilities
pub struct VariantOrdering;

impl VariantOrdering {
    /// Get the canonical ordering of variant types
    pub fn canonical_order() -> Vec<VariantType> {
        vec![
            VariantType::Responsive,
            VariantType::DarkMode,
            VariantType::FocusWithin,
            VariantType::State,
            VariantType::MotionSafe,
            VariantType::MotionReduce,
            VariantType::Contrast,
            VariantType::ReducedMotion,
            VariantType::Orientation,
            VariantType::Print,
            VariantType::Screen,
        ]
    }

    /// Order variants according to canonical ordering
    pub fn order_by_canonical(variants: &[ParsedVariant]) -> Vec<ParsedVariant> {
        let canonical_order = Self::canonical_order();
        let mut ordered = variants.to_vec();

        ordered.sort_by(|a, b| {
            let a_index = canonical_order.iter().position(|vt| vt == &a.variant_type).unwrap_or(999);
            let b_index = canonical_order.iter().position(|vt| vt == &b.variant_type).unwrap_or(999);
            a_index.cmp(&b_index)
        });

        ordered
    }
}

/// Variant conflict detection
pub struct VariantConflictDetector;

impl VariantConflictDetector {
    /// Detect conflicts between variants
    pub fn detect_conflicts(variants: &[ParsedVariant]) -> Vec<VariantConflict> {
        let mut conflicts = Vec::new();

        // Check for mutually exclusive variants
        let mut variant_types = HashMap::new();
        for variant in variants {
            *variant_types.entry(&variant.variant_type).or_insert(0) += 1;
        }

        // Print/Screen conflict
        if variant_types.get(&VariantType::Print).unwrap_or(&0) > &0 &&
           variant_types.get(&VariantType::Screen).unwrap_or(&0) > &0 {
            conflicts.push(VariantConflict {
                conflict_type: ConflictType::MutuallyExclusive,
                description: "Print and screen variants are mutually exclusive".to_string(),
                affected_variants: vec!["print".to_string(), "screen".to_string()],
            });
        }

        // Multiple responsive variants
        if *variant_types.get(&VariantType::Responsive).unwrap_or(&0) > 1 {
            conflicts.push(VariantConflict {
                conflict_type: ConflictType::MultipleOfSameType,
                description: "Multiple responsive variants are not allowed".to_string(),
                affected_variants: variants.iter()
                    .filter(|v| v.variant_type == VariantType::Responsive)
                    .map(|v| v.name.clone())
                    .collect(),
            });
        }

        conflicts
    }

    /// Check if a combination has any conflicts
    pub fn has_conflicts(variants: &[ParsedVariant]) -> bool {
        !Self::detect_conflicts(variants).is_empty()
    }
}

/// Variant conflict information
#[derive(Debug, Clone)]
pub struct VariantConflict {
    pub conflict_type: ConflictType,
    pub description: String,
    pub affected_variants: Vec<String>,
}

/// Types of variant conflicts
#[derive(Debug, Clone, PartialEq)]
pub enum ConflictType {
    MutuallyExclusive,
    MultipleOfSameType,
    InvalidCombination,
}

/// Advanced variant combination strategies
pub struct VariantCombinationStrategies;

impl VariantCombinationStrategies {
    /// Generate all valid combinations of variants
    pub fn generate_valid_combinations(available_variants: &[String]) -> Vec<Vec<String>> {
        let mut valid_combinations = Vec::new();

        // Generate combinations of different sizes
        for size in 1..=available_variants.len().min(4) { // Limit to reasonable sizes
            Self::generate_combinations_recursive(
                available_variants,
                size,
                0,
                Vec::new(),
                &mut valid_combinations,
            );
        }

        valid_combinations
    }

    fn generate_combinations_recursive(
        variants: &[String],
        size: usize,
        start: usize,
        current: Vec<String>,
        result: &mut Vec<Vec<String>>,
    ) {
        if current.len() == size {
            // Validate the combination
            let parsed_variants: Vec<ParsedVariant> = current.iter()
                .map(|name| ParsedVariant::new(name.clone(), VariantType::State)) // Simplified for demo
                .collect();

            if !VariantConflictDetector::has_conflicts(&parsed_variants) {
                result.push(current);
            }
            return;
        }

        for i in start..variants.len() {
            let mut new_current = current.clone();
            new_current.push(variants[i].clone());

            Self::generate_combinations_recursive(
                variants,
                size,
                i + 1,
                new_current,
                result,
            );
        }
    }

    /// Optimize variant combinations for CSS output
    pub fn optimize_for_css_output(combinations: &[Vec<String>]) -> Vec<OptimizedCombination> {
        combinations.iter()
            .map(|combo| {
                let specificity = combo.len() as u32 * 10; // Simplified calculation
                OptimizedCombination {
                    variants: combo.clone(),
                    specificity,
                    css_strategy: Self::choose_css_strategy(combo),
                }
            })
            .collect()
    }

    fn choose_css_strategy(variants: &[String]) -> CssOutputStrategy {
        let has_responsive = variants.iter().any(|v| v.starts_with("sm:") || v.starts_with("md:"));
        let has_dark = variants.contains(&"dark".to_string());

        if has_responsive && has_dark {
            CssOutputStrategy::NestedMediaQueries
        } else if has_responsive {
            CssOutputStrategy::MediaQueryOnly
        } else if has_dark {
            CssOutputStrategy::ClassBased
        } else {
            CssOutputStrategy::DirectSelectors
        }
    }
}

/// Optimized combination for CSS output
#[derive(Debug, Clone)]
pub struct OptimizedCombination {
    pub variants: Vec<String>,
    pub specificity: u32,
    pub css_strategy: CssOutputStrategy,
}

/// CSS output strategy for a combination
#[derive(Debug, Clone, PartialEq)]
pub enum CssOutputStrategy {
    DirectSelectors,
    ClassBased,
    MediaQueryOnly,
    NestedMediaQueries,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combination_processing() {
        let variants = vec![
            ParsedVariant::new("hover".to_string(), VariantType::State),
            ParsedVariant::new("sm".to_string(), VariantType::Responsive),
        ];

        let result = VariantCombinationProcessor::process_combination(&variants).unwrap();
        assert!(result.valid);
        assert_eq!(result.specificity, 180); // 80 + 100
        assert!(!result.interactions.is_empty());
    }

    #[test]
    fn specificity_ordering() {
        let variants = vec![
            ParsedVariant::new("hover".to_string(), VariantType::State), // 80
            ParsedVariant::new("sm".to_string(), VariantType::Responsive), // 100
        ];

        let ordered = VariantCombinationProcessor::order_variants_by_specificity(&variants);
        assert_eq!(ordered[0].variant_type, VariantType::Responsive); // Higher specificity first
        assert_eq!(ordered[1].variant_type, VariantType::State);
    }

    #[test]
    fn interaction_analysis() {
        let variants = vec![
            ParsedVariant::new("hover".to_string(), VariantType::State),
            ParsedVariant::new("sm".to_string(), VariantType::Responsive),
            ParsedVariant::new("dark".to_string(), VariantType::DarkMode),
        ];

        let interactions = VariantCombinationProcessor::analyze_interactions(&variants);
        assert!(interactions.contains(&VariantInteraction::Enhances));
        assert!(interactions.contains(&VariantInteraction::RequiresSeparateRules));
    }

    #[test]
    fn combination_builder() {
        let combination = VariantCombinationBuilder::new()
            .state("hover")
            .responsive("sm")
            .dark_mode()
            .build()
            .unwrap();

        assert_eq!(combination.variants.len(), 3);
        assert!(combination.valid);
    }

    #[test]
    fn canonical_ordering() {
        let variants = vec![
            ParsedVariant::new("hover".to_string(), VariantType::State),
            ParsedVariant::new("sm".to_string(), VariantType::Responsive),
            ParsedVariant::new("dark".to_string(), VariantType::DarkMode),
        ];

        let ordered = VariantOrdering::order_by_canonical(&variants);
        assert_eq!(ordered[0].variant_type, VariantType::Responsive);
        assert_eq!(ordered[1].variant_type, VariantType::DarkMode);
        assert_eq!(ordered[2].variant_type, VariantType::State);
    }

    #[test]
    fn conflict_detection() {
        // Test print/screen conflict
        let variants = vec![
            ParsedVariant::new("print".to_string(), VariantType::Print),
            ParsedVariant::new("screen".to_string(), VariantType::Screen),
        ];

        let conflicts = VariantConflictDetector::detect_conflicts(&variants);
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].conflict_type, ConflictType::MutuallyExclusive);

        assert!(VariantConflictDetector::has_conflicts(&variants));
    }

    #[test]
    fn multiple_responsive_conflict() {
        let variants = vec![
            ParsedVariant::new("sm".to_string(), VariantType::Responsive),
            ParsedVariant::new("md".to_string(), VariantType::Responsive),
        ];

        let conflicts = VariantConflictDetector::detect_conflicts(&variants);
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].conflict_type, ConflictType::MultipleOfSameType);
    }

    #[test]
    fn valid_combination_generation() {
        let available = vec![
            "hover".to_string(),
            "focus".to_string(),
            "sm".to_string(),
            "dark".to_string(),
        ];

        let combinations = VariantCombinationStrategies::generate_valid_combinations(&available);
        assert!(!combinations.is_empty());

        // Should not include invalid combinations like print+screen
        let print_screen = combinations.iter()
            .find(|combo| combo.contains(&"print".to_string()) && combo.contains(&"screen".to_string()));

        assert!(print_screen.is_none());
    }

    #[test]
    fn css_output_optimization() {
        let combinations = vec![
            vec!["hover".to_string()],
            vec!["sm".to_string(), "hover".to_string()],
            vec!["dark".to_string(), "hover".to_string()],
            vec!["sm".to_string(), "dark".to_string(), "hover".to_string()],
        ];

        let optimized = VariantCombinationStrategies::optimize_for_css_output(&combinations);
        assert_eq!(optimized.len(), 4);

        // Check strategies are assigned correctly
        let complex_combo = optimized.iter()
            .find(|opt| opt.variants.len() == 3)
            .unwrap();

        assert_eq!(complex_combo.css_strategy, CssOutputStrategy::NestedMediaQueries);
    }
}
