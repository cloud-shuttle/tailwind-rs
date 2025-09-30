//! Background Clip Utilities Module
//!
//! Handles background clip enum and utilities:
//! - BackgroundClip enum
//! - to_class_name() and to_css_value() methods

use serde::{Deserialize, Serialize};

/// Background clip values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackgroundClip {
    /// Border clip
    Border,
    /// Padding clip
    Padding,
    /// Content clip
    Content,
    /// Text clip
    Text,
}

impl BackgroundClip {
    /// Convert to Tailwind class name
    pub fn to_class_name(&self) -> String {
        match self {
            BackgroundClip::Border => "border".to_string(),
            BackgroundClip::Padding => "padding".to_string(),
            BackgroundClip::Content => "content".to_string(),
            BackgroundClip::Text => "text".to_string(),
        }
    }

    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            BackgroundClip::Border => "border-box".to_string(),
            BackgroundClip::Padding => "padding-box".to_string(),
            BackgroundClip::Content => "content-box".to_string(),
            BackgroundClip::Text => "text".to_string(),
        }
    }

    /// Get all available clip variants
    pub fn variants() -> &'static [BackgroundClip] {
        &[
            BackgroundClip::Border,
            BackgroundClip::Padding,
            BackgroundClip::Content,
            BackgroundClip::Text,
        ]
    }
}

impl Default for BackgroundClip {
    fn default() -> Self {
        BackgroundClip::Border
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_class_names() {
        assert_eq!(BackgroundClip::Border.to_class_name(), "border");
        assert_eq!(BackgroundClip::Padding.to_class_name(), "padding");
        assert_eq!(BackgroundClip::Content.to_class_name(), "content");
        assert_eq!(BackgroundClip::Text.to_class_name(), "text");
    }

    #[test]
    fn clip_css_values() {
        assert_eq!(BackgroundClip::Border.to_css_value(), "border-box");
        assert_eq!(BackgroundClip::Padding.to_css_value(), "padding-box");
        assert_eq!(BackgroundClip::Content.to_css_value(), "content-box");
        assert_eq!(BackgroundClip::Text.to_css_value(), "text");
    }

    #[test]
    fn clip_variants() {
        let variants = BackgroundClip::variants();
        assert_eq!(variants.len(), 4);
        assert!(variants.contains(&BackgroundClip::Border));
        assert!(variants.contains(&BackgroundClip::Padding));
        assert!(variants.contains(&BackgroundClip::Content));
        assert!(variants.contains(&BackgroundClip::Text));
    }

    #[test]
    fn clip_default() {
        assert_eq!(BackgroundClip::default(), BackgroundClip::Border);
    }
}
