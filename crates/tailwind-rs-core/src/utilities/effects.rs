//! Effects utilities for tailwind-rs
//!
//! This module provides utilities for box shadow, drop shadow, opacity, mix blend mode,
//! background blend mode, and other visual effects.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Box shadow values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BoxShadow {
    /// No shadow
    None,
    /// Small shadow
    Sm,
    /// Default shadow
    Default,
    /// Medium shadow
    Md,
    /// Large shadow
    Lg,
    /// Extra large shadow
    Xl,
    /// 2x large shadow
    Xl2,
    /// Inner shadow
    Inner,
}

/// Drop shadow values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DropShadow {
    /// No shadow
    None,
    /// Small shadow
    Sm,
    /// Default shadow
    Default,
    /// Medium shadow
    Md,
    /// Large shadow
    Lg,
    /// Extra large shadow
    Xl,
    /// 2x large shadow
    Xl2,
    /// 3x large shadow
    Xl3,
    // Colored drop shadows
    /// Red drop shadow
    Red,
    /// Blue drop shadow
    Blue,
    /// Green drop shadow
    Green,
    /// Yellow drop shadow
    Yellow,
    /// Purple drop shadow
    Purple,
    /// Pink drop shadow
    Pink,
    /// Orange drop shadow
    Orange,
    /// Indigo drop shadow
    Indigo,
    /// Cyan drop shadow
    Cyan,
    /// Teal drop shadow
    Teal,
    /// Lime drop shadow
    Lime,
    /// Emerald drop shadow
    Emerald,
    /// Rose drop shadow
    Rose,
    /// Violet drop shadow
    Violet,
    /// Fuchsia drop shadow
    Fuchsia,
    /// Sky drop shadow
    Sky,
    /// Amber drop shadow
    Amber,
    /// Stone drop shadow
    Stone,
    /// Neutral drop shadow
    Neutral,
    /// Zinc drop shadow
    Zinc,
    /// Gray drop shadow
    Gray,
    /// Slate drop shadow
    Slate,
}

/// Opacity values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Opacity {
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
    /// 100% opacity
    Hundred,
}

/// Mix blend mode values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MixBlendMode {
    /// Normal blend
    Normal,
    /// Multiply blend
    Multiply,
    /// Screen blend
    Screen,
    /// Overlay blend
    Overlay,
    /// Darken blend
    Darken,
    /// Lighten blend
    Lighten,
    /// Color dodge blend
    ColorDodge,
    /// Color burn blend
    ColorBurn,
    /// Hard light blend
    HardLight,
    /// Soft light blend
    SoftLight,
    /// Difference blend
    Difference,
    /// Exclusion blend
    Exclusion,
    /// Hue blend
    Hue,
    /// Saturation blend
    Saturation,
    /// Color blend
    Color,
    /// Luminosity blend
    Luminosity,
}

/// Background blend mode values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackgroundBlendMode {
    /// Normal blend
    Normal,
    /// Multiply blend
    Multiply,
    /// Screen blend
    Screen,
    /// Overlay blend
    Overlay,
    /// Darken blend
    Darken,
    /// Lighten blend
    Lighten,
    /// Color dodge blend
    ColorDodge,
    /// Color burn blend
    ColorBurn,
    /// Hard light blend
    HardLight,
    /// Soft light blend
    SoftLight,
    /// Difference blend
    Difference,
    /// Exclusion blend
    Exclusion,
    /// Hue blend
    Hue,
    /// Saturation blend
    Saturation,
    /// Color blend
    Color,
    /// Luminosity blend
    Luminosity,
}

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
    OneHundred,
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

impl BoxShadow {
    pub fn to_class_name(&self) -> String {
        match self {
            BoxShadow::None => "none".to_string(),
            BoxShadow::Sm => "sm".to_string(),
            BoxShadow::Default => "default".to_string(),
            BoxShadow::Md => "md".to_string(),
            BoxShadow::Lg => "lg".to_string(),
            BoxShadow::Xl => "xl".to_string(),
            BoxShadow::Xl2 => "2xl".to_string(),
            BoxShadow::Inner => "inner".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            BoxShadow::None => "none".to_string(),
            BoxShadow::Sm => "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string(),
            BoxShadow::Default => "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)".to_string(),
            BoxShadow::Md => "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)".to_string(),
            BoxShadow::Lg => "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)".to_string(),
            BoxShadow::Xl => "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)".to_string(),
            BoxShadow::Xl2 => "0 25px 50px -12px rgb(0 0 0 / 0.25)".to_string(),
            BoxShadow::Inner => "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)".to_string(),
        }
    }
}

impl DropShadow {
    pub fn to_class_name(&self) -> String {
        match self {
            DropShadow::None => "none".to_string(),
            DropShadow::Sm => "sm".to_string(),
            DropShadow::Default => "default".to_string(),
            DropShadow::Md => "md".to_string(),
            DropShadow::Lg => "lg".to_string(),
            DropShadow::Xl => "xl".to_string(),
            DropShadow::Xl2 => "2xl".to_string(),
            DropShadow::Xl3 => "3xl".to_string(),
            // Colored drop shadows
            DropShadow::Red => "red-500".to_string(),
            DropShadow::Blue => "blue-500".to_string(),
            DropShadow::Green => "green-500".to_string(),
            DropShadow::Yellow => "yellow-500".to_string(),
            DropShadow::Purple => "purple-500".to_string(),
            DropShadow::Pink => "pink-500".to_string(),
            DropShadow::Orange => "orange-500".to_string(),
            DropShadow::Indigo => "indigo-500".to_string(),
            DropShadow::Cyan => "cyan-500".to_string(),
            DropShadow::Teal => "teal-500".to_string(),
            DropShadow::Lime => "lime-500".to_string(),
            DropShadow::Emerald => "emerald-500".to_string(),
            DropShadow::Rose => "rose-500".to_string(),
            DropShadow::Violet => "violet-500".to_string(),
            DropShadow::Fuchsia => "fuchsia-500".to_string(),
            DropShadow::Sky => "sky-500".to_string(),
            DropShadow::Amber => "amber-500".to_string(),
            DropShadow::Stone => "stone-500".to_string(),
            DropShadow::Neutral => "neutral-500".to_string(),
            DropShadow::Zinc => "zinc-500".to_string(),
            DropShadow::Gray => "gray-500".to_string(),
            DropShadow::Slate => "slate-500".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            DropShadow::None => "none".to_string(),
            DropShadow::Sm => "0 1px 2px rgb(0 0 0 / 0.05)".to_string(),
            DropShadow::Default => "0 1px 3px rgb(0 0 0 / 0.1), 0 1px 2px rgb(0 0 0 / 0.06)".to_string(),
            DropShadow::Md => "0 4px 6px rgb(0 0 0 / 0.07), 0 2px 4px rgb(0 0 0 / 0.06)".to_string(),
            DropShadow::Lg => "0 10px 15px rgb(0 0 0 / 0.1), 0 4px 6px rgb(0 0 0 / 0.05)".to_string(),
            DropShadow::Xl => "0 20px 25px rgb(0 0 0 / 0.1), 0 8px 10px rgb(0 0 0 / 0.04)".to_string(),
            DropShadow::Xl2 => "0 25px 50px rgb(0 0 0 / 0.25)".to_string(),
            DropShadow::Xl3 => "0 35px 60px rgb(0 0 0 / 0.3)".to_string(),
            // Colored drop shadows
            DropShadow::Red => "0 1px 2px rgb(239 68 68)".to_string(),
            DropShadow::Blue => "0 1px 2px rgb(59 130 246)".to_string(),
            DropShadow::Green => "0 1px 2px rgb(34 197 94)".to_string(),
            DropShadow::Yellow => "0 1px 2px rgb(234 179 8)".to_string(),
            DropShadow::Purple => "0 1px 2px rgb(168 85 247)".to_string(),
            DropShadow::Pink => "0 1px 2px rgb(236 72 153)".to_string(),
            DropShadow::Orange => "0 1px 2px rgb(249 115 22)".to_string(),
            DropShadow::Indigo => "0 1px 2px rgb(99 102 241)".to_string(),
            DropShadow::Cyan => "0 1px 2px rgb(6 182 212)".to_string(),
            DropShadow::Teal => "0 1px 2px rgb(20 184 166)".to_string(),
            DropShadow::Lime => "0 1px 2px rgb(132 204 22)".to_string(),
            DropShadow::Emerald => "0 1px 2px rgb(16 185 129)".to_string(),
            DropShadow::Rose => "0 1px 2px rgb(244 63 94)".to_string(),
            DropShadow::Violet => "0 1px 2px rgb(139 92 246)".to_string(),
            DropShadow::Fuchsia => "0 1px 2px rgb(217 70 239)".to_string(),
            DropShadow::Sky => "0 1px 2px rgb(14 165 233)".to_string(),
            DropShadow::Amber => "0 1px 2px rgb(245 158 11)".to_string(),
            DropShadow::Stone => "0 1px 2px rgb(120 113 108)".to_string(),
            DropShadow::Neutral => "0 1px 2px rgb(115 115 115)".to_string(),
            DropShadow::Zinc => "0 1px 2px rgb(113 113 122)".to_string(),
            DropShadow::Gray => "0 1px 2px rgb(107 114 128)".to_string(),
            DropShadow::Slate => "0 1px 2px rgb(100 116 139)".to_string(),
        }
    }
}

impl Opacity {
    pub fn to_class_name(&self) -> String {
        match self {
            Opacity::Zero => "0".to_string(),
            Opacity::Five => "5".to_string(),
            Opacity::Ten => "10".to_string(),
            Opacity::Twenty => "20".to_string(),
            Opacity::TwentyFive => "25".to_string(),
            Opacity::Thirty => "30".to_string(),
            Opacity::Forty => "40".to_string(),
            Opacity::Fifty => "50".to_string(),
            Opacity::Sixty => "60".to_string(),
            Opacity::Seventy => "70".to_string(),
            Opacity::SeventyFive => "75".to_string(),
            Opacity::Eighty => "80".to_string(),
            Opacity::Ninety => "90".to_string(),
            Opacity::NinetyFive => "95".to_string(),
            Opacity::Hundred => "100".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Opacity::Zero => "0".to_string(),
            Opacity::Five => "0.05".to_string(),
            Opacity::Ten => "0.1".to_string(),
            Opacity::Twenty => "0.2".to_string(),
            Opacity::TwentyFive => "0.25".to_string(),
            Opacity::Thirty => "0.3".to_string(),
            Opacity::Forty => "0.4".to_string(),
            Opacity::Fifty => "0.5".to_string(),
            Opacity::Sixty => "0.6".to_string(),
            Opacity::Seventy => "0.7".to_string(),
            Opacity::SeventyFive => "0.75".to_string(),
            Opacity::Eighty => "0.8".to_string(),
            Opacity::Ninety => "0.9".to_string(),
            Opacity::NinetyFive => "0.95".to_string(),
            Opacity::Hundred => "1".to_string(),
        }
    }
}

impl MixBlendMode {
    pub fn to_class_name(&self) -> String {
        match self {
            MixBlendMode::Normal => "normal".to_string(),
            MixBlendMode::Multiply => "multiply".to_string(),
            MixBlendMode::Screen => "screen".to_string(),
            MixBlendMode::Overlay => "overlay".to_string(),
            MixBlendMode::Darken => "darken".to_string(),
            MixBlendMode::Lighten => "lighten".to_string(),
            MixBlendMode::ColorDodge => "color-dodge".to_string(),
            MixBlendMode::ColorBurn => "color-burn".to_string(),
            MixBlendMode::HardLight => "hard-light".to_string(),
            MixBlendMode::SoftLight => "soft-light".to_string(),
            MixBlendMode::Difference => "difference".to_string(),
            MixBlendMode::Exclusion => "exclusion".to_string(),
            MixBlendMode::Hue => "hue".to_string(),
            MixBlendMode::Saturation => "saturation".to_string(),
            MixBlendMode::Color => "color".to_string(),
            MixBlendMode::Luminosity => "luminosity".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            MixBlendMode::Normal => "normal".to_string(),
            MixBlendMode::Multiply => "multiply".to_string(),
            MixBlendMode::Screen => "screen".to_string(),
            MixBlendMode::Overlay => "overlay".to_string(),
            MixBlendMode::Darken => "darken".to_string(),
            MixBlendMode::Lighten => "lighten".to_string(),
            MixBlendMode::ColorDodge => "color-dodge".to_string(),
            MixBlendMode::ColorBurn => "color-burn".to_string(),
            MixBlendMode::HardLight => "hard-light".to_string(),
            MixBlendMode::SoftLight => "soft-light".to_string(),
            MixBlendMode::Difference => "difference".to_string(),
            MixBlendMode::Exclusion => "exclusion".to_string(),
            MixBlendMode::Hue => "hue".to_string(),
            MixBlendMode::Saturation => "saturation".to_string(),
            MixBlendMode::Color => "color".to_string(),
            MixBlendMode::Luminosity => "luminosity".to_string(),
        }
    }
}

impl BackgroundBlendMode {
    pub fn to_class_name(&self) -> String {
        match self {
            BackgroundBlendMode::Normal => "normal".to_string(),
            BackgroundBlendMode::Multiply => "multiply".to_string(),
            BackgroundBlendMode::Screen => "screen".to_string(),
            BackgroundBlendMode::Overlay => "overlay".to_string(),
            BackgroundBlendMode::Darken => "darken".to_string(),
            BackgroundBlendMode::Lighten => "lighten".to_string(),
            BackgroundBlendMode::ColorDodge => "color-dodge".to_string(),
            BackgroundBlendMode::ColorBurn => "color-burn".to_string(),
            BackgroundBlendMode::HardLight => "hard-light".to_string(),
            BackgroundBlendMode::SoftLight => "soft-light".to_string(),
            BackgroundBlendMode::Difference => "difference".to_string(),
            BackgroundBlendMode::Exclusion => "exclusion".to_string(),
            BackgroundBlendMode::Hue => "hue".to_string(),
            BackgroundBlendMode::Saturation => "saturation".to_string(),
            BackgroundBlendMode::Color => "color".to_string(),
            BackgroundBlendMode::Luminosity => "luminosity".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            BackgroundBlendMode::Normal => "normal".to_string(),
            BackgroundBlendMode::Multiply => "multiply".to_string(),
            BackgroundBlendMode::Screen => "screen".to_string(),
            BackgroundBlendMode::Overlay => "overlay".to_string(),
            BackgroundBlendMode::Darken => "darken".to_string(),
            BackgroundBlendMode::Lighten => "lighten".to_string(),
            BackgroundBlendMode::ColorDodge => "color-dodge".to_string(),
            BackgroundBlendMode::ColorBurn => "color-burn".to_string(),
            BackgroundBlendMode::HardLight => "hard-light".to_string(),
            BackgroundBlendMode::SoftLight => "soft-light".to_string(),
            BackgroundBlendMode::Difference => "difference".to_string(),
            BackgroundBlendMode::Exclusion => "exclusion".to_string(),
            BackgroundBlendMode::Hue => "hue".to_string(),
            BackgroundBlendMode::Saturation => "saturation".to_string(),
            BackgroundBlendMode::Color => "color".to_string(),
            BackgroundBlendMode::Luminosity => "luminosity".to_string(),
        }
    }
}

impl BackdropBlur {
    pub fn to_class_name(&self) -> String {
        match self {
            BackdropBlur::None => "backdrop-blur-none".to_string(),
            BackdropBlur::Sm => "backdrop-blur-sm".to_string(),
            BackdropBlur::Default => "backdrop-blur".to_string(),
            BackdropBlur::Md => "backdrop-blur-md".to_string(),
            BackdropBlur::Lg => "backdrop-blur-lg".to_string(),
            BackdropBlur::Xl => "backdrop-blur-xl".to_string(),
            BackdropBlur::Xl2 => "backdrop-blur-2xl".to_string(),
            BackdropBlur::Xl3 => "backdrop-blur-3xl".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            BackdropBlur::None => "blur(0px)".to_string(),
            BackdropBlur::Sm => "blur(4px)".to_string(),
            BackdropBlur::Default => "blur(8px)".to_string(),
            BackdropBlur::Md => "blur(12px)".to_string(),
            BackdropBlur::Lg => "blur(16px)".to_string(),
            BackdropBlur::Xl => "blur(24px)".to_string(),
            BackdropBlur::Xl2 => "blur(40px)".to_string(),
            BackdropBlur::Xl3 => "blur(64px)".to_string(),
        }
    }
}

impl BackdropBrightness {
    pub fn to_class_name(&self) -> String {
        match self {
            BackdropBrightness::Zero => "backdrop-brightness-0".to_string(),
            BackdropBrightness::Fifty => "backdrop-brightness-50".to_string(),
            BackdropBrightness::SeventyFive => "backdrop-brightness-75".to_string(),
            BackdropBrightness::Ninety => "backdrop-brightness-90".to_string(),
            BackdropBrightness::NinetyFive => "backdrop-brightness-95".to_string(),
            BackdropBrightness::OneHundred => "backdrop-brightness-100".to_string(),
            BackdropBrightness::OneOhFive => "backdrop-brightness-105".to_string(),
            BackdropBrightness::OneOneZero => "backdrop-brightness-110".to_string(),
            BackdropBrightness::OneTwoFive => "backdrop-brightness-125".to_string(),
            BackdropBrightness::OneFifty => "backdrop-brightness-150".to_string(),
            BackdropBrightness::TwoHundred => "backdrop-brightness-200".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            BackdropBrightness::Zero => "brightness(0)".to_string(),
            BackdropBrightness::Fifty => "brightness(0.5)".to_string(),
            BackdropBrightness::SeventyFive => "brightness(0.75)".to_string(),
            BackdropBrightness::Ninety => "brightness(0.9)".to_string(),
            BackdropBrightness::NinetyFive => "brightness(0.95)".to_string(),
            BackdropBrightness::OneHundred => "brightness(1)".to_string(),
            BackdropBrightness::OneOhFive => "brightness(1.05)".to_string(),
            BackdropBrightness::OneOneZero => "brightness(1.1)".to_string(),
            BackdropBrightness::OneTwoFive => "brightness(1.25)".to_string(),
            BackdropBrightness::OneFifty => "brightness(1.5)".to_string(),
            BackdropBrightness::TwoHundred => "brightness(2)".to_string(),
        }
    }
}

impl BackdropContrast {
    pub fn to_class_name(&self) -> String {
        match self {
            BackdropContrast::Zero => "backdrop-contrast-0".to_string(),
            BackdropContrast::Fifty => "backdrop-contrast-50".to_string(),
            BackdropContrast::SeventyFive => "backdrop-contrast-75".to_string(),
            BackdropContrast::OneHundred => "backdrop-contrast-100".to_string(),
            BackdropContrast::OneTwoFive => "backdrop-contrast-125".to_string(),
            BackdropContrast::OneFifty => "backdrop-contrast-150".to_string(),
            BackdropContrast::TwoHundred => "backdrop-contrast-200".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            BackdropContrast::Zero => "contrast(0)".to_string(),
            BackdropContrast::Fifty => "contrast(0.5)".to_string(),
            BackdropContrast::SeventyFive => "contrast(0.75)".to_string(),
            BackdropContrast::OneHundred => "contrast(1)".to_string(),
            BackdropContrast::OneTwoFive => "contrast(1.25)".to_string(),
            BackdropContrast::OneFifty => "contrast(1.5)".to_string(),
            BackdropContrast::TwoHundred => "contrast(2)".to_string(),
        }
    }
}

impl BackdropGrayscale {
    pub fn to_class_name(&self) -> String {
        match self {
            BackdropGrayscale::Zero => "backdrop-grayscale-0".to_string(),
            BackdropGrayscale::Default => "backdrop-grayscale".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            BackdropGrayscale::Zero => "grayscale(0)".to_string(),
            BackdropGrayscale::Default => "grayscale(1)".to_string(),
        }
    }
}

impl BackdropHueRotate {
    pub fn to_class_name(&self) -> String {
        match self {
            BackdropHueRotate::Zero => "backdrop-hue-rotate-0".to_string(),
            BackdropHueRotate::Fifteen => "backdrop-hue-rotate-15".to_string(),
            BackdropHueRotate::Thirty => "backdrop-hue-rotate-30".to_string(),
            BackdropHueRotate::Sixty => "backdrop-hue-rotate-60".to_string(),
            BackdropHueRotate::Ninety => "backdrop-hue-rotate-90".to_string(),
            BackdropHueRotate::OneEighty => "backdrop-hue-rotate-180".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            BackdropHueRotate::Zero => "hue-rotate(0deg)".to_string(),
            BackdropHueRotate::Fifteen => "hue-rotate(15deg)".to_string(),
            BackdropHueRotate::Thirty => "hue-rotate(30deg)".to_string(),
            BackdropHueRotate::Sixty => "hue-rotate(60deg)".to_string(),
            BackdropHueRotate::Ninety => "hue-rotate(90deg)".to_string(),
            BackdropHueRotate::OneEighty => "hue-rotate(180deg)".to_string(),
        }
    }
}

impl BackdropInvert {
    pub fn to_class_name(&self) -> String {
        match self {
            BackdropInvert::Zero => "backdrop-invert-0".to_string(),
            BackdropInvert::Default => "backdrop-invert".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            BackdropInvert::Zero => "invert(0)".to_string(),
            BackdropInvert::Default => "invert(1)".to_string(),
        }
    }
}

impl BackdropOpacity {
    pub fn to_class_name(&self) -> String {
        match self {
            BackdropOpacity::Zero => "backdrop-opacity-0".to_string(),
            BackdropOpacity::Five => "backdrop-opacity-5".to_string(),
            BackdropOpacity::Ten => "backdrop-opacity-10".to_string(),
            BackdropOpacity::Twenty => "backdrop-opacity-20".to_string(),
            BackdropOpacity::TwentyFive => "backdrop-opacity-25".to_string(),
            BackdropOpacity::Thirty => "backdrop-opacity-30".to_string(),
            BackdropOpacity::Forty => "backdrop-opacity-40".to_string(),
            BackdropOpacity::Fifty => "backdrop-opacity-50".to_string(),
            BackdropOpacity::Sixty => "backdrop-opacity-60".to_string(),
            BackdropOpacity::Seventy => "backdrop-opacity-70".to_string(),
            BackdropOpacity::SeventyFive => "backdrop-opacity-75".to_string(),
            BackdropOpacity::Eighty => "backdrop-opacity-80".to_string(),
            BackdropOpacity::Ninety => "backdrop-opacity-90".to_string(),
            BackdropOpacity::NinetyFive => "backdrop-opacity-95".to_string(),
            BackdropOpacity::OneHundred => "backdrop-opacity-100".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            BackdropOpacity::Zero => "opacity(0)".to_string(),
            BackdropOpacity::Five => "opacity(0.05)".to_string(),
            BackdropOpacity::Ten => "opacity(0.1)".to_string(),
            BackdropOpacity::Twenty => "opacity(0.2)".to_string(),
            BackdropOpacity::TwentyFive => "opacity(0.25)".to_string(),
            BackdropOpacity::Thirty => "opacity(0.3)".to_string(),
            BackdropOpacity::Forty => "opacity(0.4)".to_string(),
            BackdropOpacity::Fifty => "opacity(0.5)".to_string(),
            BackdropOpacity::Sixty => "opacity(0.6)".to_string(),
            BackdropOpacity::Seventy => "opacity(0.7)".to_string(),
            BackdropOpacity::SeventyFive => "opacity(0.75)".to_string(),
            BackdropOpacity::Eighty => "opacity(0.8)".to_string(),
            BackdropOpacity::Ninety => "opacity(0.9)".to_string(),
            BackdropOpacity::NinetyFive => "opacity(0.95)".to_string(),
            BackdropOpacity::OneHundred => "opacity(1)".to_string(),
        }
    }
}

impl BackdropSaturate {
    pub fn to_class_name(&self) -> String {
        match self {
            BackdropSaturate::Zero => "backdrop-saturate-0".to_string(),
            BackdropSaturate::Fifty => "backdrop-saturate-50".to_string(),
            BackdropSaturate::OneHundred => "backdrop-saturate-100".to_string(),
            BackdropSaturate::OneFifty => "backdrop-saturate-150".to_string(),
            BackdropSaturate::TwoHundred => "backdrop-saturate-200".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            BackdropSaturate::Zero => "saturate(0)".to_string(),
            BackdropSaturate::Fifty => "saturate(0.5)".to_string(),
            BackdropSaturate::OneHundred => "saturate(1)".to_string(),
            BackdropSaturate::OneFifty => "saturate(1.5)".to_string(),
            BackdropSaturate::TwoHundred => "saturate(2)".to_string(),
        }
    }
}

impl BackdropSepia {
    pub fn to_class_name(&self) -> String {
        match self {
            BackdropSepia::Zero => "backdrop-sepia-0".to_string(),
            BackdropSepia::Default => "backdrop-sepia".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            BackdropSepia::Zero => "sepia(0)".to_string(),
            BackdropSepia::Default => "sepia(1)".to_string(),
        }
    }
}

impl fmt::Display for BoxShadow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for DropShadow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for Opacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for MixBlendMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackgroundBlendMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackdropBlur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackdropBrightness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackdropContrast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackdropGrayscale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackdropHueRotate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackdropInvert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackdropOpacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackdropSaturate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackdropSepia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Trait for adding box shadow utilities to a class builder
pub trait BoxShadowUtilities {
    fn box_shadow(self, shadow: BoxShadow) -> Self;
}

impl BoxShadowUtilities for ClassBuilder {
    fn box_shadow(self, shadow: BoxShadow) -> Self {
        self.class(format!("shadow-{}", shadow.to_class_name()))
    }
}

/// Trait for adding drop shadow utilities to a class builder
pub trait DropShadowUtilities {
    fn drop_shadow(self, shadow: DropShadow) -> Self;
    
    // Convenience methods for colored drop shadows
    fn drop_shadow_red(self) -> Self;
    fn drop_shadow_blue(self) -> Self;
    fn drop_shadow_green(self) -> Self;
    fn drop_shadow_yellow(self) -> Self;
    fn drop_shadow_purple(self) -> Self;
    fn drop_shadow_pink(self) -> Self;
    fn drop_shadow_orange(self) -> Self;
    fn drop_shadow_indigo(self) -> Self;
    fn drop_shadow_cyan(self) -> Self;
    fn drop_shadow_teal(self) -> Self;
    fn drop_shadow_lime(self) -> Self;
    fn drop_shadow_emerald(self) -> Self;
    fn drop_shadow_rose(self) -> Self;
    fn drop_shadow_violet(self) -> Self;
    fn drop_shadow_fuchsia(self) -> Self;
    fn drop_shadow_sky(self) -> Self;
    fn drop_shadow_amber(self) -> Self;
    fn drop_shadow_stone(self) -> Self;
    fn drop_shadow_neutral(self) -> Self;
    fn drop_shadow_zinc(self) -> Self;
    fn drop_shadow_gray(self) -> Self;
    fn drop_shadow_slate(self) -> Self;
}

impl DropShadowUtilities for ClassBuilder {
    fn drop_shadow(self, shadow: DropShadow) -> Self {
        self.class(format!("drop-shadow-{}", shadow.to_class_name()))
    }
    
    // Convenience methods for colored drop shadows
    fn drop_shadow_red(self) -> Self {
        self.drop_shadow(DropShadow::Red)
    }
    
    fn drop_shadow_blue(self) -> Self {
        self.drop_shadow(DropShadow::Blue)
    }
    
    fn drop_shadow_green(self) -> Self {
        self.drop_shadow(DropShadow::Green)
    }
    
    fn drop_shadow_yellow(self) -> Self {
        self.drop_shadow(DropShadow::Yellow)
    }
    
    fn drop_shadow_purple(self) -> Self {
        self.drop_shadow(DropShadow::Purple)
    }
    
    fn drop_shadow_pink(self) -> Self {
        self.drop_shadow(DropShadow::Pink)
    }
    
    fn drop_shadow_orange(self) -> Self {
        self.drop_shadow(DropShadow::Orange)
    }
    
    fn drop_shadow_indigo(self) -> Self {
        self.drop_shadow(DropShadow::Indigo)
    }
    
    fn drop_shadow_cyan(self) -> Self {
        self.drop_shadow(DropShadow::Cyan)
    }
    
    fn drop_shadow_teal(self) -> Self {
        self.drop_shadow(DropShadow::Teal)
    }
    
    fn drop_shadow_lime(self) -> Self {
        self.drop_shadow(DropShadow::Lime)
    }
    
    fn drop_shadow_emerald(self) -> Self {
        self.drop_shadow(DropShadow::Emerald)
    }
    
    fn drop_shadow_rose(self) -> Self {
        self.drop_shadow(DropShadow::Rose)
    }
    
    fn drop_shadow_violet(self) -> Self {
        self.drop_shadow(DropShadow::Violet)
    }
    
    fn drop_shadow_fuchsia(self) -> Self {
        self.drop_shadow(DropShadow::Fuchsia)
    }
    
    fn drop_shadow_sky(self) -> Self {
        self.drop_shadow(DropShadow::Sky)
    }
    
    fn drop_shadow_amber(self) -> Self {
        self.drop_shadow(DropShadow::Amber)
    }
    
    fn drop_shadow_stone(self) -> Self {
        self.drop_shadow(DropShadow::Stone)
    }
    
    fn drop_shadow_neutral(self) -> Self {
        self.drop_shadow(DropShadow::Neutral)
    }
    
    fn drop_shadow_zinc(self) -> Self {
        self.drop_shadow(DropShadow::Zinc)
    }
    
    fn drop_shadow_gray(self) -> Self {
        self.drop_shadow(DropShadow::Gray)
    }
    
    fn drop_shadow_slate(self) -> Self {
        self.drop_shadow(DropShadow::Slate)
    }
}

/// Trait for adding opacity utilities to a class builder
pub trait OpacityUtilities {
    fn opacity(self, opacity: Opacity) -> Self;
}

impl OpacityUtilities for ClassBuilder {
    fn opacity(self, opacity: Opacity) -> Self {
        self.class(format!("opacity-{}", opacity.to_class_name()))
    }
}

/// Trait for adding mix blend mode utilities to a class builder
pub trait MixBlendModeUtilities {
    fn mix_blend_mode(self, mode: MixBlendMode) -> Self;
}

impl MixBlendModeUtilities for ClassBuilder {
    fn mix_blend_mode(self, mode: MixBlendMode) -> Self {
        self.class(format!("mix-blend-{}", mode.to_class_name()))
    }
}

/// Trait for adding background blend mode utilities to a class builder
pub trait BackgroundBlendModeUtilities {
    fn background_blend_mode(self, mode: BackgroundBlendMode) -> Self;
}

impl BackgroundBlendModeUtilities for ClassBuilder {
    fn background_blend_mode(self, mode: BackgroundBlendMode) -> Self {
        self.class(format!("bg-blend-{}", mode.to_class_name()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_box_shadow_utilities() {
        let classes = ClassBuilder::new()
            .box_shadow(BoxShadow::None)
            .box_shadow(BoxShadow::Sm)
            .box_shadow(BoxShadow::Default)
            .box_shadow(BoxShadow::Md)
            .box_shadow(BoxShadow::Lg)
            .box_shadow(BoxShadow::Xl)
            .box_shadow(BoxShadow::Xl2)
            .box_shadow(BoxShadow::Inner)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("shadow-none"));
        assert!(css_classes.contains("shadow-sm"));
        assert!(css_classes.contains("shadow-default"));
        assert!(css_classes.contains("shadow-md"));
        assert!(css_classes.contains("shadow-lg"));
        assert!(css_classes.contains("shadow-xl"));
        assert!(css_classes.contains("shadow-2xl"));
        assert!(css_classes.contains("shadow-inner"));
    }
    
    #[test]
    fn test_drop_shadow_utilities() {
        let classes = ClassBuilder::new()
            .drop_shadow(DropShadow::None)
            .drop_shadow(DropShadow::Sm)
            .drop_shadow(DropShadow::Default)
            .drop_shadow(DropShadow::Md)
            .drop_shadow(DropShadow::Lg)
            .drop_shadow(DropShadow::Xl)
            .drop_shadow(DropShadow::Xl2)
            .drop_shadow(DropShadow::Xl3)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("drop-shadow-none"));
        assert!(css_classes.contains("drop-shadow-sm"));
        assert!(css_classes.contains("drop-shadow-default"));
        assert!(css_classes.contains("drop-shadow-md"));
        assert!(css_classes.contains("drop-shadow-lg"));
        assert!(css_classes.contains("drop-shadow-xl"));
        assert!(css_classes.contains("drop-shadow-2xl"));
        assert!(css_classes.contains("drop-shadow-3xl"));
    }
    
    #[test]
    fn test_opacity_utilities() {
        let classes = ClassBuilder::new()
            .opacity(Opacity::Zero)
            .opacity(Opacity::TwentyFive)
            .opacity(Opacity::Fifty)
            .opacity(Opacity::SeventyFive)
            .opacity(Opacity::Hundred)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("opacity-0"));
        assert!(css_classes.contains("opacity-25"));
        assert!(css_classes.contains("opacity-50"));
        assert!(css_classes.contains("opacity-75"));
        assert!(css_classes.contains("opacity-100"));
    }
    
    #[test]
    fn test_mix_blend_mode_utilities() {
        let classes = ClassBuilder::new()
            .mix_blend_mode(MixBlendMode::Normal)
            .mix_blend_mode(MixBlendMode::Multiply)
            .mix_blend_mode(MixBlendMode::Screen)
            .mix_blend_mode(MixBlendMode::Overlay)
            .mix_blend_mode(MixBlendMode::Difference)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("mix-blend-normal"));
        assert!(css_classes.contains("mix-blend-multiply"));
        assert!(css_classes.contains("mix-blend-screen"));
        assert!(css_classes.contains("mix-blend-overlay"));
        assert!(css_classes.contains("mix-blend-difference"));
    }
    
    #[test]
    fn test_background_blend_mode_utilities() {
        let classes = ClassBuilder::new()
            .background_blend_mode(BackgroundBlendMode::Normal)
            .background_blend_mode(BackgroundBlendMode::Multiply)
            .background_blend_mode(BackgroundBlendMode::Screen)
            .background_blend_mode(BackgroundBlendMode::Overlay)
            .background_blend_mode(BackgroundBlendMode::Difference)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-blend-normal"));
        assert!(css_classes.contains("bg-blend-multiply"));
        assert!(css_classes.contains("bg-blend-screen"));
        assert!(css_classes.contains("bg-blend-overlay"));
        assert!(css_classes.contains("bg-blend-difference"));
    }
    
    #[test]
    fn test_complex_effects_combination() {
        let classes = ClassBuilder::new()
            .box_shadow(BoxShadow::Lg)
            .drop_shadow(DropShadow::Md)
            .opacity(Opacity::Eighty)
            .mix_blend_mode(MixBlendMode::Multiply)
            .background_blend_mode(BackgroundBlendMode::Screen)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("shadow-lg"));
        assert!(css_classes.contains("drop-shadow-md"));
        assert!(css_classes.contains("opacity-80"));
        assert!(css_classes.contains("mix-blend-multiply"));
        assert!(css_classes.contains("bg-blend-screen"));
    }
    
    /// Test that all Week 9 shadow and opacity utilities are implemented
    #[test]
    fn test_week9_shadow_opacity_utilities() {
        // Test all Week 9 shadow and opacity utilities
        let classes = ClassBuilder::new()
            // Box Shadows
            .box_shadow(BoxShadow::Sm)
            .box_shadow(BoxShadow::Default)
            .box_shadow(BoxShadow::Md)
            .box_shadow(BoxShadow::Lg)
            .box_shadow(BoxShadow::Xl)
            .box_shadow(BoxShadow::Xl2)
            .box_shadow(BoxShadow::Inner)
            .box_shadow(BoxShadow::None)
            // Drop Shadows
            .drop_shadow(DropShadow::Sm)
            .drop_shadow(DropShadow::Default)
            .drop_shadow(DropShadow::Md)
            .drop_shadow(DropShadow::Lg)
            .drop_shadow(DropShadow::Xl)
            .drop_shadow(DropShadow::Xl2)
            .drop_shadow(DropShadow::None)
            // Opacity
            .opacity(Opacity::Zero)
            .opacity(Opacity::Five)
            .opacity(Opacity::Ten)
            .opacity(Opacity::Twenty)
            .opacity(Opacity::TwentyFive)
            .opacity(Opacity::Thirty)
            .opacity(Opacity::Forty)
            .opacity(Opacity::Fifty)
            .opacity(Opacity::Sixty)
            .opacity(Opacity::Seventy)
            .opacity(Opacity::SeventyFive)
            .opacity(Opacity::Eighty)
            .opacity(Opacity::Ninety)
            .opacity(Opacity::NinetyFive)
            .opacity(Opacity::Hundred)
            .build();
        
        let css_classes = classes.to_css_classes();
        
        // Box Shadows
        assert!(css_classes.contains("shadow-sm"));
        assert!(css_classes.contains("shadow"));
        assert!(css_classes.contains("shadow-md"));
        assert!(css_classes.contains("shadow-lg"));
        assert!(css_classes.contains("shadow-xl"));
        assert!(css_classes.contains("shadow-2xl"));
        assert!(css_classes.contains("shadow-inner"));
        assert!(css_classes.contains("shadow-none"));
        
        // Drop Shadows
        assert!(css_classes.contains("drop-shadow-sm"));
        assert!(css_classes.contains("drop-shadow"));
        assert!(css_classes.contains("drop-shadow-md"));
        assert!(css_classes.contains("drop-shadow-lg"));
        assert!(css_classes.contains("drop-shadow-xl"));
        assert!(css_classes.contains("drop-shadow-2xl"));
        assert!(css_classes.contains("drop-shadow-none"));
        
        // Opacity
        assert!(css_classes.contains("opacity-0"));
        assert!(css_classes.contains("opacity-5"));
        assert!(css_classes.contains("opacity-10"));
        assert!(css_classes.contains("opacity-20"));
        assert!(css_classes.contains("opacity-25"));
        assert!(css_classes.contains("opacity-30"));
        assert!(css_classes.contains("opacity-40"));
        assert!(css_classes.contains("opacity-50"));
        assert!(css_classes.contains("opacity-60"));
        assert!(css_classes.contains("opacity-70"));
        assert!(css_classes.contains("opacity-75"));
        assert!(css_classes.contains("opacity-80"));
        assert!(css_classes.contains("opacity-90"));
        assert!(css_classes.contains("opacity-95"));
        assert!(css_classes.contains("opacity-100"));
    }

    #[test]
    fn test_box_shadow_display() {
        // Test that BoxShadow displays correctly
        assert_eq!(format!("{}", BoxShadow::None), "none");
        assert_eq!(format!("{}", BoxShadow::Sm), "sm");
        assert_eq!(format!("{}", BoxShadow::Default), "default");
        assert_eq!(format!("{}", BoxShadow::Md), "md");
        assert_eq!(format!("{}", BoxShadow::Lg), "lg");
        assert_eq!(format!("{}", BoxShadow::Xl), "xl");
        assert_eq!(format!("{}", BoxShadow::Xl2), "2xl");
        assert_eq!(format!("{}", BoxShadow::Inner), "inner");
    }

    #[test]
    fn test_drop_shadow_display() {
        // Test that DropShadow displays correctly
        assert_eq!(format!("{}", DropShadow::None), "none");
        assert_eq!(format!("{}", DropShadow::Sm), "sm");
        assert_eq!(format!("{}", DropShadow::Default), "default");
        assert_eq!(format!("{}", DropShadow::Md), "md");
        assert_eq!(format!("{}", DropShadow::Lg), "lg");
        assert_eq!(format!("{}", DropShadow::Xl), "xl");
        assert_eq!(format!("{}", DropShadow::Xl2), "2xl");
        assert_eq!(format!("{}", DropShadow::Xl3), "3xl");
    }

    #[test]
    fn test_opacity_display() {
        // Test that Opacity displays correctly
        assert_eq!(format!("{}", Opacity::Zero), "0");
        assert_eq!(format!("{}", Opacity::Five), "5");
        assert_eq!(format!("{}", Opacity::Ten), "10");
        assert_eq!(format!("{}", Opacity::Twenty), "20");
        assert_eq!(format!("{}", Opacity::TwentyFive), "25");
        assert_eq!(format!("{}", Opacity::Thirty), "30");
        assert_eq!(format!("{}", Opacity::Forty), "40");
        assert_eq!(format!("{}", Opacity::Fifty), "50");
        assert_eq!(format!("{}", Opacity::Sixty), "60");
        assert_eq!(format!("{}", Opacity::Seventy), "70");
        assert_eq!(format!("{}", Opacity::SeventyFive), "75");
        assert_eq!(format!("{}", Opacity::Eighty), "80");
        assert_eq!(format!("{}", Opacity::Ninety), "90");
        assert_eq!(format!("{}", Opacity::NinetyFive), "95");
        assert_eq!(format!("{}", Opacity::Hundred), "100");
    }

    #[test]
    fn test_mix_blend_mode_display() {
        // Test that MixBlendMode displays correctly
        assert_eq!(format!("{}", MixBlendMode::Normal), "normal");
        assert_eq!(format!("{}", MixBlendMode::Multiply), "multiply");
        assert_eq!(format!("{}", MixBlendMode::Screen), "screen");
        assert_eq!(format!("{}", MixBlendMode::Overlay), "overlay");
        assert_eq!(format!("{}", MixBlendMode::Darken), "darken");
        assert_eq!(format!("{}", MixBlendMode::Lighten), "lighten");
        assert_eq!(format!("{}", MixBlendMode::ColorDodge), "color-dodge");
        assert_eq!(format!("{}", MixBlendMode::ColorBurn), "color-burn");
        assert_eq!(format!("{}", MixBlendMode::HardLight), "hard-light");
        assert_eq!(format!("{}", MixBlendMode::SoftLight), "soft-light");
        assert_eq!(format!("{}", MixBlendMode::Difference), "difference");
        assert_eq!(format!("{}", MixBlendMode::Exclusion), "exclusion");
        assert_eq!(format!("{}", MixBlendMode::Hue), "hue");
        assert_eq!(format!("{}", MixBlendMode::Saturation), "saturation");
        assert_eq!(format!("{}", MixBlendMode::Color), "color");
        assert_eq!(format!("{}", MixBlendMode::Luminosity), "luminosity");
    }

    #[test]
    fn test_background_blend_mode_display() {
        // Test that BackgroundBlendMode displays correctly
        assert_eq!(format!("{}", BackgroundBlendMode::Normal), "normal");
        assert_eq!(format!("{}", BackgroundBlendMode::Multiply), "multiply");
        assert_eq!(format!("{}", BackgroundBlendMode::Screen), "screen");
        assert_eq!(format!("{}", BackgroundBlendMode::Overlay), "overlay");
        assert_eq!(format!("{}", BackgroundBlendMode::Darken), "darken");
        assert_eq!(format!("{}", BackgroundBlendMode::Lighten), "lighten");
        assert_eq!(format!("{}", BackgroundBlendMode::ColorDodge), "color-dodge");
        assert_eq!(format!("{}", BackgroundBlendMode::ColorBurn), "color-burn");
        assert_eq!(format!("{}", BackgroundBlendMode::HardLight), "hard-light");
        assert_eq!(format!("{}", BackgroundBlendMode::SoftLight), "soft-light");
        assert_eq!(format!("{}", BackgroundBlendMode::Difference), "difference");
        assert_eq!(format!("{}", BackgroundBlendMode::Exclusion), "exclusion");
        assert_eq!(format!("{}", BackgroundBlendMode::Hue), "hue");
        assert_eq!(format!("{}", BackgroundBlendMode::Saturation), "saturation");
        assert_eq!(format!("{}", BackgroundBlendMode::Color), "color");
        assert_eq!(format!("{}", BackgroundBlendMode::Luminosity), "luminosity");
    }

    #[test]
    fn test_box_shadow_css_values() {
        // Test that BoxShadow generates correct CSS values
        assert_eq!(BoxShadow::None.to_css_value(), "none");
        assert_eq!(BoxShadow::Sm.to_css_value(), "0 1px 2px 0 rgb(0 0 0 / 0.05)");
        assert_eq!(BoxShadow::Default.to_css_value(), "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)");
        assert_eq!(BoxShadow::Md.to_css_value(), "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)");
        assert_eq!(BoxShadow::Lg.to_css_value(), "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)");
        assert_eq!(BoxShadow::Xl.to_css_value(), "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)");
        assert_eq!(BoxShadow::Xl2.to_css_value(), "0 25px 50px -12px rgb(0 0 0 / 0.25)");
        assert_eq!(BoxShadow::Inner.to_css_value(), "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)");
    }

    #[test]
    fn test_drop_shadow_css_values() {
        // Test that DropShadow generates correct CSS values
        assert_eq!(DropShadow::None.to_css_value(), "none");
        assert_eq!(DropShadow::Sm.to_css_value(), "0 1px 2px rgb(0 0 0 / 0.05)");
        assert_eq!(DropShadow::Default.to_css_value(), "0 1px 3px rgb(0 0 0 / 0.1), 0 1px 2px rgb(0 0 0 / 0.06)");
        assert_eq!(DropShadow::Md.to_css_value(), "0 4px 6px rgb(0 0 0 / 0.07), 0 2px 4px rgb(0 0 0 / 0.06)");
        assert_eq!(DropShadow::Lg.to_css_value(), "0 10px 15px rgb(0 0 0 / 0.1), 0 4px 6px rgb(0 0 0 / 0.05)");
        assert_eq!(DropShadow::Xl.to_css_value(), "0 20px 25px rgb(0 0 0 / 0.1), 0 8px 10px rgb(0 0 0 / 0.04)");
        assert_eq!(DropShadow::Xl2.to_css_value(), "0 25px 50px rgb(0 0 0 / 0.25)");
        assert_eq!(DropShadow::Xl3.to_css_value(), "0 35px 60px rgb(0 0 0 / 0.3)");
    }

    #[test]
    fn test_opacity_css_values() {
        // Test that Opacity generates correct CSS values
        assert_eq!(Opacity::Zero.to_css_value(), "0");
        assert_eq!(Opacity::Five.to_css_value(), "0.05");
        assert_eq!(Opacity::Ten.to_css_value(), "0.1");
        assert_eq!(Opacity::Twenty.to_css_value(), "0.2");
        assert_eq!(Opacity::TwentyFive.to_css_value(), "0.25");
        assert_eq!(Opacity::Thirty.to_css_value(), "0.3");
        assert_eq!(Opacity::Forty.to_css_value(), "0.4");
        assert_eq!(Opacity::Fifty.to_css_value(), "0.5");
        assert_eq!(Opacity::Sixty.to_css_value(), "0.6");
        assert_eq!(Opacity::Seventy.to_css_value(), "0.7");
        assert_eq!(Opacity::SeventyFive.to_css_value(), "0.75");
        assert_eq!(Opacity::Eighty.to_css_value(), "0.8");
        assert_eq!(Opacity::Ninety.to_css_value(), "0.9");
        assert_eq!(Opacity::NinetyFive.to_css_value(), "0.95");
        assert_eq!(Opacity::Hundred.to_css_value(), "1");
    }

    #[test]
    fn test_mix_blend_mode_css_values() {
        // Test that MixBlendMode generates correct CSS values
        assert_eq!(MixBlendMode::Normal.to_css_value(), "normal");
        assert_eq!(MixBlendMode::Multiply.to_css_value(), "multiply");
        assert_eq!(MixBlendMode::Screen.to_css_value(), "screen");
        assert_eq!(MixBlendMode::Overlay.to_css_value(), "overlay");
        assert_eq!(MixBlendMode::Darken.to_css_value(), "darken");
        assert_eq!(MixBlendMode::Lighten.to_css_value(), "lighten");
        assert_eq!(MixBlendMode::ColorDodge.to_css_value(), "color-dodge");
        assert_eq!(MixBlendMode::ColorBurn.to_css_value(), "color-burn");
        assert_eq!(MixBlendMode::HardLight.to_css_value(), "hard-light");
        assert_eq!(MixBlendMode::SoftLight.to_css_value(), "soft-light");
        assert_eq!(MixBlendMode::Difference.to_css_value(), "difference");
        assert_eq!(MixBlendMode::Exclusion.to_css_value(), "exclusion");
        assert_eq!(MixBlendMode::Hue.to_css_value(), "hue");
        assert_eq!(MixBlendMode::Saturation.to_css_value(), "saturation");
        assert_eq!(MixBlendMode::Color.to_css_value(), "color");
        assert_eq!(MixBlendMode::Luminosity.to_css_value(), "luminosity");
    }

    #[test]
    fn test_background_blend_mode_css_values() {
        // Test that BackgroundBlendMode generates correct CSS values
        assert_eq!(BackgroundBlendMode::Normal.to_css_value(), "normal");
        assert_eq!(BackgroundBlendMode::Multiply.to_css_value(), "multiply");
        assert_eq!(BackgroundBlendMode::Screen.to_css_value(), "screen");
        assert_eq!(BackgroundBlendMode::Overlay.to_css_value(), "overlay");
        assert_eq!(BackgroundBlendMode::Darken.to_css_value(), "darken");
        assert_eq!(BackgroundBlendMode::Lighten.to_css_value(), "lighten");
        assert_eq!(BackgroundBlendMode::ColorDodge.to_css_value(), "color-dodge");
        assert_eq!(BackgroundBlendMode::ColorBurn.to_css_value(), "color-burn");
        assert_eq!(BackgroundBlendMode::HardLight.to_css_value(), "hard-light");
        assert_eq!(BackgroundBlendMode::SoftLight.to_css_value(), "soft-light");
        assert_eq!(BackgroundBlendMode::Difference.to_css_value(), "difference");
        assert_eq!(BackgroundBlendMode::Exclusion.to_css_value(), "exclusion");
        assert_eq!(BackgroundBlendMode::Hue.to_css_value(), "hue");
        assert_eq!(BackgroundBlendMode::Saturation.to_css_value(), "saturation");
        assert_eq!(BackgroundBlendMode::Color.to_css_value(), "color");
        assert_eq!(BackgroundBlendMode::Luminosity.to_css_value(), "luminosity");
    }

    #[test]
    fn test_effects_serialization() {
        // Test that effects can be serialized and deserialized
        let box_shadow = BoxShadow::Lg;
        let serialized = serde_json::to_string(&box_shadow).unwrap();
        let deserialized: BoxShadow = serde_json::from_str(&serialized).unwrap();
        assert_eq!(box_shadow, deserialized);

        let drop_shadow = DropShadow::Md;
        let serialized = serde_json::to_string(&drop_shadow).unwrap();
        let deserialized: DropShadow = serde_json::from_str(&serialized).unwrap();
        assert_eq!(drop_shadow, deserialized);

        let opacity = Opacity::Fifty;
        let serialized = serde_json::to_string(&opacity).unwrap();
        let deserialized: Opacity = serde_json::from_str(&serialized).unwrap();
        assert_eq!(opacity, deserialized);

        let mix_blend_mode = MixBlendMode::Multiply;
        let serialized = serde_json::to_string(&mix_blend_mode).unwrap();
        let deserialized: MixBlendMode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(mix_blend_mode, deserialized);

        let background_blend_mode = BackgroundBlendMode::Screen;
        let serialized = serde_json::to_string(&background_blend_mode).unwrap();
        let deserialized: BackgroundBlendMode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(background_blend_mode, deserialized);
    }

    #[test]
    fn test_effects_equality_and_hash() {
        // Test that effects can be compared for equality and hashed
        let box_shadow1 = BoxShadow::Lg;
        let box_shadow2 = BoxShadow::Lg;
        let box_shadow3 = BoxShadow::Md;
        
        assert_eq!(box_shadow1, box_shadow2);
        assert_ne!(box_shadow1, box_shadow3);
        
        // Test that equal effects have the same hash
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        box_shadow1.hash(&mut hasher1);
        box_shadow2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_comprehensive_effects_utilities() {
        // Test comprehensive usage of all effects utility methods
        let classes = ClassBuilder::new()
            // Box shadows
            .box_shadow(BoxShadow::None)
            .box_shadow(BoxShadow::Sm)
            .box_shadow(BoxShadow::Default)
            .box_shadow(BoxShadow::Md)
            .box_shadow(BoxShadow::Lg)
            .box_shadow(BoxShadow::Xl)
            .box_shadow(BoxShadow::Xl2)
            .box_shadow(BoxShadow::Inner)
            
            // Drop shadows
            .drop_shadow(DropShadow::None)
            .drop_shadow(DropShadow::Sm)
            .drop_shadow(DropShadow::Default)
            .drop_shadow(DropShadow::Md)
            .drop_shadow(DropShadow::Lg)
            .drop_shadow(DropShadow::Xl)
            .drop_shadow(DropShadow::Xl2)
            .drop_shadow(DropShadow::Xl3)
            
            // Opacity
            .opacity(Opacity::Zero)
            .opacity(Opacity::TwentyFive)
            .opacity(Opacity::Fifty)
            .opacity(Opacity::SeventyFive)
            .opacity(Opacity::Hundred)
            
            // Mix blend modes
            .mix_blend_mode(MixBlendMode::Normal)
            .mix_blend_mode(MixBlendMode::Multiply)
            .mix_blend_mode(MixBlendMode::Screen)
            .mix_blend_mode(MixBlendMode::Overlay)
            .mix_blend_mode(MixBlendMode::Difference)
            
            // Background blend modes
            .background_blend_mode(BackgroundBlendMode::Normal)
            .background_blend_mode(BackgroundBlendMode::Multiply)
            .background_blend_mode(BackgroundBlendMode::Screen)
            .background_blend_mode(BackgroundBlendMode::Overlay)
            .background_blend_mode(BackgroundBlendMode::Difference)
            .build();
        
        let css_classes = classes.to_css_classes();
        
        // Verify box shadows
        assert!(css_classes.contains("shadow-none"));
        assert!(css_classes.contains("shadow-sm"));
        assert!(css_classes.contains("shadow-default"));
        assert!(css_classes.contains("shadow-md"));
        assert!(css_classes.contains("shadow-lg"));
        assert!(css_classes.contains("shadow-xl"));
        assert!(css_classes.contains("shadow-2xl"));
        assert!(css_classes.contains("shadow-inner"));
        
        // Verify drop shadows
        assert!(css_classes.contains("drop-shadow-none"));
        assert!(css_classes.contains("drop-shadow-sm"));
        assert!(css_classes.contains("drop-shadow-default"));
        assert!(css_classes.contains("drop-shadow-md"));
        assert!(css_classes.contains("drop-shadow-lg"));
        assert!(css_classes.contains("drop-shadow-xl"));
        assert!(css_classes.contains("drop-shadow-2xl"));
        assert!(css_classes.contains("drop-shadow-3xl"));
        
        // Verify opacity
        assert!(css_classes.contains("opacity-0"));
        assert!(css_classes.contains("opacity-25"));
        assert!(css_classes.contains("opacity-50"));
        assert!(css_classes.contains("opacity-75"));
        assert!(css_classes.contains("opacity-100"));
        
        // Verify mix blend modes
        assert!(css_classes.contains("mix-blend-normal"));
        assert!(css_classes.contains("mix-blend-multiply"));
        assert!(css_classes.contains("mix-blend-screen"));
        assert!(css_classes.contains("mix-blend-overlay"));
        assert!(css_classes.contains("mix-blend-difference"));
        
        // Verify background blend modes
        assert!(css_classes.contains("bg-blend-normal"));
        assert!(css_classes.contains("bg-blend-multiply"));
        assert!(css_classes.contains("bg-blend-screen"));
        assert!(css_classes.contains("bg-blend-overlay"));
        assert!(css_classes.contains("bg-blend-difference"));
    }
}
