//! Animation Direction Utilities Module
//!
//! Handles animation direction enum:
//! - AnimationDirection enum for controlling animation playback direction

use serde::{Deserialize, Serialize};

/// Animation direction
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimationDirection {
    /// normal
    Normal,
    /// reverse
    Reverse,
    /// alternate
    Alternate,
    /// alternate-reverse
    AlternateReverse,
}

impl AnimationDirection {
    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            AnimationDirection::Normal => "normal".to_string(),
            AnimationDirection::Reverse => "reverse".to_string(),
            AnimationDirection::Alternate => "alternate".to_string(),
            AnimationDirection::AlternateReverse => "alternate-reverse".to_string(),
        }
    }

    /// Check if direction is reversed
    pub fn is_reversed(&self) -> bool {
        matches!(self, AnimationDirection::Reverse | AnimationDirection::AlternateReverse)
    }

    /// Check if direction alternates
    pub fn is_alternating(&self) -> bool {
        matches!(self, AnimationDirection::Alternate | AnimationDirection::AlternateReverse)
    }

    /// Get all animation directions
    pub fn variants() -> &'static [AnimationDirection] {
        &[
            AnimationDirection::Normal,
            AnimationDirection::Reverse,
            AnimationDirection::Alternate,
            AnimationDirection::AlternateReverse,
        ]
    }
}

impl Default for AnimationDirection {
    fn default() -> Self {
        AnimationDirection::Normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_css_values() {
        assert_eq!(AnimationDirection::Normal.to_css_value(), "normal");
        assert_eq!(AnimationDirection::Reverse.to_css_value(), "reverse");
        assert_eq!(AnimationDirection::Alternate.to_css_value(), "alternate");
        assert_eq!(AnimationDirection::AlternateReverse.to_css_value(), "alternate-reverse");
    }

    #[test]
    fn direction_is_reversed() {
        assert!(!AnimationDirection::Normal.is_reversed());
        assert!(AnimationDirection::Reverse.is_reversed());
        assert!(!AnimationDirection::Alternate.is_reversed());
        assert!(AnimationDirection::AlternateReverse.is_reversed());
    }

    #[test]
    fn direction_is_alternating() {
        assert!(!AnimationDirection::Normal.is_alternating());
        assert!(!AnimationDirection::Reverse.is_alternating());
        assert!(AnimationDirection::Alternate.is_alternating());
        assert!(AnimationDirection::AlternateReverse.is_alternating());
    }

    #[test]
    fn direction_variants() {
        let variants = AnimationDirection::variants();
        assert_eq!(variants.len(), 4);
        assert!(variants.contains(&AnimationDirection::Normal));
        assert!(variants.contains(&AnimationDirection::Reverse));
        assert!(variants.contains(&AnimationDirection::Alternate));
        assert!(variants.contains(&AnimationDirection::AlternateReverse));
    }

    #[test]
    fn direction_default() {
        assert_eq!(AnimationDirection::default(), AnimationDirection::Normal);
    }
}
