//! Background Origin Utilities Module
//!
//! Handles background origin enum and utilities:
//! - BackgroundOrigin enum
//! - to_class_name() and to_css_value() methods

use serde::{Deserialize, Serialize};

/// Background origin values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackgroundOrigin {
    /// Border origin
    Border,
    /// Padding origin
    Padding,
    /// Content origin
    Content,
}

impl BackgroundOrigin {
    /// Convert to Tailwind class name
    pub fn to_class_name(&self) -> String {
        match self {
            BackgroundOrigin::Border => "border".to_string(),
            BackgroundOrigin::Padding => "padding".to_string(),
            BackgroundOrigin::Content => "content".to_string(),
        }
    }

    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            BackgroundOrigin::Border => "border-box".to_string(),
            BackgroundOrigin::Padding => "padding-box".to_string(),
            BackgroundOrigin::Content => "content-box".to_string(),
        }
    }

    /// Get all available origin variants
    pub fn variants() -> &'static [BackgroundOrigin] {
        &[
            BackgroundOrigin::Border,
            BackgroundOrigin::Padding,
            BackgroundOrigin::Content,
        ]
    }
}

impl Default for BackgroundOrigin {
    fn default() -> Self {
        BackgroundOrigin::Padding
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_class_names() {
        assert_eq!(BackgroundOrigin::Border.to_class_name(), "border");
        assert_eq!(BackgroundOrigin::Padding.to_class_name(), "padding");
        assert_eq!(BackgroundOrigin::Content.to_class_name(), "content");
    }

    #[test]
    fn origin_css_values() {
        assert_eq!(BackgroundOrigin::Border.to_css_value(), "border-box");
        assert_eq!(BackgroundOrigin::Padding.to_css_value(), "padding-box");
        assert_eq!(BackgroundOrigin::Content.to_css_value(), "content-box");
    }

    #[test]
    fn origin_variants() {
        let variants = BackgroundOrigin::variants();
        assert_eq!(variants.len(), 3);
        assert!(variants.contains(&BackgroundOrigin::Border));
        assert!(variants.contains(&BackgroundOrigin::Padding));
        assert!(variants.contains(&BackgroundOrigin::Content));
    }

    #[test]
    fn origin_default() {
        assert_eq!(BackgroundOrigin::default(), BackgroundOrigin::Padding);
    }
}
