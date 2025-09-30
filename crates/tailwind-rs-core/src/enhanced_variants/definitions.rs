//! Enhanced Variants Definitions Module
//!
//! This module contains all the built-in variant definitions for the enhanced
//! variant system, including state variants, responsive variants, and other
//! standard Tailwind variants.

use crate::responsive::Breakpoint;
use super::types::*;
use std::collections::HashMap;

/// Built-in variant definitions registry
pub struct VariantDefinitions;

impl VariantDefinitions {
    /// Get all standard variant definitions
    pub fn standard_definitions() -> HashMap<String, VariantDefinition> {
        let mut definitions = HashMap::new();

        // State variants
        Self::add_state_variants(&mut definitions);

        // Responsive variants
        Self::add_responsive_variants(&mut definitions);

        // Dark mode variants
        Self::add_dark_mode_variants(&mut definitions);

        // Focus variants
        Self::add_focus_variants(&mut definitions);

        // Motion variants
        Self::add_motion_variants(&mut definitions);

        // Contrast variants
        Self::add_contrast_variants(&mut definitions);

        // Orientation variants
        Self::add_orientation_variants(&mut definitions);

        // Print/Screen variants
        Self::add_print_variants(&mut definitions);

        definitions
    }

    /// Add state variants (hover, focus, active, etc.)
    fn add_state_variants(definitions: &mut HashMap<String, VariantDefinition>) {
        let state_variants = vec![
            ("hover", ":hover"),
            ("focus", ":focus"),
            ("focus-within", ":focus-within"),
            ("focus-visible", ":focus-visible"),
            ("active", ":active"),
            ("visited", ":visited"),
            ("target", ":target"),
            ("first", ":first-child"),
            ("last", ":last-child"),
            ("only", ":only-child"),
            ("odd", ":nth-child(odd)"),
            ("even", ":nth-child(even)"),
            ("first-of-type", ":first-of-type"),
            ("last-of-type", ":last-of-type"),
            ("only-of-type", ":only-of-type"),
            ("empty", ":empty"),
            ("disabled", ":disabled"),
            ("enabled", ":enabled"),
            ("checked", ":checked"),
            ("indeterminate", ":indeterminate"),
            ("default", ":default"),
            ("required", ":required"),
            ("valid", ":valid"),
            ("invalid", ":invalid"),
            ("in-range", ":in-range"),
            ("out-of-range", ":out-of-range"),
            ("placeholder-shown", ":placeholder-shown"),
            ("autofill", ":autofill"),
            ("read-only", ":read-only"),
            ("read-write", ":read-write"),
        ];

        for (name, selector) in state_variants {
            definitions.insert(
                name.to_string(),
                VariantDefinition::new(
                    name.to_string(),
                    VariantType::State,
                    selector.to_string(),
                ),
            );
        }
    }

    /// Add responsive variants (sm, md, lg, xl, 2xl)
    fn add_responsive_variants(definitions: &mut HashMap<String, VariantDefinition>) {
        let responsive_variants = vec![
            ("sm", "(min-width: 640px)"),
            ("md", "(min-width: 768px)"),
            ("lg", "(min-width: 1024px)"),
            ("xl", "(min-width: 1280px)"),
            ("2xl", "(min-width: 1536px)"),
        ];

        for (name, media_query) in responsive_variants {
            definitions.insert(
                name.to_string(),
                VariantDefinition::new(
                    name.to_string(),
                    VariantType::Responsive,
                    format!("@media {}", media_query),
                )
                .with_media_query(media_query.to_string()),
            );
        }
    }

    /// Add dark mode variants
    fn add_dark_mode_variants(definitions: &mut HashMap<String, VariantDefinition>) {
        definitions.insert(
            "dark".to_string(),
            VariantDefinition::new(
                "dark".to_string(),
                VariantType::DarkMode,
                ".dark".to_string(),
            ),
        );
    }

    /// Add focus-related variants
    fn add_focus_variants(definitions: &mut HashMap<String, VariantDefinition>) {
        definitions.insert(
            "focus-within".to_string(),
            VariantDefinition::new(
                "focus-within".to_string(),
                VariantType::FocusWithin,
                ":focus-within".to_string(),
            ),
        );
    }

    /// Add motion-related variants
    fn add_motion_variants(definitions: &mut HashMap<String, VariantDefinition>) {
        definitions.insert(
            "motion-safe".to_string(),
            VariantDefinition::new(
                "motion-safe".to_string(),
                VariantType::MotionSafe,
                "@media (prefers-reduced-motion: no-preference)".to_string(),
            )
            .with_media_query("(prefers-reduced-motion: no-preference)".to_string()),
        );

        definitions.insert(
            "motion-reduce".to_string(),
            VariantDefinition::new(
                "motion-reduce".to_string(),
                VariantType::MotionReduce,
                "@media (prefers-reduced-motion: reduce)".to_string(),
            )
            .with_media_query("(prefers-reduced-motion: reduce)".to_string()),
        );
    }

    /// Add contrast variants
    fn add_contrast_variants(definitions: &mut HashMap<String, VariantDefinition>) {
        definitions.insert(
            "contrast-more".to_string(),
            VariantDefinition::new(
                "contrast-more".to_string(),
                VariantType::Contrast,
                "@media (prefers-contrast: more)".to_string(),
            )
            .with_media_query("(prefers-contrast: more)".to_string()),
        );

        definitions.insert(
            "contrast-less".to_string(),
            VariantDefinition::new(
                "contrast-less".to_string(),
                VariantType::Contrast,
                "@media (prefers-contrast: less)".to_string(),
            )
            .with_media_query("(prefers-contrast: less)".to_string()),
        );
    }

    /// Add orientation variants
    fn add_orientation_variants(definitions: &mut HashMap<String, VariantDefinition>) {
        definitions.insert(
            "portrait".to_string(),
            VariantDefinition::new(
                "portrait".to_string(),
                VariantType::Orientation,
                "@media (orientation: portrait)".to_string(),
            )
            .with_media_query("(orientation: portrait)".to_string()),
        );

        definitions.insert(
            "landscape".to_string(),
            VariantDefinition::new(
                "landscape".to_string(),
                VariantType::Orientation,
                "@media (orientation: landscape)".to_string(),
            )
            .with_media_query("(orientation: landscape)".to_string()),
        );
    }

    /// Add print/screen variants
    fn add_print_variants(definitions: &mut HashMap<String, VariantDefinition>) {
        definitions.insert(
            "print".to_string(),
            VariantDefinition::new(
                "print".to_string(),
                VariantType::Print,
                "@media print".to_string(),
            )
            .with_media_query("print".to_string())
            .with_combinable(false),
        );

        definitions.insert(
            "screen".to_string(),
            VariantDefinition::new(
                "screen".to_string(),
                VariantType::Screen,
                "@media screen".to_string(),
            )
            .with_media_query("screen".to_string())
            .with_combinable(false),
        );
    }

    /// Get responsive breakpoints from the responsive module
    pub fn get_responsive_breakpoints() -> HashMap<String, Breakpoint> {
        // This would integrate with the responsive module
        // For now, return a basic mapping
        let mut breakpoints = HashMap::new();
        breakpoints.insert("sm".to_string(), Breakpoint::Sm);
        breakpoints.insert("md".to_string(), Breakpoint::Md);
        breakpoints.insert("lg".to_string(), Breakpoint::Lg);
        breakpoints.insert("xl".to_string(), Breakpoint::Xl);
        breakpoints.insert("2xl".to_string(), Breakpoint::Xl2);
        breakpoints
    }

    /// Create a custom variant definition
    pub fn create_custom_variant(
        name: String,
        selector: String,
        media_query: Option<String>,
        specificity: Option<u32>,
    ) -> VariantDefinition {
        let mut def = VariantDefinition::new(
            name.clone(),
            VariantType::Custom(name),
            selector,
        );

        if let Some(mq) = media_query {
            def = def.with_media_query(mq);
        }

        if let Some(spec) = specificity {
            def = def.with_specificity(spec);
        }

        def
    }
}

/// Variant validation utilities
pub struct VariantValidators;

impl VariantValidators {
    /// Validate variant name format
    pub fn is_valid_variant_name(name: &str) -> bool {
        // Variant names should be lowercase, alphanumeric, with optional hyphens
        !name.is_empty() &&
        name.chars().all(|c| c.is_lowercase() || c.is_alphanumeric() || c == '-') &&
        !name.starts_with('-') &&
        !name.ends_with('-')
    }

    /// Check if a variant combination is valid
    pub fn is_valid_combination(variants: &[ParsedVariant]) -> Result<(), String> {
        // Check for conflicting variants
        let has_print = variants.iter().any(|v| v.variant_type == VariantType::Print);
        let has_screen = variants.iter().any(|v| v.variant_type == VariantType::Screen);

        if has_print && has_screen {
            return Err("Cannot combine 'print' and 'screen' variants".to_string());
        }

        // Check for duplicate responsive variants
        let responsive_count = variants.iter()
            .filter(|v| v.variant_type == VariantType::Responsive)
            .count();

        if responsive_count > 1 {
            return Err("Cannot combine multiple responsive variants".to_string());
        }

        Ok(())
    }

    /// Get recommended specificity for a variant combination
    pub fn calculate_recommended_specificity(variants: &[ParsedVariant]) -> u32 {
        variants.iter()
            .filter_map(|v| {
                if v.matched {
                    Some(v.variant_type.priority())
                } else {
                    None
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_definitions_complete() {
        let definitions = VariantDefinitions::standard_definitions();

        // Check that key variants are present
        assert!(definitions.contains_key("hover"));
        assert!(definitions.contains_key("focus"));
        assert!(definitions.contains_key("sm"));
        assert!(definitions.contains_key("md"));
        assert!(definitions.contains_key("dark"));
        assert!(definitions.contains_key("motion-safe"));
        assert!(definitions.contains_key("print"));
        assert!(definitions.contains_key("portrait"));
    }

    #[test]
    fn state_variants_correct() {
        let definitions = VariantDefinitions::standard_definitions();

        let hover = definitions.get("hover").unwrap();
        assert_eq!(hover.variant_type, VariantType::State);
        assert_eq!(hover.selector_pattern, ":hover");
        assert_eq!(hover.specificity, 80);
        assert!(hover.combinable);
    }

    #[test]
    fn responsive_variants_correct() {
        let definitions = VariantDefinitions::standard_definitions();

        let sm = definitions.get("sm").unwrap();
        assert_eq!(sm.variant_type, VariantType::Responsive);
        assert!(sm.media_query.is_some());
        assert!(sm.media_query.as_ref().unwrap().contains("640px"));
        assert_eq!(sm.specificity, 100);
    }

    #[test]
    fn non_combinable_variants() {
        let definitions = VariantDefinitions::standard_definitions();

        let print = definitions.get("print").unwrap();
        assert!(!print.combinable);

        let screen = definitions.get("screen").unwrap();
        assert!(!screen.combinable);
    }

    #[test]
    fn custom_variant_creation() {
        let custom = VariantDefinitions::create_custom_variant(
            "my-variant".to_string(),
            ":my-selector".to_string(),
            Some("(my-media)".to_string()),
            Some(50),
        );

        assert_eq!(custom.name, "my-variant");
        assert_eq!(custom.selector_pattern, ":my-selector");
        assert_eq!(custom.media_query, Some("(my-media)".to_string()));
        assert_eq!(custom.specificity, 50);
    }

    #[test]
    fn responsive_breakpoints_mapping() {
        let breakpoints = VariantDefinitions::get_responsive_breakpoints();

        assert_eq!(breakpoints.get("sm"), Some(&Breakpoint::Sm));
        assert_eq!(breakpoints.get("md"), Some(&Breakpoint::Md));
        assert_eq!(breakpoints.get("lg"), Some(&Breakpoint::Lg));
        assert_eq!(breakpoints.get("xl"), Some(&Breakpoint::Xl));
        assert_eq!(breakpoints.get("2xl"), Some(&Breakpoint::Xl2));
    }

    #[test]
    fn variant_name_validation() {
        assert!(VariantValidators::is_valid_variant_name("hover"));
        assert!(VariantValidators::is_valid_variant_name("focus-visible"));
        assert!(VariantValidators::is_valid_variant_name("sm"));
        assert!(VariantValidators::is_valid_variant_name("2xl"));

        assert!(!VariantValidators::is_valid_variant_name(""));
        assert!(!VariantValidators::is_valid_variant_name("-invalid"));
        assert!(!VariantValidators::is_valid_variant_name("invalid-"));
        assert!(!VariantValidators::is_valid_variant_name("Invalid"));
        assert!(!VariantValidators::is_valid_variant_name("invalid variant"));
    }

    #[test]
    fn variant_combination_validation() {
        // Valid combinations
        let valid_combo = vec![
            ParsedVariant::new("hover".to_string(), VariantType::State),
            ParsedVariant::new("sm".to_string(), VariantType::Responsive),
        ];
        assert!(VariantValidators::is_valid_combination(&valid_combo).is_ok());

        // Invalid: print + screen
        let invalid_combo = vec![
            ParsedVariant::new("print".to_string(), VariantType::Print),
            ParsedVariant::new("screen".to_string(), VariantType::Screen),
        ];
        assert!(VariantValidators::is_valid_combination(&invalid_combo).is_err());

        // Invalid: multiple responsive
        let invalid_combo2 = vec![
            ParsedVariant::new("sm".to_string(), VariantType::Responsive),
            ParsedVariant::new("md".to_string(), VariantType::Responsive),
        ];
        assert!(VariantValidators::is_valid_combination(&invalid_combo2).is_err());
    }

    #[test]
    fn specificity_calculation() {
        let variants = vec![
            ParsedVariant::new("hover".to_string(), VariantType::State),
            ParsedVariant::new("sm".to_string(), VariantType::Responsive),
            ParsedVariant::new("dark".to_string(), VariantType::DarkMode),
        ];

        let specificity = VariantValidators::calculate_recommended_specificity(&variants);
        assert_eq!(specificity, 80 + 100 + 60); // 240
    }

    #[test]
    fn motion_variants_have_media_queries() {
        let definitions = VariantDefinitions::standard_definitions();

        let motion_safe = definitions.get("motion-safe").unwrap();
        assert!(motion_safe.media_query.is_some());
        assert!(motion_safe.media_query.as_ref().unwrap().contains("prefers-reduced-motion"));

        let motion_reduce = definitions.get("motion-reduce").unwrap();
        assert!(motion_reduce.media_query.is_some());
        assert!(motion_reduce.media_query.as_ref().unwrap().contains("prefers-reduced-motion"));
    }

    #[test]
    fn orientation_variants_have_media_queries() {
        let definitions = VariantDefinitions::standard_definitions();

        let portrait = definitions.get("portrait").unwrap();
        assert!(portrait.media_query.is_some());
        assert!(portrait.media_query.as_ref().unwrap().contains("portrait"));

        let landscape = definitions.get("landscape").unwrap();
        assert!(landscape.media_query.is_some());
        assert!(landscape.media_query.as_ref().unwrap().contains("landscape"));
    }
}
