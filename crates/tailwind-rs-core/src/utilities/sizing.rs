//! Sizing utilities for tailwind-rs
//!
//! This module provides utilities for width, height, min-width, max-width,
//! min-height, and max-height. It follows Tailwind CSS conventions and
//! provides type-safe sizing values.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Sizing utility values for width and height
/// 
/// # Examples
/// 
/// ```rust
/// use tailwind_rs_core::utilities::sizing::SizingValue;
/// use tailwind_rs_core::classes::ClassBuilder;
/// 
/// let classes = ClassBuilder::new()
///     .width(SizingValue::Full)
///     .height(SizingValue::Screen)
///     .build();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SizingValue {
    /// Zero sizing (0)
    Zero,
    /// 1px sizing
    Px,
    /// Fractional sizing (0.5, 1.5, 2.5, 3.5)
    Fractional(f32),
    /// Integer sizing (1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 40, 48, 56, 64, 72, 80, 96)
    Integer(u32),
    /// Auto sizing
    Auto,
    /// Full sizing (100%)
    Full,
    /// Screen sizing (100vh for height, 100vw for width)
    Screen,
    /// Min content sizing
    Min,
    /// Max content sizing
    Max,
    /// Fit content sizing
    Fit,
    /// Fractional sizing (1/2, 1/3, 2/3, 1/4, 2/4, 3/4, 1/5, 2/5, 3/5, 4/5, 1/6, 2/6, 3/6, 4/6, 5/6)
    Fraction(Fraction),
    /// 12-column grid fractions (1/12, 2/12, 3/12, 4/12, 5/12, 6/12, 7/12, 8/12, 9/12, 10/12, 11/12)
    GridFraction(GridFraction),
}

/// Fractional sizing values
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Fraction {
    /// 1/2
    Half,
    /// 1/3
    Third,
    /// 2/3
    TwoThirds,
    /// 1/4
    Quarter,
    /// 2/4
    TwoQuarters,
    /// 3/4
    ThreeQuarters,
    /// 1/5
    Fifth,
    /// 2/5
    TwoFifths,
    /// 3/5
    ThreeFifths,
    /// 4/5
    FourFifths,
    /// 1/6
    Sixth,
    /// 2/6
    TwoSixths,
    /// 3/6
    ThreeSixths,
    /// 4/6
    FourSixths,
    /// 5/6
    FiveSixths,
}

/// 12-column grid fractions
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GridFraction {
    /// 1/12
    OneTwelfth,
    /// 2/12
    TwoTwelfths,
    /// 3/12
    ThreeTwelfths,
    /// 4/12
    FourTwelfths,
    /// 5/12
    FiveTwelfths,
    /// 6/12
    SixTwelfths,
    /// 7/12
    SevenTwelfths,
    /// 8/12
    EightTwelfths,
    /// 9/12
    NineTwelfths,
    /// 10/12
    TenTwelfths,
    /// 11/12
    ElevenTwelfths,
}

impl std::hash::Hash for SizingValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            SizingValue::Zero => 0u8.hash(state),
            SizingValue::Px => 1u8.hash(state),
            SizingValue::Fractional(f) => {
                2u8.hash(state);
                ((f * 1000.0) as u32).hash(state);
            }
            SizingValue::Integer(i) => {
                3u8.hash(state);
                i.hash(state);
            }
            SizingValue::Auto => 4u8.hash(state),
            SizingValue::Full => 5u8.hash(state),
            SizingValue::Screen => 6u8.hash(state),
            SizingValue::Min => 7u8.hash(state),
            SizingValue::Max => 8u8.hash(state),
            SizingValue::Fit => 9u8.hash(state),
            SizingValue::Fraction(f) => {
                10u8.hash(state);
                std::mem::discriminant(f).hash(state);
            }
            SizingValue::GridFraction(gf) => {
                11u8.hash(state);
                std::mem::discriminant(gf).hash(state);
            }
        }
    }
}

impl std::hash::Hash for Fraction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

impl std::hash::Hash for GridFraction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

impl std::cmp::Eq for SizingValue {}
impl std::cmp::Eq for Fraction {}
impl std::cmp::Eq for GridFraction {}

impl SizingValue {
    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            SizingValue::Zero => "0".to_string(),
            SizingValue::Px => "1px".to_string(),
            SizingValue::Fractional(f) => format!("{}rem", f * 0.25),
            SizingValue::Integer(i) => format!("{}rem", *i as f32 * 0.25),
            SizingValue::Auto => "auto".to_string(),
            SizingValue::Full => "100%".to_string(),
            SizingValue::Screen => "100vh".to_string(), // For height, will be overridden for width
            SizingValue::Min => "min-content".to_string(),
            SizingValue::Max => "max-content".to_string(),
            SizingValue::Fit => "fit-content".to_string(),
            SizingValue::Fraction(f) => f.to_css_value(),
            SizingValue::GridFraction(gf) => gf.to_css_value(),
        }
    }
    
    /// Convert to CSS value for width (screen becomes 100vw)
    pub fn to_css_value_width(&self) -> String {
        match self {
            SizingValue::Screen => "100vw".to_string(),
            _ => self.to_css_value(),
        }
    }
    
    /// Convert to CSS value for height (screen becomes 100vh)
    pub fn to_css_value_height(&self) -> String {
        match self {
            SizingValue::Screen => "100vh".to_string(),
            _ => self.to_css_value(),
        }
    }
    
    /// Convert to class name
    pub fn to_class_name(&self) -> String {
        match self {
            SizingValue::Zero => "0".to_string(),
            SizingValue::Px => "px".to_string(),
            SizingValue::Fractional(f) => format!("{}", f),
            SizingValue::Integer(i) => i.to_string(),
            SizingValue::Auto => "auto".to_string(),
            SizingValue::Full => "full".to_string(),
            SizingValue::Screen => "screen".to_string(),
            SizingValue::Min => "min".to_string(),
            SizingValue::Max => "max".to_string(),
            SizingValue::Fit => "fit".to_string(),
            SizingValue::Fraction(f) => f.to_class_name(),
            SizingValue::GridFraction(gf) => gf.to_class_name(),
        }
    }
    
    /// Get all valid sizing values
    pub fn all_values() -> Vec<SizingValue> {
        let mut values = vec![
            SizingValue::Zero,
            SizingValue::Px,
            SizingValue::Fractional(0.5),
            SizingValue::Fractional(1.5),
            SizingValue::Fractional(2.5),
            SizingValue::Fractional(3.5),
            SizingValue::Integer(1),
            SizingValue::Integer(2),
            SizingValue::Integer(3),
            SizingValue::Integer(4),
            SizingValue::Integer(5),
            SizingValue::Integer(6),
            SizingValue::Integer(8),
            SizingValue::Integer(10),
            SizingValue::Integer(12),
            SizingValue::Integer(16),
            SizingValue::Integer(20),
            SizingValue::Integer(24),
            SizingValue::Integer(32),
            SizingValue::Integer(40),
            SizingValue::Integer(48),
            SizingValue::Integer(56),
            SizingValue::Integer(64),
            SizingValue::Integer(72),
            SizingValue::Integer(80),
            SizingValue::Integer(96),
            SizingValue::Auto,
            SizingValue::Full,
            SizingValue::Screen,
            SizingValue::Min,
            SizingValue::Max,
            SizingValue::Fit,
        ];
        
        // Add fractions
        values.extend([
            SizingValue::Fraction(Fraction::Half),
            SizingValue::Fraction(Fraction::Third),
            SizingValue::Fraction(Fraction::TwoThirds),
            SizingValue::Fraction(Fraction::Quarter),
            SizingValue::Fraction(Fraction::TwoQuarters),
            SizingValue::Fraction(Fraction::ThreeQuarters),
            SizingValue::Fraction(Fraction::Fifth),
            SizingValue::Fraction(Fraction::TwoFifths),
            SizingValue::Fraction(Fraction::ThreeFifths),
            SizingValue::Fraction(Fraction::FourFifths),
            SizingValue::Fraction(Fraction::Sixth),
            SizingValue::Fraction(Fraction::TwoSixths),
            SizingValue::Fraction(Fraction::ThreeSixths),
            SizingValue::Fraction(Fraction::FourSixths),
            SizingValue::Fraction(Fraction::FiveSixths),
        ]);
        
        // Add grid fractions
        values.extend([
            SizingValue::GridFraction(GridFraction::OneTwelfth),
            SizingValue::GridFraction(GridFraction::TwoTwelfths),
            SizingValue::GridFraction(GridFraction::ThreeTwelfths),
            SizingValue::GridFraction(GridFraction::FourTwelfths),
            SizingValue::GridFraction(GridFraction::FiveTwelfths),
            SizingValue::GridFraction(GridFraction::SixTwelfths),
            SizingValue::GridFraction(GridFraction::SevenTwelfths),
            SizingValue::GridFraction(GridFraction::EightTwelfths),
            SizingValue::GridFraction(GridFraction::NineTwelfths),
            SizingValue::GridFraction(GridFraction::TenTwelfths),
            SizingValue::GridFraction(GridFraction::ElevenTwelfths),
        ]);
        
        values
    }
}

impl Fraction {
    pub fn to_css_value(&self) -> String {
        match self {
            Fraction::Half => "50%".to_string(),
            Fraction::Third => "33.333333%".to_string(),
            Fraction::TwoThirds => "66.666667%".to_string(),
            Fraction::Quarter => "25%".to_string(),
            Fraction::TwoQuarters => "50%".to_string(),
            Fraction::ThreeQuarters => "75%".to_string(),
            Fraction::Fifth => "20%".to_string(),
            Fraction::TwoFifths => "40%".to_string(),
            Fraction::ThreeFifths => "60%".to_string(),
            Fraction::FourFifths => "80%".to_string(),
            Fraction::Sixth => "16.666667%".to_string(),
            Fraction::TwoSixths => "33.333333%".to_string(),
            Fraction::ThreeSixths => "50%".to_string(),
            Fraction::FourSixths => "66.666667%".to_string(),
            Fraction::FiveSixths => "83.333333%".to_string(),
        }
    }
    
    pub fn to_class_name(&self) -> String {
        match self {
            Fraction::Half => "1/2".to_string(),
            Fraction::Third => "1/3".to_string(),
            Fraction::TwoThirds => "2/3".to_string(),
            Fraction::Quarter => "1/4".to_string(),
            Fraction::TwoQuarters => "2/4".to_string(),
            Fraction::ThreeQuarters => "3/4".to_string(),
            Fraction::Fifth => "1/5".to_string(),
            Fraction::TwoFifths => "2/5".to_string(),
            Fraction::ThreeFifths => "3/5".to_string(),
            Fraction::FourFifths => "4/5".to_string(),
            Fraction::Sixth => "1/6".to_string(),
            Fraction::TwoSixths => "2/6".to_string(),
            Fraction::ThreeSixths => "3/6".to_string(),
            Fraction::FourSixths => "4/6".to_string(),
            Fraction::FiveSixths => "5/6".to_string(),
        }
    }
}

impl GridFraction {
    pub fn to_css_value(&self) -> String {
        match self {
            GridFraction::OneTwelfth => "8.333333%".to_string(),
            GridFraction::TwoTwelfths => "16.666667%".to_string(),
            GridFraction::ThreeTwelfths => "25%".to_string(),
            GridFraction::FourTwelfths => "33.333333%".to_string(),
            GridFraction::FiveTwelfths => "41.666667%".to_string(),
            GridFraction::SixTwelfths => "50%".to_string(),
            GridFraction::SevenTwelfths => "58.333333%".to_string(),
            GridFraction::EightTwelfths => "66.666667%".to_string(),
            GridFraction::NineTwelfths => "75%".to_string(),
            GridFraction::TenTwelfths => "83.333333%".to_string(),
            GridFraction::ElevenTwelfths => "91.666667%".to_string(),
        }
    }
    
    pub fn to_class_name(&self) -> String {
        match self {
            GridFraction::OneTwelfth => "1/12".to_string(),
            GridFraction::TwoTwelfths => "2/12".to_string(),
            GridFraction::ThreeTwelfths => "3/12".to_string(),
            GridFraction::FourTwelfths => "4/12".to_string(),
            GridFraction::FiveTwelfths => "5/12".to_string(),
            GridFraction::SixTwelfths => "6/12".to_string(),
            GridFraction::SevenTwelfths => "7/12".to_string(),
            GridFraction::EightTwelfths => "8/12".to_string(),
            GridFraction::NineTwelfths => "9/12".to_string(),
            GridFraction::TenTwelfths => "10/12".to_string(),
            GridFraction::ElevenTwelfths => "11/12".to_string(),
        }
    }
}

impl fmt::Display for SizingValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Trait for adding width utilities to a class builder
pub trait WidthUtilities {
    /// Set width
    fn width(self, value: SizingValue) -> Self;
    
    /// Set min-width
    fn min_width(self, value: SizingValue) -> Self;
    
    /// Set max-width
    fn max_width(self, value: SizingValue) -> Self;
}

impl WidthUtilities for ClassBuilder {
    fn width(self, value: SizingValue) -> Self {
        self.class(format!("w-{}", value.to_class_name()))
    }
    
    fn min_width(self, value: SizingValue) -> Self {
        self.class(format!("min-w-{}", value.to_class_name()))
    }
    
    fn max_width(self, value: SizingValue) -> Self {
        self.class(format!("max-w-{}", value.to_class_name()))
    }
}

/// Trait for adding height utilities to a class builder
pub trait HeightUtilities {
    /// Set height
    fn height(self, value: SizingValue) -> Self;
    
    /// Set min-height
    fn min_height(self, value: SizingValue) -> Self;
    
    /// Set max-height
    fn max_height(self, value: SizingValue) -> Self;
}

impl HeightUtilities for ClassBuilder {
    fn height(self, value: SizingValue) -> Self {
        self.class(format!("h-{}", value.to_class_name()))
    }
    
    fn min_height(self, value: SizingValue) -> Self {
        self.class(format!("min-h-{}", value.to_class_name()))
    }
    
    fn max_height(self, value: SizingValue) -> Self {
        self.class(format!("max-h-{}", value.to_class_name()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sizing_value_to_css_value() {
        assert_eq!(SizingValue::Zero.to_css_value(), "0");
        assert_eq!(SizingValue::Px.to_css_value(), "1px");
        assert_eq!(SizingValue::Fractional(0.5).to_css_value(), "0.125rem");
        assert_eq!(SizingValue::Integer(4).to_css_value(), "1rem");
        assert_eq!(SizingValue::Auto.to_css_value(), "auto");
        assert_eq!(SizingValue::Full.to_css_value(), "100%");
        assert_eq!(SizingValue::Screen.to_css_value(), "100vh");
    }
    
    #[test]
    fn test_sizing_value_to_css_value_width() {
        assert_eq!(SizingValue::Screen.to_css_value_width(), "100vw");
        assert_eq!(SizingValue::Full.to_css_value_width(), "100%");
    }
    
    #[test]
    fn test_sizing_value_to_css_value_height() {
        assert_eq!(SizingValue::Screen.to_css_value_height(), "100vh");
        assert_eq!(SizingValue::Full.to_css_value_height(), "100%");
    }
    
    #[test]
    fn test_fraction_to_css_value() {
        assert_eq!(Fraction::Half.to_css_value(), "50%");
        assert_eq!(Fraction::Third.to_css_value(), "33.333333%");
        assert_eq!(Fraction::TwoThirds.to_css_value(), "66.666667%");
    }
    
    #[test]
    fn test_fraction_to_class_name() {
        assert_eq!(Fraction::Half.to_class_name(), "1/2");
        assert_eq!(Fraction::Third.to_class_name(), "1/3");
        assert_eq!(Fraction::TwoThirds.to_class_name(), "2/3");
    }
    
    #[test]
    fn test_grid_fraction_to_css_value() {
        assert_eq!(GridFraction::OneTwelfth.to_css_value(), "8.333333%");
        assert_eq!(GridFraction::SixTwelfths.to_css_value(), "50%");
        assert_eq!(GridFraction::ElevenTwelfths.to_css_value(), "91.666667%");
    }
    
    #[test]
    fn test_grid_fraction_to_class_name() {
        assert_eq!(GridFraction::OneTwelfth.to_class_name(), "1/12");
        assert_eq!(GridFraction::SixTwelfths.to_class_name(), "6/12");
        assert_eq!(GridFraction::ElevenTwelfths.to_class_name(), "11/12");
    }
    
    #[test]
    fn test_width_utilities() {
        let classes = ClassBuilder::new()
            .width(SizingValue::Full)
            .min_width(SizingValue::Integer(4))
            .max_width(SizingValue::Integer(8))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("w-full"));
        assert!(css_classes.contains("min-w-4"));
        assert!(css_classes.contains("max-w-8"));
    }
    
    #[test]
    fn test_height_utilities() {
        let classes = ClassBuilder::new()
            .height(SizingValue::Screen)
            .min_height(SizingValue::Integer(4))
            .max_height(SizingValue::Integer(8))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("h-screen"));
        assert!(css_classes.contains("min-h-4"));
        assert!(css_classes.contains("max-h-8"));
    }
    
    #[test]
    fn test_fractional_sizing() {
        let classes = ClassBuilder::new()
            .width(SizingValue::Fraction(Fraction::Half))
            .height(SizingValue::Fraction(Fraction::Third))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("w-1/2"));
        assert!(css_classes.contains("h-1/3"));
    }
    
    #[test]
    fn test_grid_fractional_sizing() {
        let classes = ClassBuilder::new()
            .width(SizingValue::GridFraction(GridFraction::SixTwelfths))
            .height(SizingValue::GridFraction(GridFraction::FourTwelfths))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("w-6/12"));
        assert!(css_classes.contains("h-4/12"));
    }
    
    #[test]
    fn test_special_sizing_values() {
        let classes = ClassBuilder::new()
            .width(SizingValue::Auto)
            .height(SizingValue::Fit)
            .min_width(SizingValue::Min)
            .max_width(SizingValue::Max)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("w-auto"));
        assert!(css_classes.contains("h-fit"));
        assert!(css_classes.contains("min-w-min"));
        assert!(css_classes.contains("max-w-max"));
    }
}
