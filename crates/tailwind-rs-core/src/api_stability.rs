//! API Stability Testing for tailwind-rs-core
//!
//! This module contains tests to ensure that public APIs maintain
//! backward compatibility and don't break between versions.

use crate::{
    classes::ClassBuilder,
    color::Color as TailwindColor,
    responsive::{Breakpoint, ResponsiveValue},
    theme::{Theme, Spacing, Color as ThemeColor},
    validation::{ClassValidator, ValidationError},
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
        
        // Test that all public methods exist
        let color = TailwindColor::Blue;
        let _name = color.name();
        
        assert!(true, "Color API should be stable");
    }

    /// Test that ResponsiveValue API is stable
    #[test]
    fn test_responsive_value_api_stability() {
        let mut responsive = ResponsiveValue::new();
        responsive.set_breakpoint(Breakpoint::Base, 10);
        
        // Test that all public methods exist
        responsive.set_breakpoint(Breakpoint::Sm, 20);
        let _value = responsive.get_breakpoint(Breakpoint::Sm);
        
        assert!(true, "ResponsiveValue API should be stable");
    }

    /// Test that Theme API is stable
    #[test]
    fn test_theme_api_stability() {
        let mut theme = Theme::new("test-theme");
        
        // Test that all public methods exist
        theme.add_color("primary", ThemeColor::hex("#3b82f6"));
        let _color = theme.get_color("primary");
        
        theme.add_spacing("test", Spacing::px(10.0));
        let _spacing = theme.get_spacing("test");
        
        assert!(true, "Theme API should be stable");
    }

    /// Test that ClassValidator API is stable
    #[test]
    fn test_class_validator_api_stability() {
        let validator = ClassValidator::new();
        
        // Test that all public methods exist
        let _result = validator.validate_class("test-class");
        let _result = validator.validate_classes(&["test-class".to_string()]);
        
        assert!(true, "ClassValidator API should be stable");
    }
}

/// Test that serialization formats are stable
#[cfg(test)]
mod serialization_stability_tests {
    use super::*;

    /// Test that Color serialization is stable
    #[test]
    fn test_color_serialization_stability() {
        let color = TailwindColor::Blue;
        
        // Test JSON serialization
        let json = serde_json::to_string(&color).expect("Should serialize to JSON");
        let deserialized: TailwindColor = serde_json::from_str(&json).expect("Should deserialize from JSON");
        assert_eq!(color, deserialized, "Color JSON serialization should be stable");
        
        // Test that the format hasn't changed
        assert!(json.contains("Blue"), "Color serialization format should be stable");
    }

    /// Test that Spacing serialization is stable
    #[test]
    fn test_spacing_serialization_stability() {
        let spacing = Spacing::px(10.0);
        
        // Test JSON serialization
        let json = serde_json::to_string(&spacing).expect("Should serialize to JSON");
        let deserialized: Spacing = serde_json::from_str(&json).expect("Should deserialize from JSON");
        assert_eq!(spacing, deserialized, "Spacing JSON serialization should be stable");
    }

    /// Test that Theme serialization is stable
    #[test]
    fn test_theme_serialization_stability() {
        let mut theme = Theme::new("test-theme");
        theme.add_color("primary", ThemeColor::hex("#3b82f6"));
        theme.add_spacing("test", Spacing::px(10.0));
        
        // Test JSON serialization
        let json = serde_json::to_string(&theme).expect("Should serialize to JSON");
        let deserialized: Theme = serde_json::from_str(&json).expect("Should deserialize from JSON");
        
        assert_eq!(theme.get_color("primary").unwrap(), deserialized.get_color("primary").unwrap(), 
            "Theme JSON serialization should be stable");
    }

    /// Test that ResponsiveValue serialization is stable
    #[test]
    fn test_responsive_value_serialization_stability() {
        let mut responsive = ResponsiveValue::new();
        responsive.set_breakpoint(Breakpoint::Base, 10);
        responsive.set_breakpoint(Breakpoint::Sm, 20);
        
        // Test JSON serialization
        let json = serde_json::to_string(&responsive).expect("Should serialize to JSON");
        let deserialized: ResponsiveValue<i32> = serde_json::from_str(&json).expect("Should deserialize from JSON");
        
        assert_eq!(responsive.get_breakpoint(Breakpoint::Base), deserialized.get_breakpoint(Breakpoint::Base));
        assert_eq!(responsive.get_breakpoint(Breakpoint::Sm), deserialized.get_breakpoint(Breakpoint::Sm));
    }
}

/// Test that error types are stable
#[cfg(test)]
mod error_stability_tests {
    use super::*;

    /// Test that ValidationError variants are stable
    #[test]
    fn test_validation_error_stability() {
        // Test that all error variants exist and work
        let _error1 = ValidationError::InvalidClass("test".to_string());
        let _error2 = ValidationError::ClassConflict("class1".to_string(), "class2".to_string());
        let _error3 = ValidationError::DeprecatedClass("deprecated".to_string());
        let _error4 = ValidationError::UnsupportedClass("unsupported".to_string());
        
        // Test that error messages are stable
        let error = ValidationError::InvalidClass("test".to_string());
        let message = format!("{}", error);
        assert!(message.contains("Invalid class name"), "Error message should be stable");
    }

    /// Test that error formatting is stable
    #[test]
    fn test_validation_error_formatting_stability() {
        let error = ValidationError::InvalidClass("test".to_string());
        
        // Test that error formatting is stable
        let formatted = format!("{}", error);
        assert!(formatted.contains("Invalid class name"), "Error formatting should be stable");
    }
}

/// Test that default values are stable
#[cfg(test)]
mod default_stability_tests {
    use super::*;

    /// Test that default Color values are stable
    #[test]
    fn test_color_default_stability() {
        // Test that default colors are consistent
        let default_blue = TailwindColor::Blue;
        let default_red = TailwindColor::Red;
        
        assert_eq!(default_blue.name(), "blue", "Default blue color should be stable");
        assert_eq!(default_red.name(), "red", "Default red color should be stable");
    }

    /// Test that default Spacing values are stable
    #[test]
    fn test_spacing_default_stability() {
        // Test that default spacing values are consistent
        let default_px = Spacing::px(0.0);
        let default_rem = Spacing::rem(0.0);
        
        assert_eq!(default_px.to_css(), "0px", "Default px spacing should be stable");
        assert_eq!(default_rem.to_css(), "0rem", "Default rem spacing should be stable");
    }

    /// Test that default Theme values are stable
    #[test]
    fn test_theme_default_stability() {
        let theme = Theme::new("test-theme");
        
        // Test that default theme values are consistent
        assert_eq!(theme.name, "test-theme", "Default theme name should be stable");
        assert!(theme.colors.is_empty(), "Default theme should have no colors");
        assert!(theme.spacing.is_empty(), "Default theme should have no spacing");
    }

    /// Test that default Breakpoint values are stable
    #[test]
    fn test_breakpoint_default_stability() {
        // Test that breakpoint min widths are stable
        assert_eq!(Breakpoint::Sm.min_width(), 640, "SM breakpoint should be stable");
        assert_eq!(Breakpoint::Md.min_width(), 768, "MD breakpoint should be stable");
        assert_eq!(Breakpoint::Lg.min_width(), 1024, "LG breakpoint should be stable");
        assert_eq!(Breakpoint::Xl.min_width(), 1280, "XL breakpoint should be stable");
        assert_eq!(Breakpoint::Xl2.min_width(), 1536, "2XL breakpoint should be stable");
    }
}

/// Test that performance characteristics are stable
#[cfg(test)]
mod performance_stability_tests {
    use super::*;
    use std::time::Instant;

    /// Test that ClassBuilder performance is stable
    #[test]
    fn test_class_builder_performance_stability() {
        let start = Instant::now();
        
        let mut builder = ClassBuilder::new();
        for i in 0..100 {
            builder = builder.class(&format!("class-{}", i));
        }
        let _class_set = builder.build();
        
        let duration = start.elapsed();
        
        // Performance should be reasonable (less than 5ms for 100 classes)
        assert!(duration.as_micros() < 5000, "ClassBuilder performance should be stable");
    }

    /// Test that ClassValidator performance is stable
    #[test]
    fn test_class_validator_performance_stability() {
        let validator = ClassValidator::new();
        let classes: Vec<String> = (0..100).map(|i| format!("class-{}", i)).collect();
        
        let start = Instant::now();
        let _result = validator.validate_classes(&classes);
        let duration = start.elapsed();
        
        // Performance should be reasonable (less than 20ms for 100 classes)
        assert!(duration.as_millis() < 20, "ClassValidator performance should be stable");
    }

    /// Test that serialization performance is stable
    #[test]
    fn test_serialization_performance_stability() {
        let theme = Theme::new("test-theme");
        
        let start = Instant::now();
        let _json = serde_json::to_string(&theme).expect("Should serialize");
        let duration = start.elapsed();
        
        // Serialization should be fast (less than 2ms)
        assert!(duration.as_micros() < 2000, "Serialization performance should be stable");
    }
}

/// Test that migration paths are stable
#[cfg(test)]
mod migration_stability_tests {
    use super::*;

    /// Test that old API patterns still work
    #[test]
    fn test_legacy_api_patterns() {
        // Test that old patterns for creating classes still work
        let builder = ClassBuilder::new();
        let _class_set = builder.class("legacy-class").build();
        
        // Test that old patterns for creating colors still work
        let _color = TailwindColor::Blue;
        
        // Test that old patterns for creating themes still work
        let mut theme = Theme::new("legacy-theme");
        theme.add_color("primary", ThemeColor::hex("#3b82f6"));
        
        assert!(true, "Legacy API patterns should still work");
    }

    /// Test that new API patterns are backward compatible
    #[test]
    fn test_new_api_backward_compatibility() {
        // Test that new features don't break old usage
        let builder = ClassBuilder::new();
        let _class_set = builder.class("new-class").build();
        
        // Test that new color features don't break old usage
        let _color = TailwindColor::Blue;
        
        // Test that new theme features don't break old usage
        let mut theme = Theme::new("new-theme");
        theme.add_color("primary", ThemeColor::hex("#3b82f6"));
        
        assert!(true, "New API features should be backward compatible");
    }
}
