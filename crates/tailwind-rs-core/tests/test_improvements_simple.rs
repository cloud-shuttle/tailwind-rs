//! Simple test to verify our improvements work

use tailwind_rs_core::classes::{ClassBuilder, ClassSet};
use tailwind_rs_core::css_generator::{CssGenerationConfig, CssGenerator};

#[test]
fn test_hover_states_work() {
    let mut generator = CssGenerator::new();

    // Test hover states that were previously broken
    let hover_classes = vec![
        "hover:bg-zinc-700",
        "hover:text-teal-400",
        "hover:shadow-xl",
        "hover:opacity-80",
    ];

    for class in hover_classes {
        let result = generator.add_class(class);
        assert!(result.is_ok(), "Failed to add hover class: {}", class);
    }

    let css = generator.generate_css();
    assert!(!css.is_empty(), "Generated CSS should not be empty");

    // Verify hover states are in the CSS
    assert!(css.contains(":hover"), "CSS should contain hover selectors");
    assert!(
        css.contains("background-color"),
        "CSS should contain background-color properties"
    );
}

#[test]
fn test_dark_mode_works() {
    let mut generator = CssGenerator::new();

    // Test dark mode classes that were previously broken
    let dark_classes = vec![
        "dark:bg-zinc-800",
        "dark:text-zinc-200",
        "dark:border-zinc-700",
    ];

    for class in dark_classes {
        let result = generator.add_class(class);
        assert!(result.is_ok(), "Failed to add dark mode class: {}", class);
    }

    let css = generator.generate_css();
    assert!(!css.is_empty(), "Generated CSS should not be empty");

    // Verify dark mode is in the CSS
    assert!(
        css.contains(".dark"),
        "CSS should contain dark mode selectors"
    );
}

#[test]
fn test_interactive_utilities_work() {
    let mut generator = CssGenerator::new();

    // Test interactive utilities that were previously broken
    let interactive_classes = vec![
        "pointer-events-none",
        "pointer-events-auto",
        "cursor-pointer",
        "select-none",
    ];

    for class in interactive_classes {
        let result = generator.add_class(class);
        assert!(result.is_ok(), "Failed to add interactive class: {}", class);
    }

    let css = generator.generate_css();
    assert!(!css.is_empty(), "Generated CSS should not be empty");

    // Verify interactive properties are in the CSS
    assert!(
        css.contains("pointer-events"),
        "CSS should contain pointer-events"
    );
    assert!(css.contains("cursor"), "CSS should contain cursor");
    assert!(
        css.contains("user-select"),
        "CSS should contain user-select"
    );
}

#[test]
fn test_advanced_utilities_work() {
    let mut generator = CssGenerator::new();

    // Test advanced utilities that were previously broken
    let advanced_classes = vec![
        "text-transparent",
        "backdrop-blur-sm",
        "backdrop-opacity-50",
        "shadow-lg",
        "opacity-75",
    ];

    for class in advanced_classes {
        let result = generator.add_class(class);
        assert!(result.is_ok(), "Failed to add advanced class: {}", class);
    }

    let css = generator.generate_css();
    assert!(!css.is_empty(), "Generated CSS should not be empty");

    // Verify advanced properties are in the CSS
    assert!(
        css.contains("transparent"),
        "CSS should contain transparent"
    );
    assert!(
        css.contains("backdrop-filter"),
        "CSS should contain backdrop-filter"
    );
    assert!(css.contains("box-shadow"), "CSS should contain box-shadow");
    assert!(css.contains("opacity"), "CSS should contain opacity");
}

#[test]
fn test_classset_api_works() {
    let mut class_set = ClassSet::new();
    class_set.add_class("bg-blue-500");
    class_set.add_class("text-white");
    class_set.add_class("p-4");

    // Test the to_css_classes method that was mentioned as missing
    let css_classes = class_set.to_css_classes();
    assert!(
        !css_classes.is_empty(),
        "to_css_classes should return non-empty string"
    );
    assert!(
        css_classes.contains("bg-blue-500"),
        "Should contain bg-blue-500"
    );
    assert!(
        css_classes.contains("text-white"),
        "Should contain text-white"
    );
    assert!(css_classes.contains("p-4"), "Should contain p-4");
}

#[test]
fn test_classbuilder_api_works() {
    let class_set = ClassBuilder::new()
        .class("bg-blue-500")
        .class("text-white")
        .class("p-4")
        .build();

    let css_classes = class_set.to_css_classes();
    assert!(
        !css_classes.is_empty(),
        "ClassBuilder should produce non-empty CSS classes"
    );
    assert!(
        css_classes.contains("bg-blue-500"),
        "Should contain bg-blue-500"
    );
}
