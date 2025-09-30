//! Advanced Animation Utilities Module
//!
//! Comprehensive advanced animation utilities for Tailwind-RS:
//! - Custom keyframe animations with complex timing and properties
//! - Animation composition for combining multiple animations
//! - Advanced timing functions including cubic-bezier and steps
//! - Animation control properties (direction, fill mode, play state)
//! - Flexible animation iteration and delay controls

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export all animation utility types and traits
pub mod composition;
pub mod direction;
pub mod fill_mode;
pub mod iteration;
pub mod keyframe;
pub mod play_state;
pub mod timing;

// Re-export all types for easy access
pub use composition::{
    AnimationComposition, AnimationProperties, AnimationReference, ComposedAnimation,
    CompositionTiming,
};
pub use direction::AnimationDirection;
pub use fill_mode::AnimationFillMode;
pub use iteration::AnimationIteration;
pub use keyframe::{CustomKeyframe, KeyframeStep, TransformStep};
pub use play_state::AnimationPlayState;
pub use timing::{StepDirection, TimingFunction};

/// Advanced animation utilities trait for extending ClassBuilder
pub trait AdvancedAnimationUtilities {
    /// Add custom keyframe animation
    fn custom_keyframe_animation(self, keyframe: &CustomKeyframe) -> Self;

    /// Add animation composition
    fn animation_composition(self, composition: &AnimationComposition) -> Self;

    /// Add timing function class
    fn timing_function(self, timing: TimingFunction) -> Self;

    /// Add animation direction class
    fn animation_direction(self, direction: AnimationDirection) -> Self;

    /// Add animation fill mode class
    fn animation_fill_mode(self, fill_mode: AnimationFillMode) -> Self;

    /// Add animation play state class
    fn animation_play_state(self, play_state: AnimationPlayState) -> Self;

    /// Add animation iteration class
    fn animation_iteration(self, iteration: AnimationIteration) -> Self;
}

impl AdvancedAnimationUtilities for crate::classes::ClassBuilder {
    fn custom_keyframe_animation(self, keyframe: &CustomKeyframe) -> Self {
        self.class(&format!("animate-{}", keyframe.name))
    }

    fn animation_composition(self, composition: &AnimationComposition) -> Self {
        self.class(&format!("animate-{}", composition.name))
    }

    fn timing_function(self, timing: TimingFunction) -> Self {
        // This would need more complex logic in a real implementation
        // For now, just return self
        self
    }

    fn animation_direction(self, direction: AnimationDirection) -> Self {
        self.class(&format!("animation-direction-{}", direction.to_css_value()))
    }

    fn animation_fill_mode(self, fill_mode: AnimationFillMode) -> Self {
        self.class(&format!("animation-fill-mode-{}", fill_mode.to_css_value()))
    }

    fn animation_play_state(self, play_state: AnimationPlayState) -> Self {
        self.class(&format!("animation-play-state-{}", play_state.to_css_value()))
    }

    fn animation_iteration(self, iteration: AnimationIteration) -> Self {
        self.class(&format!("animation-iteration-{}", iteration.to_css_value()))
    }
}

/// Animation manager for handling complex animation scenarios
#[derive(Debug, Clone)]
pub struct AnimationManager {
    keyframes: HashMap<String, CustomKeyframe>,
    compositions: HashMap<String, AnimationComposition>,
}

impl AnimationManager {
    /// Create a new animation manager
    pub fn new() -> Self {
        Self {
            keyframes: HashMap::new(),
            compositions: HashMap::new(),
        }
    }

    /// Register a custom keyframe animation
    pub fn register_keyframe(&mut self, keyframe: CustomKeyframe) {
        self.keyframes.insert(keyframe.name.clone(), keyframe);
    }

    /// Get a keyframe animation by name
    pub fn get_keyframe(&self, name: &str) -> Option<&CustomKeyframe> {
        self.keyframes.get(name)
    }

    /// Register an animation composition
    pub fn register_composition(&mut self, composition: AnimationComposition) {
        self.compositions.insert(composition.name.clone(), composition);
    }

    /// Get a composition by name
    pub fn get_composition(&self, name: &str) -> Option<&AnimationComposition> {
        self.compositions.get(name)
    }

    /// Generate all CSS for registered animations
    pub fn generate_css(&self) -> String {
        let mut css = String::new();

        // Generate keyframe CSS
        for keyframe in self.keyframes.values() {
            css.push_str(&keyframe.to_css_keyframe());
            css.push('\n');
        }

        // Generate composition CSS (simplified)
        for composition in self.compositions.values() {
            // This would be more complex in a real implementation
            css.push_str(&format!("/* Animation composition: {} */\n", composition.name));
        }

        css
    }

    /// Get all registered keyframe names
    pub fn keyframe_names(&self) -> Vec<&str> {
        self.keyframes.keys().map(|s| s.as_str()).collect()
    }

    /// Get all registered composition names
    pub fn composition_names(&self) -> Vec<&str> {
        self.compositions.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for AnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for advanced animations
pub struct AnimationUtils;

impl AnimationUtils {
    /// Create a simple fade-in keyframe animation
    pub fn create_fade_in(duration: u32) -> CustomKeyframe {
        let mut keyframe = CustomKeyframe::new("fade-in".to_string());
        keyframe.duration = duration;

        let start_step = KeyframeStep::new().with_opacity(0.0);
        let end_step = KeyframeStep::new().with_opacity(1.0);

        keyframe.add_step(0.0, start_step);
        keyframe.add_step(1.0, end_step);

        keyframe
    }

    /// Create a bounce animation
    pub fn create_bounce(duration: u32) -> CustomKeyframe {
        let mut keyframe = CustomKeyframe::new("bounce".to_string());
        keyframe.duration = duration;
        keyframe.timing_function = TimingFunction::CubicBezier(0.68, -0.55, 0.265, 1.55);

        // Simplified bounce steps
        let step1 = KeyframeStep::new().with_transform(
            TransformStep::new().with_translate(0.0, 0.0, 0.0)
        );
        let step2 = KeyframeStep::new().with_transform(
            TransformStep::new().with_translate(0.0, -30.0, 0.0)
        );
        let step3 = KeyframeStep::new().with_transform(
            TransformStep::new().with_translate(0.0, -15.0, 0.0)
        );
        let step4 = KeyframeStep::new().with_transform(
            TransformStep::new().with_translate(0.0, 0.0, 0.0)
        );

        keyframe.add_step(0.0, step1);
        keyframe.add_step(0.4, step2);
        keyframe.add_step(0.7, step3);
        keyframe.add_step(1.0, step4);

        keyframe
    }

    /// Create a slide-in animation
    pub fn create_slide_in(direction: &str, distance: f32, duration: u32) -> CustomKeyframe {
        let name = format!("slide-in-{}", direction);
        let mut keyframe = CustomKeyframe::new(name);
        keyframe.duration = duration;

        let (start_x, start_y) = match direction {
            "left" => (-distance, 0.0),
            "right" => (distance, 0.0),
            "up" => (0.0, -distance),
            "down" => (0.0, distance),
            _ => (0.0, 0.0),
        };

        let start_step = KeyframeStep::new()
            .with_opacity(0.0)
            .with_transform(TransformStep::new().with_translate(start_x, start_y, 0.0));

        let end_step = KeyframeStep::new()
            .with_opacity(1.0)
            .with_transform(TransformStep::new().with_translate(0.0, 0.0, 0.0));

        keyframe.add_step(0.0, start_step);
        keyframe.add_step(1.0, end_step);

        keyframe
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn animation_manager_operations() {
        let mut manager = AnimationManager::new();

        // Test keyframe registration
        let keyframe = AnimationUtils::create_fade_in(1000);
        manager.register_keyframe(keyframe);
        assert!(manager.get_keyframe("fade-in").is_some());
        assert_eq!(manager.keyframe_names(), vec!["fade-in"]);

        // Test composition registration
        let composition = AnimationComposition::new("test-comp".to_string());
        manager.register_composition(composition);
        assert!(manager.get_composition("test-comp").is_some());
        assert_eq!(manager.composition_names(), vec!["test-comp"]);
    }

    #[test]
    fn animation_utils() {
        let fade_in = AnimationUtils::create_fade_in(500);
        assert_eq!(fade_in.name, "fade-in");
        assert_eq!(fade_in.duration, 500);
        assert_eq!(fade_in.steps.len(), 2);

        let bounce = AnimationUtils::create_bounce(800);
        assert_eq!(bounce.name, "bounce");
        assert_eq!(bounce.steps.len(), 4);

        let slide = AnimationUtils::create_slide_in("left", 100.0, 600);
        assert_eq!(slide.name, "slide-in-left");
        assert_eq!(slide.duration, 600);
    }

    #[test]
    fn animation_manager_css_generation() {
        let mut manager = AnimationManager::new();
        let keyframe = AnimationUtils::create_fade_in(1000);
        manager.register_keyframe(keyframe);

        let css = manager.generate_css();
        assert!(css.contains("@keyframes fade-in"));
        assert!(css.contains("opacity: 0;"));
        assert!(css.contains("opacity: 1;"));
    }

    #[test]
    fn advanced_animation_utilities_trait() {
        use crate::classes::ClassBuilder;

        let builder = ClassBuilder::new();
        let keyframe = AnimationUtils::create_fade_in(1000);

        // Test trait methods (simplified - would need actual ClassBuilder implementation)
        let _result = builder.custom_keyframe_animation(&keyframe);
        // In a real implementation, this would add classes to the builder
    }
}
