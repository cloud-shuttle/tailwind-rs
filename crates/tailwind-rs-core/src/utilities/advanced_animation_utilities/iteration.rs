//! Animation Iteration Utilities Module
//!
//! Handles animation iteration count enum:
//! - AnimationIteration enum for controlling animation repeats

use serde::{Deserialize, Serialize};

/// Animation iteration count
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimationIteration {
    /// infinite
    Infinite,
    /// specific number
    Count(f32),
}

impl AnimationIteration {
    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            AnimationIteration::Infinite => "infinite".to_string(),
            AnimationIteration::Count(count) => count.to_string(),
        }
    }

    /// Check if animation is infinite
    pub fn is_infinite(&self) -> bool {
        matches!(self, AnimationIteration::Infinite)
    }

    /// Get the iteration count as a float, or None for infinite
    pub fn as_count(&self) -> Option<f32> {
        match self {
            AnimationIteration::Infinite => None,
            AnimationIteration::Count(count) => Some(*count),
        }
    }
}

impl Default for AnimationIteration {
    fn default() -> Self {
        AnimationIteration::Count(1.0)
    }
}

impl From<f32> for AnimationIteration {
    fn from(count: f32) -> Self {
        AnimationIteration::Count(count)
    }
}

impl From<u32> for AnimationIteration {
    fn from(count: u32) -> Self {
        AnimationIteration::Count(count as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iteration_css_values() {
        assert_eq!(AnimationIteration::Infinite.to_css_value(), "infinite");
        assert_eq!(AnimationIteration::Count(2.5).to_css_value(), "2.5");
        assert_eq!(AnimationIteration::Count(1.0).to_css_value(), "1");
    }

    #[test]
    fn iteration_infinite_check() {
        assert!(AnimationIteration::Infinite.is_infinite());
        assert!(!AnimationIteration::Count(1.0).is_infinite());
        assert!(!AnimationIteration::Count(2.5).is_infinite());
    }

    #[test]
    fn iteration_as_count() {
        assert_eq!(AnimationIteration::Infinite.as_count(), None);
        assert_eq!(AnimationIteration::Count(1.0).as_count(), Some(1.0));
        assert_eq!(AnimationIteration::Count(3.5).as_count(), Some(3.5));
    }

    #[test]
    fn iteration_from_conversions() {
        assert_eq!(AnimationIteration::from(1.0), AnimationIteration::Count(1.0));
        assert_eq!(AnimationIteration::from(2.5), AnimationIteration::Count(2.5));
        assert_eq!(AnimationIteration::from(5u32), AnimationIteration::Count(5.0));
        assert_eq!(AnimationIteration::from(10u32), AnimationIteration::Count(10.0));
    }

    #[test]
    fn iteration_default() {
        assert_eq!(AnimationIteration::default(), AnimationIteration::Count(1.0));
    }
}
