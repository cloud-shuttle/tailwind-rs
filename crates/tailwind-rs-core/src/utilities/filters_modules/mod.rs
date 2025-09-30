//! Filter Utilities Module
//!
//! Comprehensive filter utilities system for Tailwind-RS:
//! - Blur filters (blur, blur-sm, blur-md, etc.)
//! - Brightness filters (brightness-0, brightness-50, etc.)
//! - Contrast filters (contrast-0, contrast-50, etc.)
//! - Grayscale filters (grayscale, grayscale-0)
//! - Hue rotate filters (hue-rotate-0, hue-rotate-15, etc.)
//! - Invert filters (invert, invert-0)
//! - Saturate filters (saturate-0, saturate-50, etc.)
//! - Sepia filters (sepia, sepia-0)
//! - Drop shadow filters (drop-shadow-sm, drop-shadow-md, etc.)

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

// Re-export all filter utility types and traits
pub mod blur;
pub mod brightness;
pub mod contrast;
pub mod grayscale;

// Re-export all types for easy access
pub use blur::{Blur, BlurUtilities};
pub use brightness::{Brightness, BrightnessUtilities};
pub use contrast::{Contrast, ContrastUtilities};
pub use grayscale::{Grayscale, GrayscaleUtilities};

/// Hue rotate values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HueRotate {
    /// 0 degrees
    Zero,
    /// 15 degrees
    Fifteen,
    /// 30 degrees
    Thirty,
    /// 60 degrees
    Sixty,
    /// 90 degrees
    Ninety,
    /// 180 degrees
    OneEighty,
}

impl HueRotate {
    /// Returns the CSS value for the hue-rotate filter
    pub fn css_value(&self) -> String {
        let degrees = match self {
            HueRotate::Zero => 0,
            HueRotate::Fifteen => 15,
            HueRotate::Thirty => 30,
            HueRotate::Sixty => 60,
            HueRotate::Ninety => 90,
            HueRotate::OneEighty => 180,
        };
        format!("hue-rotate({}deg)", degrees)
    }

    /// Returns the class name suffix for the hue-rotate filter
    pub fn class_suffix(&self) -> &'static str {
        match self {
            HueRotate::Zero => "0",
            HueRotate::Fifteen => "15",
            HueRotate::Thirty => "30",
            HueRotate::Sixty => "60",
            HueRotate::Ninety => "90",
            HueRotate::OneEighty => "180",
        }
    }

    /// Returns the full Tailwind class name
    pub fn class_name(&self) -> String {
        format!("hue-rotate-{}", self.class_suffix())
    }

    /// Returns all available hue rotate variants
    pub fn variants() -> &'static [HueRotate] {
        &[
            HueRotate::Zero,
            HueRotate::Fifteen,
            HueRotate::Thirty,
            HueRotate::Sixty,
            HueRotate::Ninety,
            HueRotate::OneEighty,
        ]
    }

    /// Get the hue rotate degrees value
    pub fn degrees(&self) -> u32 {
        match self {
            HueRotate::Zero => 0,
            HueRotate::Fifteen => 15,
            HueRotate::Thirty => 30,
            HueRotate::Sixty => 60,
            HueRotate::Ninety => 90,
            HueRotate::OneEighty => 180,
        }
    }
}

impl fmt::Display for HueRotate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class_name())
    }
}

impl Default for HueRotate {
    fn default() -> Self {
        HueRotate::Zero
    }
}

/// Invert values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Invert {
    /// 0% invert
    Zero,
    /// 100% invert
    Hundred,
}

impl Invert {
    /// Returns the CSS value for the invert filter
    pub fn css_value(&self) -> String {
        let percentage = match self {
            Invert::Zero => 0,
            Invert::Hundred => 100,
        };
        format!("invert({}%)", percentage)
    }

    /// Returns the class name suffix for the invert filter
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Invert::Zero => "0",
            Invert::Hundred => "",
        }
    }

    /// Returns the full Tailwind class name
    pub fn class_name(&self) -> String {
        if self.class_suffix().is_empty() {
            "invert".to_string()
        } else {
            format!("invert-{}", self.class_suffix())
        }
    }

    /// Returns all available invert variants
    pub fn variants() -> &'static [Invert] {
        &[Invert::Zero, Invert::Hundred]
    }

    /// Get the invert percentage value
    pub fn percentage(&self) -> u32 {
        match self {
            Invert::Zero => 0,
            Invert::Hundred => 100,
        }
    }
}

impl fmt::Display for Invert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class_name())
    }
}

impl Default for Invert {
    fn default() -> Self {
        Invert::Zero
    }
}

/// Saturate values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Saturate {
    /// 0% saturate
    Zero,
    /// 50% saturate
    Fifty,
    /// 100% saturate
    Hundred,
    /// 150% saturate
    HundredFifty,
    /// 200% saturate
    TwoHundred,
}

impl Saturate {
    /// Returns the CSS value for the saturate filter
    pub fn css_value(&self) -> String {
        let percentage = match self {
            Saturate::Zero => 0,
            Saturate::Fifty => 50,
            Saturate::Hundred => 100,
            Saturate::HundredFifty => 150,
            Saturate::TwoHundred => 200,
        };
        format!("saturate({}%)", percentage)
    }

    /// Returns the class name suffix for the saturate filter
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Saturate::Zero => "0",
            Saturate::Fifty => "50",
            Saturate::Hundred => "100",
            Saturate::HundredFifty => "150",
            Saturate::TwoHundred => "200",
        }
    }

    /// Returns the full Tailwind class name
    pub fn class_name(&self) -> String {
        format!("saturate-{}", self.class_suffix())
    }

    /// Returns all available saturate variants
    pub fn variants() -> &'static [Saturate] {
        &[
            Saturate::Zero,
            Saturate::Fifty,
            Saturate::Hundred,
            Saturate::HundredFifty,
            Saturate::TwoHundred,
        ]
    }

    /// Get the saturate percentage value
    pub fn percentage(&self) -> u32 {
        match self {
            Saturate::Zero => 0,
            Saturate::Fifty => 50,
            Saturate::Hundred => 100,
            Saturate::HundredFifty => 150,
            Saturate::TwoHundred => 200,
        }
    }
}

impl fmt::Display for Saturate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class_name())
    }
}

impl Default for Saturate {
    fn default() -> Self {
        Saturate::Hundred
    }
}

/// Sepia values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Sepia {
    /// 0% sepia
    Zero,
    /// 100% sepia
    Hundred,
}

impl Sepia {
    /// Returns the CSS value for the sepia filter
    pub fn css_value(&self) -> String {
        let percentage = match self {
            Sepia::Zero => 0,
            Sepia::Hundred => 100,
        };
        format!("sepia({}%)", percentage)
    }

    /// Returns the class name suffix for the sepia filter
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Sepia::Zero => "0",
            Sepia::Hundred => "",
        }
    }

    /// Returns the full Tailwind class name
    pub fn class_name(&self) -> String {
        if self.class_suffix().is_empty() {
            "sepia".to_string()
        } else {
            format!("sepia-{}", self.class_suffix())
        }
    }

    /// Returns all available sepia variants
    pub fn variants() -> &'static [Sepia] {
        &[Sepia::Zero, Sepia::Hundred]
    }

    /// Get the sepia percentage value
    pub fn percentage(&self) -> u32 {
        match self {
            Sepia::Zero => 0,
            Sepia::Hundred => 100,
        }
    }
}

impl fmt::Display for Sepia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class_name())
    }
}

impl Default for Sepia {
    fn default() -> Self {
        Sepia::Zero
    }
}

/// Drop shadow values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DropShadow {
    /// Small drop shadow
    Sm,
    /// Default drop shadow
    Default,
    /// Medium drop shadow
    Md,
    /// Large drop shadow
    Lg,
    /// Extra large drop shadow
    Xl,
    /// 2x large drop shadow
    Xl2,
    /// No drop shadow
    None,
}

impl DropShadow {
    /// Returns the CSS value for the drop-shadow filter
    pub fn css_value(&self) -> &'static str {
        match self {
            DropShadow::Sm => "drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))",
            DropShadow::Default => "drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06))",
            DropShadow::Md => "drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06))",
            DropShadow::Lg => "drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1))",
            DropShadow::Xl => "drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08))",
            DropShadow::Xl2 => "drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))",
            DropShadow::None => "drop-shadow(0 0 #0000)",
        }
    }

    /// Returns the class name suffix for the drop-shadow filter
    pub fn class_suffix(&self) -> &'static str {
        match self {
            DropShadow::Sm => "sm",
            DropShadow::Default => "",
            DropShadow::Md => "md",
            DropShadow::Lg => "lg",
            DropShadow::Xl => "xl",
            DropShadow::Xl2 => "2xl",
            DropShadow::None => "none",
        }
    }

    /// Returns the full Tailwind class name
    pub fn class_name(&self) -> String {
        if self.class_suffix().is_empty() {
            "drop-shadow".to_string()
        } else {
            format!("drop-shadow-{}", self.class_suffix())
        }
    }

    /// Check if this drop shadow has no effect
    pub fn is_none(&self) -> bool {
        matches!(self, DropShadow::None)
    }

    /// Check if this is a large drop shadow
    pub fn is_large(&self) -> bool {
        matches!(self, DropShadow::Xl | DropShadow::Xl2)
    }

    /// Returns all available drop shadow variants
    pub fn variants() -> &'static [DropShadow] {
        &[
            DropShadow::Sm,
            DropShadow::Default,
            DropShadow::Md,
            DropShadow::Lg,
            DropShadow::Xl,
            DropShadow::Xl2,
            DropShadow::None,
        ]
    }
}

impl fmt::Display for DropShadow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class_name())
    }
}

impl Default for DropShadow {
    fn default() -> Self {
        DropShadow::None
    }
}

/// Comprehensive filter utilities trait for extending ClassBuilder
pub trait FilterUtilities: BlurUtilities + BrightnessUtilities + ContrastUtilities + GrayscaleUtilities {
    /// Add hue-rotate filter class
    fn hue_rotate(self, hue_rotate: HueRotate) -> Self;

    /// Add invert filter class
    fn invert(self, invert: Invert) -> Self;

    /// Add saturate filter class
    fn saturate(self, saturate: Saturate) -> Self;

    /// Add sepia filter class
    fn sepia(self, sepia: Sepia) -> Self;

    /// Add drop-shadow filter class
    fn drop_shadow(self, drop_shadow: DropShadow) -> Self;

    // Convenience methods for common filters
    fn hue_rotate_0(self) -> Self where Self: Sized { self.hue_rotate(HueRotate::Zero) }
    fn hue_rotate_15(self) -> Self where Self: Sized { self.hue_rotate(HueRotate::Fifteen) }
    fn hue_rotate_30(self) -> Self where Self: Sized { self.hue_rotate(HueRotate::Thirty) }
    fn hue_rotate_60(self) -> Self where Self: Sized { self.hue_rotate(HueRotate::Sixty) }
    fn hue_rotate_90(self) -> Self where Self: Sized { self.hue_rotate(HueRotate::Ninety) }
    fn hue_rotate_180(self) -> Self where Self: Sized { self.hue_rotate(HueRotate::OneEighty) }

    fn invert_full(self) -> Self where Self: Sized { self.invert(Invert::Hundred) }

    fn saturate_0(self) -> Self where Self: Sized { self.saturate(Saturate::Zero) }
    fn saturate_50(self) -> Self where Self: Sized { self.saturate(Saturate::Fifty) }
    fn saturate_100(self) -> Self where Self: Sized { self.saturate(Saturate::Hundred) }
    fn saturate_150(self) -> Self where Self: Sized { self.saturate(Saturate::HundredFifty) }
    fn saturate_200(self) -> Self where Self: Sized { self.saturate(Saturate::TwoHundred) }

    fn sepia_full(self) -> Self where Self: Sized { self.sepia(Sepia::Hundred) }

    fn drop_shadow_sm(self) -> Self where Self: Sized { self.drop_shadow(DropShadow::Sm) }
    fn drop_shadow_default(self) -> Self where Self: Sized { self.drop_shadow(DropShadow::Default) }
    fn drop_shadow_md(self) -> Self where Self: Sized { self.drop_shadow(DropShadow::Md) }
    fn drop_shadow_lg(self) -> Self where Self: Sized { self.drop_shadow(DropShadow::Lg) }
    fn drop_shadow_xl(self) -> Self where Self: Sized { self.drop_shadow(DropShadow::Xl) }
    fn drop_shadow_2xl(self) -> Self where Self: Sized { self.drop_shadow(DropShadow::Xl2) }
    fn drop_shadow_none(self) -> Self where Self: Sized { self.drop_shadow(DropShadow::None) }
}

impl FilterUtilities for ClassBuilder {
    fn hue_rotate(self, hue_rotate: HueRotate) -> Self {
        self.class(&hue_rotate.class_name())
    }

    fn invert(self, invert: Invert) -> Self {
        self.class(&invert.class_name())
    }

    fn saturate(self, saturate: Saturate) -> Self {
        self.class(&saturate.class_name())
    }

    fn sepia(self, sepia: Sepia) -> Self {
        self.class(&sepia.class_name())
    }

    fn drop_shadow(self, drop_shadow: DropShadow) -> Self {
        self.class(&drop_shadow.class_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hue_rotate_functionality() {
        let rotate = HueRotate::Thirty;
        assert_eq!(rotate.css_value(), "hue-rotate(30deg)");
        assert_eq!(rotate.class_name(), "hue-rotate-30");
        assert_eq!(rotate.degrees(), 30);
    }

    #[test]
    fn invert_functionality() {
        let invert = Invert::Hundred;
        assert_eq!(invert.css_value(), "invert(100%)");
        assert_eq!(invert.class_name(), "invert");
        assert_eq!(invert.percentage(), 100);
    }

    #[test]
    fn saturate_functionality() {
        let saturate = Saturate::HundredFifty;
        assert_eq!(saturate.css_value(), "saturate(150%)");
        assert_eq!(saturate.class_name(), "saturate-150");
        assert_eq!(saturate.percentage(), 150);
    }

    #[test]
    fn sepia_functionality() {
        let sepia = Sepia::Hundred;
        assert_eq!(sepia.css_value(), "sepia(100%)");
        assert_eq!(sepia.class_name(), "sepia");
        assert_eq!(sepia.percentage(), 100);
    }

    #[test]
    fn drop_shadow_functionality() {
        let shadow = DropShadow::Md;
        assert!(shadow.css_value().contains("drop-shadow"));
        assert_eq!(shadow.class_name(), "drop-shadow-md");
        assert!(!shadow.is_none());
        assert!(!shadow.is_large());
    }

    #[test]
    fn filter_utilities_trait() {
        let builder = ClassBuilder::new();

        // Test trait methods (simplified - would need actual ClassBuilder implementation)
        let _result = builder
            .blur(Blur::Sm)
            .brightness(Brightness::Hundred)
            .contrast(Contrast::Hundred)
            .grayscale(Grayscale::Zero)
            .hue_rotate(HueRotate::Thirty)
            .invert(Invert::Hundred)
            .saturate(Saturate::HundredFifty)
            .sepia(Sepia::Hundred)
            .drop_shadow(DropShadow::Md);
        // In a real implementation, this would add classes to the builder
    }
}
