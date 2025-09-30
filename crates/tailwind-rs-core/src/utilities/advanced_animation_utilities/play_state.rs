//! Animation Play State Utilities Module
//!
//! Handles animation play state enum:
//! - AnimationPlayState enum for controlling animation playback

use serde::{Deserialize, Serialize};

/// Animation play state
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimationPlayState {
    /// running
    Running,
    /// paused
    Paused,
}

impl AnimationPlayState {
    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            AnimationPlayState::Running => "running".to_string(),
            AnimationPlayState::Paused => "paused".to_string(),
        }
    }

    /// Check if animation is running
    pub fn is_running(&self) -> bool {
        matches!(self, AnimationPlayState::Running)
    }

    /// Check if animation is paused
    pub fn is_paused(&self) -> bool {
        matches!(self, AnimationPlayState::Paused)
    }

    /// Get all animation play states
    pub fn variants() -> &'static [AnimationPlayState] {
        &[AnimationPlayState::Running, AnimationPlayState::Paused]
    }
}

impl Default for AnimationPlayState {
    fn default() -> Self {
        AnimationPlayState::Running
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_state_css_values() {
        assert_eq!(AnimationPlayState::Running.to_css_value(), "running");
        assert_eq!(AnimationPlayState::Paused.to_css_value(), "paused");
    }

    #[test]
    fn play_state_running_check() {
        assert!(AnimationPlayState::Running.is_running());
        assert!(!AnimationPlayState::Paused.is_running());
    }

    #[test]
    fn play_state_paused_check() {
        assert!(!AnimationPlayState::Running.is_paused());
        assert!(AnimationPlayState::Paused.is_paused());
    }

    #[test]
    fn play_state_variants() {
        let variants = AnimationPlayState::variants();
        assert_eq!(variants.len(), 2);
        assert!(variants.contains(&AnimationPlayState::Running));
        assert!(variants.contains(&AnimationPlayState::Paused));
    }

    #[test]
    fn play_state_default() {
        assert_eq!(AnimationPlayState::default(), AnimationPlayState::Running);
    }
}
