//! Contrast Filter Utilities Module
//!
//! Handles contrast filter utilities:
//! - Contrast enum for different contrast levels
//! - CSS value generation and class name utilities

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Contrast values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Contrast {
    /// 0% contrast
    Zero,
    /// 50% contrast
    Fifty,
    /// 75% contrast
    SeventyFive,
    /// 100% contrast
    Hundred,
    /// 125% contrast
    HundredTwentyFive,
    /// 150% contrast
    HundredFifty,
    /// 200% contrast
    TwoHundred,
}

impl Contrast {
    /// Returns the CSS value for the contrast filter
    pub fn css_value(&self) -> String {
        let percentage = match self {
            Contrast::Zero => 0,
            Contrast::Fifty => 50,
            Contrast::SeventyFive => 75,
            Contrast::Hundred => 100,
            Contrast::HundredTwentyFive => 125,
            Contrast::HundredFifty => 150,
            Contrast::TwoHundred => 200,
        };
        format!("contrast({}%)", percentage)
    }

    /// Returns the class name suffix for the contrast filter
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Contrast::Zero => "0",
            Contrast::Fifty => "50",
            Contrast::SeventyFive => "75",
            Contrast::Hundred => "100",
            Contrast::HundredTwentyFive => "125",
            Contrast::HundredFifty => "150",
            Contrast::TwoHundred => "200",
        }
    }

    /// Returns the full Tailwind class name
    pub fn class_name(&self) -> String {
        format!("contrast-{}", self.class_suffix())
    }

    /// Check if this contrast is at minimum (no contrast)
    pub fn is_minimum(&self) -> bool {
        matches!(self, Contrast::Zero)
    }

    /// Check if this contrast is at maximum (high contrast)
    pub fn is_maximum(&self) -> bool {
        matches!(self, Contrast::TwoHundred)
    }

    /// Check if this contrast is above normal (>100%)
    pub fn is_high(&self) -> bool {
        matches!(self, Contrast::HundredTwentyFive | Contrast::HundredFifty | Contrast::TwoHundred)
    }

    /// Check if this contrast is below normal (<100%)
    pub fn is_low(&self) -> bool {
        matches!(self, Contrast::Zero | Contrast::Fifty | Contrast::SeventyFive)
    }

    /// Returns all available contrast variants
    pub fn variants() -> &'static [Contrast] {
        &[
            Contrast::Zero,
            Contrast::Fifty,
            Contrast::SeventyFive,
            Contrast::Hundred,
            Contrast::HundredTwentyFive,
            Contrast::HundredFifty,
            Contrast::TwoHundred,
        ]
    }

    /// Get the contrast percentage value
    pub fn percentage(&self) -> u32 {
        match self {
            Contrast::Zero => 0,
            Contrast::Fifty => 50,
            Contrast::SeventyFive => 75,
            Contrast::Hundred => 100,
            Contrast::HundredTwentyFive => 125,
            Contrast::HundredFifty => 150,
            Contrast::TwoHundred => 200,
        }
    }
}

impl fmt::Display for Contrast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class_name())
    }
}

impl Default for Contrast {
    fn default() -> Self {
        Contrast::Hundred
    }
}

/// Contrast filter utilities trait for extending ClassBuilder
pub trait ContrastUtilities {
    /// Add contrast filter class
    fn contrast(self, contrast: Contrast) -> Self;

    /// Add contrast-0 class
    fn contrast_0(self) -> Self;

    /// Add contrast-50 class
    fn contrast_50(self) -> Self;

    /// Add contrast-75 class
    fn contrast_75(self) -> Self;

    /// Add contrast-100 class
    fn contrast_100(self) -> Self;

    /// Add contrast-125 class
    fn contrast_125(self) -> Self;

    /// Add contrast-150 class
    fn contrast_150(self) -> Self;

    /// Add contrast-200 class
    fn contrast_200(self) -> Self;
}

impl ContrastUtilities for ClassBuilder {
    fn contrast(self, contrast: Contrast) -> Self {
        self.class(&contrast.class_name())
    }

    fn contrast_0(self) -> Self {
        self.class("contrast-0")
    }

    fn contrast_50(self) -> Self {
        self.class("contrast-50")
    }

    fn contrast_75(self) -> Self {
        self.class("contrast-75")
    }

    fn contrast_100(self) -> Self {
        self.class("contrast-100")
    }

    fn contrast_125(self) -> Self {
        self.class("contrast-125")
    }

    fn contrast_150(self) -> Self {
        self.class("contrast-150")
    }

    fn contrast_200(self) -> Self {
        self.class("contrast-200")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contrast_css_values() {
        assert_eq!(Contrast::Zero.css_value(), "contrast(0%)");
        assert_eq!(Contrast::Fifty.css_value(), "contrast(50%)");
        assert_eq!(Contrast::Hundred.css_value(), "contrast(100%)");
        assert_eq!(Contrast::TwoHundred.css_value(), "contrast(200%)");
    }

    #[test]
    fn contrast_class_names() {
        assert_eq!(Contrast::Zero.class_name(), "contrast-0");
        assert_eq!(Contrast::Hundred.class_name(), "contrast-100");
        assert_eq!(Contrast::TwoHundred.class_name(), "contrast-200");
    }

    #[test]
    fn contrast_properties() {
        assert!(Contrast::Zero.is_minimum());
        assert!(Contrast::TwoHundred.is_maximum());

        assert!(Contrast::HundredTwentyFive.is_high());
        assert!(Contrast::TwoHundred.is_high());
        assert!(!Contrast::Hundred.is_high());

        assert!(Contrast::Fifty.is_low());
        assert!(Contrast::Zero.is_low());
        assert!(!Contrast::Hundred.is_low());
    }

    #[test]
    fn contrast_percentage() {
        assert_eq!(Contrast::Zero.percentage(), 0);
        assert_eq!(Contrast::Fifty.percentage(), 50);
        assert_eq!(Contrast::Hundred.percentage(), 100);
        assert_eq!(Contrast::TwoHundred.percentage(), 200);
    }

    #[test]
    fn contrast_variants() {
        let variants = Contrast::variants();
        assert_eq!(variants.len(), 7);
        assert!(variants.contains(&Contrast::Zero));
        assert!(variants.contains(&Contrast::TwoHundred));
    }

    #[test]
    fn contrast_display() {
        assert_eq!(Contrast::Hundred.to_string(), "contrast-100");
        assert_eq!(Contrast::Fifty.to_string(), "contrast-50");
    }

    #[test]
    fn contrast_utilities_trait() {
        let builder = ClassBuilder::new();

        // Test trait methods (simplified - would need actual ClassBuilder implementation)
        let _result = builder.contrast(Contrast::Fifty).contrast_100().contrast_200();
        // In a real implementation, this would add classes to the builder
    }
}
