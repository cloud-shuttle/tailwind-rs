//! Filter utilities for tailwind-rs
//!
//! This module provides utilities for CSS filters including blur, brightness,
//! contrast, drop-shadow, grayscale, hue-rotate, invert, saturate, and sepia.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Blur values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Blur {
    /// No blur
    None,
    /// Small blur
    Sm,
    /// Default blur
    Default,
    /// Medium blur
    Md,
    /// Large blur
    Lg,
    /// Extra large blur
    Xl,
    /// 2x large blur
    Xl2,
    /// 3x large blur
    Xl3,
}

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

/// Grayscale values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Grayscale {
    /// 0% grayscale
    Zero,
    /// 100% grayscale
    Hundred,
}

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
    HundredEighty,
}

/// Invert values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Invert {
    /// 0% invert
    Zero,
    /// 100% invert
    Hundred,
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

/// Sepia values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Sepia {
    /// 0% sepia
    Zero,
    /// 100% sepia
    Hundred,
}

impl Blur {
    pub fn to_class_name(&self) -> String {
        match self {
            Blur::None => "none".to_string(),
            Blur::Sm => "sm".to_string(),
            Blur::Default => "default".to_string(),
            Blur::Md => "md".to_string(),
            Blur::Lg => "lg".to_string(),
            Blur::Xl => "xl".to_string(),
            Blur::Xl2 => "2xl".to_string(),
            Blur::Xl3 => "3xl".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Blur::None => "0".to_string(),
            Blur::Sm => "4px".to_string(),
            Blur::Default => "8px".to_string(),
            Blur::Md => "12px".to_string(),
            Blur::Lg => "16px".to_string(),
            Blur::Xl => "24px".to_string(),
            Blur::Xl2 => "40px".to_string(),
            Blur::Xl3 => "64px".to_string(),
        }
    }
}

impl Brightness {
    pub fn to_class_name(&self) -> String {
        match self {
            Brightness::Zero => "0".to_string(),
            Brightness::Fifty => "50".to_string(),
            Brightness::SeventyFive => "75".to_string(),
            Brightness::Ninety => "90".to_string(),
            Brightness::NinetyFive => "95".to_string(),
            Brightness::Hundred => "100".to_string(),
            Brightness::HundredFive => "105".to_string(),
            Brightness::HundredTen => "110".to_string(),
            Brightness::HundredTwentyFive => "125".to_string(),
            Brightness::HundredFifty => "150".to_string(),
            Brightness::TwoHundred => "200".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Brightness::Zero => "0".to_string(),
            Brightness::Fifty => "0.5".to_string(),
            Brightness::SeventyFive => "0.75".to_string(),
            Brightness::Ninety => "0.9".to_string(),
            Brightness::NinetyFive => "0.95".to_string(),
            Brightness::Hundred => "1".to_string(),
            Brightness::HundredFive => "1.05".to_string(),
            Brightness::HundredTen => "1.1".to_string(),
            Brightness::HundredTwentyFive => "1.25".to_string(),
            Brightness::HundredFifty => "1.5".to_string(),
            Brightness::TwoHundred => "2".to_string(),
        }
    }
}

impl Contrast {
    pub fn to_class_name(&self) -> String {
        match self {
            Contrast::Zero => "0".to_string(),
            Contrast::Fifty => "50".to_string(),
            Contrast::SeventyFive => "75".to_string(),
            Contrast::Hundred => "100".to_string(),
            Contrast::HundredTwentyFive => "125".to_string(),
            Contrast::HundredFifty => "150".to_string(),
            Contrast::TwoHundred => "200".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Contrast::Zero => "0".to_string(),
            Contrast::Fifty => "0.5".to_string(),
            Contrast::SeventyFive => "0.75".to_string(),
            Contrast::Hundred => "1".to_string(),
            Contrast::HundredTwentyFive => "1.25".to_string(),
            Contrast::HundredFifty => "1.5".to_string(),
            Contrast::TwoHundred => "2".to_string(),
        }
    }
}

impl Grayscale {
    pub fn to_class_name(&self) -> String {
        match self {
            Grayscale::Zero => "0".to_string(),
            Grayscale::Hundred => "100".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Grayscale::Zero => "0".to_string(),
            Grayscale::Hundred => "1".to_string(),
        }
    }
}

impl HueRotate {
    pub fn to_class_name(&self) -> String {
        match self {
            HueRotate::Zero => "0".to_string(),
            HueRotate::Fifteen => "15".to_string(),
            HueRotate::Thirty => "30".to_string(),
            HueRotate::Sixty => "60".to_string(),
            HueRotate::Ninety => "90".to_string(),
            HueRotate::HundredEighty => "180".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            HueRotate::Zero => "0deg".to_string(),
            HueRotate::Fifteen => "15deg".to_string(),
            HueRotate::Thirty => "30deg".to_string(),
            HueRotate::Sixty => "60deg".to_string(),
            HueRotate::Ninety => "90deg".to_string(),
            HueRotate::HundredEighty => "180deg".to_string(),
        }
    }
}

impl Invert {
    pub fn to_class_name(&self) -> String {
        match self {
            Invert::Zero => "0".to_string(),
            Invert::Hundred => "100".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Invert::Zero => "0".to_string(),
            Invert::Hundred => "1".to_string(),
        }
    }
}

impl Saturate {
    pub fn to_class_name(&self) -> String {
        match self {
            Saturate::Zero => "0".to_string(),
            Saturate::Fifty => "50".to_string(),
            Saturate::Hundred => "100".to_string(),
            Saturate::HundredFifty => "150".to_string(),
            Saturate::TwoHundred => "200".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Saturate::Zero => "0".to_string(),
            Saturate::Fifty => "0.5".to_string(),
            Saturate::Hundred => "1".to_string(),
            Saturate::HundredFifty => "1.5".to_string(),
            Saturate::TwoHundred => "2".to_string(),
        }
    }
}

impl Sepia {
    pub fn to_class_name(&self) -> String {
        match self {
            Sepia::Zero => "0".to_string(),
            Sepia::Hundred => "100".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Sepia::Zero => "0".to_string(),
            Sepia::Hundred => "1".to_string(),
        }
    }
}

impl fmt::Display for Blur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for Brightness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for Contrast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for Grayscale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for HueRotate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for Invert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for Saturate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for Sepia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Trait for adding blur utilities to a class builder
pub trait BlurUtilities {
    fn blur(self, blur: Blur) -> Self;
}

impl BlurUtilities for ClassBuilder {
    fn blur(self, blur: Blur) -> Self {
        self.class(format!("blur-{}", blur.to_class_name()))
    }
}

/// Trait for adding brightness utilities to a class builder
pub trait BrightnessUtilities {
    fn brightness(self, brightness: Brightness) -> Self;
}

impl BrightnessUtilities for ClassBuilder {
    fn brightness(self, brightness: Brightness) -> Self {
        self.class(format!("brightness-{}", brightness.to_class_name()))
    }
}

/// Trait for adding contrast utilities to a class builder
pub trait ContrastUtilities {
    fn contrast(self, contrast: Contrast) -> Self;
}

impl ContrastUtilities for ClassBuilder {
    fn contrast(self, contrast: Contrast) -> Self {
        self.class(format!("contrast-{}", contrast.to_class_name()))
    }
}

/// Trait for adding grayscale utilities to a class builder
pub trait GrayscaleUtilities {
    fn grayscale(self, grayscale: Grayscale) -> Self;
}

impl GrayscaleUtilities for ClassBuilder {
    fn grayscale(self, grayscale: Grayscale) -> Self {
        self.class(format!("grayscale-{}", grayscale.to_class_name()))
    }
}

/// Trait for adding hue rotate utilities to a class builder
pub trait HueRotateUtilities {
    fn hue_rotate(self, hue_rotate: HueRotate) -> Self;
}

impl HueRotateUtilities for ClassBuilder {
    fn hue_rotate(self, hue_rotate: HueRotate) -> Self {
        self.class(format!("hue-rotate-{}", hue_rotate.to_class_name()))
    }
}

/// Trait for adding invert utilities to a class builder
pub trait InvertUtilities {
    fn invert(self, invert: Invert) -> Self;
}

impl InvertUtilities for ClassBuilder {
    fn invert(self, invert: Invert) -> Self {
        self.class(format!("invert-{}", invert.to_class_name()))
    }
}

/// Trait for adding saturate utilities to a class builder
pub trait SaturateUtilities {
    fn saturate(self, saturate: Saturate) -> Self;
}

impl SaturateUtilities for ClassBuilder {
    fn saturate(self, saturate: Saturate) -> Self {
        self.class(format!("saturate-{}", saturate.to_class_name()))
    }
}

/// Trait for adding sepia utilities to a class builder
pub trait SepiaUtilities {
    fn sepia(self, sepia: Sepia) -> Self;
}

impl SepiaUtilities for ClassBuilder {
    fn sepia(self, sepia: Sepia) -> Self {
        self.class(format!("sepia-{}", sepia.to_class_name()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_blur_utilities() {
        let classes = ClassBuilder::new()
            .blur(Blur::None)
            .blur(Blur::Sm)
            .blur(Blur::Default)
            .blur(Blur::Md)
            .blur(Blur::Lg)
            .blur(Blur::Xl)
            .blur(Blur::Xl2)
            .blur(Blur::Xl3)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("blur-none"));
        assert!(css_classes.contains("blur-sm"));
        assert!(css_classes.contains("blur-default"));
        assert!(css_classes.contains("blur-md"));
        assert!(css_classes.contains("blur-lg"));
        assert!(css_classes.contains("blur-xl"));
        assert!(css_classes.contains("blur-2xl"));
        assert!(css_classes.contains("blur-3xl"));
    }
    
    #[test]
    fn test_brightness_utilities() {
        let classes = ClassBuilder::new()
            .brightness(Brightness::Zero)
            .brightness(Brightness::Fifty)
            .brightness(Brightness::SeventyFive)
            .brightness(Brightness::Ninety)
            .brightness(Brightness::NinetyFive)
            .brightness(Brightness::Hundred)
            .brightness(Brightness::HundredFive)
            .brightness(Brightness::HundredTen)
            .brightness(Brightness::HundredTwentyFive)
            .brightness(Brightness::HundredFifty)
            .brightness(Brightness::TwoHundred)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("brightness-0"));
        assert!(css_classes.contains("brightness-50"));
        assert!(css_classes.contains("brightness-75"));
        assert!(css_classes.contains("brightness-90"));
        assert!(css_classes.contains("brightness-95"));
        assert!(css_classes.contains("brightness-100"));
        assert!(css_classes.contains("brightness-105"));
        assert!(css_classes.contains("brightness-110"));
        assert!(css_classes.contains("brightness-125"));
        assert!(css_classes.contains("brightness-150"));
        assert!(css_classes.contains("brightness-200"));
    }
    
    #[test]
    fn test_contrast_utilities() {
        let classes = ClassBuilder::new()
            .contrast(Contrast::Zero)
            .contrast(Contrast::Fifty)
            .contrast(Contrast::SeventyFive)
            .contrast(Contrast::Hundred)
            .contrast(Contrast::HundredTwentyFive)
            .contrast(Contrast::HundredFifty)
            .contrast(Contrast::TwoHundred)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("contrast-0"));
        assert!(css_classes.contains("contrast-50"));
        assert!(css_classes.contains("contrast-75"));
        assert!(css_classes.contains("contrast-100"));
        assert!(css_classes.contains("contrast-125"));
        assert!(css_classes.contains("contrast-150"));
        assert!(css_classes.contains("contrast-200"));
    }
    
    #[test]
    fn test_grayscale_utilities() {
        let classes = ClassBuilder::new()
            .grayscale(Grayscale::Zero)
            .grayscale(Grayscale::Hundred)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("grayscale-0"));
        assert!(css_classes.contains("grayscale-100"));
    }
    
    #[test]
    fn test_hue_rotate_utilities() {
        let classes = ClassBuilder::new()
            .hue_rotate(HueRotate::Zero)
            .hue_rotate(HueRotate::Fifteen)
            .hue_rotate(HueRotate::Thirty)
            .hue_rotate(HueRotate::Sixty)
            .hue_rotate(HueRotate::Ninety)
            .hue_rotate(HueRotate::HundredEighty)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("hue-rotate-0"));
        assert!(css_classes.contains("hue-rotate-15"));
        assert!(css_classes.contains("hue-rotate-30"));
        assert!(css_classes.contains("hue-rotate-60"));
        assert!(css_classes.contains("hue-rotate-90"));
        assert!(css_classes.contains("hue-rotate-180"));
    }
    
    #[test]
    fn test_invert_utilities() {
        let classes = ClassBuilder::new()
            .invert(Invert::Zero)
            .invert(Invert::Hundred)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("invert-0"));
        assert!(css_classes.contains("invert-100"));
    }
    
    #[test]
    fn test_saturate_utilities() {
        let classes = ClassBuilder::new()
            .saturate(Saturate::Zero)
            .saturate(Saturate::Fifty)
            .saturate(Saturate::Hundred)
            .saturate(Saturate::HundredFifty)
            .saturate(Saturate::TwoHundred)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("saturate-0"));
        assert!(css_classes.contains("saturate-50"));
        assert!(css_classes.contains("saturate-100"));
        assert!(css_classes.contains("saturate-150"));
        assert!(css_classes.contains("saturate-200"));
    }
    
    #[test]
    fn test_sepia_utilities() {
        let classes = ClassBuilder::new()
            .sepia(Sepia::Zero)
            .sepia(Sepia::Hundred)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("sepia-0"));
        assert!(css_classes.contains("sepia-100"));
    }
    
    #[test]
    fn test_complex_filters_combination() {
        let classes = ClassBuilder::new()
            .blur(Blur::Md)
            .brightness(Brightness::HundredTen)
            .contrast(Contrast::HundredTwentyFive)
            .grayscale(Grayscale::Zero)
            .hue_rotate(HueRotate::Thirty)
            .invert(Invert::Zero)
            .saturate(Saturate::HundredFifty)
            .sepia(Sepia::Zero)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("blur-md"));
        assert!(css_classes.contains("brightness-110"));
        assert!(css_classes.contains("contrast-125"));
        assert!(css_classes.contains("grayscale-0"));
        assert!(css_classes.contains("hue-rotate-30"));
        assert!(css_classes.contains("invert-0"));
        assert!(css_classes.contains("saturate-150"));
        assert!(css_classes.contains("sepia-0"));
    }
}
