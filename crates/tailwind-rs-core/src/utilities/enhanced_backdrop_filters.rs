//! Enhanced backdrop filter utilities for tailwind-rs
//!
//! This module provides enhanced backdrop filter utilities.
//! It includes additional backdrop blur variants and other backdrop filter effects.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Enhanced backdrop blur values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnhancedBackdropBlur {
    /// No backdrop blur
    None,
    /// Small backdrop blur (4px)
    Sm,
    /// Default backdrop blur (8px)
    Default,
    /// Medium backdrop blur (12px)
    Md,
    /// Large backdrop blur (16px)
    Lg,
    /// Extra large backdrop blur (24px)
    Xl,
    /// 2XL backdrop blur (40px)
    Xl2,
    /// 3XL backdrop blur (64px)
    Xl3,
}

impl fmt::Display for EnhancedBackdropBlur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnhancedBackdropBlur::None => write!(f, "none"),
            EnhancedBackdropBlur::Sm => write!(f, "4px"),
            EnhancedBackdropBlur::Default => write!(f, "8px"),
            EnhancedBackdropBlur::Md => write!(f, "12px"),
            EnhancedBackdropBlur::Lg => write!(f, "16px"),
            EnhancedBackdropBlur::Xl => write!(f, "24px"),
            EnhancedBackdropBlur::Xl2 => write!(f, "40px"),
            EnhancedBackdropBlur::Xl3 => write!(f, "64px"),
        }
    }
}

impl EnhancedBackdropBlur {
    /// Get the CSS class name for this backdrop blur
    pub fn to_class_name(&self) -> String {
        match self {
            EnhancedBackdropBlur::None => "backdrop-blur-none".to_string(),
            EnhancedBackdropBlur::Sm => "backdrop-blur-sm".to_string(),
            EnhancedBackdropBlur::Default => "backdrop-blur".to_string(),
            EnhancedBackdropBlur::Md => "backdrop-blur-md".to_string(),
            EnhancedBackdropBlur::Lg => "backdrop-blur-lg".to_string(),
            EnhancedBackdropBlur::Xl => "backdrop-blur-xl".to_string(),
            EnhancedBackdropBlur::Xl2 => "backdrop-blur-2xl".to_string(),
            EnhancedBackdropBlur::Xl3 => "backdrop-blur-3xl".to_string(),
        }
    }

    /// Get the CSS value for this backdrop blur
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }
}

/// Enhanced backdrop brightness values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnhancedBackdropBrightness {
    /// 0% brightness
    Zero,
    /// 50% brightness
    Half,
    /// 75% brightness
    ThreeQuarters,
    /// 90% brightness
    Ninety,
    /// 95% brightness
    NinetyFive,
    /// 100% brightness (default)
    Default,
    /// 105% brightness
    OneHundredFive,
    /// 110% brightness
    OneHundredTen,
    /// 125% brightness
    OneHundredTwentyFive,
    /// 150% brightness
    OneHundredFifty,
    /// 200% brightness
    TwoHundred,
}

impl fmt::Display for EnhancedBackdropBrightness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnhancedBackdropBrightness::Zero => write!(f, "0"),
            EnhancedBackdropBrightness::Half => write!(f, "0.5"),
            EnhancedBackdropBrightness::ThreeQuarters => write!(f, "0.75"),
            EnhancedBackdropBrightness::Ninety => write!(f, "0.9"),
            EnhancedBackdropBrightness::NinetyFive => write!(f, "0.95"),
            EnhancedBackdropBrightness::Default => write!(f, "1"),
            EnhancedBackdropBrightness::OneHundredFive => write!(f, "1.05"),
            EnhancedBackdropBrightness::OneHundredTen => write!(f, "1.1"),
            EnhancedBackdropBrightness::OneHundredTwentyFive => write!(f, "1.25"),
            EnhancedBackdropBrightness::OneHundredFifty => write!(f, "1.5"),
            EnhancedBackdropBrightness::TwoHundred => write!(f, "2"),
        }
    }
}

/// Enhanced backdrop contrast values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnhancedBackdropContrast {
    /// 0% contrast
    Zero,
    /// 50% contrast
    Half,
    /// 75% contrast
    ThreeQuarters,
    /// 90% contrast
    Ninety,
    /// 95% contrast
    NinetyFive,
    /// 100% contrast (default)
    Default,
    /// 105% contrast
    OneHundredFive,
    /// 110% contrast
    OneHundredTen,
    /// 125% contrast
    OneHundredTwentyFive,
    /// 150% contrast
    OneHundredFifty,
    /// 200% contrast
    TwoHundred,
}

impl fmt::Display for EnhancedBackdropContrast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnhancedBackdropContrast::Zero => write!(f, "0"),
            EnhancedBackdropContrast::Half => write!(f, "0.5"),
            EnhancedBackdropContrast::ThreeQuarters => write!(f, "0.75"),
            EnhancedBackdropContrast::Ninety => write!(f, "0.9"),
            EnhancedBackdropContrast::NinetyFive => write!(f, "0.95"),
            EnhancedBackdropContrast::Default => write!(f, "1"),
            EnhancedBackdropContrast::OneHundredFive => write!(f, "1.05"),
            EnhancedBackdropContrast::OneHundredTen => write!(f, "1.1"),
            EnhancedBackdropContrast::OneHundredTwentyFive => write!(f, "1.25"),
            EnhancedBackdropContrast::OneHundredFifty => write!(f, "1.5"),
            EnhancedBackdropContrast::TwoHundred => write!(f, "2"),
        }
    }
}

/// Enhanced backdrop grayscale values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnhancedBackdropGrayscale {
    /// No grayscale
    None,
    /// 25% grayscale
    Quarter,
    /// 50% grayscale
    Half,
    /// 75% grayscale
    ThreeQuarters,
    /// 100% grayscale
    Full,
}

impl fmt::Display for EnhancedBackdropGrayscale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnhancedBackdropGrayscale::None => write!(f, "0"),
            EnhancedBackdropGrayscale::Quarter => write!(f, "0.25"),
            EnhancedBackdropGrayscale::Half => write!(f, "0.5"),
            EnhancedBackdropGrayscale::ThreeQuarters => write!(f, "0.75"),
            EnhancedBackdropGrayscale::Full => write!(f, "1"),
        }
    }
}

/// Enhanced backdrop hue rotate values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnhancedBackdropHueRotate {
    /// No rotation
    None,
    /// 15 degrees
    Fifteen,
    /// 30 degrees
    Thirty,
    /// 60 degrees
    Sixty,
    /// 90 degrees
    Ninety,
    /// 180 degrees
    OneHundredEighty,
    /// 270 degrees
    TwoHundredSeventy,
}

impl fmt::Display for EnhancedBackdropHueRotate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnhancedBackdropHueRotate::None => write!(f, "0deg"),
            EnhancedBackdropHueRotate::Fifteen => write!(f, "15deg"),
            EnhancedBackdropHueRotate::Thirty => write!(f, "30deg"),
            EnhancedBackdropHueRotate::Sixty => write!(f, "60deg"),
            EnhancedBackdropHueRotate::Ninety => write!(f, "90deg"),
            EnhancedBackdropHueRotate::OneHundredEighty => write!(f, "180deg"),
            EnhancedBackdropHueRotate::TwoHundredSeventy => write!(f, "270deg"),
        }
    }
}

/// Enhanced backdrop invert values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnhancedBackdropInvert {
    /// No invert
    None,
    /// 25% invert
    Quarter,
    /// 50% invert
    Half,
    /// 75% invert
    ThreeQuarters,
    /// 100% invert
    Full,
}

impl fmt::Display for EnhancedBackdropInvert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnhancedBackdropInvert::None => write!(f, "0"),
            EnhancedBackdropInvert::Quarter => write!(f, "0.25"),
            EnhancedBackdropInvert::Half => write!(f, "0.5"),
            EnhancedBackdropInvert::ThreeQuarters => write!(f, "0.75"),
            EnhancedBackdropInvert::Full => write!(f, "1"),
        }
    }
}

/// Enhanced backdrop opacity values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnhancedBackdropOpacity {
    /// 0% opacity
    Zero,
    /// 5% opacity
    Five,
    /// 10% opacity
    Ten,
    /// 20% opacity
    Twenty,
    /// 25% opacity
    TwentyFive,
    /// 30% opacity
    Thirty,
    /// 40% opacity
    Forty,
    /// 50% opacity
    Fifty,
    /// 60% opacity
    Sixty,
    /// 70% opacity
    Seventy,
    /// 75% opacity
    SeventyFive,
    /// 80% opacity
    Eighty,
    /// 90% opacity
    Ninety,
    /// 95% opacity
    NinetyFive,
    /// 100% opacity (default)
    Default,
}

impl fmt::Display for EnhancedBackdropOpacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnhancedBackdropOpacity::Zero => write!(f, "0"),
            EnhancedBackdropOpacity::Five => write!(f, "0.05"),
            EnhancedBackdropOpacity::Ten => write!(f, "0.1"),
            EnhancedBackdropOpacity::Twenty => write!(f, "0.2"),
            EnhancedBackdropOpacity::TwentyFive => write!(f, "0.25"),
            EnhancedBackdropOpacity::Thirty => write!(f, "0.3"),
            EnhancedBackdropOpacity::Forty => write!(f, "0.4"),
            EnhancedBackdropOpacity::Fifty => write!(f, "0.5"),
            EnhancedBackdropOpacity::Sixty => write!(f, "0.6"),
            EnhancedBackdropOpacity::Seventy => write!(f, "0.7"),
            EnhancedBackdropOpacity::SeventyFive => write!(f, "0.75"),
            EnhancedBackdropOpacity::Eighty => write!(f, "0.8"),
            EnhancedBackdropOpacity::Ninety => write!(f, "0.9"),
            EnhancedBackdropOpacity::NinetyFive => write!(f, "0.95"),
            EnhancedBackdropOpacity::Default => write!(f, "1"),
        }
    }
}

/// Enhanced backdrop saturate values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnhancedBackdropSaturate {
    /// 0% saturation
    Zero,
    /// 50% saturation
    Half,
    /// 75% saturation
    ThreeQuarters,
    /// 90% saturation
    Ninety,
    /// 95% saturation
    NinetyFive,
    /// 100% saturation (default)
    Default,
    /// 105% saturation
    OneHundredFive,
    /// 110% saturation
    OneHundredTen,
    /// 125% saturation
    OneHundredTwentyFive,
    /// 150% saturation
    OneHundredFifty,
    /// 200% saturation
    TwoHundred,
}

impl fmt::Display for EnhancedBackdropSaturate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnhancedBackdropSaturate::Zero => write!(f, "0"),
            EnhancedBackdropSaturate::Half => write!(f, "0.5"),
            EnhancedBackdropSaturate::ThreeQuarters => write!(f, "0.75"),
            EnhancedBackdropSaturate::Ninety => write!(f, "0.9"),
            EnhancedBackdropSaturate::NinetyFive => write!(f, "0.95"),
            EnhancedBackdropSaturate::Default => write!(f, "1"),
            EnhancedBackdropSaturate::OneHundredFive => write!(f, "1.05"),
            EnhancedBackdropSaturate::OneHundredTen => write!(f, "1.1"),
            EnhancedBackdropSaturate::OneHundredTwentyFive => write!(f, "1.25"),
            EnhancedBackdropSaturate::OneHundredFifty => write!(f, "1.5"),
            EnhancedBackdropSaturate::TwoHundred => write!(f, "2"),
        }
    }
}

/// Enhanced backdrop sepia values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnhancedBackdropSepia {
    /// No sepia
    None,
    /// 25% sepia
    Quarter,
    /// 50% sepia
    Half,
    /// 75% sepia
    ThreeQuarters,
    /// 100% sepia
    Full,
}

impl fmt::Display for EnhancedBackdropSepia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnhancedBackdropSepia::None => write!(f, "0"),
            EnhancedBackdropSepia::Quarter => write!(f, "0.25"),
            EnhancedBackdropSepia::Half => write!(f, "0.5"),
            EnhancedBackdropSepia::ThreeQuarters => write!(f, "0.75"),
            EnhancedBackdropSepia::Full => write!(f, "1"),
        }
    }
}

/// Trait for adding enhanced backdrop filter utilities to ClassBuilder
pub trait EnhancedBackdropFilterUtilities {
    /// Set backdrop blur to none
    fn backdrop_blur_none(self) -> Self;
    /// Set backdrop blur to small
    fn backdrop_blur_sm(self) -> Self;
    /// Set backdrop blur to default
    fn backdrop_blur(self) -> Self;
    /// Set backdrop blur to medium
    fn backdrop_blur_md(self) -> Self;
    /// Set backdrop blur to large
    fn backdrop_blur_lg(self) -> Self;
    /// Set backdrop blur to extra large
    fn backdrop_blur_xl(self) -> Self;
    /// Set backdrop blur to 2XL
    fn backdrop_blur_2xl(self) -> Self;
    /// Set backdrop blur to 3XL
    fn backdrop_blur_3xl(self) -> Self;
    /// Set backdrop blur with custom value
    fn backdrop_blur_custom(self, blur: EnhancedBackdropBlur) -> Self;
}

impl EnhancedBackdropFilterUtilities for ClassBuilder {
    fn backdrop_blur_none(self) -> Self {
        self.class("backdrop-blur-none")
    }

    fn backdrop_blur_sm(self) -> Self {
        self.class("backdrop-blur-sm")
    }

    fn backdrop_blur(self) -> Self {
        self.class("backdrop-blur")
    }

    fn backdrop_blur_md(self) -> Self {
        self.class("backdrop-blur-md")
    }

    fn backdrop_blur_lg(self) -> Self {
        self.class("backdrop-blur-lg")
    }

    fn backdrop_blur_xl(self) -> Self {
        self.class("backdrop-blur-xl")
    }

    fn backdrop_blur_2xl(self) -> Self {
        self.class("backdrop-blur-2xl")
    }

    fn backdrop_blur_3xl(self) -> Self {
        self.class("backdrop-blur-3xl")
    }

    fn backdrop_blur_custom(self, blur: EnhancedBackdropBlur) -> Self {
        self.class(&blur.to_class_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::ClassBuilder;

    #[test]
    fn test_enhanced_backdrop_blur_enum_values() {
        assert_eq!(EnhancedBackdropBlur::None.to_string(), "none");
        assert_eq!(EnhancedBackdropBlur::Sm.to_string(), "4px");
        assert_eq!(EnhancedBackdropBlur::Default.to_string(), "8px");
        assert_eq!(EnhancedBackdropBlur::Md.to_string(), "12px");
        assert_eq!(EnhancedBackdropBlur::Lg.to_string(), "16px");
        assert_eq!(EnhancedBackdropBlur::Xl.to_string(), "24px");
        assert_eq!(EnhancedBackdropBlur::Xl2.to_string(), "40px");
        assert_eq!(EnhancedBackdropBlur::Xl3.to_string(), "64px");
    }

    #[test]
    fn test_enhanced_backdrop_blur_class_names() {
        assert_eq!(EnhancedBackdropBlur::None.to_class_name(), "backdrop-blur-none");
        assert_eq!(EnhancedBackdropBlur::Sm.to_class_name(), "backdrop-blur-sm");
        assert_eq!(EnhancedBackdropBlur::Default.to_class_name(), "backdrop-blur");
        assert_eq!(EnhancedBackdropBlur::Md.to_class_name(), "backdrop-blur-md");
        assert_eq!(EnhancedBackdropBlur::Lg.to_class_name(), "backdrop-blur-lg");
        assert_eq!(EnhancedBackdropBlur::Xl.to_class_name(), "backdrop-blur-xl");
        assert_eq!(EnhancedBackdropBlur::Xl2.to_class_name(), "backdrop-blur-2xl");
        assert_eq!(EnhancedBackdropBlur::Xl3.to_class_name(), "backdrop-blur-3xl");
    }

    #[test]
    fn test_enhanced_backdrop_filter_utilities() {
        let classes = ClassBuilder::new()
            .backdrop_blur_none()
            .backdrop_blur_sm()
            .backdrop_blur()
            .backdrop_blur_md()
            .backdrop_blur_lg()
            .backdrop_blur_xl()
            .backdrop_blur_2xl()
            .backdrop_blur_3xl();

        let result = classes.build();
        assert!(result.contains("backdrop-blur-none"));
        assert!(result.contains("backdrop-blur-sm"));
        assert!(result.contains("backdrop-blur"));
        assert!(result.contains("backdrop-blur-md"));
        assert!(result.contains("backdrop-blur-lg"));
        assert!(result.contains("backdrop-blur-xl"));
        assert!(result.contains("backdrop-blur-2xl"));
        assert!(result.contains("backdrop-blur-3xl"));
    }

    #[test]
    fn test_enhanced_backdrop_blur_css_values() {
        assert_eq!(EnhancedBackdropBlur::None.to_css_value(), "none");
        assert_eq!(EnhancedBackdropBlur::Sm.to_css_value(), "4px");
        assert_eq!(EnhancedBackdropBlur::Default.to_css_value(), "8px");
        assert_eq!(EnhancedBackdropBlur::Md.to_css_value(), "12px");
        assert_eq!(EnhancedBackdropBlur::Lg.to_css_value(), "16px");
        assert_eq!(EnhancedBackdropBlur::Xl.to_css_value(), "24px");
        assert_eq!(EnhancedBackdropBlur::Xl2.to_css_value(), "40px");
        assert_eq!(EnhancedBackdropBlur::Xl3.to_css_value(), "64px");
    }

    #[test]
    fn test_enhanced_backdrop_blur_serialization() {
        let blur = EnhancedBackdropBlur::Lg;
        let serialized = serde_json::to_string(&blur).unwrap();
        let deserialized: EnhancedBackdropBlur = serde_json::from_str(&serialized).unwrap();
        assert_eq!(blur, deserialized);
    }

    #[test]
    fn test_enhanced_backdrop_blur_comprehensive_usage() {
        let classes = ClassBuilder::new()
            .backdrop_blur_sm()
            .backdrop_blur_lg()
            .backdrop_blur_3xl();

        let result = classes.build();
        assert!(result.contains("backdrop-blur-sm"));
        assert!(result.contains("backdrop-blur-lg"));
        assert!(result.contains("backdrop-blur-3xl"));
    }

    #[test]
    fn test_enhanced_backdrop_blur_custom_usage() {
        let classes = ClassBuilder::new()
            .backdrop_blur_custom(EnhancedBackdropBlur::Xl)
            .backdrop_blur_custom(EnhancedBackdropBlur::Xl3);

        let result = classes.build();
        assert!(result.contains("backdrop-blur-xl"));
        assert!(result.contains("backdrop-blur-3xl"));
    }

    #[test]
    fn test_enhanced_backdrop_brightness_enum_values() {
        assert_eq!(EnhancedBackdropBrightness::Zero.to_string(), "0");
        assert_eq!(EnhancedBackdropBrightness::Half.to_string(), "0.5");
        assert_eq!(EnhancedBackdropBrightness::Default.to_string(), "1");
        assert_eq!(EnhancedBackdropBrightness::OneHundredFifty.to_string(), "1.5");
        assert_eq!(EnhancedBackdropBrightness::TwoHundred.to_string(), "2");
    }

    #[test]
    fn test_enhanced_backdrop_contrast_enum_values() {
        assert_eq!(EnhancedBackdropContrast::Zero.to_string(), "0");
        assert_eq!(EnhancedBackdropContrast::Half.to_string(), "0.5");
        assert_eq!(EnhancedBackdropContrast::Default.to_string(), "1");
        assert_eq!(EnhancedBackdropContrast::OneHundredFifty.to_string(), "1.5");
        assert_eq!(EnhancedBackdropContrast::TwoHundred.to_string(), "2");
    }

    #[test]
    fn test_enhanced_backdrop_grayscale_enum_values() {
        assert_eq!(EnhancedBackdropGrayscale::None.to_string(), "0");
        assert_eq!(EnhancedBackdropGrayscale::Quarter.to_string(), "0.25");
        assert_eq!(EnhancedBackdropGrayscale::Half.to_string(), "0.5");
        assert_eq!(EnhancedBackdropGrayscale::ThreeQuarters.to_string(), "0.75");
        assert_eq!(EnhancedBackdropGrayscale::Full.to_string(), "1");
    }

    #[test]
    fn test_enhanced_backdrop_hue_rotate_enum_values() {
        assert_eq!(EnhancedBackdropHueRotate::None.to_string(), "0deg");
        assert_eq!(EnhancedBackdropHueRotate::Fifteen.to_string(), "15deg");
        assert_eq!(EnhancedBackdropHueRotate::Thirty.to_string(), "30deg");
        assert_eq!(EnhancedBackdropHueRotate::Ninety.to_string(), "90deg");
        assert_eq!(EnhancedBackdropHueRotate::OneHundredEighty.to_string(), "180deg");
        assert_eq!(EnhancedBackdropHueRotate::TwoHundredSeventy.to_string(), "270deg");
    }

    #[test]
    fn test_enhanced_backdrop_invert_enum_values() {
        assert_eq!(EnhancedBackdropInvert::None.to_string(), "0");
        assert_eq!(EnhancedBackdropInvert::Quarter.to_string(), "0.25");
        assert_eq!(EnhancedBackdropInvert::Half.to_string(), "0.5");
        assert_eq!(EnhancedBackdropInvert::ThreeQuarters.to_string(), "0.75");
        assert_eq!(EnhancedBackdropInvert::Full.to_string(), "1");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_enum_values() {
        assert_eq!(EnhancedBackdropOpacity::Zero.to_string(), "0");
        assert_eq!(EnhancedBackdropOpacity::Five.to_string(), "0.05");
        assert_eq!(EnhancedBackdropOpacity::Ten.to_string(), "0.1");
        assert_eq!(EnhancedBackdropOpacity::TwentyFive.to_string(), "0.25");
        assert_eq!(EnhancedBackdropOpacity::Fifty.to_string(), "0.5");
        assert_eq!(EnhancedBackdropOpacity::SeventyFive.to_string(), "0.75");
        assert_eq!(EnhancedBackdropOpacity::Default.to_string(), "1");
    }

    #[test]
    fn test_enhanced_backdrop_saturate_enum_values() {
        assert_eq!(EnhancedBackdropSaturate::Zero.to_string(), "0");
        assert_eq!(EnhancedBackdropSaturate::Half.to_string(), "0.5");
        assert_eq!(EnhancedBackdropSaturate::Default.to_string(), "1");
        assert_eq!(EnhancedBackdropSaturate::OneHundredFifty.to_string(), "1.5");
        assert_eq!(EnhancedBackdropSaturate::TwoHundred.to_string(), "2");
    }

    #[test]
    fn test_enhanced_backdrop_sepia_enum_values() {
        assert_eq!(EnhancedBackdropSepia::None.to_string(), "0");
        assert_eq!(EnhancedBackdropSepia::Quarter.to_string(), "0.25");
        assert_eq!(EnhancedBackdropSepia::Half.to_string(), "0.5");
        assert_eq!(EnhancedBackdropSepia::ThreeQuarters.to_string(), "0.75");
        assert_eq!(EnhancedBackdropSepia::Full.to_string(), "1");
    }
}
