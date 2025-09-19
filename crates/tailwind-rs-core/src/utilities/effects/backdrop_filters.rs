//! Backdrop filter utilities for tailwind-rs
//!
//! This module provides utilities for backdrop filter effects.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Backdrop filter blur values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackdropBlur {
    /// No backdrop blur
    None,
    /// backdrop-blur-sm (4px)
    Sm,
    /// backdrop-blur (8px)
    Default,
    /// backdrop-blur-md (12px)
    Md,
    /// backdrop-blur-lg (16px)
    Lg,
    /// backdrop-blur-xl (24px)
    Xl,
    /// backdrop-blur-2xl (40px)
    Xl2,
    /// backdrop-blur-3xl (64px)
    Xl3,
}

/// Backdrop filter brightness values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackdropBrightness {
    /// backdrop-brightness-0 (0)
    Zero,
    /// backdrop-brightness-50 (0.5)
    Fifty,
    /// backdrop-brightness-75 (0.75)
    SeventyFive,
    /// backdrop-brightness-90 (0.9)
    Ninety,
    /// backdrop-brightness-95 (0.95)
    NinetyFive,
    /// backdrop-brightness-100 (1)
    OneHundred,
    /// backdrop-brightness-105 (1.05)
    OneOhFive,
    /// backdrop-brightness-110 (1.1)
    OneOneZero,
    /// backdrop-brightness-125 (1.25)
    OneTwoFive,
    /// backdrop-brightness-150 (1.5)
    OneFifty,
    /// backdrop-brightness-200 (2)
    TwoHundred,
}

/// Backdrop filter contrast values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackdropContrast {
    /// backdrop-contrast-0 (0)
    Zero,
    /// backdrop-contrast-50 (0.5)
    Fifty,
    /// backdrop-contrast-75 (0.75)
    SeventyFive,
    /// backdrop-contrast-100 (1)
    OneHundred,
    /// backdrop-contrast-125 (1.25)
    OneTwoFive,
    /// backdrop-contrast-150 (1.5)
    OneFifty,
    /// backdrop-contrast-200 (2)
    TwoHundred,
}

/// Backdrop filter grayscale values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackdropGrayscale {
    /// backdrop-grayscale-0 (0)
    Zero,
    /// backdrop-grayscale (1)
    Default,
}

/// Backdrop filter hue rotate values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackdropHueRotate {
    /// backdrop-hue-rotate-0 (0deg)
    Zero,
    /// backdrop-hue-rotate-15 (15deg)
    Fifteen,
    /// backdrop-hue-rotate-30 (30deg)
    Thirty,
    /// backdrop-hue-rotate-60 (60deg)
    Sixty,
    /// backdrop-hue-rotate-90 (90deg)
    Ninety,
    /// backdrop-hue-rotate-180 (180deg)
    OneEighty,
}

/// Backdrop filter invert values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackdropInvert {
    /// backdrop-invert-0 (0)
    Zero,
    /// backdrop-invert (1)
    Default,
}

/// Backdrop filter opacity values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackdropOpacity {
    /// backdrop-opacity-0 (0)
    Zero,
    /// backdrop-opacity-5 (0.05)
    Five,
    /// backdrop-opacity-10 (0.1)
    Ten,
    /// backdrop-opacity-20 (0.2)
    Twenty,
    /// backdrop-opacity-25 (0.25)
    TwentyFive,
    /// backdrop-opacity-30 (0.3)
    Thirty,
    /// backdrop-opacity-40 (0.4)
    Forty,
    /// backdrop-opacity-50 (0.5)
    Fifty,
    /// backdrop-opacity-60 (0.6)
    Sixty,
    /// backdrop-opacity-70 (0.7)
    Seventy,
    /// backdrop-opacity-75 (0.75)
    SeventyFive,
    /// backdrop-opacity-80 (0.8)
    Eighty,
    /// backdrop-opacity-90 (0.9)
    Ninety,
    /// backdrop-opacity-95 (0.95)
    NinetyFive,
    /// backdrop-opacity-100 (1)
    Hundred,
}

/// Backdrop filter saturate values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackdropSaturate {
    /// backdrop-saturate-0 (0)
    Zero,
    /// backdrop-saturate-50 (0.5)
    Fifty,
    /// backdrop-saturate-100 (1)
    OneHundred,
    /// backdrop-saturate-150 (1.5)
    OneFifty,
    /// backdrop-saturate-200 (2)
    TwoHundred,
}

/// Backdrop filter sepia values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackdropSepia {
    /// backdrop-sepia-0 (0)
    Zero,
    /// backdrop-sepia (1)
    Default,
}

// Display implementations
impl fmt::Display for BackdropBlur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackdropBlur::None => write!(f, "backdrop-blur-none"),
            BackdropBlur::Sm => write!(f, "backdrop-blur-sm"),
            BackdropBlur::Default => write!(f, "backdrop-blur"),
            BackdropBlur::Md => write!(f, "backdrop-blur-md"),
            BackdropBlur::Lg => write!(f, "backdrop-blur-lg"),
            BackdropBlur::Xl => write!(f, "backdrop-blur-xl"),
            BackdropBlur::Xl2 => write!(f, "backdrop-blur-2xl"),
            BackdropBlur::Xl3 => write!(f, "backdrop-blur-3xl"),
        }
    }
}

impl fmt::Display for BackdropBrightness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackdropBrightness::Zero => write!(f, "backdrop-brightness-0"),
            BackdropBrightness::Fifty => write!(f, "backdrop-brightness-50"),
            BackdropBrightness::SeventyFive => write!(f, "backdrop-brightness-75"),
            BackdropBrightness::Ninety => write!(f, "backdrop-brightness-90"),
            BackdropBrightness::NinetyFive => write!(f, "backdrop-brightness-95"),
            BackdropBrightness::OneHundred => write!(f, "backdrop-brightness-100"),
            BackdropBrightness::OneOhFive => write!(f, "backdrop-brightness-105"),
            BackdropBrightness::OneOneZero => write!(f, "backdrop-brightness-110"),
            BackdropBrightness::OneTwoFive => write!(f, "backdrop-brightness-125"),
            BackdropBrightness::OneFifty => write!(f, "backdrop-brightness-150"),
            BackdropBrightness::TwoHundred => write!(f, "backdrop-brightness-200"),
        }
    }
}

impl fmt::Display for BackdropContrast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackdropContrast::Zero => write!(f, "backdrop-contrast-0"),
            BackdropContrast::Fifty => write!(f, "backdrop-contrast-50"),
            BackdropContrast::SeventyFive => write!(f, "backdrop-contrast-75"),
            BackdropContrast::OneHundred => write!(f, "backdrop-contrast-100"),
            BackdropContrast::OneTwoFive => write!(f, "backdrop-contrast-125"),
            BackdropContrast::OneFifty => write!(f, "backdrop-contrast-150"),
            BackdropContrast::TwoHundred => write!(f, "backdrop-contrast-200"),
        }
    }
}

impl fmt::Display for BackdropGrayscale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackdropGrayscale::Zero => write!(f, "backdrop-grayscale-0"),
            BackdropGrayscale::Default => write!(f, "backdrop-grayscale"),
        }
    }
}

impl fmt::Display for BackdropHueRotate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackdropHueRotate::Zero => write!(f, "backdrop-hue-rotate-0"),
            BackdropHueRotate::Fifteen => write!(f, "backdrop-hue-rotate-15"),
            BackdropHueRotate::Thirty => write!(f, "backdrop-hue-rotate-30"),
            BackdropHueRotate::Sixty => write!(f, "backdrop-hue-rotate-60"),
            BackdropHueRotate::Ninety => write!(f, "backdrop-hue-rotate-90"),
            BackdropHueRotate::OneEighty => write!(f, "backdrop-hue-rotate-180"),
        }
    }
}

impl fmt::Display for BackdropInvert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackdropInvert::Zero => write!(f, "backdrop-invert-0"),
            BackdropInvert::Default => write!(f, "backdrop-invert"),
        }
    }
}

impl fmt::Display for BackdropOpacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackdropOpacity::Zero => write!(f, "backdrop-opacity-0"),
            BackdropOpacity::Five => write!(f, "backdrop-opacity-5"),
            BackdropOpacity::Ten => write!(f, "backdrop-opacity-10"),
            BackdropOpacity::Twenty => write!(f, "backdrop-opacity-20"),
            BackdropOpacity::TwentyFive => write!(f, "backdrop-opacity-25"),
            BackdropOpacity::Thirty => write!(f, "backdrop-opacity-30"),
            BackdropOpacity::Forty => write!(f, "backdrop-opacity-40"),
            BackdropOpacity::Fifty => write!(f, "backdrop-opacity-50"),
            BackdropOpacity::Sixty => write!(f, "backdrop-opacity-60"),
            BackdropOpacity::Seventy => write!(f, "backdrop-opacity-70"),
            BackdropOpacity::SeventyFive => write!(f, "backdrop-opacity-75"),
            BackdropOpacity::Eighty => write!(f, "backdrop-opacity-80"),
            BackdropOpacity::Ninety => write!(f, "backdrop-opacity-90"),
            BackdropOpacity::NinetyFive => write!(f, "backdrop-opacity-95"),
            BackdropOpacity::Hundred => write!(f, "backdrop-opacity-100"),
        }
    }
}

impl fmt::Display for BackdropSaturate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackdropSaturate::Zero => write!(f, "backdrop-saturate-0"),
            BackdropSaturate::Fifty => write!(f, "backdrop-saturate-50"),
            BackdropSaturate::OneHundred => write!(f, "backdrop-saturate-100"),
            BackdropSaturate::OneFifty => write!(f, "backdrop-saturate-150"),
            BackdropSaturate::TwoHundred => write!(f, "backdrop-saturate-200"),
        }
    }
}

impl fmt::Display for BackdropSepia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackdropSepia::Zero => write!(f, "backdrop-sepia-0"),
            BackdropSepia::Default => write!(f, "backdrop-sepia"),
        }
    }
}

/// Trait for adding backdrop filter utilities to a class builder
pub trait BackdropFilterUtilities {
    fn backdrop_blur(self, blur: BackdropBlur) -> Self;
    fn backdrop_brightness(self, brightness: BackdropBrightness) -> Self;
    fn backdrop_contrast(self, contrast: BackdropContrast) -> Self;
    fn backdrop_grayscale(self, grayscale: BackdropGrayscale) -> Self;
    fn backdrop_hue_rotate(self, hue_rotate: BackdropHueRotate) -> Self;
    fn backdrop_invert(self, invert: BackdropInvert) -> Self;
    fn backdrop_opacity(self, opacity: BackdropOpacity) -> Self;
    fn backdrop_saturate(self, saturate: BackdropSaturate) -> Self;
    fn backdrop_sepia(self, sepia: BackdropSepia) -> Self;
}

impl BackdropFilterUtilities for ClassBuilder {
    fn backdrop_blur(self, blur: BackdropBlur) -> Self {
        self.class(blur.to_string())
    }
    
    fn backdrop_brightness(self, brightness: BackdropBrightness) -> Self {
        self.class(brightness.to_string())
    }
    
    fn backdrop_contrast(self, contrast: BackdropContrast) -> Self {
        self.class(contrast.to_string())
    }
    
    fn backdrop_grayscale(self, grayscale: BackdropGrayscale) -> Self {
        self.class(grayscale.to_string())
    }
    
    fn backdrop_hue_rotate(self, hue_rotate: BackdropHueRotate) -> Self {
        self.class(hue_rotate.to_string())
    }
    
    fn backdrop_invert(self, invert: BackdropInvert) -> Self {
        self.class(invert.to_string())
    }
    
    fn backdrop_opacity(self, opacity: BackdropOpacity) -> Self {
        self.class(opacity.to_string())
    }
    
    fn backdrop_saturate(self, saturate: BackdropSaturate) -> Self {
        self.class(saturate.to_string())
    }
    
    fn backdrop_sepia(self, sepia: BackdropSepia) -> Self {
        self.class(sepia.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backdrop_blur_display() {
        assert_eq!(BackdropBlur::None.to_string(), "backdrop-blur-none");
        assert_eq!(BackdropBlur::Sm.to_string(), "backdrop-blur-sm");
        assert_eq!(BackdropBlur::Default.to_string(), "backdrop-blur");
        assert_eq!(BackdropBlur::Md.to_string(), "backdrop-blur-md");
        assert_eq!(BackdropBlur::Lg.to_string(), "backdrop-blur-lg");
        assert_eq!(BackdropBlur::Xl.to_string(), "backdrop-blur-xl");
        assert_eq!(BackdropBlur::Xl2.to_string(), "backdrop-blur-2xl");
        assert_eq!(BackdropBlur::Xl3.to_string(), "backdrop-blur-3xl");
    }

    #[test]
    fn test_backdrop_brightness_display() {
        assert_eq!(BackdropBrightness::Zero.to_string(), "backdrop-brightness-0");
        assert_eq!(BackdropBrightness::Fifty.to_string(), "backdrop-brightness-50");
        assert_eq!(BackdropBrightness::SeventyFive.to_string(), "backdrop-brightness-75");
        assert_eq!(BackdropBrightness::Ninety.to_string(), "backdrop-brightness-90");
        assert_eq!(BackdropBrightness::NinetyFive.to_string(), "backdrop-brightness-95");
        assert_eq!(BackdropBrightness::OneHundred.to_string(), "backdrop-brightness-100");
        assert_eq!(BackdropBrightness::OneOhFive.to_string(), "backdrop-brightness-105");
        assert_eq!(BackdropBrightness::OneOneZero.to_string(), "backdrop-brightness-110");
        assert_eq!(BackdropBrightness::OneTwoFive.to_string(), "backdrop-brightness-125");
        assert_eq!(BackdropBrightness::OneFifty.to_string(), "backdrop-brightness-150");
        assert_eq!(BackdropBrightness::TwoHundred.to_string(), "backdrop-brightness-200");
    }

    #[test]
    fn test_backdrop_filter_utilities() {
        let classes = ClassBuilder::new()
            .backdrop_blur(BackdropBlur::Lg)
            .backdrop_brightness(BackdropBrightness::OneHundred)
            .backdrop_contrast(BackdropContrast::OneHundred)
            .backdrop_grayscale(BackdropGrayscale::Zero)
            .backdrop_hue_rotate(BackdropHueRotate::Zero)
            .backdrop_invert(BackdropInvert::Zero)
            .backdrop_opacity(BackdropOpacity::Fifty)
            .backdrop_saturate(BackdropSaturate::OneHundred)
            .backdrop_sepia(BackdropSepia::Zero)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("backdrop-blur-lg"));
        assert!(css_classes.contains("backdrop-brightness-100"));
        assert!(css_classes.contains("backdrop-contrast-100"));
        assert!(css_classes.contains("backdrop-grayscale-0"));
        assert!(css_classes.contains("backdrop-hue-rotate-0"));
        assert!(css_classes.contains("backdrop-invert-0"));
        assert!(css_classes.contains("backdrop-opacity-50"));
        assert!(css_classes.contains("backdrop-saturate-100"));
        assert!(css_classes.contains("backdrop-sepia-0"));
    }

    #[test]
    fn test_backdrop_filter_serialization() {
        let backdrop_blur = BackdropBlur::Md;
        let serialized = serde_json::to_string(&backdrop_blur).unwrap();
        let deserialized: BackdropBlur = serde_json::from_str(&serialized).unwrap();
        assert_eq!(backdrop_blur, deserialized);

        let backdrop_brightness = BackdropBrightness::OneHundred;
        let serialized = serde_json::to_string(&backdrop_brightness).unwrap();
        let deserialized: BackdropBrightness = serde_json::from_str(&serialized).unwrap();
        assert_eq!(backdrop_brightness, deserialized);
    }

    #[test]
    fn test_backdrop_filter_equality_and_hash() {
        let backdrop_blur1 = BackdropBlur::Lg;
        let backdrop_blur2 = BackdropBlur::Lg;
        let backdrop_blur3 = BackdropBlur::Md;
        
        assert_eq!(backdrop_blur1, backdrop_blur2);
        assert_ne!(backdrop_blur1, backdrop_blur3);
        
        // Test that equal effects have the same hash
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        backdrop_blur1.hash(&mut hasher1);
        backdrop_blur2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }
}
