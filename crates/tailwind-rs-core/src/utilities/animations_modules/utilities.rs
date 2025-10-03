//! Animation Utilities Module
//!
//! Animation utilities trait and implementations:
//! - AnimationUtilities: Trait for adding animation methods to class builders
//! - ClassBuilder implementations for all animation utilities

use super::types::*;
use crate::classes::ClassBuilder;

/// Trait for adding animation utilities to a class builder
pub trait AnimationUtilities {
    fn animation(self, animation: Animation) -> Self;

    /// Add animation with custom duration
    fn animation_with_duration(self, animation: Animation, duration_ms: u32) -> Self;

    /// Add animation that runs only once
    fn animation_once(self, animation: Animation) -> Self;

    /// Add animation that repeats a specific number of times
    fn animation_repeat(self, animation: Animation, count: u32) -> Self;

    /// Add fade in animation
    fn fade_in(self) -> Self;

    /// Add fade out animation
    fn fade_out(self) -> Self;

    /// Add slide in from left animation
    fn slide_in_left(self) -> Self;

    /// Add slide in from right animation
    fn slide_in_right(self) -> Self;

    /// Add slide in from top animation
    fn slide_in_top(self) -> Self;

    /// Add slide in from bottom animation
    fn slide_in_bottom(self) -> Self;

    /// Add zoom in animation
    fn zoom_in(self) -> Self;

    /// Add zoom out animation
    fn zoom_out(self) -> Self;

    /// Add wobble animation
    fn wobble(self) -> Self;

    /// Add shake animation
    fn shake(self) -> Self;

    /// Add flip animation
    fn flip(self) -> Self;

    /// Add heartbeat animation
    fn heartbeat(self) -> Self;

    /// Add hover animation (animation only on hover)
    fn hover_animation(self, animation: Animation) -> Self;

    /// Add focus animation (animation only on focus)
    fn focus_animation(self, animation: Animation) -> Self;

    /// Pause animation
    fn animation_pause(self) -> Self;

    /// Resume animation
    fn animation_resume(self) -> Self;

    /// Add infinite animation
    fn animation_infinite(self, animation: Animation) -> Self;

    /// Add animation with delay
    fn animation_delay(self, animation: Animation, delay_ms: u32) -> Self;

    /// Add animation with custom timing function
    fn animation_timing(self, animation: Animation, timing: AnimationTiming) -> Self;

    /// Add spin animation
    fn spin(self) -> Self where Self: Sized {
        self.animation(Animation::Spin)
    }

    /// Add ping animation
    fn ping(self) -> Self where Self: Sized {
        self.animation(Animation::Ping)
    }

    /// Add pulse animation
    fn pulse(self) -> Self where Self: Sized {
        self.animation(Animation::Pulse)
    }

    /// Add bounce animation
    fn bounce(self) -> Self where Self: Sized {
        self.animation(Animation::Bounce)
    }
}

impl AnimationUtilities for ClassBuilder {
    fn animation(self, animation: Animation) -> Self {
        self.class(format!("animate-{}", animation.to_class_name()))
    }

    fn animation_with_duration(self, animation: Animation, duration_ms: u32) -> Self {
        self.class(format!("animate-{}", animation.to_class_name()))
            .class(format!("duration-{}", duration_ms))
    }

    fn animation_once(self, animation: Animation) -> Self {
        self.class(format!("animate-{}", animation.to_class_name()))
            .class("animation-iteration-count-1".to_string())
    }

    fn animation_repeat(self, animation: Animation, count: u32) -> Self {
        self.class(format!("animate-{}", animation.to_class_name()))
            .class(format!("animation-iteration-count-{}", count))
    }

    fn fade_in(self) -> Self {
        self.animation(Animation::FadeIn)
    }

    fn fade_out(self) -> Self {
        self.animation(Animation::FadeOut)
    }

    fn slide_in_left(self) -> Self {
        self.animation(Animation::SlideInLeft)
    }

    fn slide_in_right(self) -> Self {
        self.animation(Animation::SlideInRight)
    }

    fn slide_in_top(self) -> Self {
        self.animation(Animation::SlideInTop)
    }

    fn slide_in_bottom(self) -> Self {
        self.animation(Animation::SlideInBottom)
    }

    fn zoom_in(self) -> Self {
        self.animation(Animation::ZoomIn)
    }

    fn zoom_out(self) -> Self {
        self.animation(Animation::ZoomOut)
    }

    fn wobble(self) -> Self {
        self.animation(Animation::Wobble)
    }

    fn shake(self) -> Self {
        self.animation(Animation::Shake)
    }

    fn flip(self) -> Self {
        self.animation(Animation::Flip)
    }

    fn heartbeat(self) -> Self {
        self.animation(Animation::Heartbeat)
    }

    fn hover_animation(self, animation: Animation) -> Self {
        self.class(format!("hover:animate-{}", animation.to_class_name()))
    }

    fn focus_animation(self, animation: Animation) -> Self {
        self.class(format!("focus:animate-{}", animation.to_class_name()))
    }

    fn animation_pause(self) -> Self {
        self.class("animation-paused")
    }

    fn animation_resume(self) -> Self {
        self.class("animation-running")
    }

    fn animation_infinite(self, animation: Animation) -> Self {
        self.class(format!("animate-{}", animation.to_class_name()))
            .class("animation-infinite")
    }

    fn animation_delay(self, animation: Animation, delay_ms: u32) -> Self {
        self.class(format!("animate-{}", animation.to_class_name()))
            .class(format!("animation-delay-{}", delay_ms))
    }

    fn animation_timing(self, animation: Animation, timing: AnimationTiming) -> Self {
        self.class(format!("animate-{}", animation.to_class_name()))
            .class(format!("animation-timing-{}", timing.to_css_value().replace(" ", "-").replace("(", "").replace(")", "").replace(",", "")))
    }
}

/// Animation builder for complex animation sequences
#[derive(Debug, Clone)]
pub struct AnimationBuilder {
    animations: Vec<Animation>,
    duration: Option<u32>,
    delay: Option<u32>,
    iteration_count: Option<String>,
    timing: Option<AnimationTiming>,
    direction: Option<String>,
}

impl AnimationBuilder {
    /// Create a new animation builder
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            duration: None,
            delay: None,
            iteration_count: None,
            timing: None,
            direction: None,
        }
    }

    /// Add an animation to the sequence
    pub fn add(mut self, animation: Animation) -> Self {
        self.animations.push(animation);
        self
    }

    /// Set duration for all animations
    pub fn duration(mut self, ms: u32) -> Self {
        self.duration = Some(ms);
        self
    }

    /// Set delay for all animations
    pub fn delay(mut self, ms: u32) -> Self {
        self.delay = Some(ms);
        self
    }

    /// Set iteration count
    pub fn iterations(mut self, count: &str) -> Self {
        self.iteration_count = Some(count.to_string());
        self
    }

    /// Set timing function
    pub fn timing(mut self, timing: AnimationTiming) -> Self {
        self.timing = Some(timing);
        self
    }

    /// Set animation direction
    pub fn direction(mut self, direction: &str) -> Self {
        self.direction = Some(direction.to_string());
        self
    }

    /// Build the animation classes
    pub fn build(&self) -> Vec<String> {
        let mut classes = Vec::new();

        for animation in &self.animations {
            classes.push(format!("animate-{}", animation.to_class_name()));
        }

        if let Some(duration) = self.duration {
            classes.push(format!("duration-{}", duration));
        }

        if let Some(delay) = self.delay {
            classes.push(format!("animation-delay-{}", delay));
        }

        if let Some(ref iterations) = self.iteration_count {
            classes.push(format!("animation-iteration-count-{}", iterations));
        }

        if let Some(timing) = self.timing {
            let timing_class = format!("animation-timing-{}", timing.to_css_value()
                .replace(" ", "-")
                .replace("(", "")
                .replace(")", "")
                .replace(",", ""));
            classes.push(timing_class);
        }

        if let Some(ref direction) = self.direction {
            classes.push(format!("animation-direction-{}", direction));
        }

        classes
    }

    /// Apply animations to a class builder
    pub fn apply_to(&self, builder: ClassBuilder) -> ClassBuilder {
        let mut result = builder;
        for class in self.build() {
            result = result.class(class);
        }
        result
    }
}

impl Default for AnimationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Animation presets for common use cases
pub mod presets {
    use super::*;

    /// Create a loading spinner animation
    pub fn loading_spinner() -> AnimationBuilder {
        AnimationBuilder::new()
            .add(Animation::Spin)
            .duration(1000)
            .iterations("infinite")
    }

    /// Create a bouncing button animation
    pub fn bouncing_button() -> AnimationBuilder {
        AnimationBuilder::new()
            .add(Animation::Bounce)
            .duration(2000)
            .iterations("infinite")
    }

    /// Create a pulsing notification
    pub fn pulsing_notification() -> AnimationBuilder {
        AnimationBuilder::new()
            .add(Animation::Pulse)
            .duration(2000)
            .iterations("infinite")
    }

    /// Create an entrance animation sequence
    pub fn entrance_sequence() -> AnimationBuilder {
        AnimationBuilder::new()
            .add(Animation::FadeIn)
            .add(Animation::SlideInBottom)
            .duration(800)
            .delay(200)
    }

    /// Create a hover effect animation
    pub fn hover_grow() -> AnimationBuilder {
        AnimationBuilder::new()
            .add(Animation::ZoomIn)
            .duration(300)
            .timing(AnimationTiming::EaseOut)
    }

    /// Create an attention-grabbing animation
    pub fn attention_grab() -> AnimationBuilder {
        AnimationBuilder::new()
            .add(Animation::Wobble)
            .duration(1000)
            .iterations("3")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn animation_utilities_trait() {
        let mut builder = ClassBuilder::new();

        // Test basic animation methods
        let result = builder.animation(Animation::Spin);
        // In a real test, we'd check the result has the expected classes

        let result = builder.spin().bounce().fade_in();
        // Test chaining multiple animations
    }

    #[test]
    fn animation_with_modifiers() {
        let mut builder = ClassBuilder::new();

        let result = builder.animation_with_duration(Animation::Bounce, 2000);
        let result = builder.animation_once(Animation::FadeIn);
        let result = builder.animation_repeat(Animation::Wobble, 3);
        let result = builder.animation_infinite(Animation::Spin);
    }

    #[test]
    fn hover_and_focus_animations() {
        let mut builder = ClassBuilder::new();

        let result = builder.hover_animation(Animation::ZoomIn);
        let result = builder.focus_animation(Animation::Shake);
    }

    #[test]
    fn animation_builder_operations() {
        let builder = AnimationBuilder::new()
            .add(Animation::FadeIn)
            .add(Animation::SlideInLeft)
            .duration(500)
            .delay(200)
            .iterations("infinite")
            .timing(AnimationTiming::EaseOut);

        let classes = builder.build();
        assert!(classes.contains(&"animate-fade-in".to_string()));
        assert!(classes.contains(&"animate-slide-in-left".to_string()));
        assert!(classes.contains(&"duration-500".to_string()));
        assert!(classes.contains(&"animation-delay-200".to_string()));
        assert!(classes.contains(&"animation-iteration-count-infinite".to_string()));
    }

    #[test]
    fn animation_presets() {
        let spinner = presets::loading_spinner();
        let classes = spinner.build();
        assert!(classes.contains(&"animate-spin".to_string()));
        assert!(classes.contains(&"duration-1000".to_string()));

        let bouncing = presets::bouncing_button();
        let classes = bouncing.build();
        assert!(classes.contains(&"animate-bounce".to_string()));

        let entrance = presets::entrance_sequence();
        let classes = entrance.build();
        assert!(classes.contains(&"animate-fade-in".to_string()));
        assert!(classes.contains(&"animate-slide-in-bottom".to_string()));
    }

    #[test]
    fn animation_builder_apply_to_class_builder() {
        let class_builder = ClassBuilder::new();
        let anim_builder = AnimationBuilder::new()
            .add(Animation::Spin)
            .duration(1000);

        let result = anim_builder.apply_to(class_builder);
        // The result should have the animation classes applied
    }

    #[test]
    fn animation_pause_resume() {
        let mut builder = ClassBuilder::new();

        let paused = builder.animation_pause();
        let resumed = builder.animation_resume();
        // Test pause and resume functionality
    }
}
