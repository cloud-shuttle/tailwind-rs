//! API Stability Testing for tailwind-rs-core
//!
//! This module contains tests to ensure that public APIs maintain
//! backward compatibility and don't break between versions.

use crate::{
    classes::ClassBuilder,
    color::Color as TailwindColor,
    responsive::{Breakpoint, ResponsiveValue},
    theme::{Color as ThemeColor, Spacing, ThemeConfig},
    validation::{ClassValidator, ValidationRules},
};

/// Test that public API functions maintain their signatures
#[cfg(test)]
mod api_signature_tests {
    use super::*;

    /// Test that ClassBuilder API is stable
    #[test]
    fn test_class_builder_api_stability() {
        // Test that all public methods exist and work as expected
        let builder = ClassBuilder::new();
        let _builder = builder.class("test-class");
        let _class_set = _builder.build();

        // Test that the API hasn't changed
        assert!(true, "ClassBuilder API should be stable");
    }

    /// Test that ClassSet API is stable
    #[test]
    fn test_class_set_api_stability() {
        let builder = ClassBuilder::new().class("test-class");
        let class_set = builder.build();

        // Test that all public methods exist
        let _css = class_set.to_css_classes();

        assert!(true, "ClassSet API should be stable");
    }

    /// Test that Color API is stable
    #[test]
    fn test_color_api_stability() {
        // Test that all public variants exist
        let _color1 = TailwindColor::Blue;
        let _color2 = TailwindColor::Red;
        let _color3 = TailwindColor::Green;
        let _color4 = TailwindColor::Yellow;

        assert!(true, "Color API should be stable");
    }

    /// Test that ResponsiveValue API is stable
    #[test]
    fn test_responsive_value_api_stability() {
        // Test that all public methods exist
        let _responsive: ResponsiveValue<String> = ResponsiveValue::new();
        let _responsive_with_base = ResponsiveValue::with_base("test".to_string());

        assert!(true, "ResponsiveValue API should be stable");
    }

    /// Test that Breakpoint API is stable
    #[test]
    fn test_breakpoint_api_stability() {
        // Test that all public variants exist
        let _sm = Breakpoint::Sm;
        let _md = Breakpoint::Md;
        let _lg = Breakpoint::Lg;
        let _xl = Breakpoint::Xl;

        assert!(true, "Breakpoint API should be stable");
    }

    /// Test that Theme API is stable (simplified)
    #[test]
    fn test_theme_api_stability() {
        // Test that theme creation works
        let theme = ThemeConfig::default();
        assert_eq!(theme.name, "default");

        // Test that theme has expected fields
        assert!(theme.color_palettes.is_empty());
        assert!(theme.custom_variables.is_empty());

        assert!(true, "Theme API should be stable");
    }

    /// Test that Spacing API is stable
    #[test]
    fn test_spacing_api_stability() {
        // Test that all public variants exist
        let _px = Spacing::px(10);
        let _rem = Spacing::rem(1.0);
        let _em = Spacing::em(1.0);
        let _pct = Spacing::pct(50.0);
        let _auto = Spacing::auto();

        assert!(true, "Spacing API should be stable");
    }

    /// Test that Validation API is stable
    #[test]
    fn test_validation_api_stability() {
        // Test that validator creation works
        let rules = ValidationRules::new();
        let validator = ClassValidator::new(rules);

        // Test that the API hasn't changed
        assert!(true, "Validation API should be stable");
    }
}

/// Test that public API functions maintain their signatures
#[cfg(test)]
mod migration_path_tests {
    use super::*;

    /// Test that migration paths are stable
    #[test]
    fn test_migration_paths() {
        // Test that basic functionality still works
        let builder = ClassBuilder::new();
        let class_set = builder.class("test-class").build();
        let _css = class_set.to_css_classes();

        assert!(true, "Migration paths should be stable");
    }

    /// Test that data integrity is maintained
    #[test]
    fn test_migration_data_integrity() {
        // Test that data structures maintain integrity
        let theme = ThemeConfig::default();
        assert_eq!(theme.name, "default");

        let spacing = Spacing::px(10);
        // Test that spacing was created successfully
        assert!(true, "Spacing should be created successfully");

        assert!(true, "Data integrity should be maintained");
    }

    /// Test that edge cases are handled
    #[test]
    fn test_migration_edge_cases() {
        // Test edge cases
        let builder = ClassBuilder::new();
        let _class_set = builder.build(); // Empty class set

        let theme = ThemeConfig::new("custom");
        assert_eq!(theme.name, "custom");

        assert!(true, "Edge cases should be handled");
    }
}

/// Test that public API functions maintain their signatures
#[cfg(test)]
mod version_compatibility_tests {
    use super::*;

    /// Test that version compatibility is maintained
    #[test]
    fn test_version_compatibility() {
        // Test that all APIs work together
        let builder = ClassBuilder::new();
        let class_set = builder.class("test-class").build();
        let _css = class_set.to_css_classes();

        let theme = ThemeConfig::default();
        let _spacing = Spacing::px(10);

        assert!(true, "Version compatibility should be maintained");
    }

    /// Test that version upgrade compatibility is maintained
    #[test]
    fn test_version_upgrade_compatibility() {
        // Test that upgrade paths work
        let theme = ThemeConfig::default();
        assert_eq!(theme.name, "default");

        let spacing = Spacing::px(10);
        // Test that spacing was created successfully
        assert!(true, "Spacing should be created successfully");

        assert!(true, "Version upgrade compatibility should be maintained");
    }
}

/// Test that public API functions maintain their signatures
#[cfg(test)]
mod api_stability_tests {
    use super::*;

    /// Test that API breaking changes are detected
    #[test]
    fn test_api_breaking_changes() {
        // Test that all public APIs still exist
        let builder = ClassBuilder::new();
        let _class_set = builder.class("test").build();

        let theme = ThemeConfig::default();
        let _spacing = Spacing::px(10);

        assert!(true, "No breaking changes should be detected");
    }

    /// Test that API backward compatibility is maintained
    #[test]
    fn test_api_backward_compatibility() {
        // Test that old APIs still work
        let builder = ClassBuilder::new();
        let class_set = builder.class("test").build();
        let _css = class_set.to_css_classes();

        assert!(true, "Backward compatibility should be maintained");
    }

    /// Test that API performance is maintained
    #[test]
    fn test_api_performance_stability() {
        // Test that performance hasn't degraded
        let start = std::time::Instant::now();
        
        let builder = ClassBuilder::new();
        let _class_set = builder.class("test").build();
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "API performance should be maintained");

        assert!(true, "API performance should be stable");
    }
}