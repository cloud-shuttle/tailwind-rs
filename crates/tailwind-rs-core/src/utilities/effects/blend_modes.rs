//! Blend mode utilities for tailwind-rs
//!
//! This module provides utilities for mix blend mode and background blend mode effects.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

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

impl fmt::Display for MixBlendMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MixBlendMode::Normal => write!(f, "mix-blend-normal"),
            MixBlendMode::Multiply => write!(f, "mix-blend-multiply"),
            MixBlendMode::Screen => write!(f, "mix-blend-screen"),
            MixBlendMode::Overlay => write!(f, "mix-blend-overlay"),
            MixBlendMode::Darken => write!(f, "mix-blend-darken"),
            MixBlendMode::Lighten => write!(f, "mix-blend-lighten"),
            MixBlendMode::ColorDodge => write!(f, "mix-blend-color-dodge"),
            MixBlendMode::ColorBurn => write!(f, "mix-blend-color-burn"),
            MixBlendMode::HardLight => write!(f, "mix-blend-hard-light"),
            MixBlendMode::SoftLight => write!(f, "mix-blend-soft-light"),
            MixBlendMode::Difference => write!(f, "mix-blend-difference"),
            MixBlendMode::Exclusion => write!(f, "mix-blend-exclusion"),
            MixBlendMode::Hue => write!(f, "mix-blend-hue"),
            MixBlendMode::Saturation => write!(f, "mix-blend-saturation"),
            MixBlendMode::Color => write!(f, "mix-blend-color"),
            MixBlendMode::Luminosity => write!(f, "mix-blend-luminosity"),
        }
    }
}

impl fmt::Display for BackgroundBlendMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackgroundBlendMode::Normal => write!(f, "bg-blend-normal"),
            BackgroundBlendMode::Multiply => write!(f, "bg-blend-multiply"),
            BackgroundBlendMode::Screen => write!(f, "bg-blend-screen"),
            BackgroundBlendMode::Overlay => write!(f, "bg-blend-overlay"),
            BackgroundBlendMode::Darken => write!(f, "bg-blend-darken"),
            BackgroundBlendMode::Lighten => write!(f, "bg-blend-lighten"),
            BackgroundBlendMode::ColorDodge => write!(f, "bg-blend-color-dodge"),
            BackgroundBlendMode::ColorBurn => write!(f, "bg-blend-color-burn"),
            BackgroundBlendMode::HardLight => write!(f, "bg-blend-hard-light"),
            BackgroundBlendMode::SoftLight => write!(f, "bg-blend-soft-light"),
            BackgroundBlendMode::Difference => write!(f, "bg-blend-difference"),
            BackgroundBlendMode::Exclusion => write!(f, "bg-blend-exclusion"),
            BackgroundBlendMode::Hue => write!(f, "bg-blend-hue"),
            BackgroundBlendMode::Saturation => write!(f, "bg-blend-saturation"),
            BackgroundBlendMode::Color => write!(f, "bg-blend-color"),
            BackgroundBlendMode::Luminosity => write!(f, "bg-blend-luminosity"),
        }
    }
}

/// Trait for adding mix blend mode utilities to a class builder
pub trait MixBlendModeUtilities {
    fn mix_blend_mode(self, mode: MixBlendMode) -> Self;
}

/// Trait for adding background blend mode utilities to a class builder
pub trait BackgroundBlendModeUtilities {
    fn background_blend_mode(self, mode: BackgroundBlendMode) -> Self;
}

impl MixBlendModeUtilities for ClassBuilder {
    fn mix_blend_mode(self, mode: MixBlendMode) -> Self {
        self.class(mode.to_string())
    }
}

impl BackgroundBlendModeUtilities for ClassBuilder {
    fn background_blend_mode(self, mode: BackgroundBlendMode) -> Self {
        self.class(mode.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_blend_mode_display() {
        assert_eq!(MixBlendMode::Normal.to_string(), "mix-blend-normal");
        assert_eq!(MixBlendMode::Multiply.to_string(), "mix-blend-multiply");
        assert_eq!(MixBlendMode::Screen.to_string(), "mix-blend-screen");
        assert_eq!(MixBlendMode::Overlay.to_string(), "mix-blend-overlay");
        assert_eq!(MixBlendMode::Darken.to_string(), "mix-blend-darken");
        assert_eq!(MixBlendMode::Lighten.to_string(), "mix-blend-lighten");
        assert_eq!(MixBlendMode::ColorDodge.to_string(), "mix-blend-color-dodge");
        assert_eq!(MixBlendMode::ColorBurn.to_string(), "mix-blend-color-burn");
        assert_eq!(MixBlendMode::HardLight.to_string(), "mix-blend-hard-light");
        assert_eq!(MixBlendMode::SoftLight.to_string(), "mix-blend-soft-light");
        assert_eq!(MixBlendMode::Difference.to_string(), "mix-blend-difference");
        assert_eq!(MixBlendMode::Exclusion.to_string(), "mix-blend-exclusion");
        assert_eq!(MixBlendMode::Hue.to_string(), "mix-blend-hue");
        assert_eq!(MixBlendMode::Saturation.to_string(), "mix-blend-saturation");
        assert_eq!(MixBlendMode::Color.to_string(), "mix-blend-color");
        assert_eq!(MixBlendMode::Luminosity.to_string(), "mix-blend-luminosity");
    }

    #[test]
    fn test_background_blend_mode_display() {
        assert_eq!(BackgroundBlendMode::Normal.to_string(), "bg-blend-normal");
        assert_eq!(BackgroundBlendMode::Multiply.to_string(), "bg-blend-multiply");
        assert_eq!(BackgroundBlendMode::Screen.to_string(), "bg-blend-screen");
        assert_eq!(BackgroundBlendMode::Overlay.to_string(), "bg-blend-overlay");
        assert_eq!(BackgroundBlendMode::Darken.to_string(), "bg-blend-darken");
        assert_eq!(BackgroundBlendMode::Lighten.to_string(), "bg-blend-lighten");
        assert_eq!(BackgroundBlendMode::ColorDodge.to_string(), "bg-blend-color-dodge");
        assert_eq!(BackgroundBlendMode::ColorBurn.to_string(), "bg-blend-color-burn");
        assert_eq!(BackgroundBlendMode::HardLight.to_string(), "bg-blend-hard-light");
        assert_eq!(BackgroundBlendMode::SoftLight.to_string(), "bg-blend-soft-light");
        assert_eq!(BackgroundBlendMode::Difference.to_string(), "bg-blend-difference");
        assert_eq!(BackgroundBlendMode::Exclusion.to_string(), "bg-blend-exclusion");
        assert_eq!(BackgroundBlendMode::Hue.to_string(), "bg-blend-hue");
        assert_eq!(BackgroundBlendMode::Saturation.to_string(), "bg-blend-saturation");
        assert_eq!(BackgroundBlendMode::Color.to_string(), "bg-blend-color");
        assert_eq!(BackgroundBlendMode::Luminosity.to_string(), "bg-blend-luminosity");
    }

    #[test]
    fn test_mix_blend_mode_utilities() {
        let classes = ClassBuilder::new()
            .mix_blend_mode(MixBlendMode::Multiply)
            .build();
        
        assert!(classes.to_css_classes().contains("mix-blend-multiply"));
    }

    #[test]
    fn test_background_blend_mode_utilities() {
        let classes = ClassBuilder::new()
            .background_blend_mode(BackgroundBlendMode::Screen)
            .build();
        
        assert!(classes.to_css_classes().contains("bg-blend-screen"));
    }

    #[test]
    fn test_blend_mode_serialization() {
        let mix_blend_mode = MixBlendMode::Overlay;
        let serialized = serde_json::to_string(&mix_blend_mode).unwrap();
        let deserialized: MixBlendMode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(mix_blend_mode, deserialized);

        let background_blend_mode = BackgroundBlendMode::Overlay;
        let serialized = serde_json::to_string(&background_blend_mode).unwrap();
        let deserialized: BackgroundBlendMode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(background_blend_mode, deserialized);
    }

    #[test]
    fn test_blend_mode_equality_and_hash() {
        let mix_blend_mode1 = MixBlendMode::Multiply;
        let mix_blend_mode2 = MixBlendMode::Multiply;
        let mix_blend_mode3 = MixBlendMode::Screen;
        
        assert_eq!(mix_blend_mode1, mix_blend_mode2);
        assert_ne!(mix_blend_mode1, mix_blend_mode3);
        
        let background_blend_mode1 = BackgroundBlendMode::Multiply;
        let background_blend_mode2 = BackgroundBlendMode::Multiply;
        let background_blend_mode3 = BackgroundBlendMode::Screen;
        
        assert_eq!(background_blend_mode1, background_blend_mode2);
        assert_ne!(background_blend_mode1, background_blend_mode3);
        
        // Test that equal effects have the same hash
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        mix_blend_mode1.hash(&mut hasher1);
        mix_blend_mode2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
        
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        background_blend_mode1.hash(&mut hasher1);
        background_blend_mode2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }
}
