//! Comprehensive tests for spacing utilities
//!
//! This module contains all the tests for spacing utilities, including
//! SpacingValue conversions, padding utilities, margin utilities, gap utilities,
//! space-between utilities, and divide utilities.

use super::spacing_values::SpacingValue;
use super::spacing_utilities::*;
use crate::classes::ClassBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_value_to_css_value() {
        assert_eq!(SpacingValue::Zero.to_css_value(), "0");
        assert_eq!(SpacingValue::Px.to_css_value(), "1px");
        assert_eq!(SpacingValue::Fractional(0.5).to_css_value(), "0.125rem");
        assert_eq!(SpacingValue::Integer(4).to_css_value(), "1rem");
        assert_eq!(SpacingValue::Auto.to_css_value(), "auto");
        assert_eq!(SpacingValue::Full.to_css_value(), "100%");
        assert_eq!(SpacingValue::Screen.to_css_value(), "100vh");
    }

    #[test]
    fn test_spacing_value_to_class_name() {
        assert_eq!(SpacingValue::Zero.to_class_name(), "0");
        assert_eq!(SpacingValue::Px.to_class_name(), "px");
        assert_eq!(SpacingValue::Fractional(0.5).to_class_name(), "0.5");
        assert_eq!(SpacingValue::Integer(4).to_class_name(), "4");
        assert_eq!(SpacingValue::Auto.to_class_name(), "auto");
        assert_eq!(SpacingValue::Full.to_class_name(), "full");
        assert_eq!(SpacingValue::Screen.to_class_name(), "screen");
    }

    #[test]
    fn test_padding_utilities() {
        let classes = ClassBuilder::new()
            .padding(SpacingValue::Integer(4))
            .padding_x(SpacingValue::Integer(6))
            .padding_y(SpacingValue::Integer(2))
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("p-4"));
        assert!(css_classes.contains("px-6"));
        assert!(css_classes.contains("py-2"));
    }

    #[test]
    fn test_margin_utilities() {
        let classes = ClassBuilder::new()
            .margin(SpacingValue::Integer(8))
            .margin_x(SpacingValue::Integer(4))
            .margin_y(SpacingValue::Integer(2))
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("m-8"));
        assert!(css_classes.contains("mx-4"));
        assert!(css_classes.contains("my-2"));
    }

    #[test]
    fn test_negative_margin_utilities() {
        let classes = ClassBuilder::new()
            .margin_negative(SpacingValue::Integer(4))
            .margin_x_negative(SpacingValue::Integer(2))
            .margin_y_negative(SpacingValue::Integer(1))
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("-m-4"));
        assert!(css_classes.contains("-mx-2"));
        assert!(css_classes.contains("-my-1"));
    }

    #[test]
    fn test_gap_utilities() {
        let classes = ClassBuilder::new()
            .gap(SpacingValue::Integer(4))
            .gap_x(SpacingValue::Integer(6))
            .gap_y(SpacingValue::Integer(2))
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("gap-4"));
        assert!(css_classes.contains("gap-x-6"));
        assert!(css_classes.contains("gap-y-2"));
    }

    #[test]
    fn test_fractional_spacing() {
        let classes = ClassBuilder::new()
            .padding(SpacingValue::Fractional(0.5))
            .padding_x(SpacingValue::Fractional(1.5))
            .padding_y(SpacingValue::Fractional(2.5))
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("p-0.5"));
        assert!(css_classes.contains("px-1.5"));
        assert!(css_classes.contains("py-2.5"));
    }

    #[test]
    fn test_special_spacing_values() {
        let classes = ClassBuilder::new()
            .padding(SpacingValue::Auto)
            .margin(SpacingValue::Full)
            .gap(SpacingValue::Screen)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("p-auto"));
        assert!(css_classes.contains("m-full"));
        assert!(css_classes.contains("gap-screen"));
    }

    /// Test that all Tailwind CSS spacing values are supported
    #[test]
    fn test_all_tailwind_spacing_values() {
        // Tailwind CSS spacing scale: 0, px, 0.5, 1, 1.5, 2, 2.5, 3, 3.5, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64, 72, 80, 96
        let expected_values = vec![
            "0", "px", "0.5", "1", "1.5", "2", "2.5", "3", "3.5", "4", "5", "6", "7", "8", "9",
            "10", "11", "12", "14", "16", "20", "24", "28", "32", "36", "40", "44", "48", "52",
            "56", "60", "64", "72", "80", "96",
        ];

        let actual_values: Vec<String> = SpacingValue::all_values()
            .iter()
            .map(|v| v.to_class_name())
            .collect();

        for expected in expected_values {
            assert!(
                actual_values.contains(&expected.to_string()),
                "Missing spacing value: {}",
                expected
            );
        }
    }

    /// Test that space-between utilities are implemented
    #[test]
    fn test_space_between_utilities() {
        let classes = ClassBuilder::new()
            .space_x_4() // space-x-4
            .space_y_2() // space-y-2
            .space_x_reverse() // space-x-reverse
            .space_y_reverse() // space-y-reverse
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("space-x-4"));
        assert!(css_classes.contains("space-y-2"));
        assert!(css_classes.contains("space-x-reverse"));
        assert!(css_classes.contains("space-y-reverse"));
    }

    /// Test that divide utilities are implemented
    #[test]
    fn test_divide_utilities() {
        let classes = ClassBuilder::new()
            .divide_x_2() // divide-x-2
            .divide_y_4() // divide-y-4
            .divide_x_reverse() // divide-x-reverse
            .divide_y_reverse() // divide-y-reverse
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("divide-x-2"));
        assert!(css_classes.contains("divide-y-4"));
        assert!(css_classes.contains("divide-x-reverse"));
        assert!(css_classes.contains("divide-y-reverse"));
    }
}
