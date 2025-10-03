//! Animation Composition Utilities Module
//!
//! Handles animation composition structures:
//! - AnimationComposition for combining multiple animations
//! - ComposedAnimation for individual animation references
//! - AnimationReference for built-in or custom animations

use serde::{Deserialize, Serialize};
use crate::utilities::advanced_animation_utilities::TimingFunction;

/// Animation composition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationComposition {
    /// Composition name
    pub name: String,
    /// Animations to compose
    pub animations: Vec<ComposedAnimation>,
    /// Composition timing
    pub timing: CompositionTiming,
}

impl AnimationComposition {
    /// Create a new animation composition
    pub fn new(name: String) -> Self {
        Self {
            name,
            animations: Vec::new(),
            timing: CompositionTiming::Sequential,
        }
    }

    /// Add an animation to the composition
    pub fn add_animation(&mut self, animation: ComposedAnimation) {
        self.animations.push(animation);
    }

    /// Get the total duration of the composition
    pub fn total_duration(&self) -> u32 {
        match self.timing {
            CompositionTiming::Sequential => {
                self.animations.iter().map(|anim| anim.duration()).sum()
            }
            CompositionTiming::Parallel => {
                self.animations.iter().map(|anim| anim.duration()).max().unwrap_or(0)
            }
        }
    }
}

impl Default for AnimationComposition {
    fn default() -> Self {
        Self::new("unnamed-composition".to_string())
    }
}

/// Composed animation reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComposedAnimation {
    /// Animation name or custom keyframe
    pub animation: AnimationReference,
    /// Start offset (0.0 to 1.0)
    pub start_offset: f32,
    /// End offset (0.0 to 1.0)
    pub end_offset: f32,
    /// Animation properties override
    pub properties: Option<AnimationProperties>,
}

impl ComposedAnimation {
    /// Create a new composed animation
    pub fn new(animation: AnimationReference) -> Self {
        Self {
            animation,
            start_offset: 0.0,
            end_offset: 1.0,
            properties: None,
        }
    }

    /// Set timing offsets
    pub fn with_timing(mut self, start: f32, end: f32) -> Self {
        self.start_offset = start;
        self.end_offset = end;
        self
    }

    /// Set properties override
    pub fn with_properties(mut self, properties: AnimationProperties) -> Self {
        self.properties = Some(properties);
        self
    }

    /// Get the duration of this animation
    pub fn duration(&self) -> u32 {
        match &self.animation {
            AnimationReference::BuiltIn(_) => 1000, // Default duration
            AnimationReference::Custom(keyframe) => keyframe.duration,
        }
    }
}

/// Animation reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimationReference {
    /// Built-in animation name
    BuiltIn(String),
    /// Custom keyframe animation
    Custom(super::keyframe::CustomKeyframe),
}

impl AnimationReference {
    /// Get the animation name
    pub fn name(&self) -> &str {
        match self {
            AnimationReference::BuiltIn(name) => name,
            AnimationReference::Custom(keyframe) => &keyframe.name,
        }
    }
}

/// Animation properties override
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationProperties {
    /// Duration override
    pub duration: Option<u32>,
    /// Timing function override
    pub timing_function: Option<super::timing::TimingFunction>,
    /// Delay override
    pub delay: Option<u32>,
    /// Iteration count override
    pub iteration_count: Option<super::iteration::AnimationIteration>,
}

impl AnimationProperties {
    /// Create new animation properties
    pub fn new() -> Self {
        Self {
            duration: None,
            timing_function: None,
            delay: None,
            iteration_count: None,
        }
    }

    /// Set duration
    pub fn with_duration(mut self, duration: u32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set timing function
    pub fn with_timing_function(mut self, timing: super::timing::TimingFunction) -> Self {
        self.timing_function = Some(timing);
        self
    }
}

impl Default for AnimationProperties {
    fn default() -> Self {
        Self::new()
    }
}

/// Composition timing mode
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompositionTiming {
    /// Animations play sequentially
    Sequential,
    /// Animations play in parallel
    Parallel,
}

impl CompositionTiming {
    /// Check if timing is sequential
    pub fn is_sequential(&self) -> bool {
        matches!(self, CompositionTiming::Sequential)
    }

    /// Check if timing is parallel
    pub fn is_parallel(&self) -> bool {
        matches!(self, CompositionTiming::Parallel)
    }
}

impl Default for CompositionTiming {
    fn default() -> Self {
        CompositionTiming::Sequential
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::keyframe::CustomKeyframe;

    #[test]
    fn composition_creation() {
        let composition = AnimationComposition::new("test-composition".to_string());
        assert_eq!(composition.name, "test-composition");
        assert!(composition.animations.is_empty());
        assert_eq!(composition.timing, CompositionTiming::Sequential);
    }

    #[test]
    fn composed_animation_creation() {
        let keyframe = CustomKeyframe::new("test-keyframe".to_string());
        let animation = AnimationReference::Custom(keyframe);
        let composed = ComposedAnimation::new(animation)
            .with_timing(0.2, 0.8);

        assert_eq!(composed.start_offset, 0.2);
        assert_eq!(composed.end_offset, 0.8);
    }

    #[test]
    fn animation_reference_name() {
        let built_in = AnimationReference::BuiltIn("fade-in".to_string());
        assert_eq!(built_in.name(), "fade-in");

        let keyframe = CustomKeyframe::new("custom-fade".to_string());
        let custom = AnimationReference::Custom(keyframe);
        assert_eq!(custom.name(), "custom-fade");
    }

    #[test]
    fn animation_properties() {
        let props = AnimationProperties::new()
            .with_duration(2000)
            .with_timing_function(TimingFunction::Linear);

        assert_eq!(props.duration, Some(2000));
        assert_eq!(props.timing_function, Some(TimingFunction::Linear));
    }

    #[test]
    fn composition_timing() {
        assert!(CompositionTiming::Sequential.is_sequential());
        assert!(!CompositionTiming::Sequential.is_parallel());
        assert!(CompositionTiming::Parallel.is_parallel());
        assert!(!CompositionTiming::Parallel.is_sequential());
    }

    #[test]
    fn composition_total_duration() {
        let mut composition = AnimationComposition::new("test".to_string());

        // Add some animations
        let keyframe1 = CustomKeyframe::new("anim1".to_string());
        let keyframe2 = CustomKeyframe::new("anim2".to_string());
        composition.add_animation(ComposedAnimation::new(AnimationReference::Custom(keyframe1)));
        composition.add_animation(ComposedAnimation::new(AnimationReference::Custom(keyframe2)));

        // Sequential timing should sum durations
        composition.timing = CompositionTiming::Sequential;
        assert_eq!(composition.total_duration(), 2000); // 1000 + 1000

        // Parallel timing should take max duration
        composition.timing = CompositionTiming::Parallel;
        assert_eq!(composition.total_duration(), 1000); // max(1000, 1000)
    }
}
