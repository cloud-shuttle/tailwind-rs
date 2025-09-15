//! Animation utilities for tailwind-rs
//!
//! This module provides utilities for CSS animations including animate-none,
//! animate-spin, animate-ping, animate-pulse, and animate-bounce.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Animation values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Animation {
    /// No animation
    None,
    /// Spin animation
    Spin,
    /// Ping animation
    Ping,
    /// Pulse animation
    Pulse,
    /// Bounce animation
    Bounce,
    /// Fade in animation
    FadeIn,
    /// Fade out animation
    FadeOut,
    /// Slide in from left
    SlideInLeft,
    /// Slide in from right
    SlideInRight,
    /// Slide in from top
    SlideInTop,
    /// Slide in from bottom
    SlideInBottom,
    /// Zoom in animation
    ZoomIn,
    /// Zoom out animation
    ZoomOut,
    /// Wobble animation
    Wobble,
    /// Shake animation
    Shake,
    /// Flip animation
    Flip,
    /// Heartbeat animation
    Heartbeat,
}

impl Animation {
    pub fn to_class_name(&self) -> String {
        match self {
            Animation::None => "none".to_string(),
            Animation::Spin => "spin".to_string(),
            Animation::Ping => "ping".to_string(),
            Animation::Pulse => "pulse".to_string(),
            Animation::Bounce => "bounce".to_string(),
            Animation::FadeIn => "fade-in".to_string(),
            Animation::FadeOut => "fade-out".to_string(),
            Animation::SlideInLeft => "slide-in-left".to_string(),
            Animation::SlideInRight => "slide-in-right".to_string(),
            Animation::SlideInTop => "slide-in-top".to_string(),
            Animation::SlideInBottom => "slide-in-bottom".to_string(),
            Animation::ZoomIn => "zoom-in".to_string(),
            Animation::ZoomOut => "zoom-out".to_string(),
            Animation::Wobble => "wobble".to_string(),
            Animation::Shake => "shake".to_string(),
            Animation::Flip => "flip".to_string(),
            Animation::Heartbeat => "heartbeat".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            Animation::None => "none".to_string(),
            Animation::Spin => "spin 1s linear infinite".to_string(),
            Animation::Ping => "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite".to_string(),
            Animation::Pulse => "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite".to_string(),
            Animation::Bounce => "bounce 1s infinite".to_string(),
            Animation::FadeIn => "fadeIn 0.5s ease-in".to_string(),
            Animation::FadeOut => "fadeOut 0.5s ease-out".to_string(),
            Animation::SlideInLeft => "slideInLeft 0.5s ease-out".to_string(),
            Animation::SlideInRight => "slideInRight 0.5s ease-out".to_string(),
            Animation::SlideInTop => "slideInTop 0.5s ease-out".to_string(),
            Animation::SlideInBottom => "slideInBottom 0.5s ease-out".to_string(),
            Animation::ZoomIn => "zoomIn 0.5s ease-out".to_string(),
            Animation::ZoomOut => "zoomOut 0.5s ease-in".to_string(),
            Animation::Wobble => "wobble 1s ease-in-out".to_string(),
            Animation::Shake => "shake 0.5s ease-in-out".to_string(),
            Animation::Flip => "flip 1s ease-in-out".to_string(),
            Animation::Heartbeat => "heartbeat 1.5s ease-in-out infinite".to_string(),
        }
    }

    /// Get all available animation values
    pub fn all_values() -> Vec<Animation> {
        vec![
            Animation::None,
            Animation::Spin,
            Animation::Ping,
            Animation::Pulse,
            Animation::Bounce,
            Animation::FadeIn,
            Animation::FadeOut,
            Animation::SlideInLeft,
            Animation::SlideInRight,
            Animation::SlideInTop,
            Animation::SlideInBottom,
            Animation::ZoomIn,
            Animation::ZoomOut,
            Animation::Wobble,
            Animation::Shake,
            Animation::Flip,
            Animation::Heartbeat,
        ]
    }

    /// Check if animation is an infinite animation
    pub fn is_infinite(&self) -> bool {
        matches!(self, Animation::Spin | Animation::Ping | Animation::Pulse | Animation::Bounce | Animation::Heartbeat)
    }

    /// Get animation duration in milliseconds
    pub fn duration_ms(&self) -> u32 {
        match self {
            Animation::None => 0,
            Animation::Spin => 1000,
            Animation::Ping => 1000,
            Animation::Pulse => 2000,
            Animation::Bounce => 1000,
            Animation::FadeIn | Animation::FadeOut => 500,
            Animation::SlideInLeft | Animation::SlideInRight |
            Animation::SlideInTop | Animation::SlideInBottom => 500,
            Animation::ZoomIn | Animation::ZoomOut => 500,
            Animation::Shake => 500,
            Animation::Wobble | Animation::Flip => 1000,
            Animation::Heartbeat => 1500,
        }
    }
}

impl fmt::Display for Animation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

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
        self.class("animation-play-state-paused".to_string())
    }

    fn animation_resume(self) -> Self {
        self.class("animation-play-state-running".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_animation_utilities() {
        let classes = ClassBuilder::new()
            .animation(Animation::None)
            .animation(Animation::Spin)
            .animation(Animation::Ping)
            .animation(Animation::Pulse)
            .animation(Animation::Bounce)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("animate-none"));
        assert!(css_classes.contains("animate-spin"));
        assert!(css_classes.contains("animate-ping"));
        assert!(css_classes.contains("animate-pulse"));
        assert!(css_classes.contains("animate-bounce"));
    }
    
    /// Test that all Week 12 animation utilities are implemented
    #[test]
    fn test_week12_animation_utilities() {
        // Test all Week 12 animation utilities
        let classes = ClassBuilder::new()
            .animation(Animation::None)
            .animation(Animation::Spin)
            .animation(Animation::Ping)
            .animation(Animation::Pulse)
            .animation(Animation::Bounce)
            .build();

        let css_classes = classes.to_css_classes();

        // Animations
        assert!(css_classes.contains("animate-none"));
        assert!(css_classes.contains("animate-spin"));
        assert!(css_classes.contains("animate-ping"));
        assert!(css_classes.contains("animate-pulse"));
        assert!(css_classes.contains("animate-bounce"));
    }

    /// Test extended animation utilities
    #[test]
    fn test_extended_animation_utilities() {
        let classes = ClassBuilder::new()
            .animation(Animation::FadeIn)
            .animation(Animation::FadeOut)
            .animation(Animation::SlideInLeft)
            .animation(Animation::SlideInRight)
            .animation(Animation::SlideInTop)
            .animation(Animation::SlideInBottom)
            .animation(Animation::ZoomIn)
            .animation(Animation::ZoomOut)
            .animation(Animation::Wobble)
            .animation(Animation::Shake)
            .animation(Animation::Flip)
            .animation(Animation::Heartbeat)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("animate-fade-in"));
        assert!(css_classes.contains("animate-fade-out"));
        assert!(css_classes.contains("animate-slide-in-left"));
        assert!(css_classes.contains("animate-slide-in-right"));
        assert!(css_classes.contains("animate-slide-in-top"));
        assert!(css_classes.contains("animate-slide-in-bottom"));
        assert!(css_classes.contains("animate-zoom-in"));
        assert!(css_classes.contains("animate-zoom-out"));
        assert!(css_classes.contains("animate-wobble"));
        assert!(css_classes.contains("animate-shake"));
        assert!(css_classes.contains("animate-flip"));
        assert!(css_classes.contains("animate-heartbeat"));
    }

    /// Test convenience animation methods
    #[test]
    fn test_convenience_animation_methods() {
        let classes = ClassBuilder::new()
            .fade_in()
            .fade_out()
            .slide_in_left()
            .slide_in_right()
            .slide_in_top()
            .slide_in_bottom()
            .zoom_in()
            .zoom_out()
            .wobble()
            .shake()
            .flip()
            .heartbeat()
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("animate-fade-in"));
        assert!(css_classes.contains("animate-fade-out"));
        assert!(css_classes.contains("animate-slide-in-left"));
        assert!(css_classes.contains("animate-slide-in-right"));
        assert!(css_classes.contains("animate-slide-in-top"));
        assert!(css_classes.contains("animate-slide-in-bottom"));
        assert!(css_classes.contains("animate-zoom-in"));
        assert!(css_classes.contains("animate-zoom-out"));
        assert!(css_classes.contains("animate-wobble"));
        assert!(css_classes.contains("animate-shake"));
        assert!(css_classes.contains("animate-flip"));
        assert!(css_classes.contains("animate-heartbeat"));
    }

    /// Test animation with duration
    #[test]
    fn test_animation_with_duration() {
        let classes = ClassBuilder::new()
            .animation_with_duration(Animation::FadeIn, 1000)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("animate-fade-in"));
        assert!(css_classes.contains("duration-1000"));
    }

    /// Test animation repetition controls
    #[test]
    fn test_animation_repetition_controls() {
        let classes = ClassBuilder::new()
            .animation_once(Animation::Bounce)
            .animation_repeat(Animation::Shake, 3)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("animate-bounce"));
        assert!(css_classes.contains("animation-iteration-count-1"));
        assert!(css_classes.contains("animate-shake"));
        assert!(css_classes.contains("animation-iteration-count-3"));
    }

    /// Test hover and focus animations
    #[test]
    fn test_hover_focus_animations() {
        let classes = ClassBuilder::new()
            .hover_animation(Animation::Bounce)
            .focus_animation(Animation::Pulse)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("hover:animate-bounce"));
        assert!(css_classes.contains("focus:animate-pulse"));
    }

    /// Test animation play state controls
    #[test]
    fn test_animation_play_state() {
        let classes = ClassBuilder::new()
            .animation_pause()
            .animation_resume()
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("animation-play-state-paused"));
        assert!(css_classes.contains("animation-play-state-running"));
    }

    /// Test animation properties
    #[test]
    fn test_animation_properties() {
        // Test all animations are available
        let all_animations = Animation::all_values();
        assert_eq!(all_animations.len(), 17);
        assert!(all_animations.contains(&Animation::None));
        assert!(all_animations.contains(&Animation::FadeIn));
        assert!(all_animations.contains(&Animation::Heartbeat));

        // Test infinite animations
        assert!(Animation::Spin.is_infinite());
        assert!(Animation::Heartbeat.is_infinite());
        assert!(!Animation::FadeIn.is_infinite());
        assert!(!Animation::ZoomOut.is_infinite());

        // Test durations
        assert_eq!(Animation::None.duration_ms(), 0);
        assert_eq!(Animation::FadeIn.duration_ms(), 500);
        assert_eq!(Animation::Spin.duration_ms(), 1000);
        assert_eq!(Animation::Heartbeat.duration_ms(), 1500);
        assert_eq!(Animation::Pulse.duration_ms(), 2000);
    }

    /// Test animation CSS values
    #[test]
    fn test_animation_css_values() {
        assert_eq!(Animation::None.to_css_value(), "none");
        assert_eq!(Animation::FadeIn.to_css_value(), "fadeIn 0.5s ease-in");
        assert_eq!(Animation::SlideInLeft.to_css_value(), "slideInLeft 0.5s ease-out");
        assert_eq!(Animation::Wobble.to_css_value(), "wobble 1s ease-in-out");
        assert_eq!(Animation::Heartbeat.to_css_value(), "heartbeat 1.5s ease-in-out infinite");
    }

    /// Test animation class names
    #[test]
    fn test_animation_class_names() {
        assert_eq!(Animation::FadeIn.to_class_name(), "fade-in");
        assert_eq!(Animation::SlideInLeft.to_class_name(), "slide-in-left");
        assert_eq!(Animation::ZoomOut.to_class_name(), "zoom-out");
        assert_eq!(Animation::Heartbeat.to_class_name(), "heartbeat");
    }

    /// Test comprehensive animation system
    #[test]
    fn test_comprehensive_animation_system() {
        let classes = ClassBuilder::new()
            // Basic animations
            .animation(Animation::Spin)
            .animation(Animation::Bounce)
            // Advanced animations
            .fade_in()
            .slide_in_right()
            .zoom_in()
            // State-based animations
            .hover_animation(Animation::Shake)
            .focus_animation(Animation::Wobble)
            // Animation controls
            .animation_once(Animation::Flip)
            .animation_with_duration(Animation::Heartbeat, 2000)
            .animation_pause()
            .build();

        let css_classes = classes.to_css_classes();

        // Basic animations
        assert!(css_classes.contains("animate-spin"));
        assert!(css_classes.contains("animate-bounce"));

        // Advanced animations
        assert!(css_classes.contains("animate-fade-in"));
        assert!(css_classes.contains("animate-slide-in-right"));
        assert!(css_classes.contains("animate-zoom-in"));

        // State-based animations
        assert!(css_classes.contains("hover:animate-shake"));
        assert!(css_classes.contains("focus:animate-wobble"));

        // Animation controls
        assert!(css_classes.contains("animate-flip"));
        assert!(css_classes.contains("animation-iteration-count-1"));
        assert!(css_classes.contains("animate-heartbeat"));
        assert!(css_classes.contains("duration-2000"));
        assert!(css_classes.contains("animation-play-state-paused"));
    }
}
