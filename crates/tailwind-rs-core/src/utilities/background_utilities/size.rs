//! Background Size Utilities Module
//!
//! Handles background size enum and utilities:
//! - BackgroundSize enum
//! - to_class_name() and to_css_value() methods

use serde::{Deserialize, Serialize};

/// Background size values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackgroundSize {
    /// Auto size
    Auto,
    /// Cover size
    Cover,
    /// Contain size
    Contain,
}

impl BackgroundSize {
    /// Convert to Tailwind class name
    pub fn to_class_name(&self) -> String {
        match self {
            BackgroundSize::Auto => "auto".to_string(),
            BackgroundSize::Cover => "cover".to_string(),
            BackgroundSize::Contain => "contain".to_string(),
        }
    }

    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            BackgroundSize::Auto => "auto".to_string(),
            BackgroundSize::Cover => "cover".to_string(),
            BackgroundSize::Contain => "contain".to_string(),
        }
    }

    /// Get all available size variants
    pub fn variants() -> &'static [BackgroundSize] {
        &[
            BackgroundSize::Auto,
            BackgroundSize::Cover,
            BackgroundSize::Contain,
        ]
    }
}

impl Default for BackgroundSize {
    fn default() -> Self {
        BackgroundSize::Auto
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_class_names() {
        assert_eq!(BackgroundSize::Auto.to_class_name(), "auto");
        assert_eq!(BackgroundSize::Cover.to_class_name(), "cover");
        assert_eq!(BackgroundSize::Contain.to_class_name(), "contain");
    }

    #[test]
    fn size_css_values() {
        assert_eq!(BackgroundSize::Auto.to_css_value(), "auto");
        assert_eq!(BackgroundSize::Cover.to_css_value(), "cover");
        assert_eq!(BackgroundSize::Contain.to_css_value(), "contain");
    }

    #[test]
    fn size_variants() {
        let variants = BackgroundSize::variants();
        assert_eq!(variants.len(), 3);
        assert!(variants.contains(&BackgroundSize::Auto));
        assert!(variants.contains(&BackgroundSize::Cover));
        assert!(variants.contains(&BackgroundSize::Contain));
    }

    #[test]
    fn size_default() {
        assert_eq!(BackgroundSize::default(), BackgroundSize::Auto);
    }
}
