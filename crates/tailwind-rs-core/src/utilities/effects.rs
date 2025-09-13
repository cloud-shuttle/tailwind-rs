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
}

impl DropShadowUtilities for ClassBuilder {
    fn drop_shadow(self, shadow: DropShadow) -> Self {
        self.class(format!("drop-shadow-{}", shadow.to_class_name()))
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
}
