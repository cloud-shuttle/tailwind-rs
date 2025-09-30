//! Background Attachment Utilities Module
//!
//! Handles background attachment enum and utilities:
//! - BackgroundAttachment enum
//! - to_class_name() and to_css_value() methods

use serde::{Deserialize, Serialize};

/// Background attachment values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackgroundAttachment {
    /// Fixed attachment
    Fixed,
    /// Local attachment
    Local,
    /// Scroll attachment
    Scroll,
}

impl BackgroundAttachment {
    /// Convert to Tailwind class name
    pub fn to_class_name(&self) -> String {
        match self {
            BackgroundAttachment::Fixed => "fixed".to_string(),
            BackgroundAttachment::Local => "local".to_string(),
            BackgroundAttachment::Scroll => "scroll".to_string(),
        }
    }

    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            BackgroundAttachment::Fixed => "fixed".to_string(),
            BackgroundAttachment::Local => "local".to_string(),
            BackgroundAttachment::Scroll => "scroll".to_string(),
        }
    }

    /// Get all available attachment variants
    pub fn variants() -> &'static [BackgroundAttachment] {
        &[
            BackgroundAttachment::Fixed,
            BackgroundAttachment::Local,
            BackgroundAttachment::Scroll,
        ]
    }
}

impl Default for BackgroundAttachment {
    fn default() -> Self {
        BackgroundAttachment::Scroll
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attachment_class_names() {
        assert_eq!(BackgroundAttachment::Fixed.to_class_name(), "fixed");
        assert_eq!(BackgroundAttachment::Local.to_class_name(), "local");
        assert_eq!(BackgroundAttachment::Scroll.to_class_name(), "scroll");
    }

    #[test]
    fn attachment_css_values() {
        assert_eq!(BackgroundAttachment::Fixed.to_css_value(), "fixed");
        assert_eq!(BackgroundAttachment::Local.to_css_value(), "local");
        assert_eq!(BackgroundAttachment::Scroll.to_css_value(), "scroll");
    }

    #[test]
    fn attachment_variants() {
        let variants = BackgroundAttachment::variants();
        assert_eq!(variants.len(), 3);
        assert!(variants.contains(&BackgroundAttachment::Fixed));
        assert!(variants.contains(&BackgroundAttachment::Local));
        assert!(variants.contains(&BackgroundAttachment::Scroll));
    }

    #[test]
    fn attachment_default() {
        assert_eq!(BackgroundAttachment::default(), BackgroundAttachment::Scroll);
    }

    #[test]
    fn attachment_debug() {
        let attachment = BackgroundAttachment::Fixed;
        assert_eq!(format!("{:?}", attachment), "Fixed");
    }
}
