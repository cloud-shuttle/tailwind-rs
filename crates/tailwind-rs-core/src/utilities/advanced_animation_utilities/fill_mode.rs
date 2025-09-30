//! Animation Fill Mode Utilities Module
//!
//! Handles animation fill mode enum:
//! - AnimationFillMode enum for controlling animation fill behavior

use serde::{Deserialize, Serialize};

/// Animation fill mode
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimationFillMode {
    /// none
    None,
    /// forwards
    Forwards,
    /// backwards
    Backwards,
    /// both
    Both,
}

impl AnimationFillMode {
    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            AnimationFillMode::None => "none".to_string(),
            AnimationFillMode::Forwards => "forwards".to_string(),
            AnimationFillMode::Backwards => "backwards".to_string(),
            AnimationFillMode::Both => "both".to_string(),
        }
    }

    /// Check if fill mode keeps final state
    pub fn keeps_final_state(&self) -> bool {
        matches!(self, AnimationFillMode::Forwards | AnimationFillMode::Both)
    }

    /// Check if fill mode uses initial state
    pub fn uses_initial_state(&self) -> bool {
        matches!(self, AnimationFillMode::Backwards | AnimationFillMode::Both)
    }

    /// Get all animation fill modes
    pub fn variants() -> &'static [AnimationFillMode] {
        &[
            AnimationFillMode::None,
            AnimationFillMode::Forwards,
            AnimationFillMode::Backwards,
            AnimationFillMode::Both,
        ]
    }
}

impl Default for AnimationFillMode {
    fn default() -> Self {
        AnimationFillMode::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_mode_css_values() {
        assert_eq!(AnimationFillMode::None.to_css_value(), "none");
        assert_eq!(AnimationFillMode::Forwards.to_css_value(), "forwards");
        assert_eq!(AnimationFillMode::Backwards.to_css_value(), "backwards");
        assert_eq!(AnimationFillMode::Both.to_css_value(), "both");
    }

    #[test]
    fn fill_mode_keeps_final_state() {
        assert!(!AnimationFillMode::None.keeps_final_state());
        assert!(AnimationFillMode::Forwards.keeps_final_state());
        assert!(!AnimationFillMode::Backwards.keeps_final_state());
        assert!(AnimationFillMode::Both.keeps_final_state());
    }

    #[test]
    fn fill_mode_uses_initial_state() {
        assert!(!AnimationFillMode::None.uses_initial_state());
        assert!(!AnimationFillMode::Forwards.uses_initial_state());
        assert!(AnimationFillMode::Backwards.uses_initial_state());
        assert!(AnimationFillMode::Both.uses_initial_state());
    }

    #[test]
    fn fill_mode_variants() {
        let variants = AnimationFillMode::variants();
        assert_eq!(variants.len(), 4);
        assert!(variants.contains(&AnimationFillMode::None));
        assert!(variants.contains(&AnimationFillMode::Forwards));
        assert!(variants.contains(&AnimationFillMode::Backwards));
        assert!(variants.contains(&AnimationFillMode::Both));
    }

    #[test]
    fn fill_mode_default() {
        assert_eq!(AnimationFillMode::default(), AnimationFillMode::None);
    }
}
