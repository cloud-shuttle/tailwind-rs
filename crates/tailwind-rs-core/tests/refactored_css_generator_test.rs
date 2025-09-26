//! Integration test for the refactored CSS generator structure

use tailwind_rs_core::css_generator::{CssGenerationConfig, CssGenerator};

#[test]
fn test_refactored_css_generator_basic() {
    let mut generator = CssGenerator::new();

    // Test that we can add classes
    assert!(generator.add_class("p-4").is_ok());
    assert!(generator.add_class("m-2").is_ok());

    // Test that we can generate CSS
    let css = generator.generate_css();
    assert!(!css.is_empty());

    // Test that we have the expected number of rules
    assert_eq!(generator.rule_count(), 2);
}

#[test]
fn test_refactored_css_generator_spacing() {
    let mut generator = CssGenerator::new();

    // Test spacing classes (using the spacing parser)
    assert!(generator.add_class("p-4").is_ok());
    assert!(generator.add_class("m-2").is_ok());
    assert!(generator.add_class("px-6").is_ok());
    assert!(generator.add_class("py-3").is_ok());

    let css = generator.generate_css();
    assert!(css.contains("padding"));
    assert!(css.contains("margin"));
}

#[test]
fn test_refactored_css_generator_animations() {
    let mut generator = CssGenerator::new();

    // Test animation classes (using the animation parser)
    assert!(generator.add_class("animate-spin").is_ok());
    assert!(generator.add_class("animate-pulse").is_ok());

    let css = generator.generate_css();
    assert!(css.contains("animation"));
}

#[test]
fn test_refactored_css_generator_variants() {
    let mut generator = CssGenerator::new();

    // Test variant classes (using the variant parser)
    assert!(generator.add_class("hover:p-4").is_ok());
    assert!(generator.add_class("dark:m-2").is_ok());
    assert!(generator.add_class("md:px-6").is_ok());

    let css = generator.generate_css();
    assert!(css.contains(":hover"));
    assert!(css.contains(".dark"));
    assert!(css.contains("@media"));
}

#[test]
fn test_refactored_css_generator_device_variants() {
    let mut generator = CssGenerator::new();

    // Test device variant classes
    assert!(generator.add_class("pointer-coarse:p-4").is_ok());
    assert!(generator.add_class("motion-reduce:m-2").is_ok());

    let css = generator.generate_css();
    assert!(css.contains("@media"));
    assert!(css.contains("pointer: coarse") || css.contains("prefers-reduced-motion"));
}

#[test]
fn test_refactored_css_generator_with_config() {
    let config = CssGenerationConfig {
        include_responsive: true,
        include_dark_mode: true,
        include_interactive: true,
        include_device_variants: true,
        ..Default::default()
    };

    let generator = CssGenerator::with_config(config);

    // Test that the generator was created successfully
    assert_eq!(generator.rule_count(), 0);

    // Test that the configuration is set correctly
    assert!(generator.config().include_responsive);
    assert!(generator.config().include_dark_mode);
    assert!(generator.config().include_interactive);
    assert!(generator.config().include_device_variants);
}

#[test]
fn test_refactored_css_generator_minified_output() {
    let mut generator = CssGenerator::new();

    generator.add_class("p-4").unwrap();
    generator.add_class("m-2").unwrap();

    let minified_css = generator.generate_minified_css();
    assert!(!minified_css.is_empty());

    // Minified CSS should be shorter (no extra whitespace)
    let regular_css = generator.generate_css();
    assert!(minified_css.len() < regular_css.len());
}
