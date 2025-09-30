//! Effects Types Module
//!
//! Core types and enums for effects utilities:
//! - ShadowType: Different types of box shadows
//! - OpacityLevel: Predefined opacity values
//! - MixBlendMode: CSS blend modes
//! - EffectType: Categories of visual effects

/// Types of box shadows supported by Tailwind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShadowType {
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
    /// 2x extra large shadow
    Xxl,
    /// Inner shadow (inset)
    Inner,
    /// No shadow
    None,
}

impl ShadowType {
    /// Get the CSS value for this shadow type
    pub fn css_value(&self) -> &'static str {
        match self {
            ShadowType::Sm => "0 1px 2px 0 rgb(0 0 0 / 0.05)",
            ShadowType::Default => "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)",
            ShadowType::Md => "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)",
            ShadowType::Lg => "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)",
            ShadowType::Xl => "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)",
            ShadowType::Xxl => "0 25px 50px -12px rgb(0 0 0 / 0.25)",
            ShadowType::Inner => "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)",
            ShadowType::None => "none",
        }
    }

    /// Get the class suffix for this shadow type
    pub fn class_suffix(&self) -> &'static str {
        match self {
            ShadowType::Sm => "sm",
            ShadowType::Default => "",
            ShadowType::Md => "md",
            ShadowType::Lg => "lg",
            ShadowType::Xl => "xl",
            ShadowType::Xxl => "2xl",
            ShadowType::Inner => "inner",
            ShadowType::None => "none",
        }
    }
}

/// Predefined opacity levels for Tailwind CSS
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpacityLevel {
    /// 0% opacity (transparent)
    Zero = 0,
    /// 5% opacity
    Five = 5,
    /// 10% opacity
    Ten = 10,
    /// 20% opacity
    Twenty = 20,
    /// 25% opacity
    TwentyFive = 25,
    /// 30% opacity
    Thirty = 30,
    /// 40% opacity
    Forty = 40,
    /// 50% opacity
    Fifty = 50,
    /// 60% opacity
    Sixty = 60,
    /// 70% opacity
    Seventy = 70,
    /// 75% opacity
    SeventyFive = 75,
    /// 80% opacity
    Eighty = 80,
    /// 90% opacity
    Ninety = 90,
    /// 95% opacity
    NinetyFive = 95,
    /// 100% opacity (fully opaque)
    Hundred = 100,
}

impl OpacityLevel {
    /// Convert to CSS opacity value
    pub fn to_css_value(&self) -> String {
        format!("{:.2}", *self as u32 as f32 / 100.0)
    }

    /// Get the class suffix for this opacity level
    pub fn class_suffix(&self) -> String {
        match self {
            OpacityLevel::Zero => "0".to_string(),
            OpacityLevel::Five => "5".to_string(),
            OpacityLevel::Ten => "10".to_string(),
            OpacityLevel::Twenty => "20".to_string(),
            OpacityLevel::TwentyFive => "25".to_string(),
            OpacityLevel::Thirty => "30".to_string(),
            OpacityLevel::Forty => "40".to_string(),
            OpacityLevel::Fifty => "50".to_string(),
            OpacityLevel::Sixty => "60".to_string(),
            OpacityLevel::Seventy => "70".to_string(),
            OpacityLevel::SeventyFive => "75".to_string(),
            OpacityLevel::Eighty => "80".to_string(),
            OpacityLevel::Ninety => "90".to_string(),
            OpacityLevel::NinetyFive => "95".to_string(),
            OpacityLevel::Hundred => "100".to_string(),
        }
    }

    /// Get all available opacity levels
    pub fn all_levels() -> Vec<OpacityLevel> {
        vec![
            OpacityLevel::Zero,
            OpacityLevel::Five,
            OpacityLevel::Ten,
            OpacityLevel::Twenty,
            OpacityLevel::TwentyFive,
            OpacityLevel::Thirty,
            OpacityLevel::Forty,
            OpacityLevel::Fifty,
            OpacityLevel::Sixty,
            OpacityLevel::Seventy,
            OpacityLevel::SeventyFive,
            OpacityLevel::Eighty,
            OpacityLevel::Ninety,
            OpacityLevel::NinetyFive,
            OpacityLevel::Hundred,
        ]
    }
}

/// CSS blend modes for mix-blend-mode property
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MixBlendMode {
    /// Normal blending
    Normal,
    /// Multiply blending
    Multiply,
    /// Screen blending
    Screen,
    /// Overlay blending
    Overlay,
    /// Darken blending
    Darken,
    /// Lighten blending
    Lighten,
    /// Color dodge blending
    ColorDodge,
    /// Color burn blending
    ColorBurn,
    /// Hard light blending
    HardLight,
    /// Soft light blending
    SoftLight,
    /// Difference blending
    Difference,
    /// Exclusion blending
    Exclusion,
    /// Hue blending
    Hue,
    /// Saturation blending
    Saturation,
    /// Color blending
    Color,
    /// Luminosity blending
    Luminosity,
}

impl MixBlendMode {
    /// Get the CSS value for this blend mode
    pub fn css_value(&self) -> &'static str {
        match self {
            MixBlendMode::Normal => "normal",
            MixBlendMode::Multiply => "multiply",
            MixBlendMode::Screen => "screen",
            MixBlendMode::Overlay => "overlay",
            MixBlendMode::Darken => "darken",
            MixBlendMode::Lighten => "lighten",
            MixBlendMode::ColorDodge => "color-dodge",
            MixBlendMode::ColorBurn => "color-burn",
            MixBlendMode::HardLight => "hard-light",
            MixBlendMode::SoftLight => "soft-light",
            MixBlendMode::Difference => "difference",
            MixBlendMode::Exclusion => "exclusion",
            MixBlendMode::Hue => "hue",
            MixBlendMode::Saturation => "saturation",
            MixBlendMode::Color => "color",
            MixBlendMode::Luminosity => "luminosity",
        }
    }

    /// Get the class suffix for this blend mode
    pub fn class_suffix(&self) -> &'static str {
        match self {
            MixBlendMode::Normal => "normal",
            MixBlendMode::Multiply => "multiply",
            MixBlendMode::Screen => "screen",
            MixBlendMode::Overlay => "overlay",
            MixBlendMode::Darken => "darken",
            MixBlendMode::Lighten => "lighten",
            MixBlendMode::ColorDodge => "color-dodge",
            MixBlendMode::ColorBurn => "color-burn",
            MixBlendMode::HardLight => "hard-light",
            MixBlendMode::SoftLight => "soft-light",
            MixBlendMode::Difference => "difference",
            MixBlendMode::Exclusion => "exclusion",
            MixBlendMode::Hue => "hue",
            MixBlendMode::Saturation => "saturation",
            MixBlendMode::Color => "color",
            MixBlendMode::Luminosity => "luminosity",
        }
    }
}

/// Categories of visual effects
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EffectType {
    /// Box shadow effects
    Shadow,
    /// Opacity effects
    Opacity,
    /// Blend mode effects
    BlendMode,
    /// Mask effects
    Mask,
}

impl EffectType {
    /// Get the CSS property name for this effect type
    pub fn css_property(&self) -> &'static str {
        match self {
            EffectType::Shadow => "box-shadow",
            EffectType::Opacity => "opacity",
            EffectType::BlendMode => "mix-blend-mode",
            EffectType::Mask => "mask",
        }
    }

    /// Check if this effect type uses percentage values
    pub fn uses_percentage(&self) -> bool {
        matches!(self, EffectType::Opacity)
    }

    /// Check if this effect type uses color values
    pub fn uses_color(&self) -> bool {
        matches!(self, EffectType::Shadow)
    }

    /// Check if this effect type is performance-sensitive
    pub fn is_performance_sensitive(&self) -> bool {
        matches!(self, EffectType::Shadow | EffectType::BlendMode)
    }
}

/// Effect configuration combining type and value
#[derive(Debug, Clone, PartialEq)]
pub struct EffectConfig {
    pub effect_type: EffectType,
    pub value: String,
}

impl EffectConfig {
    /// Create a new effect configuration
    pub fn new(effect_type: EffectType, value: String) -> Self {
        Self { effect_type, value }
    }

    /// Convert to CSS property
    pub fn to_css_property(&self) -> crate::css_generator::types::CssProperty {
        crate::css_generator::types::CssProperty {
            name: self.effect_type.css_property().to_string(),
            value: self.value.clone(),
            important: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_type_css_values() {
        assert_eq!(ShadowType::None.css_value(), "none");
        assert_eq!(ShadowType::Sm.css_value(), "0 1px 2px 0 rgb(0 0 0 / 0.05)");
        assert!(ShadowType::Default.css_value().contains("0 1px 3px"));
    }

    #[test]
    fn shadow_type_class_suffixes() {
        assert_eq!(ShadowType::Sm.class_suffix(), "sm");
        assert_eq!(ShadowType::Default.class_suffix(), "");
        assert_eq!(ShadowType::Inner.class_suffix(), "inner");
    }

    #[test]
    fn opacity_level_values() {
        assert_eq!(OpacityLevel::Zero.to_css_value(), "0.00");
        assert_eq!(OpacityLevel::Fifty.to_css_value(), "0.50");
        assert_eq!(OpacityLevel::Hundred.to_css_value(), "1.00");
    }

    #[test]
    fn opacity_level_class_suffixes() {
        assert_eq!(OpacityLevel::Zero.class_suffix(), "0");
        assert_eq!(OpacityLevel::TwentyFive.class_suffix(), "25");
        assert_eq!(OpacityLevel::Hundred.class_suffix(), "100");
    }

    #[test]
    fn mix_blend_mode_css_values() {
        assert_eq!(MixBlendMode::Normal.css_value(), "normal");
        assert_eq!(MixBlendMode::Multiply.css_value(), "multiply");
        assert_eq!(MixBlendMode::Overlay.css_value(), "overlay");
    }

    #[test]
    fn effect_type_properties() {
        assert_eq!(EffectType::Shadow.css_property(), "box-shadow");
        assert_eq!(EffectType::Opacity.css_property(), "opacity");
        assert_eq!(EffectType::BlendMode.css_property(), "mix-blend-mode");

        assert!(EffectType::Opacity.uses_percentage());
        assert!(!EffectType::Shadow.uses_percentage());

        assert!(EffectType::Shadow.uses_color());
        assert!(!EffectType::Opacity.uses_color());

        assert!(EffectType::Shadow.is_performance_sensitive());
        assert!(EffectType::BlendMode.is_performance_sensitive());
        assert!(!EffectType::Opacity.is_performance_sensitive());
    }

    #[test]
    fn effect_config_to_css() {
        let config = EffectConfig::new(EffectType::Opacity, "0.5".to_string());
        let css_prop = config.to_css_property();

        assert_eq!(css_prop.name, "opacity");
        assert_eq!(css_prop.value, "0.5");
        assert!(!css_prop.important);
    }
}
