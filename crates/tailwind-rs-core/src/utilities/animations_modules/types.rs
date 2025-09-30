//! Animation Types Module
//!
//! Core types and enums for animations:
//! - Animation: Different animation types supported by Tailwind
//! - Animation properties and behaviors

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
    /// Convert to class name for CSS
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

    /// Convert to CSS animation value
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
        matches!(
            self,
            Animation::Spin
                | Animation::Ping
                | Animation::Pulse
                | Animation::Bounce
                | Animation::Heartbeat
        )
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
            Animation::SlideInLeft
            | Animation::SlideInRight
            | Animation::SlideInTop
            | Animation::SlideInBottom => 500,
            Animation::ZoomIn | Animation::ZoomOut => 500,
            Animation::Shake => 500,
            Animation::Wobble | Animation::Flip => 1000,
            Animation::Heartbeat => 1500,
        }
    }

    /// Get animation category
    pub fn category(&self) -> AnimationCategory {
        match self {
            Animation::None => AnimationCategory::None,
            Animation::Spin | Animation::Ping | Animation::Pulse | Animation::Bounce => AnimationCategory::Infinite,
            Animation::FadeIn | Animation::FadeOut => AnimationCategory::Fade,
            Animation::SlideInLeft | Animation::SlideInRight | Animation::SlideInTop | Animation::SlideInBottom => AnimationCategory::Slide,
            Animation::ZoomIn | Animation::ZoomOut => AnimationCategory::Zoom,
            Animation::Wobble | Animation::Shake | Animation::Flip | Animation::Heartbeat => AnimationCategory::Attention,
        }
    }

    /// Check if animation is a basic Tailwind animation
    pub fn is_basic(&self) -> bool {
        matches!(self, Animation::Spin | Animation::Ping | Animation::Pulse | Animation::Bounce)
    }

    /// Check if animation is an extended animation
    pub fn is_extended(&self) -> bool {
        !self.is_basic() && !matches!(self, Animation::None)
    }
}

/// Animation categories for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationCategory {
    /// No animation
    None,
    /// Infinite looping animations
    Infinite,
    /// Fade animations
    Fade,
    /// Slide animations
    Slide,
    /// Zoom animations
    Zoom,
    /// Attention-grabbing animations
    Attention,
}

impl AnimationCategory {
    /// Get all animations in this category
    pub fn animations(&self) -> Vec<Animation> {
        Animation::all_values().into_iter().filter(|a| a.category() == *self).collect()
    }

    /// Get category display name
    pub fn display_name(&self) -> &'static str {
        match self {
            AnimationCategory::None => "None",
            AnimationCategory::Infinite => "Infinite",
            AnimationCategory::Fade => "Fade",
            AnimationCategory::Slide => "Slide",
            AnimationCategory::Zoom => "Zoom",
            AnimationCategory::Attention => "Attention",
        }
    }
}

/// Animation timing function types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationTiming {
    /// Linear timing
    Linear,
    /// Ease timing
    Ease,
    /// Ease in timing
    EaseIn,
    /// Ease out timing
    EaseOut,
    /// Ease in out timing
    EaseInOut,
    /// Cubic bezier timing
    CubicBezier,
}

impl AnimationTiming {
    /// Convert to CSS timing function
    pub fn to_css_value(&self) -> &'static str {
        match self {
            AnimationTiming::Linear => "linear",
            AnimationTiming::Ease => "ease",
            AnimationTiming::EaseIn => "ease-in",
            AnimationTiming::EaseOut => "ease-out",
            AnimationTiming::EaseInOut => "ease-in-out",
            AnimationTiming::CubicBezier => "cubic-bezier(0.4, 0, 0.2, 1)",
        }
    }
}

impl fmt::Display for Animation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn animation_class_names() {
        assert_eq!(Animation::Spin.to_class_name(), "spin");
        assert_eq!(Animation::Bounce.to_class_name(), "bounce");
        assert_eq!(Animation::FadeIn.to_class_name(), "fade-in");
    }

    #[test]
    fn animation_css_values() {
        assert_eq!(Animation::None.to_css_value(), "none");
        assert!(Animation::Spin.to_css_value().contains("spin"));
        assert!(Animation::Bounce.to_css_value().contains("bounce"));
    }

    #[test]
    fn animation_properties() {
        assert!(Animation::Spin.is_infinite());
        assert!(Animation::Bounce.is_infinite());
        assert!(!Animation::FadeIn.is_infinite());

        assert_eq!(Animation::Spin.duration_ms(), 1000);
        assert_eq!(Animation::FadeIn.duration_ms(), 500);
    }

    #[test]
    fn animation_categories() {
        assert_eq!(Animation::Spin.category(), AnimationCategory::Infinite);
        assert_eq!(Animation::FadeIn.category(), AnimationCategory::Fade);
        assert_eq!(Animation::SlideInLeft.category(), AnimationCategory::Slide);
        assert_eq!(Animation::Wobble.category(), AnimationCategory::Attention);
    }

    #[test]
    fn animation_classification() {
        assert!(Animation::Spin.is_basic());
        assert!(Animation::Bounce.is_basic());
        assert!(!Animation::FadeIn.is_basic());

        assert!(Animation::FadeIn.is_extended());
        assert!(Animation::Wobble.is_extended());
        assert!(!Animation::Spin.is_extended());
    }

    #[test]
    fn category_animations() {
        let infinite = AnimationCategory::Infinite.animations();
        assert!(infinite.contains(&Animation::Spin));
        assert!(infinite.contains(&Animation::Bounce));

        let fade = AnimationCategory::Fade.animations();
        assert!(fade.contains(&Animation::FadeIn));
        assert!(fade.contains(&Animation::FadeOut));
    }

    #[test]
    fn timing_functions() {
        assert_eq!(AnimationTiming::Linear.to_css_value(), "linear");
        assert_eq!(AnimationTiming::Ease.to_css_value(), "ease");
        assert!(AnimationTiming::CubicBezier.to_css_value().contains("cubic-bezier"));
    }
}
