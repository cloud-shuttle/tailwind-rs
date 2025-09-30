//! Brightness Filter Utilities Module
//!
//! Handles brightness filter utilities:
//! - Brightness enum for different brightness levels
//! - CSS value generation and class name utilities

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Brightness values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Brightness {
    /// 0% brightness
    Zero,
    /// 50% brightness
    Fifty,
    /// 75% brightness
    SeventyFive,
    /// 90% brightness
    Ninety,
    /// 95% brightness
    NinetyFive,
    /// 100% brightness
    Hundred,
    /// 105% brightness
    HundredFive,
    /// 110% brightness
    HundredTen,
    /// 125% brightness
    HundredTwentyFive,
    /// 150% brightness
    HundredFifty,
    /// 200% brightness
    TwoHundred,
}

impl Brightness {
    /// Returns the CSS value for the brightness filter
    pub fn css_value(&self) -> String {
        let percentage = match self {
            Brightness::Zero => 0,
            Brightness::Fifty => 50,
            Brightness::SeventyFive => 75,
            Brightness::Ninety => 90,
            Brightness::NinetyFive => 95,
            Brightness::Hundred => 100,
            Brightness::HundredFive => 105,
            Brightness::HundredTen => 110,
            Brightness::HundredTwentyFive => 125,
            Brightness::HundredFifty => 150,
            Brightness::TwoHundred => 200,
        };
        format!("brightness({}%)", percentage)
    }

    /// Returns the class name suffix for the brightness filter
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Brightness::Zero => "0",
            Brightness::Fifty => "50",
            Brightness::SeventyFive => "75",
            Brightness::Ninety => "90",
            Brightness::NinetyFive => "95",
            Brightness::Hundred => "100",
            Brightness::HundredFive => "105",
            Brightness::HundredTen => "110",
            Brightness::HundredTwentyFive => "125",
            Brightness::HundredFifty => "150",
            Brightness::TwoHundred => "200",
        }
    }

    /// Returns the full Tailwind class name
    pub fn class_name(&self) -> String {
        format!("brightness-{}", self.class_suffix())
    }

    /// Check if this brightness is at minimum (darkest)
    pub fn is_minimum(&self) -> bool {
        matches!(self, Brightness::Zero)
    }

    /// Check if this brightness is at maximum (brightest)
    pub fn is_maximum(&self) -> bool {
        matches!(self, Brightness::TwoHundred)
    }

    /// Check if this brightness is above normal (>100%)
    pub fn is_bright(&self) -> bool {
        matches!(self, Brightness::HundredFive | Brightness::HundredTen | Brightness::HundredTwentyFive | Brightness::HundredFifty | Brightness::TwoHundred)
    }

    /// Check if this brightness is below normal (<100%)
    pub fn is_dim(&self) -> bool {
        matches!(self, Brightness::Zero | Brightness::Fifty | Brightness::SeventyFive | Brightness::Ninety | Brightness::NinetyFive)
    }

    /// Returns all available brightness variants
    pub fn variants() -> &'static [Brightness] {
        &[
            Brightness::Zero,
            Brightness::Fifty,
            Brightness::SeventyFive,
            Brightness::Ninety,
            Brightness::NinetyFive,
            Brightness::Hundred,
            Brightness::HundredFive,
            Brightness::HundredTen,
            Brightness::HundredTwentyFive,
            Brightness::HundredFifty,
            Brightness::TwoHundred,
        ]
    }

    /// Get the brightness percentage value
    pub fn percentage(&self) -> u32 {
        match self {
            Brightness::Zero => 0,
            Brightness::Fifty => 50,
            Brightness::SeventyFive => 75,
            Brightness::Ninety => 90,
            Brightness::NinetyFive => 95,
            Brightness::Hundred => 100,
            Brightness::HundredFive => 105,
            Brightness::HundredTen => 110,
            Brightness::HundredTwentyFive => 125,
            Brightness::HundredFifty => 150,
            Brightness::TwoHundred => 200,
        }
    }
}

impl fmt::Display for Brightness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class_name())
    }
}

impl Default for Brightness {
    fn default() -> Self {
        Brightness::Hundred
    }
}

/// Brightness filter utilities trait for extending ClassBuilder
pub trait BrightnessUtilities {
    /// Add brightness filter class
    fn brightness(self, brightness: Brightness) -> Self;

    /// Add brightness-0 class
    fn brightness_0(self) -> Self;

    /// Add brightness-50 class
    fn brightness_50(self) -> Self;

    /// Add brightness-75 class
    fn brightness_75(self) -> Self;

    /// Add brightness-90 class
    fn brightness_90(self) -> Self;

    /// Add brightness-95 class
    fn brightness_95(self) -> Self;

    /// Add brightness-100 class
    fn brightness_100(self) -> Self;

    /// Add brightness-105 class
    fn brightness_105(self) -> Self;

    /// Add brightness-110 class
    fn brightness_110(self) -> Self;

    /// Add brightness-125 class
    fn brightness_125(self) -> Self;

    /// Add brightness-150 class
    fn brightness_150(self) -> Self;

    /// Add brightness-200 class
    fn brightness_200(self) -> Self;
}

impl BrightnessUtilities for ClassBuilder {
    fn brightness(self, brightness: Brightness) -> Self {
        self.class(&brightness.class_name())
    }

    fn brightness_0(self) -> Self {
        self.class("brightness-0")
    }

    fn brightness_50(self) -> Self {
        self.class("brightness-50")
    }

    fn brightness_75(self) -> Self {
        self.class("brightness-75")
    }

    fn brightness_90(self) -> Self {
        self.class("brightness-90")
    }

    fn brightness_95(self) -> Self {
        self.class("brightness-95")
    }

    fn brightness_100(self) -> Self {
        self.class("brightness-100")
    }

    fn brightness_105(self) -> Self {
        self.class("brightness-105")
    }

    fn brightness_110(self) -> Self {
        self.class("brightness-110")
    }

    fn brightness_125(self) -> Self {
        self.class("brightness-125")
    }

    fn brightness_150(self) -> Self {
        self.class("brightness-150")
    }

    fn brightness_200(self) -> Self {
        self.class("brightness-200")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brightness_css_values() {
        assert_eq!(Brightness::Zero.css_value(), "brightness(0%)");
        assert_eq!(Brightness::Fifty.css_value(), "brightness(50%)");
        assert_eq!(Brightness::Hundred.css_value(), "brightness(100%)");
        assert_eq!(Brightness::TwoHundred.css_value(), "brightness(200%)");
    }

    #[test]
    fn brightness_class_names() {
        assert_eq!(Brightness::Zero.class_name(), "brightness-0");
        assert_eq!(Brightness::Hundred.class_name(), "brightness-100");
        assert_eq!(Brightness::TwoHundred.class_name(), "brightness-200");
    }

    #[test]
    fn brightness_properties() {
        assert!(Brightness::Zero.is_minimum());
        assert!(Brightness::TwoHundred.is_maximum());

        assert!(Brightness::HundredFive.is_bright());
        assert!(Brightness::TwoHundred.is_bright());
        assert!(!Brightness::Hundred.is_bright());

        assert!(Brightness::Fifty.is_dim());
        assert!(Brightness::Zero.is_dim());
        assert!(!Brightness::Hundred.is_dim());
    }

    #[test]
    fn brightness_percentage() {
        assert_eq!(Brightness::Zero.percentage(), 0);
        assert_eq!(Brightness::Fifty.percentage(), 50);
        assert_eq!(Brightness::Hundred.percentage(), 100);
        assert_eq!(Brightness::TwoHundred.percentage(), 200);
    }

    #[test]
    fn brightness_variants() {
        let variants = Brightness::variants();
        assert_eq!(variants.len(), 11);
        assert!(variants.contains(&Brightness::Zero));
        assert!(variants.contains(&Brightness::TwoHundred));
    }

    #[test]
    fn brightness_display() {
        assert_eq!(Brightness::Hundred.to_string(), "brightness-100");
        assert_eq!(Brightness::Fifty.to_string(), "brightness-50");
    }

    #[test]
    fn brightness_utilities_trait() {
        let builder = ClassBuilder::new();

        // Test trait methods (simplified - would need actual ClassBuilder implementation)
        let _result = builder.brightness(Brightness::Fifty).brightness_100().brightness_200();
        // In a real implementation, this would add classes to the builder
    }
}
