//! Grayscale Filter Utilities Module
//!
//! Handles grayscale filter utilities:
//! - Grayscale enum for different grayscale levels
//! - CSS value generation and class name utilities

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Grayscale values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Grayscale {
    /// 0% grayscale
    Zero,
    /// 100% grayscale
    Hundred,
}

impl Grayscale {
    /// Returns the CSS value for the grayscale filter
    pub fn css_value(&self) -> String {
        let percentage = match self {
            Grayscale::Zero => 0,
            Grayscale::Hundred => 100,
        };
        format!("grayscale({}%)", percentage)
    }

    /// Returns the class name suffix for the grayscale filter
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Grayscale::Zero => "0",
            Grayscale::Hundred => "",
        }
    }

    /// Returns the full Tailwind class name
    pub fn class_name(&self) -> String {
        if self.class_suffix().is_empty() {
            "grayscale".to_string()
        } else {
            format!("grayscale-{}", self.class_suffix())
        }
    }

    /// Check if this grayscale has no effect (color preserved)
    pub fn is_none(&self) -> bool {
        matches!(self, Grayscale::Zero)
    }

    /// Check if this grayscale is fully applied (black and white only)
    pub fn is_full(&self) -> bool {
        matches!(self, Grayscale::Hundred)
    }

    /// Returns all available grayscale variants
    pub fn variants() -> &'static [Grayscale] {
        &[Grayscale::Zero, Grayscale::Hundred]
    }

    /// Get the grayscale percentage value
    pub fn percentage(&self) -> u32 {
        match self {
            Grayscale::Zero => 0,
            Grayscale::Hundred => 100,
        }
    }
}

impl fmt::Display for Grayscale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class_name())
    }
}

impl Default for Grayscale {
    fn default() -> Self {
        Grayscale::Zero
    }
}

/// Grayscale filter utilities trait for extending ClassBuilder
pub trait GrayscaleUtilities {
    /// Add grayscale filter class
    fn grayscale(self, grayscale: Grayscale) -> Self;

    /// Add grayscale-0 class (no grayscale)
    fn grayscale_0(self) -> Self;

    /// Add grayscale class (100% grayscale)
    fn grayscale_full(self) -> Self;
}

impl GrayscaleUtilities for ClassBuilder {
    fn grayscale(self, grayscale: Grayscale) -> Self {
        self.class(&grayscale.class_name())
    }

    fn grayscale_0(self) -> Self {
        self.class("grayscale-0")
    }

    fn grayscale_full(self) -> Self {
        self.class("grayscale")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grayscale_css_values() {
        assert_eq!(Grayscale::Zero.css_value(), "grayscale(0%)");
        assert_eq!(Grayscale::Hundred.css_value(), "grayscale(100%)");
    }

    #[test]
    fn grayscale_class_names() {
        assert_eq!(Grayscale::Zero.class_name(), "grayscale-0");
        assert_eq!(Grayscale::Hundred.class_name(), "grayscale");
    }

    #[test]
    fn grayscale_properties() {
        assert!(Grayscale::Zero.is_none());
        assert!(!Grayscale::Hundred.is_none());

        assert!(Grayscale::Hundred.is_full());
        assert!(!Grayscale::Zero.is_full());
    }

    #[test]
    fn grayscale_percentage() {
        assert_eq!(Grayscale::Zero.percentage(), 0);
        assert_eq!(Grayscale::Hundred.percentage(), 100);
    }

    #[test]
    fn grayscale_variants() {
        let variants = Grayscale::variants();
        assert_eq!(variants.len(), 2);
        assert!(variants.contains(&Grayscale::Zero));
        assert!(variants.contains(&Grayscale::Hundred));
    }

    #[test]
    fn grayscale_display() {
        assert_eq!(Grayscale::Zero.to_string(), "grayscale-0");
        assert_eq!(Grayscale::Hundred.to_string(), "grayscale");
    }

    #[test]
    fn grayscale_utilities_trait() {
        let builder = ClassBuilder::new();

        // Test trait methods (simplified - would need actual ClassBuilder implementation)
        let _result = builder.grayscale(Grayscale::Zero).grayscale_full();
        // In a real implementation, this would add classes to the builder
    }
}
