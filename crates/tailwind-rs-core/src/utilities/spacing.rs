//! Spacing utilities for tailwind-rs
//!
//! This module provides utilities for padding, margin, and gap spacing.
//! It follows Tailwind CSS conventions and provides type-safe spacing values.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Spacing utility values for padding and margin
/// 
/// # Examples
/// 
/// ```rust
/// use tailwind_rs_core::utilities::spacing::{SpacingValue, PaddingUtilities};
/// use tailwind_rs_core::classes::ClassBuilder;
/// 
/// let classes = ClassBuilder::new()
///     .padding(SpacingValue::Integer(4))
///     .padding_x(SpacingValue::Integer(6))
///     .build();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SpacingValue {
    /// Zero spacing (0)
    Zero,
    /// 1px spacing
    Px,
    /// Fractional spacing (0.5, 1.5, 2.5, 3.5)
    Fractional(f32),
    /// Integer spacing (1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 40, 48, 56, 64, 72, 80, 96)
    Integer(u32),
    /// Auto spacing
    Auto,
    /// Full spacing (100%)
    Full,
    /// Screen spacing (100vh)
    Screen,
    /// Min content spacing
    Min,
    /// Max content spacing
    Max,
    /// Fit content spacing
    Fit,
}

impl std::hash::Hash for SpacingValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            SpacingValue::Zero => 0u8.hash(state),
            SpacingValue::Px => 1u8.hash(state),
            SpacingValue::Fractional(f) => {
                2u8.hash(state);
                // Convert f32 to u32 for hashing (multiply by 1000 to preserve 3 decimal places)
                ((f * 1000.0) as u32).hash(state);
            }
            SpacingValue::Integer(i) => {
                3u8.hash(state);
                i.hash(state);
            }
            SpacingValue::Auto => 4u8.hash(state),
            SpacingValue::Full => 5u8.hash(state),
            SpacingValue::Screen => 6u8.hash(state),
            SpacingValue::Min => 7u8.hash(state),
            SpacingValue::Max => 8u8.hash(state),
            SpacingValue::Fit => 9u8.hash(state),
        }
    }
}

impl std::cmp::Eq for SpacingValue {}

impl SpacingValue {
    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            SpacingValue::Zero => "0".to_string(),
            SpacingValue::Px => "1px".to_string(),
            SpacingValue::Fractional(f) => format!("{}rem", f * 0.25), // 0.25rem per unit
            SpacingValue::Integer(i) => format!("{}rem", *i as f32 * 0.25),
            SpacingValue::Auto => "auto".to_string(),
            SpacingValue::Full => "100%".to_string(),
            SpacingValue::Screen => "100vh".to_string(),
            SpacingValue::Min => "min-content".to_string(),
            SpacingValue::Max => "max-content".to_string(),
            SpacingValue::Fit => "fit-content".to_string(),
        }
    }
    
    /// Convert to class name
    pub fn to_class_name(&self) -> String {
        match self {
            SpacingValue::Zero => "0".to_string(),
            SpacingValue::Px => "px".to_string(),
            SpacingValue::Fractional(f) => format!("{}", f),
            SpacingValue::Integer(i) => i.to_string(),
            SpacingValue::Auto => "auto".to_string(),
            SpacingValue::Full => "full".to_string(),
            SpacingValue::Screen => "screen".to_string(),
            SpacingValue::Min => "min".to_string(),
            SpacingValue::Max => "max".to_string(),
            SpacingValue::Fit => "fit".to_string(),
        }
    }
    
    /// Get all valid spacing values
    pub fn all_values() -> Vec<SpacingValue> {
        vec![
            SpacingValue::Zero,
            SpacingValue::Px,
            SpacingValue::Fractional(0.5),
            SpacingValue::Integer(1),
            SpacingValue::Fractional(1.5),
            SpacingValue::Integer(2),
            SpacingValue::Fractional(2.5),
            SpacingValue::Integer(3),
            SpacingValue::Fractional(3.5),
            SpacingValue::Integer(4),
            SpacingValue::Integer(5),
            SpacingValue::Integer(6),
            SpacingValue::Integer(7),
            SpacingValue::Integer(8),
            SpacingValue::Integer(9),
            SpacingValue::Integer(10),
            SpacingValue::Integer(11),
            SpacingValue::Integer(12),
            SpacingValue::Integer(14),
            SpacingValue::Integer(16),
            SpacingValue::Integer(20),
            SpacingValue::Integer(24),
            SpacingValue::Integer(28),
            SpacingValue::Integer(32),
            SpacingValue::Integer(36),
            SpacingValue::Integer(40),
            SpacingValue::Integer(44),
            SpacingValue::Integer(48),
            SpacingValue::Integer(52),
            SpacingValue::Integer(56),
            SpacingValue::Integer(60),
            SpacingValue::Integer(64),
            SpacingValue::Integer(72),
            SpacingValue::Integer(80),
            SpacingValue::Integer(96),
            SpacingValue::Auto,
            SpacingValue::Full,
            SpacingValue::Screen,
            SpacingValue::Min,
            SpacingValue::Max,
            SpacingValue::Fit,
        ]
    }
}

impl fmt::Display for SpacingValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Trait for adding padding utilities to a class builder
pub trait PaddingUtilities {
    /// Add padding to all sides
    fn padding(self, value: SpacingValue) -> Self;
    
    /// Add horizontal padding (left and right)
    fn padding_x(self, value: SpacingValue) -> Self;
    
    /// Add vertical padding (top and bottom)
    fn padding_y(self, value: SpacingValue) -> Self;
    
    /// Add top padding
    fn padding_top(self, value: SpacingValue) -> Self;
    
    /// Add right padding
    fn padding_right(self, value: SpacingValue) -> Self;
    
    /// Add bottom padding
    fn padding_bottom(self, value: SpacingValue) -> Self;
    
    /// Add left padding
    fn padding_left(self, value: SpacingValue) -> Self;
    
    /// Add padding to start (left in LTR, right in RTL)
    fn padding_start(self, value: SpacingValue) -> Self;
    
    /// Add padding to end (right in LTR, left in RTL)
    fn padding_end(self, value: SpacingValue) -> Self;
}

impl PaddingUtilities for ClassBuilder {
    fn padding(self, value: SpacingValue) -> Self {
        self.class(format!("p-{}", value.to_class_name()))
    }
    
    fn padding_x(self, value: SpacingValue) -> Self {
        self.class(format!("px-{}", value.to_class_name()))
    }
    
    fn padding_y(self, value: SpacingValue) -> Self {
        self.class(format!("py-{}", value.to_class_name()))
    }
    
    fn padding_top(self, value: SpacingValue) -> Self {
        self.class(format!("pt-{}", value.to_class_name()))
    }
    
    fn padding_right(self, value: SpacingValue) -> Self {
        self.class(format!("pr-{}", value.to_class_name()))
    }
    
    fn padding_bottom(self, value: SpacingValue) -> Self {
        self.class(format!("pb-{}", value.to_class_name()))
    }
    
    fn padding_left(self, value: SpacingValue) -> Self {
        self.class(format!("pl-{}", value.to_class_name()))
    }
    
    fn padding_start(self, value: SpacingValue) -> Self {
        self.class(format!("ps-{}", value.to_class_name()))
    }
    
    fn padding_end(self, value: SpacingValue) -> Self {
        self.class(format!("pe-{}", value.to_class_name()))
    }
}

/// Trait for adding margin utilities to a class builder
pub trait MarginUtilities {
    /// Add margin to all sides
    fn margin(self, value: SpacingValue) -> Self;
    
    /// Add horizontal margin (left and right)
    fn margin_x(self, value: SpacingValue) -> Self;
    
    /// Add vertical margin (top and bottom)
    fn margin_y(self, value: SpacingValue) -> Self;
    
    /// Add top margin
    fn margin_top(self, value: SpacingValue) -> Self;
    
    /// Add right margin
    fn margin_right(self, value: SpacingValue) -> Self;
    
    /// Add bottom margin
    fn margin_bottom(self, value: SpacingValue) -> Self;
    
    /// Add left margin
    fn margin_left(self, value: SpacingValue) -> Self;
    
    /// Add margin to start (left in LTR, right in RTL)
    fn margin_start(self, value: SpacingValue) -> Self;
    
    /// Add margin to end (right in LTR, left in RTL)
    fn margin_end(self, value: SpacingValue) -> Self;
    
    /// Add negative margin to all sides
    fn margin_negative(self, value: SpacingValue) -> Self;
    
    /// Add negative horizontal margin
    fn margin_x_negative(self, value: SpacingValue) -> Self;
    
    /// Add negative vertical margin
    fn margin_y_negative(self, value: SpacingValue) -> Self;
    
    /// Add negative top margin
    fn margin_top_negative(self, value: SpacingValue) -> Self;
    
    /// Add negative right margin
    fn margin_right_negative(self, value: SpacingValue) -> Self;
    
    /// Add negative bottom margin
    fn margin_bottom_negative(self, value: SpacingValue) -> Self;
    
    /// Add negative left margin
    fn margin_left_negative(self, value: SpacingValue) -> Self;
}

impl MarginUtilities for ClassBuilder {
    fn margin(self, value: SpacingValue) -> Self {
        self.class(format!("m-{}", value.to_class_name()))
    }
    
    fn margin_x(self, value: SpacingValue) -> Self {
        self.class(format!("mx-{}", value.to_class_name()))
    }
    
    fn margin_y(self, value: SpacingValue) -> Self {
        self.class(format!("my-{}", value.to_class_name()))
    }
    
    fn margin_top(self, value: SpacingValue) -> Self {
        self.class(format!("mt-{}", value.to_class_name()))
    }
    
    fn margin_right(self, value: SpacingValue) -> Self {
        self.class(format!("mr-{}", value.to_class_name()))
    }
    
    fn margin_bottom(self, value: SpacingValue) -> Self {
        self.class(format!("mb-{}", value.to_class_name()))
    }
    
    fn margin_left(self, value: SpacingValue) -> Self {
        self.class(format!("ml-{}", value.to_class_name()))
    }
    
    fn margin_start(self, value: SpacingValue) -> Self {
        self.class(format!("ms-{}", value.to_class_name()))
    }
    
    fn margin_end(self, value: SpacingValue) -> Self {
        self.class(format!("me-{}", value.to_class_name()))
    }
    
    fn margin_negative(self, value: SpacingValue) -> Self {
        self.class(format!("-m-{}", value.to_class_name()))
    }
    
    fn margin_x_negative(self, value: SpacingValue) -> Self {
        self.class(format!("-mx-{}", value.to_class_name()))
    }
    
    fn margin_y_negative(self, value: SpacingValue) -> Self {
        self.class(format!("-my-{}", value.to_class_name()))
    }
    
    fn margin_top_negative(self, value: SpacingValue) -> Self {
        self.class(format!("-mt-{}", value.to_class_name()))
    }
    
    fn margin_right_negative(self, value: SpacingValue) -> Self {
        self.class(format!("-mr-{}", value.to_class_name()))
    }
    
    fn margin_bottom_negative(self, value: SpacingValue) -> Self {
        self.class(format!("-mb-{}", value.to_class_name()))
    }
    
    fn margin_left_negative(self, value: SpacingValue) -> Self {
        self.class(format!("-ml-{}", value.to_class_name()))
    }
}

/// Trait for adding gap utilities to a class builder
pub trait GapUtilities {
    /// Add gap between grid/flex items
    fn gap(self, value: SpacingValue) -> Self;
    
    /// Add horizontal gap between grid/flex items
    fn gap_x(self, value: SpacingValue) -> Self;
    
    /// Add vertical gap between grid/flex items
    fn gap_y(self, value: SpacingValue) -> Self;
}

impl GapUtilities for ClassBuilder {
    fn gap(self, value: SpacingValue) -> Self {
        self.class(format!("gap-{}", value.to_class_name()))
    }
    
    fn gap_x(self, value: SpacingValue) -> Self {
        self.class(format!("gap-x-{}", value.to_class_name()))
    }
    
    fn gap_y(self, value: SpacingValue) -> Self {
        self.class(format!("gap-y-{}", value.to_class_name()))
    }
}

/// Trait for adding space-between utilities to a class builder
pub trait SpaceBetweenUtilities {
    /// Add horizontal space between child elements
    fn space_x(self, value: SpacingValue) -> Self;
    
    /// Add vertical space between child elements
    fn space_y(self, value: SpacingValue) -> Self;
    
    /// Reverse horizontal space between child elements
    fn space_x_reverse(self) -> Self;
    
    /// Reverse vertical space between child elements
    fn space_y_reverse(self) -> Self;
}

impl SpaceBetweenUtilities for ClassBuilder {
    fn space_x(self, value: SpacingValue) -> Self {
        self.class(format!("space-x-{}", value.to_class_name()))
    }
    
    fn space_y(self, value: SpacingValue) -> Self {
        self.class(format!("space-y-{}", value.to_class_name()))
    }
    
    fn space_x_reverse(self) -> Self {
        self.class("space-x-reverse".to_string())
    }
    
    fn space_y_reverse(self) -> Self {
        self.class("space-y-reverse".to_string())
    }
}

/// Convenience methods for space-between utilities
impl ClassBuilder {
    /// Add horizontal space between child elements with value 2
    pub fn space_x_2(self) -> Self {
        self.space_x(SpacingValue::Integer(2))
    }
    
    /// Add horizontal space between child elements with value 4
    pub fn space_x_4(self) -> Self {
        self.space_x(SpacingValue::Integer(4))
    }
    
    /// Add vertical space between child elements with value 2
    pub fn space_y_2(self) -> Self {
        self.space_y(SpacingValue::Integer(2))
    }
    
    /// Add vertical space between child elements with value 4
    pub fn space_y_4(self) -> Self {
        self.space_y(SpacingValue::Integer(4))
    }
}

/// Trait for adding divide utilities to a class builder
pub trait SpacingDivideUtilities {
    /// Add horizontal divider between child elements
    fn divide_x(self, value: SpacingValue) -> Self;
    
    /// Add vertical divider between child elements
    fn divide_y(self, value: SpacingValue) -> Self;
    
    /// Reverse horizontal divider between child elements
    fn divide_x_reverse(self) -> Self;
    
    /// Reverse vertical divider between child elements
    fn divide_y_reverse(self) -> Self;
}

impl SpacingDivideUtilities for ClassBuilder {
    fn divide_x(self, value: SpacingValue) -> Self {
        self.class(format!("divide-x-{}", value.to_class_name()))
    }
    
    fn divide_y(self, value: SpacingValue) -> Self {
        self.class(format!("divide-y-{}", value.to_class_name()))
    }
    
    fn divide_x_reverse(self) -> Self {
        self.class("divide-x-reverse".to_string())
    }
    
    fn divide_y_reverse(self) -> Self {
        self.class("divide-y-reverse".to_string())
    }
}

/// Convenience methods for divide utilities
impl ClassBuilder {
    /// Add horizontal divider between child elements with value 2
    pub fn divide_x_2(self) -> Self {
        self.divide_x(SpacingValue::Integer(2))
    }
    
    /// Add horizontal divider between child elements with value 4
    pub fn divide_x_4(self) -> Self {
        self.divide_x(SpacingValue::Integer(4))
    }
    
    /// Add vertical divider between child elements with value 2
    pub fn divide_y_2(self) -> Self {
        self.divide_y(SpacingValue::Integer(2))
    }
    
    /// Add vertical divider between child elements with value 4
    pub fn divide_y_4(self) -> Self {
        self.divide_y(SpacingValue::Integer(4))
    }
}

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
            "0", "px", "0.5", "1", "1.5", "2", "2.5", "3", "3.5", "4", "5", "6", "7", "8", "9", "10", 
            "11", "12", "14", "16", "20", "24", "28", "32", "36", "40", "44", "48", "52", "56", "60", 
            "64", "72", "80", "96"
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
            .space_x_4()  // space-x-4
            .space_y_2()  // space-y-2
            .space_x_reverse()  // space-x-reverse
            .space_y_reverse()  // space-y-reverse
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
            .divide_x_2()  // divide-x-2
            .divide_y_4()  // divide-y-4
            .divide_x_reverse()  // divide-x-reverse
            .divide_y_reverse()  // divide-y-reverse
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("divide-x-2"));
        assert!(css_classes.contains("divide-y-4"));
        assert!(css_classes.contains("divide-x-reverse"));
        assert!(css_classes.contains("divide-y-reverse"));
    }
}
