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
}

impl Animation {
    pub fn to_class_name(&self) -> String {
        match self {
            Animation::None => "none".to_string(),
            Animation::Spin => "spin".to_string(),
            Animation::Ping => "ping".to_string(),
            Animation::Pulse => "pulse".to_string(),
            Animation::Bounce => "bounce".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Animation::None => "none".to_string(),
            Animation::Spin => "spin 1s linear infinite".to_string(),
            Animation::Ping => "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite".to_string(),
            Animation::Pulse => "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite".to_string(),
            Animation::Bounce => "bounce 1s infinite".to_string(),
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
}

impl AnimationUtilities for ClassBuilder {
    fn animation(self, animation: Animation) -> Self {
        self.class(format!("animate-{}", animation.to_class_name()))
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
}
