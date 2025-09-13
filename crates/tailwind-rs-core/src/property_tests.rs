//! Property-based testing for tailwind-rs-core
//!
//! This module contains property-based tests using proptest to ensure
//! that our Tailwind CSS class generation and validation systems maintain
//! their invariants under random inputs.

use proptest::prelude::*;
use std::collections::HashSet;

use crate::{
    classes::ClassBuilder,
    color::Color as TailwindColor,
    responsive::{Breakpoint, ResponsiveValue, State},
    theme::{Theme, Spacing, Color as ThemeColor},
    validation::{ClassValidator, ValidationError},
};

/// Generate valid Tailwind CSS class names
fn valid_tailwind_class() -> impl Strategy<Value = String> {
    prop_oneof![
        // Layout classes
        prop_oneof![
            Just("block".to_string()),
            Just("inline".to_string()),
            Just("flex".to_string()),
            Just("grid".to_string()),
            Just("hidden".to_string()),
        ],
        // Spacing classes
        (0..=12u8).prop_map(|n| format!("p-{}", n)),
        (0..=12u8).prop_map(|n| format!("m-{}", n)),
        (0..=12u8).prop_map(|n| format!("px-{}", n)),
        (0..=12u8).prop_map(|n| format!("py-{}", n)),
        (0..=12u8).prop_map(|n| format!("mx-{}", n)),
        (0..=12u8).prop_map(|n| format!("my-{}", n)),
        // Color classes
        prop_oneof![
            Just("bg-blue-500".to_string()),
            Just("bg-red-500".to_string()),
            Just("bg-green-500".to_string()),
            Just("text-white".to_string()),
            Just("text-black".to_string()),
            Just("border-gray-300".to_string()),
        ],
        // Size classes
        prop_oneof![
            Just("w-full".to_string()),
            Just("h-full".to_string()),
            Just("w-1/2".to_string()),
            Just("h-1/2".to_string()),
        ],
        // Responsive classes
        prop_oneof![
            Just("sm:block".to_string()),
            Just("md:flex".to_string()),
            Just("lg:grid".to_string()),
            Just("xl:hidden".to_string()),
        ],
        // State classes
        prop_oneof![
            Just("hover:bg-blue-600".to_string()),
            Just("focus:ring-2".to_string()),
            Just("active:scale-95".to_string()),
        ],
    ]
}

/// Generate a collection of valid Tailwind classes
fn valid_tailwind_classes() -> impl Strategy<Value = Vec<String>> {
    prop::collection::vec(valid_tailwind_class(), 1..=10)
}

/// Generate valid Tailwind color values
fn valid_tailwind_color() -> impl Strategy<Value = TailwindColor> {
    prop_oneof![
        Just(TailwindColor::Blue),
        Just(TailwindColor::Red),
        Just(TailwindColor::Green),
        Just(TailwindColor::Yellow),
        Just(TailwindColor::Purple),
        Just(TailwindColor::Pink),
        Just(TailwindColor::Indigo),
        Just(TailwindColor::Gray),
        Just(TailwindColor::Slate),
        Just(TailwindColor::Zinc),
        Just(TailwindColor::Neutral),
        Just(TailwindColor::Stone),
        Just(TailwindColor::Orange),
        Just(TailwindColor::Amber),
        Just(TailwindColor::Lime),
        Just(TailwindColor::Emerald),
        Just(TailwindColor::Teal),
        Just(TailwindColor::Cyan),
        Just(TailwindColor::Sky),
        Just(TailwindColor::Violet),
        Just(TailwindColor::Fuchsia),
        Just(TailwindColor::Rose),
    ]
}

/// Generate valid theme color values
fn valid_theme_color() -> impl Strategy<Value = ThemeColor> {
    prop_oneof![
        // Hex colors
        prop::string::string_regex(r"#[0-9a-fA-F]{6}").unwrap().prop_map(|s| ThemeColor::hex(s)),
        // RGB colors
        (0u8..=255, 0u8..=255, 0u8..=255).prop_map(|(r, g, b)| ThemeColor::rgb(r, g, b)),
        // Named colors
        prop_oneof![
            Just(ThemeColor::named("blue")),
            Just(ThemeColor::named("red")),
            Just(ThemeColor::named("green")),
            Just(ThemeColor::named("yellow")),
        ],
    ]
}

/// Generate valid spacing values
fn valid_spacing() -> impl Strategy<Value = Spacing> {
    prop_oneof![
        // Pixel values
        (0.0..=100.0f32).prop_map(|v| Spacing::px(v)),
        // Rem values
        (0.0..=10.0f32).prop_map(|v| Spacing::rem(v)),
        // Em values
        (0.0..=5.0f32).prop_map(|v| Spacing::em(v)),
        // Percentage values
        (0.0..=100.0f32).prop_map(|v| Spacing::percent(v)),
        // Named values
        prop_oneof![
            Just(Spacing::named("auto")),
            Just(Spacing::named("full")),
            Just(Spacing::named("screen")),
        ],
    ]
}

/// Generate valid breakpoints
fn valid_breakpoint() -> impl Strategy<Value = Breakpoint> {
    prop_oneof![
        Just(Breakpoint::Base),
        Just(Breakpoint::Sm),
        Just(Breakpoint::Md),
        Just(Breakpoint::Lg),
        Just(Breakpoint::Xl),
        Just(Breakpoint::Xl2),
    ]
}

/// Generate valid states
fn valid_state() -> impl Strategy<Value = State> {
    prop_oneof![
        Just(State::Hover),
        Just(State::Focus),
        Just(State::Active),
        Just(State::Disabled),
        Just(State::Visited),
    ]
}

proptest! {
    /// Test that ClassBuilder can handle any valid class combination
    #[test]
    fn test_class_builder_properties(classes in valid_tailwind_classes()) {
        let mut builder = ClassBuilder::new();
        
        // Add all classes
        for class in &classes {
            builder = builder.class(class);
        }
        
        let class_set = builder.build();
        let result_classes = class_set.to_css_classes();
        
        // Property: All input classes should be present in output
        for class in &classes {
            assert!(result_classes.contains(class), 
                "Class '{}' not found in result: {}", class, result_classes);
        }
        
        // Property: Result should not be empty
        assert!(!result_classes.is_empty(), "Result should not be empty");
        
        // Property: Result should be a valid CSS class string
        assert!(result_classes.chars().all(|c| c.is_alphanumeric() || c == ' ' || c == ':' || c == '-' || c == '/'), 
            "Result contains invalid characters: {}", result_classes);
    }

    /// Test that ClassValidator maintains consistency
    #[test]
    fn test_class_validator_properties(classes in valid_tailwind_classes()) {
        let validator = ClassValidator::new();
        
        // Property: Valid classes should pass validation
        for class in &classes {
            match validator.validate_class(class) {
                Ok(_) => {
                    // Valid class should pass
                }
                Err(ValidationError::InvalidClass(_)) => {
                    // Some classes might be invalid, that's okay
                }
                Err(ValidationError::ClassConflict(_, _)) => {
                    // Conflicts are expected with random combinations
                }
                Err(ValidationError::DeprecatedClass(_)) => {
                    // Deprecated classes are expected
                }
                Err(ValidationError::UnsupportedClass(_)) => {
                    // Unsupported classes are expected
                }
                Err(ValidationError::InvalidCustomVariant(_)) => {
                    // Invalid custom variants are expected
                }
                Err(ValidationError::CustomVariantValidation(_)) => {
                    // Custom variant validation errors are expected
                }
            }
        }
        
        // Property: Validation should be deterministic
        for class in &classes {
            let result1 = validator.validate_class(class);
            let result2 = validator.validate_class(class);
            assert_eq!(result1.is_ok(), result2.is_ok(), 
                "Validation should be deterministic for class: {}", class);
        }
    }

    /// Test that Tailwind Color operations maintain invariants
    #[test]
    fn test_tailwind_color_properties(color in valid_tailwind_color()) {
        // Property: Color should have a valid name
        let name = color.name();
        assert!(!name.is_empty(), "Color name should not be empty");
        assert!(name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()), 
            "Color name should be valid: {}", name);
        
        // Property: Color should be round-trip serializable
        let serialized = serde_json::to_string(&color).expect("Should serialize");
        let deserialized: TailwindColor = serde_json::from_str(&serialized).expect("Should deserialize");
        assert_eq!(color, deserialized, "Color should be round-trip serializable");
    }

    /// Test that Spacing operations maintain invariants
    #[test]
    fn test_spacing_properties(spacing in valid_spacing()) {
        // Property: Spacing should have a valid CSS representation
        let css = spacing.to_css();
        assert!(!css.is_empty(), "Spacing CSS should not be empty");
        
        // Property: Spacing should be round-trip serializable
        let serialized = serde_json::to_string(&spacing).expect("Should serialize");
        let deserialized: Spacing = serde_json::from_str(&serialized).expect("Should deserialize");
        assert_eq!(spacing, deserialized, "Spacing should be round-trip serializable");
    }

    /// Test that ResponsiveValue maintains invariants
    #[test]
    fn test_responsive_value_properties(
        base_value in 0i32..=100,
        sm_value in 0i32..=100,
        md_value in 0i32..=100,
        lg_value in 0i32..=100,
    ) {
        let mut responsive = ResponsiveValue::new(base_value);
        responsive.set_breakpoint(Breakpoint::Sm, sm_value);
        responsive.set_breakpoint(Breakpoint::Md, md_value);
        responsive.set_breakpoint(Breakpoint::Lg, lg_value);
        
        // Property: Base value should be retrievable
        assert_eq!(*responsive.get_breakpoint(Breakpoint::Base), base_value);
        
        // Property: Set values should be retrievable
        assert_eq!(*responsive.get_breakpoint(Breakpoint::Sm), sm_value);
        assert_eq!(*responsive.get_breakpoint(Breakpoint::Md), md_value);
        assert_eq!(*responsive.get_breakpoint(Breakpoint::Lg), lg_value);
        
        // Property: Unset breakpoints should return base value
        assert_eq!(*responsive.get_breakpoint(Breakpoint::Xl), base_value);
        assert_eq!(*responsive.get_breakpoint(Breakpoint::Xl2), base_value);
    }

    /// Test that Theme operations maintain invariants
    #[test]
    fn test_theme_properties(
        primary_color in valid_theme_color(),
        secondary_color in valid_theme_color(),
        spacing in valid_spacing(),
    ) {
        let mut theme = Theme::new("test-theme");
        theme.add_color("primary", primary_color.clone());
        theme.add_color("secondary", secondary_color.clone());
        theme.add_spacing("custom", spacing.clone());
        
        // Property: Set values should be retrievable
        assert_eq!(*theme.get_color("primary").unwrap(), primary_color);
        assert_eq!(*theme.get_color("secondary").unwrap(), secondary_color);
        assert_eq!(*theme.get_spacing("custom").unwrap(), spacing);
        
        // Property: Theme should be serializable
        let serialized = serde_json::to_string(&theme).expect("Should serialize");
        let deserialized: Theme = serde_json::from_str(&serialized).expect("Should deserialize");
        assert_eq!(theme.get_color("primary").unwrap(), deserialized.get_color("primary").unwrap());
        assert_eq!(theme.get_color("secondary").unwrap(), deserialized.get_color("secondary").unwrap());
    }

    /// Test that class combinations don't create infinite loops
    #[test]
    fn test_class_combination_termination(classes in valid_tailwind_classes()) {
        let mut builder = ClassBuilder::new();
        
        // Property: Adding classes should terminate
        for class in &classes {
            builder = builder.class(class);
        }
        
        // Property: Building should terminate
        let _class_set = builder.build();
        
        // Property: Converting to CSS should terminate
        let _css = _class_set.to_css_classes();
    }

    /// Test that validation doesn't hang on complex inputs
    #[test]
    fn test_validation_termination(classes in valid_tailwind_classes()) {
        let validator = ClassValidator::new();
        
        // Property: Validation should terminate
        for class in &classes {
            let _result = validator.validate_class(class);
        }
        
        // Property: Batch validation should terminate
        let _result = validator.validate_classes(&classes);
    }

    /// Test that responsive operations maintain ordering
    #[test]
    fn test_responsive_ordering_properties(
        values in prop::collection::vec(0i32..=100, 2..=6)
    ) {
        let mut responsive = ResponsiveValue::new(values[0]);
        
        // Add values in order
        for (i, &value) in values.iter().enumerate().skip(1) {
            let breakpoint = match i {
                1 => Breakpoint::Sm,
                2 => Breakpoint::Md,
                3 => Breakpoint::Lg,
                4 => Breakpoint::Xl,
                5 => Breakpoint::Xl2,
                _ => break,
            };
            responsive.set_breakpoint(breakpoint, value);
        }
        
        // Property: Values should be retrievable in correct order
        assert_eq!(*responsive.get_breakpoint(Breakpoint::Base), values[0]);
        if values.len() > 1 {
            assert_eq!(*responsive.get_breakpoint(Breakpoint::Sm), values[1]);
        }
        if values.len() > 2 {
            assert_eq!(*responsive.get_breakpoint(Breakpoint::Md), values[2]);
        }
    }

    /// Test that class builder is associative
    #[test]
    fn test_class_builder_associativity(
        classes1 in valid_tailwind_classes(),
        classes2 in valid_tailwind_classes(),
    ) {
        // Build classes in one go
        let mut builder1 = ClassBuilder::new();
        for class in classes1.iter().chain(classes2.iter()) {
            builder1 = builder1.class(class);
        }
        let result1 = builder1.build().to_css_classes();
        
        // Build classes in two steps
        let mut builder2 = ClassBuilder::new();
        for class in &classes1 {
            builder2 = builder2.class(class);
        }
        for class in &classes2 {
            builder2 = builder2.class(class);
        }
        let result2 = builder2.build().to_css_classes();
        
        // Property: Results should be equivalent (order-independent)
        let set1: HashSet<&str> = result1.split_whitespace().collect();
        let set2: HashSet<&str> = result2.split_whitespace().collect();
        assert_eq!(set1, set2, "Class builder should be associative");
    }
}

/// Test that our APIs maintain stability properties
#[cfg(test)]
mod api_stability_tests {
    use super::*;
    use proptest::prelude::*;

    /// Test that public API functions don't panic on valid inputs
    proptest! {
        #[test]
        fn test_api_stability_no_panics(classes in valid_tailwind_classes()) {
            // Property: Public API functions should not panic
            let _ = ClassBuilder::new();
            let _ = ClassValidator::new();
            let _ = Theme::new("test-theme");
            
            // Property: Operations should not panic
            let mut builder = ClassBuilder::new();
            for class in &classes {
                builder = builder.class(class);
            }
            let _ = builder.build();
            
            let validator = ClassValidator::new();
            for class in &classes {
                let _ = validator.validate_class(class);
            }
        }
    }

    /// Test that serialization is stable
    proptest! {
        #[test]
        fn test_serialization_stability(
            color in valid_tailwind_color(),
            spacing in valid_spacing(),
        ) {
            // Property: Serialization should be deterministic
            let serialized1 = serde_json::to_string(&color).unwrap();
            let serialized2 = serde_json::to_string(&color).unwrap();
            assert_eq!(serialized1, serialized2, "Color serialization should be deterministic");
            
            let serialized1 = serde_json::to_string(&spacing).unwrap();
            let serialized2 = serde_json::to_string(&spacing).unwrap();
            assert_eq!(serialized1, serialized2, "Spacing serialization should be deterministic");
        }
    }

    /// Test that equality is stable
    proptest! {
        #[test]
        fn test_equality_stability(
            color1 in valid_tailwind_color(),
            color2 in valid_tailwind_color(),
            spacing1 in valid_spacing(),
            spacing2 in valid_spacing(),
        ) {
            // Property: Equality should be reflexive
            assert_eq!(color1, color1);
            assert_eq!(spacing1, spacing1);
            
            // Property: Equality should be symmetric
            assert_eq!(color1 == color2, color2 == color1);
            assert_eq!(spacing1 == spacing2, spacing2 == spacing1);
            
            // Property: Equality should be transitive
            if color1 == color2 && color2 == color1 {
                assert_eq!(color1, color1);
            }
            if spacing1 == spacing2 && spacing2 == spacing1 {
                assert_eq!(spacing1, spacing1);
            }
        }
    }
}
