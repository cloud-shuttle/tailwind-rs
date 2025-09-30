//! Background Image Utilities Module
//!
//! Handles background image enum and utilities:
//! - BackgroundImage enum
//! - to_class_name() and to_css_value() methods

use serde::{Deserialize, Serialize};

/// Background image values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackgroundImage {
    /// None image
    None,
    /// Linear gradient
    LinearGradient,
    /// Radial gradient
    RadialGradient,
    /// Conic gradient
    ConicGradient,
}

impl BackgroundImage {
    /// Convert to Tailwind class name
    pub fn to_class_name(&self) -> String {
        match self {
            BackgroundImage::None => "none".to_string(),
            BackgroundImage::LinearGradient => "gradient-to-r".to_string(),
            BackgroundImage::RadialGradient => "radial-gradient".to_string(),
            BackgroundImage::ConicGradient => "conic-gradient".to_string(),
        }
    }

    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            BackgroundImage::None => "none".to_string(),
            BackgroundImage::LinearGradient => {
                "linear-gradient(to right, var(--tw-gradient-stops))".to_string()
            }
            BackgroundImage::RadialGradient => {
                "radial-gradient(ellipse at center, var(--tw-gradient-stops))".to_string()
            }
            BackgroundImage::ConicGradient => {
                "conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))".to_string()
            }
        }
    }

    /// Get all available image variants
    pub fn variants() -> &'static [BackgroundImage] {
        &[
            BackgroundImage::None,
            BackgroundImage::LinearGradient,
            BackgroundImage::RadialGradient,
            BackgroundImage::ConicGradient,
        ]
    }
}

impl Default for BackgroundImage {
    fn default() -> Self {
        BackgroundImage::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_class_names() {
        assert_eq!(BackgroundImage::None.to_class_name(), "none");
        assert_eq!(BackgroundImage::LinearGradient.to_class_name(), "gradient-to-r");
        assert_eq!(BackgroundImage::RadialGradient.to_class_name(), "radial-gradient");
        assert_eq!(BackgroundImage::ConicGradient.to_class_name(), "conic-gradient");
    }

    #[test]
    fn image_css_values() {
        assert_eq!(BackgroundImage::None.to_css_value(), "none");
        assert!(BackgroundImage::LinearGradient.to_css_value().contains("linear-gradient"));
        assert!(BackgroundImage::RadialGradient.to_css_value().contains("radial-gradient"));
        assert!(BackgroundImage::ConicGradient.to_css_value().contains("conic-gradient"));
    }

    #[test]
    fn image_variants() {
        let variants = BackgroundImage::variants();
        assert_eq!(variants.len(), 4);
        assert!(variants.contains(&BackgroundImage::None));
        assert!(variants.contains(&BackgroundImage::LinearGradient));
        assert!(variants.contains(&BackgroundImage::RadialGradient));
        assert!(variants.contains(&BackgroundImage::ConicGradient));
    }

    #[test]
    fn image_default() {
        assert_eq!(BackgroundImage::default(), BackgroundImage::None);
    }
}
