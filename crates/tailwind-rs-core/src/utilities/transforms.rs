//! Transform utilities for tailwind-rs
//!
//! This module provides utilities for CSS transforms including scale, rotate,
//! translate, skew, and transform origin.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Scale values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Scale {
    /// 0% scale
    Zero,
    /// 50% scale
    Fifty,
    /// 75% scale
    SeventyFive,
    /// 90% scale
    Ninety,
    /// 95% scale
    NinetyFive,
    /// 100% scale
    Hundred,
    /// 105% scale
    HundredFive,
    /// 110% scale
    HundredTen,
    /// 125% scale
    HundredTwentyFive,
    /// 150% scale
    HundredFifty,
}

/// Rotate values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rotate {
    /// 0 degrees
    Zero,
    /// 1 degree
    One,
    /// 2 degrees
    Two,
    /// 3 degrees
    Three,
    /// 6 degrees
    Six,
    /// 12 degrees
    Twelve,
    /// 45 degrees
    FortyFive,
    /// 90 degrees
    Ninety,
    /// 180 degrees
    HundredEighty,
}

/// Translate values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Translate {
    /// 0px translate
    Zero,
    /// 1px translate
    One,
    /// 2px translate
    Two,
    /// 3px translate
    Three,
    /// 4px translate
    Four,
    /// 5px translate
    Five,
    /// 6px translate
    Six,
    /// 7px translate
    Seven,
    /// 8px translate
    Eight,
    /// 9px translate
    Nine,
    /// 10px translate
    Ten,
    /// 11px translate
    Eleven,
    /// 12px translate
    Twelve,
    /// 14px translate
    Fourteen,
    /// 16px translate
    Sixteen,
    /// 20px translate
    Twenty,
    /// 24px translate
    TwentyFour,
    /// 28px translate
    TwentyEight,
    /// 32px translate
    ThirtyTwo,
    /// 36px translate
    ThirtySix,
    /// 40px translate
    Forty,
    /// 44px translate
    FortyFour,
    /// 48px translate
    FortyEight,
    /// 52px translate
    FiftyTwo,
    /// 56px translate
    FiftySix,
    /// 60px translate
    Sixty,
    /// 64px translate
    SixtyFour,
    /// 72px translate
    SeventyTwo,
    /// 80px translate
    Eighty,
    /// 96px translate
    NinetySix,
}

/// Skew values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Skew {
    /// 0 degrees
    Zero,
    /// 1 degree
    One,
    /// 2 degrees
    Two,
    /// 3 degrees
    Three,
    /// 6 degrees
    Six,
    /// 12 degrees
    Twelve,
}

/// Transform origin values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransformOrigin {
    /// Center origin
    Center,
    /// Top origin
    Top,
    /// Top right origin
    TopRight,
    /// Right origin
    Right,
    /// Bottom right origin
    BottomRight,
    /// Bottom origin
    Bottom,
    /// Bottom left origin
    BottomLeft,
    /// Left origin
    Left,
    /// Top left origin
    TopLeft,
}

impl Scale {
    pub fn to_class_name(&self) -> String {
        match self {
            Scale::Zero => "0".to_string(),
            Scale::Fifty => "50".to_string(),
            Scale::SeventyFive => "75".to_string(),
            Scale::Ninety => "90".to_string(),
            Scale::NinetyFive => "95".to_string(),
            Scale::Hundred => "100".to_string(),
            Scale::HundredFive => "105".to_string(),
            Scale::HundredTen => "110".to_string(),
            Scale::HundredTwentyFive => "125".to_string(),
            Scale::HundredFifty => "150".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Scale::Zero => "0".to_string(),
            Scale::Fifty => "0.5".to_string(),
            Scale::SeventyFive => "0.75".to_string(),
            Scale::Ninety => "0.9".to_string(),
            Scale::NinetyFive => "0.95".to_string(),
            Scale::Hundred => "1".to_string(),
            Scale::HundredFive => "1.05".to_string(),
            Scale::HundredTen => "1.1".to_string(),
            Scale::HundredTwentyFive => "1.25".to_string(),
            Scale::HundredFifty => "1.5".to_string(),
        }
    }
}

impl Rotate {
    pub fn to_class_name(&self) -> String {
        match self {
            Rotate::Zero => "0".to_string(),
            Rotate::One => "1".to_string(),
            Rotate::Two => "2".to_string(),
            Rotate::Three => "3".to_string(),
            Rotate::Six => "6".to_string(),
            Rotate::Twelve => "12".to_string(),
            Rotate::FortyFive => "45".to_string(),
            Rotate::Ninety => "90".to_string(),
            Rotate::HundredEighty => "180".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Rotate::Zero => "0deg".to_string(),
            Rotate::One => "1deg".to_string(),
            Rotate::Two => "2deg".to_string(),
            Rotate::Three => "3deg".to_string(),
            Rotate::Six => "6deg".to_string(),
            Rotate::Twelve => "12deg".to_string(),
            Rotate::FortyFive => "45deg".to_string(),
            Rotate::Ninety => "90deg".to_string(),
            Rotate::HundredEighty => "180deg".to_string(),
        }
    }
}

impl Translate {
    pub fn to_class_name(&self) -> String {
        match self {
            Translate::Zero => "0".to_string(),
            Translate::One => "1".to_string(),
            Translate::Two => "2".to_string(),
            Translate::Three => "3".to_string(),
            Translate::Four => "4".to_string(),
            Translate::Five => "5".to_string(),
            Translate::Six => "6".to_string(),
            Translate::Seven => "7".to_string(),
            Translate::Eight => "8".to_string(),
            Translate::Nine => "9".to_string(),
            Translate::Ten => "10".to_string(),
            Translate::Eleven => "11".to_string(),
            Translate::Twelve => "12".to_string(),
            Translate::Fourteen => "14".to_string(),
            Translate::Sixteen => "16".to_string(),
            Translate::Twenty => "20".to_string(),
            Translate::TwentyFour => "24".to_string(),
            Translate::TwentyEight => "28".to_string(),
            Translate::ThirtyTwo => "32".to_string(),
            Translate::ThirtySix => "36".to_string(),
            Translate::Forty => "40".to_string(),
            Translate::FortyFour => "44".to_string(),
            Translate::FortyEight => "48".to_string(),
            Translate::FiftyTwo => "52".to_string(),
            Translate::FiftySix => "56".to_string(),
            Translate::Sixty => "60".to_string(),
            Translate::SixtyFour => "64".to_string(),
            Translate::SeventyTwo => "72".to_string(),
            Translate::Eighty => "80".to_string(),
            Translate::NinetySix => "96".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Translate::Zero => "0px".to_string(),
            Translate::One => "0.25rem".to_string(),
            Translate::Two => "0.5rem".to_string(),
            Translate::Three => "0.75rem".to_string(),
            Translate::Four => "1rem".to_string(),
            Translate::Five => "1.25rem".to_string(),
            Translate::Six => "1.5rem".to_string(),
            Translate::Seven => "1.75rem".to_string(),
            Translate::Eight => "2rem".to_string(),
            Translate::Nine => "2.25rem".to_string(),
            Translate::Ten => "2.5rem".to_string(),
            Translate::Eleven => "2.75rem".to_string(),
            Translate::Twelve => "3rem".to_string(),
            Translate::Fourteen => "3.5rem".to_string(),
            Translate::Sixteen => "4rem".to_string(),
            Translate::Twenty => "5rem".to_string(),
            Translate::TwentyFour => "6rem".to_string(),
            Translate::TwentyEight => "7rem".to_string(),
            Translate::ThirtyTwo => "8rem".to_string(),
            Translate::ThirtySix => "9rem".to_string(),
            Translate::Forty => "10rem".to_string(),
            Translate::FortyFour => "11rem".to_string(),
            Translate::FortyEight => "12rem".to_string(),
            Translate::FiftyTwo => "13rem".to_string(),
            Translate::FiftySix => "14rem".to_string(),
            Translate::Sixty => "15rem".to_string(),
            Translate::SixtyFour => "16rem".to_string(),
            Translate::SeventyTwo => "18rem".to_string(),
            Translate::Eighty => "20rem".to_string(),
            Translate::NinetySix => "24rem".to_string(),
        }
    }
}

impl Skew {
    pub fn to_class_name(&self) -> String {
        match self {
            Skew::Zero => "0".to_string(),
            Skew::One => "1".to_string(),
            Skew::Two => "2".to_string(),
            Skew::Three => "3".to_string(),
            Skew::Six => "6".to_string(),
            Skew::Twelve => "12".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Skew::Zero => "0deg".to_string(),
            Skew::One => "1deg".to_string(),
            Skew::Two => "2deg".to_string(),
            Skew::Three => "3deg".to_string(),
            Skew::Six => "6deg".to_string(),
            Skew::Twelve => "12deg".to_string(),
        }
    }
}

impl TransformOrigin {
    pub fn to_class_name(&self) -> String {
        match self {
            TransformOrigin::Center => "center".to_string(),
            TransformOrigin::Top => "top".to_string(),
            TransformOrigin::TopRight => "top-right".to_string(),
            TransformOrigin::Right => "right".to_string(),
            TransformOrigin::BottomRight => "bottom-right".to_string(),
            TransformOrigin::Bottom => "bottom".to_string(),
            TransformOrigin::BottomLeft => "bottom-left".to_string(),
            TransformOrigin::Left => "left".to_string(),
            TransformOrigin::TopLeft => "top-left".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            TransformOrigin::Center => "center".to_string(),
            TransformOrigin::Top => "top".to_string(),
            TransformOrigin::TopRight => "top right".to_string(),
            TransformOrigin::Right => "right".to_string(),
            TransformOrigin::BottomRight => "bottom right".to_string(),
            TransformOrigin::Bottom => "bottom".to_string(),
            TransformOrigin::BottomLeft => "bottom left".to_string(),
            TransformOrigin::Left => "left".to_string(),
            TransformOrigin::TopLeft => "top left".to_string(),
        }
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for Rotate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for Translate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for Skew {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for TransformOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Trait for adding scale utilities to a class builder
pub trait ScaleUtilities {
    fn scale(self, scale: Scale) -> Self;
    fn scale_x(self, scale: Scale) -> Self;
    fn scale_y(self, scale: Scale) -> Self;
}

impl ScaleUtilities for ClassBuilder {
    fn scale(self, scale: Scale) -> Self {
        self.class(format!("scale-{}", scale.to_class_name()))
    }
    
    fn scale_x(self, scale: Scale) -> Self {
        self.class(format!("scale-x-{}", scale.to_class_name()))
    }
    
    fn scale_y(self, scale: Scale) -> Self {
        self.class(format!("scale-y-{}", scale.to_class_name()))
    }
}

/// Trait for adding rotate utilities to a class builder
pub trait RotateUtilities {
    fn rotate(self, rotate: Rotate) -> Self;
}

impl RotateUtilities for ClassBuilder {
    fn rotate(self, rotate: Rotate) -> Self {
        self.class(format!("rotate-{}", rotate.to_class_name()))
    }
}

/// Trait for adding translate utilities to a class builder
pub trait TranslateUtilities {
    fn translate_x(self, translate: Translate) -> Self;
    fn translate_y(self, translate: Translate) -> Self;
}

impl TranslateUtilities for ClassBuilder {
    fn translate_x(self, translate: Translate) -> Self {
        self.class(format!("translate-x-{}", translate.to_class_name()))
    }
    
    fn translate_y(self, translate: Translate) -> Self {
        self.class(format!("translate-y-{}", translate.to_class_name()))
    }
}

/// Trait for adding skew utilities to a class builder
pub trait SkewUtilities {
    fn skew_x(self, skew: Skew) -> Self;
    fn skew_y(self, skew: Skew) -> Self;
}

impl SkewUtilities for ClassBuilder {
    fn skew_x(self, skew: Skew) -> Self {
        self.class(format!("skew-x-{}", skew.to_class_name()))
    }
    
    fn skew_y(self, skew: Skew) -> Self {
        self.class(format!("skew-y-{}", skew.to_class_name()))
    }
}

/// Trait for adding transform origin utilities to a class builder
pub trait TransformOriginUtilities {
    fn transform_origin(self, origin: TransformOrigin) -> Self;
}

impl TransformOriginUtilities for ClassBuilder {
    fn transform_origin(self, origin: TransformOrigin) -> Self {
        self.class(format!("origin-{}", origin.to_class_name()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scale_utilities() {
        let classes = ClassBuilder::new()
            .scale(Scale::Zero)
            .scale(Scale::Fifty)
            .scale(Scale::SeventyFive)
            .scale(Scale::Ninety)
            .scale(Scale::NinetyFive)
            .scale(Scale::Hundred)
            .scale(Scale::HundredFive)
            .scale(Scale::HundredTen)
            .scale(Scale::HundredTwentyFive)
            .scale(Scale::HundredFifty)
            .scale_x(Scale::Hundred)
            .scale_y(Scale::Hundred)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("scale-0"));
        assert!(css_classes.contains("scale-50"));
        assert!(css_classes.contains("scale-75"));
        assert!(css_classes.contains("scale-90"));
        assert!(css_classes.contains("scale-95"));
        assert!(css_classes.contains("scale-100"));
        assert!(css_classes.contains("scale-105"));
        assert!(css_classes.contains("scale-110"));
        assert!(css_classes.contains("scale-125"));
        assert!(css_classes.contains("scale-150"));
        assert!(css_classes.contains("scale-x-100"));
        assert!(css_classes.contains("scale-y-100"));
    }
    
    #[test]
    fn test_rotate_utilities() {
        let classes = ClassBuilder::new()
            .rotate(Rotate::Zero)
            .rotate(Rotate::One)
            .rotate(Rotate::Two)
            .rotate(Rotate::Three)
            .rotate(Rotate::Six)
            .rotate(Rotate::Twelve)
            .rotate(Rotate::FortyFive)
            .rotate(Rotate::Ninety)
            .rotate(Rotate::HundredEighty)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("rotate-0"));
        assert!(css_classes.contains("rotate-1"));
        assert!(css_classes.contains("rotate-2"));
        assert!(css_classes.contains("rotate-3"));
        assert!(css_classes.contains("rotate-6"));
        assert!(css_classes.contains("rotate-12"));
        assert!(css_classes.contains("rotate-45"));
        assert!(css_classes.contains("rotate-90"));
        assert!(css_classes.contains("rotate-180"));
    }
    
    #[test]
    fn test_translate_utilities() {
        let classes = ClassBuilder::new()
            .translate_x(Translate::Zero)
            .translate_x(Translate::One)
            .translate_x(Translate::Two)
            .translate_x(Translate::Three)
            .translate_x(Translate::Four)
            .translate_x(Translate::Five)
            .translate_x(Translate::Six)
            .translate_x(Translate::Seven)
            .translate_x(Translate::Eight)
            .translate_x(Translate::Nine)
            .translate_x(Translate::Ten)
            .translate_y(Translate::Zero)
            .translate_y(Translate::One)
            .translate_y(Translate::Two)
            .translate_y(Translate::Three)
            .translate_y(Translate::Four)
            .translate_y(Translate::Five)
            .translate_y(Translate::Six)
            .translate_y(Translate::Seven)
            .translate_y(Translate::Eight)
            .translate_y(Translate::Nine)
            .translate_y(Translate::Ten)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("translate-x-0"));
        assert!(css_classes.contains("translate-x-1"));
        assert!(css_classes.contains("translate-x-2"));
        assert!(css_classes.contains("translate-x-3"));
        assert!(css_classes.contains("translate-x-4"));
        assert!(css_classes.contains("translate-x-5"));
        assert!(css_classes.contains("translate-x-6"));
        assert!(css_classes.contains("translate-x-7"));
        assert!(css_classes.contains("translate-x-8"));
        assert!(css_classes.contains("translate-x-9"));
        assert!(css_classes.contains("translate-x-10"));
        assert!(css_classes.contains("translate-y-0"));
        assert!(css_classes.contains("translate-y-1"));
        assert!(css_classes.contains("translate-y-2"));
        assert!(css_classes.contains("translate-y-3"));
        assert!(css_classes.contains("translate-y-4"));
        assert!(css_classes.contains("translate-y-5"));
        assert!(css_classes.contains("translate-y-6"));
        assert!(css_classes.contains("translate-y-7"));
        assert!(css_classes.contains("translate-y-8"));
        assert!(css_classes.contains("translate-y-9"));
        assert!(css_classes.contains("translate-y-10"));
    }
    
    #[test]
    fn test_skew_utilities() {
        let classes = ClassBuilder::new()
            .skew_x(Skew::Zero)
            .skew_x(Skew::One)
            .skew_x(Skew::Two)
            .skew_x(Skew::Three)
            .skew_x(Skew::Six)
            .skew_x(Skew::Twelve)
            .skew_y(Skew::Zero)
            .skew_y(Skew::One)
            .skew_y(Skew::Two)
            .skew_y(Skew::Three)
            .skew_y(Skew::Six)
            .skew_y(Skew::Twelve)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("skew-x-0"));
        assert!(css_classes.contains("skew-x-1"));
        assert!(css_classes.contains("skew-x-2"));
        assert!(css_classes.contains("skew-x-3"));
        assert!(css_classes.contains("skew-x-6"));
        assert!(css_classes.contains("skew-x-12"));
        assert!(css_classes.contains("skew-y-0"));
        assert!(css_classes.contains("skew-y-1"));
        assert!(css_classes.contains("skew-y-2"));
        assert!(css_classes.contains("skew-y-3"));
        assert!(css_classes.contains("skew-y-6"));
        assert!(css_classes.contains("skew-y-12"));
    }
    
    #[test]
    fn test_transform_origin_utilities() {
        let classes = ClassBuilder::new()
            .transform_origin(TransformOrigin::Center)
            .transform_origin(TransformOrigin::Top)
            .transform_origin(TransformOrigin::TopRight)
            .transform_origin(TransformOrigin::Right)
            .transform_origin(TransformOrigin::BottomRight)
            .transform_origin(TransformOrigin::Bottom)
            .transform_origin(TransformOrigin::BottomLeft)
            .transform_origin(TransformOrigin::Left)
            .transform_origin(TransformOrigin::TopLeft)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("origin-center"));
        assert!(css_classes.contains("origin-top"));
        assert!(css_classes.contains("origin-top-right"));
        assert!(css_classes.contains("origin-right"));
        assert!(css_classes.contains("origin-bottom-right"));
        assert!(css_classes.contains("origin-bottom"));
        assert!(css_classes.contains("origin-bottom-left"));
        assert!(css_classes.contains("origin-left"));
        assert!(css_classes.contains("origin-top-left"));
    }
    
    #[test]
    fn test_complex_transforms_combination() {
        let classes = ClassBuilder::new()
            .scale(Scale::HundredTen)
            .scale_x(Scale::HundredFive)
            .scale_y(Scale::HundredFive)
            .rotate(Rotate::FortyFive)
            .translate_x(Translate::Four)
            .translate_y(Translate::Four)
            .skew_x(Skew::Two)
            .skew_y(Skew::Two)
            .transform_origin(TransformOrigin::Center)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("scale-110"));
        assert!(css_classes.contains("scale-x-105"));
        assert!(css_classes.contains("scale-y-105"));
        assert!(css_classes.contains("rotate-45"));
        assert!(css_classes.contains("translate-x-4"));
        assert!(css_classes.contains("translate-y-4"));
        assert!(css_classes.contains("skew-x-2"));
        assert!(css_classes.contains("skew-y-2"));
        assert!(css_classes.contains("origin-center"));
    }
    
    /// Test that all Week 11 transform utilities are implemented
    #[test]
    fn test_week11_transform_utilities() {
        // Test all Week 11 transform utilities
        let classes = ClassBuilder::new()
            // Scale
            .scale(Scale::Zero)
            .scale(Scale::Fifty)
            .scale(Scale::SeventyFive)
            .scale(Scale::Ninety)
            .scale(Scale::NinetyFive)
            .scale(Scale::Hundred)
            .scale(Scale::HundredFive)
            .scale(Scale::HundredTen)
            .scale(Scale::HundredTwentyFive)
            .scale(Scale::HundredFifty)
            .scale_x(Scale::Hundred)
            .scale_y(Scale::Hundred)
            // Rotate
            .rotate(Rotate::Zero)
            .rotate(Rotate::One)
            .rotate(Rotate::Two)
            .rotate(Rotate::Three)
            .rotate(Rotate::Six)
            .rotate(Rotate::Twelve)
            .rotate(Rotate::FortyFive)
            .rotate(Rotate::Ninety)
            .rotate(Rotate::HundredEighty)
            // Translate
            .translate_x(Translate::Zero)
            .translate_x(Translate::Four)
            .translate_y(Translate::Two)
            .translate_y(Translate::Eight)
            .build();
        
        let css_classes = classes.to_css_classes();
        
        // Scale
        assert!(css_classes.contains("scale-0"));
        assert!(css_classes.contains("scale-50"));
        assert!(css_classes.contains("scale-75"));
        assert!(css_classes.contains("scale-90"));
        assert!(css_classes.contains("scale-95"));
        assert!(css_classes.contains("scale-100"));
        assert!(css_classes.contains("scale-105"));
        assert!(css_classes.contains("scale-110"));
        assert!(css_classes.contains("scale-125"));
        assert!(css_classes.contains("scale-150"));
        assert!(css_classes.contains("scale-x-100"));
        assert!(css_classes.contains("scale-y-100"));
        
        // Rotate
        assert!(css_classes.contains("rotate-0"));
        assert!(css_classes.contains("rotate-1"));
        assert!(css_classes.contains("rotate-2"));
        assert!(css_classes.contains("rotate-3"));
        assert!(css_classes.contains("rotate-6"));
        assert!(css_classes.contains("rotate-12"));
        assert!(css_classes.contains("rotate-45"));
        assert!(css_classes.contains("rotate-90"));
        assert!(css_classes.contains("rotate-180"));
        
        // Translate
        assert!(css_classes.contains("translate-x-0"));
        assert!(css_classes.contains("translate-x-4"));
        assert!(css_classes.contains("translate-y-2"));
        assert!(css_classes.contains("translate-y-8"));
    }
}
