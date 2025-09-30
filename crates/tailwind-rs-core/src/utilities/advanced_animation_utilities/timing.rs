//! Animation Timing Utilities Module
//!
//! Handles animation timing function enums:
//! - TimingFunction enum with various easing functions
//! - StepDirection enum for step timing functions

use serde::{Deserialize, Serialize};

/// Animation timing function
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimingFunction {
    /// linear
    Linear,
    /// ease
    Ease,
    /// ease-in
    EaseIn,
    /// ease-out
    EaseOut,
    /// ease-in-out
    EaseInOut,
    /// cubic-bezier(n,n,n,n)
    CubicBezier(f32, f32, f32, f32),
    /// steps(n)
    Steps(u32),
    /// steps(n, start|end)
    StepsWithDirection(u32, StepDirection),
}

impl TimingFunction {
    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            TimingFunction::Linear => "linear".to_string(),
            TimingFunction::Ease => "ease".to_string(),
            TimingFunction::EaseIn => "ease-in".to_string(),
            TimingFunction::EaseOut => "ease-out".to_string(),
            TimingFunction::EaseInOut => "ease-in-out".to_string(),
            TimingFunction::CubicBezier(x1, y1, x2, y2) => {
                format!("cubic-bezier({}, {}, {}, {})", x1, y1, x2, y2)
            }
            TimingFunction::Steps(count) => format!("steps({})", count),
            TimingFunction::StepsWithDirection(count, direction) => {
                let dir = match direction {
                    StepDirection::Start => "start",
                    StepDirection::End => "end",
                };
                format!("steps({}, {})", count, dir)
            }
        }
    }

    /// Get all predefined timing functions
    pub fn variants() -> &'static [TimingFunction] {
        &[
            TimingFunction::Linear,
            TimingFunction::Ease,
            TimingFunction::EaseIn,
            TimingFunction::EaseOut,
            TimingFunction::EaseInOut,
        ]
    }
}

impl Default for TimingFunction {
    fn default() -> Self {
        TimingFunction::Ease
    }
}

/// Step direction for steps timing function
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StepDirection {
    /// start
    Start,
    /// end
    End,
}

impl StepDirection {
    /// Get all step directions
    pub fn variants() -> &'static [StepDirection] {
        &[StepDirection::Start, StepDirection::End]
    }
}

impl Default for StepDirection {
    fn default() -> Self {
        StepDirection::End
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timing_function_css_values() {
        assert_eq!(TimingFunction::Linear.to_css_value(), "linear");
        assert_eq!(TimingFunction::Ease.to_css_value(), "ease");
        assert_eq!(TimingFunction::EaseIn.to_css_value(), "ease-in");
        assert_eq!(TimingFunction::EaseOut.to_css_value(), "ease-out");
        assert_eq!(TimingFunction::EaseInOut.to_css_value(), "ease-in-out");
    }

    #[test]
    fn cubic_bezier_timing() {
        let cubic = TimingFunction::CubicBezier(0.25, 0.46, 0.45, 0.94);
        assert_eq!(cubic.to_css_value(), "cubic-bezier(0.25, 0.46, 0.45, 0.94)");
    }

    #[test]
    fn steps_timing() {
        let steps = TimingFunction::Steps(5);
        assert_eq!(steps.to_css_value(), "steps(5)");
    }

    #[test]
    fn steps_with_direction_timing() {
        let steps_start = TimingFunction::StepsWithDirection(4, StepDirection::Start);
        assert_eq!(steps_start.to_css_value(), "steps(4, start)");

        let steps_end = TimingFunction::StepsWithDirection(4, StepDirection::End);
        assert_eq!(steps_end.to_css_value(), "steps(4, end)");
    }

    #[test]
    fn timing_function_variants() {
        let variants = TimingFunction::variants();
        assert_eq!(variants.len(), 5);
        assert!(variants.contains(&TimingFunction::Linear));
        assert!(variants.contains(&TimingFunction::Ease));
    }

    #[test]
    fn step_direction_variants() {
        let variants = StepDirection::variants();
        assert_eq!(variants.len(), 2);
        assert!(variants.contains(&StepDirection::Start));
        assert!(variants.contains(&StepDirection::End));
    }

    #[test]
    fn defaults() {
        assert_eq!(TimingFunction::default(), TimingFunction::Ease);
        assert_eq!(StepDirection::default(), StepDirection::End);
    }
}
