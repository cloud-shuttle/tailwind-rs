//! Sizing value utilities for tailwind-rs

use crate::utilities::sizing::{Fraction, GridFraction};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Sizing value enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SizingValue {
    /// Auto sizing
    Auto,
    /// Full sizing (100%)
    Full,
    /// Screen sizing (100vh/100vw)
    Screen,
    /// Min content sizing
    Min,
    /// Max content sizing
    Max,
    /// Fit content sizing
    Fit,
    /// Fraction sizing
    Fraction(Fraction),
    /// Grid fraction sizing
    GridFraction(GridFraction),
    /// Integer sizing (in rem units)
    Integer(u32),
    /// Pixel sizing
    Px(u32),
}

impl fmt::Display for SizingValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SizingValue::Auto => write!(f, "auto"),
            SizingValue::Full => write!(f, "full"),
            SizingValue::Screen => write!(f, "screen"),
            SizingValue::Min => write!(f, "min"),
            SizingValue::Max => write!(f, "max"),
            SizingValue::Fit => write!(f, "fit"),
            SizingValue::Fraction(fraction) => write!(f, "{}", fraction),
            SizingValue::GridFraction(grid_fraction) => write!(f, "{}", grid_fraction),
            SizingValue::Integer(value) => write!(f, "{}", value),
            SizingValue::Px(value) => write!(f, "{}px", value),
        }
    }
}

impl SizingValue {
    pub fn to_class_name(&self) -> String {
        match self {
            SizingValue::Auto => "auto".to_string(),
            SizingValue::Full => "full".to_string(),
            SizingValue::Screen => "screen".to_string(),
            SizingValue::Min => "min".to_string(),
            SizingValue::Max => "max".to_string(),
            SizingValue::Fit => "fit".to_string(),
            SizingValue::Fraction(fraction) => fraction.to_class_name(),
            SizingValue::GridFraction(grid_fraction) => grid_fraction.to_class_name(),
            SizingValue::Integer(value) => value.to_string(),
            SizingValue::Px(value) => format!("{}px", value),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            SizingValue::Auto => "auto".to_string(),
            SizingValue::Full => "100%".to_string(),
            SizingValue::Screen => "100vh".to_string(),
            SizingValue::Min => "min-content".to_string(),
            SizingValue::Max => "max-content".to_string(),
            SizingValue::Fit => "fit-content".to_string(),
            SizingValue::Fraction(fraction) => fraction.to_css_value(),
            SizingValue::GridFraction(grid_fraction) => grid_fraction.to_css_value(),
            SizingValue::Integer(value) => format!("{}rem", value * 4 / 16), // Convert to rem
            SizingValue::Px(value) => format!("{}px", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sizing_value_display() {
        assert_eq!(SizingValue::Auto.to_string(), "auto");
        assert_eq!(SizingValue::Full.to_string(), "full");
        assert_eq!(SizingValue::Integer(4).to_string(), "4");
    }

    #[test]
    fn test_sizing_value_class_name() {
        assert_eq!(SizingValue::Auto.to_class_name(), "auto");
        assert_eq!(SizingValue::Full.to_class_name(), "full");
        assert_eq!(SizingValue::Integer(4).to_class_name(), "4");
    }

    #[test]
    fn test_sizing_value_css_value() {
        assert_eq!(SizingValue::Auto.to_css_value(), "auto");
        assert_eq!(SizingValue::Full.to_css_value(), "100%");
        assert_eq!(SizingValue::Integer(4).to_css_value(), "1rem");
    }
}
