//! Background Repeat Utilities Module
//!
//! Handles background repeat enum and utilities:
//! - BackgroundRepeat enum
//! - to_class_name() and to_css_value() methods

use serde::{Deserialize, Serialize};

/// Background repeat values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackgroundRepeat {
    /// No repeat
    NoRepeat,
    /// Repeat
    Repeat,
    /// Repeat X
    RepeatX,
    /// Repeat Y
    RepeatY,
    /// Round repeat
    Round,
    /// Space repeat
    Space,
}

impl BackgroundRepeat {
    /// Convert to Tailwind class name
    pub fn to_class_name(&self) -> String {
        match self {
            BackgroundRepeat::NoRepeat => "no-repeat".to_string(),
            BackgroundRepeat::Repeat => "repeat".to_string(),
            BackgroundRepeat::RepeatX => "repeat-x".to_string(),
            BackgroundRepeat::RepeatY => "repeat-y".to_string(),
            BackgroundRepeat::Round => "round".to_string(),
            BackgroundRepeat::Space => "space".to_string(),
        }
    }

    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            BackgroundRepeat::NoRepeat => "no-repeat".to_string(),
            BackgroundRepeat::Repeat => "repeat".to_string(),
            BackgroundRepeat::RepeatX => "repeat-x".to_string(),
            BackgroundRepeat::RepeatY => "repeat-y".to_string(),
            BackgroundRepeat::Round => "round".to_string(),
            BackgroundRepeat::Space => "space".to_string(),
        }
    }

    /// Get all available repeat variants
    pub fn variants() -> &'static [BackgroundRepeat] {
        &[
            BackgroundRepeat::NoRepeat,
            BackgroundRepeat::Repeat,
            BackgroundRepeat::RepeatX,
            BackgroundRepeat::RepeatY,
            BackgroundRepeat::Round,
            BackgroundRepeat::Space,
        ]
    }
}

impl Default for BackgroundRepeat {
    fn default() -> Self {
        BackgroundRepeat::Repeat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeat_class_names() {
        assert_eq!(BackgroundRepeat::NoRepeat.to_class_name(), "no-repeat");
        assert_eq!(BackgroundRepeat::Repeat.to_class_name(), "repeat");
        assert_eq!(BackgroundRepeat::RepeatX.to_class_name(), "repeat-x");
        assert_eq!(BackgroundRepeat::RepeatY.to_class_name(), "repeat-y");
        assert_eq!(BackgroundRepeat::Round.to_class_name(), "round");
        assert_eq!(BackgroundRepeat::Space.to_class_name(), "space");
    }

    #[test]
    fn repeat_css_values() {
        assert_eq!(BackgroundRepeat::NoRepeat.to_css_value(), "no-repeat");
        assert_eq!(BackgroundRepeat::Repeat.to_css_value(), "repeat");
        assert_eq!(BackgroundRepeat::RepeatX.to_css_value(), "repeat-x");
        assert_eq!(BackgroundRepeat::RepeatY.to_css_value(), "repeat-y");
        assert_eq!(BackgroundRepeat::Round.to_css_value(), "round");
        assert_eq!(BackgroundRepeat::Space.to_css_value(), "space");
    }

    #[test]
    fn repeat_variants() {
        let variants = BackgroundRepeat::variants();
        assert_eq!(variants.len(), 6);
        assert!(variants.contains(&BackgroundRepeat::NoRepeat));
        assert!(variants.contains(&BackgroundRepeat::Repeat));
        assert!(variants.contains(&BackgroundRepeat::RepeatX));
        assert!(variants.contains(&BackgroundRepeat::RepeatY));
        assert!(variants.contains(&BackgroundRepeat::Round));
        assert!(variants.contains(&BackgroundRepeat::Space));
    }

    #[test]
    fn repeat_default() {
        assert_eq!(BackgroundRepeat::default(), BackgroundRepeat::Repeat);
    }
}
