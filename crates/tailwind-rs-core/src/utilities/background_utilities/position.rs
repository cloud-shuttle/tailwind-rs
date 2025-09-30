//! Background Position Utilities Module
//!
//! Handles background position enum and utilities:
//! - BackgroundPosition enum
//! - to_class_name() and to_css_value() methods

use serde::{Deserialize, Serialize};

/// Background position values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackgroundPosition {
    /// Bottom position
    Bottom,
    /// Center position
    Center,
    /// Left position
    Left,
    /// Left bottom position
    LeftBottom,
    /// Left top position
    LeftTop,
    /// Right position
    Right,
    /// Right bottom position
    RightBottom,
    /// Right top position
    RightTop,
    /// Top position
    Top,
}

impl BackgroundPosition {
    /// Convert to Tailwind class name
    pub fn to_class_name(&self) -> String {
        match self {
            BackgroundPosition::Bottom => "bottom".to_string(),
            BackgroundPosition::Center => "center".to_string(),
            BackgroundPosition::Left => "left".to_string(),
            BackgroundPosition::LeftBottom => "left-bottom".to_string(),
            BackgroundPosition::LeftTop => "left-top".to_string(),
            BackgroundPosition::Right => "right".to_string(),
            BackgroundPosition::RightBottom => "right-bottom".to_string(),
            BackgroundPosition::RightTop => "right-top".to_string(),
            BackgroundPosition::Top => "top".to_string(),
        }
    }

    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            BackgroundPosition::Bottom => "bottom".to_string(),
            BackgroundPosition::Center => "center".to_string(),
            BackgroundPosition::Left => "left".to_string(),
            BackgroundPosition::LeftBottom => "left bottom".to_string(),
            BackgroundPosition::LeftTop => "left top".to_string(),
            BackgroundPosition::Right => "right".to_string(),
            BackgroundPosition::RightBottom => "right bottom".to_string(),
            BackgroundPosition::RightTop => "right top".to_string(),
            BackgroundPosition::Top => "top".to_string(),
        }
    }

    /// Get all available position variants
    pub fn variants() -> &'static [BackgroundPosition] {
        &[
            BackgroundPosition::Bottom,
            BackgroundPosition::Center,
            BackgroundPosition::Left,
            BackgroundPosition::LeftBottom,
            BackgroundPosition::LeftTop,
            BackgroundPosition::Right,
            BackgroundPosition::RightBottom,
            BackgroundPosition::RightTop,
            BackgroundPosition::Top,
        ]
    }
}

impl Default for BackgroundPosition {
    fn default() -> Self {
        BackgroundPosition::Center
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_class_names() {
        assert_eq!(BackgroundPosition::Bottom.to_class_name(), "bottom");
        assert_eq!(BackgroundPosition::Center.to_class_name(), "center");
        assert_eq!(BackgroundPosition::Left.to_class_name(), "left");
        assert_eq!(BackgroundPosition::LeftBottom.to_class_name(), "left-bottom");
        assert_eq!(BackgroundPosition::LeftTop.to_class_name(), "left-top");
        assert_eq!(BackgroundPosition::Right.to_class_name(), "right");
        assert_eq!(BackgroundPosition::RightBottom.to_class_name(), "right-bottom");
        assert_eq!(BackgroundPosition::RightTop.to_class_name(), "right-top");
        assert_eq!(BackgroundPosition::Top.to_class_name(), "top");
    }

    #[test]
    fn position_css_values() {
        assert_eq!(BackgroundPosition::Bottom.to_css_value(), "bottom");
        assert_eq!(BackgroundPosition::Center.to_css_value(), "center");
        assert_eq!(BackgroundPosition::Left.to_css_value(), "left");
        assert_eq!(BackgroundPosition::LeftBottom.to_css_value(), "left bottom");
        assert_eq!(BackgroundPosition::LeftTop.to_css_value(), "left top");
        assert_eq!(BackgroundPosition::Right.to_css_value(), "right");
        assert_eq!(BackgroundPosition::RightBottom.to_css_value(), "right bottom");
        assert_eq!(BackgroundPosition::RightTop.to_css_value(), "right top");
        assert_eq!(BackgroundPosition::Top.to_css_value(), "top");
    }

    #[test]
    fn position_variants() {
        let variants = BackgroundPosition::variants();
        assert_eq!(variants.len(), 9);
        assert!(variants.contains(&BackgroundPosition::Center));
        assert!(variants.contains(&BackgroundPosition::Top));
        assert!(variants.contains(&BackgroundPosition::Bottom));
        assert!(variants.contains(&BackgroundPosition::Left));
        assert!(variants.contains(&BackgroundPosition::Right));
        assert!(variants.contains(&BackgroundPosition::LeftTop));
        assert!(variants.contains(&BackgroundPosition::LeftBottom));
        assert!(variants.contains(&BackgroundPosition::RightTop));
        assert!(variants.contains(&BackgroundPosition::RightBottom));
    }

    #[test]
    fn position_default() {
        assert_eq!(BackgroundPosition::default(), BackgroundPosition::Center);
    }
}
