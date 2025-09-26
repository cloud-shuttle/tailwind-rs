//! Transition utilities for tailwind-rs
//!
//! This module provides utilities for CSS transitions including duration,
//! timing function, delay, and property-specific transitions.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Transition duration values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransitionDuration {
    /// 75ms duration
    Duration75,
    /// 100ms duration
    Duration100,
    /// 150ms duration
    Duration150,
    /// 200ms duration
    Duration200,
    /// 300ms duration
    Duration300,
    /// 500ms duration
    Duration500,
    /// 700ms duration
    Duration700,
    /// 1000ms duration
    Duration1000,
}

/// Transition timing function values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransitionTimingFunction {
    /// Linear timing
    Linear,
    /// In timing
    In,
    /// Out timing
    Out,
    /// In-out timing
    InOut,
}

/// Transition delay values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransitionDelay {
    /// 75ms delay
    Delay75,
    /// 100ms delay
    Delay100,
    /// 150ms delay
    Delay150,
    /// 200ms delay
    Delay200,
    /// 300ms delay
    Delay300,
    /// 500ms delay
    Delay500,
    /// 700ms delay
    Delay700,
    /// 1000ms delay
    Delay1000,
}

/// Transition property values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransitionProperty {
    /// All properties
    All,
    /// No properties
    None,
    /// Default properties
    Default,
    /// Colors only
    Colors,
    /// Opacity only
    Opacity,
    /// Shadow only
    Shadow,
    /// Transform only
    Transform,
}

impl TransitionDuration {
    pub fn to_class_name(&self) -> String {
        match self {
            TransitionDuration::Duration75 => "75".to_string(),
            TransitionDuration::Duration100 => "100".to_string(),
            TransitionDuration::Duration150 => "150".to_string(),
            TransitionDuration::Duration200 => "200".to_string(),
            TransitionDuration::Duration300 => "300".to_string(),
            TransitionDuration::Duration500 => "500".to_string(),
            TransitionDuration::Duration700 => "700".to_string(),
            TransitionDuration::Duration1000 => "1000".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            TransitionDuration::Duration75 => "75ms".to_string(),
            TransitionDuration::Duration100 => "100ms".to_string(),
            TransitionDuration::Duration150 => "150ms".to_string(),
            TransitionDuration::Duration200 => "200ms".to_string(),
            TransitionDuration::Duration300 => "300ms".to_string(),
            TransitionDuration::Duration500 => "500ms".to_string(),
            TransitionDuration::Duration700 => "700ms".to_string(),
            TransitionDuration::Duration1000 => "1000ms".to_string(),
        }
    }
}

impl TransitionTimingFunction {
    pub fn to_class_name(&self) -> String {
        match self {
            TransitionTimingFunction::Linear => "linear".to_string(),
            TransitionTimingFunction::In => "in".to_string(),
            TransitionTimingFunction::Out => "out".to_string(),
            TransitionTimingFunction::InOut => "in-out".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            TransitionTimingFunction::Linear => "linear".to_string(),
            TransitionTimingFunction::In => "cubic-bezier(0.4, 0, 1, 1)".to_string(),
            TransitionTimingFunction::Out => "cubic-bezier(0, 0, 0.2, 1)".to_string(),
            TransitionTimingFunction::InOut => "cubic-bezier(0.4, 0, 0.2, 1)".to_string(),
        }
    }
}

impl TransitionDelay {
    pub fn to_class_name(&self) -> String {
        match self {
            TransitionDelay::Delay75 => "75".to_string(),
            TransitionDelay::Delay100 => "100".to_string(),
            TransitionDelay::Delay150 => "150".to_string(),
            TransitionDelay::Delay200 => "200".to_string(),
            TransitionDelay::Delay300 => "300".to_string(),
            TransitionDelay::Delay500 => "500".to_string(),
            TransitionDelay::Delay700 => "700".to_string(),
            TransitionDelay::Delay1000 => "1000".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            TransitionDelay::Delay75 => "75ms".to_string(),
            TransitionDelay::Delay100 => "100ms".to_string(),
            TransitionDelay::Delay150 => "150ms".to_string(),
            TransitionDelay::Delay200 => "200ms".to_string(),
            TransitionDelay::Delay300 => "300ms".to_string(),
            TransitionDelay::Delay500 => "500ms".to_string(),
            TransitionDelay::Delay700 => "700ms".to_string(),
            TransitionDelay::Delay1000 => "1000ms".to_string(),
        }
    }
}

impl TransitionProperty {
    pub fn to_class_name(&self) -> String {
        match self {
            TransitionProperty::All => "all".to_string(),
            TransitionProperty::None => "none".to_string(),
            TransitionProperty::Default => "default".to_string(),
            TransitionProperty::Colors => "colors".to_string(),
            TransitionProperty::Opacity => "opacity".to_string(),
            TransitionProperty::Shadow => "shadow".to_string(),
            TransitionProperty::Transform => "transform".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            TransitionProperty::All => "all".to_string(),
            TransitionProperty::None => "none".to_string(),
            TransitionProperty::Default => "background-color, border-color, color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter".to_string(),
            TransitionProperty::Colors => "background-color, border-color, color, fill, stroke".to_string(),
            TransitionProperty::Opacity => "opacity".to_string(),
            TransitionProperty::Shadow => "box-shadow".to_string(),
            TransitionProperty::Transform => "transform".to_string(),
        }
    }
}

impl fmt::Display for TransitionDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for TransitionTimingFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for TransitionDelay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for TransitionProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Trait for adding transition duration utilities to a class builder
pub trait TransitionDurationUtilities {
    fn transition_duration(self, duration: TransitionDuration) -> Self;
}

impl TransitionDurationUtilities for ClassBuilder {
    fn transition_duration(self, duration: TransitionDuration) -> Self {
        self.class(format!("duration-{}", duration.to_class_name()))
    }
}

/// Trait for adding transition timing function utilities to a class builder
pub trait TransitionTimingFunctionUtilities {
    fn transition_timing_function(self, timing: TransitionTimingFunction) -> Self;
}

impl TransitionTimingFunctionUtilities for ClassBuilder {
    fn transition_timing_function(self, timing: TransitionTimingFunction) -> Self {
        self.class(format!("ease-{}", timing.to_class_name()))
    }
}

/// Trait for adding transition delay utilities to a class builder
pub trait TransitionDelayUtilities {
    fn transition_delay(self, delay: TransitionDelay) -> Self;
}

impl TransitionDelayUtilities for ClassBuilder {
    fn transition_delay(self, delay: TransitionDelay) -> Self {
        self.class(format!("delay-{}", delay.to_class_name()))
    }
}

/// Trait for adding transition property utilities to a class builder
pub trait TransitionPropertyUtilities {
    fn transition_property(self, property: TransitionProperty) -> Self;
}

impl TransitionPropertyUtilities for ClassBuilder {
    fn transition_property(self, property: TransitionProperty) -> Self {
        self.class(format!("transition-{}", property.to_class_name()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transition_duration_utilities() {
        let classes = ClassBuilder::new()
            .transition_duration(TransitionDuration::Duration75)
            .transition_duration(TransitionDuration::Duration100)
            .transition_duration(TransitionDuration::Duration150)
            .transition_duration(TransitionDuration::Duration200)
            .transition_duration(TransitionDuration::Duration300)
            .transition_duration(TransitionDuration::Duration500)
            .transition_duration(TransitionDuration::Duration700)
            .transition_duration(TransitionDuration::Duration1000)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("duration-75"));
        assert!(css_classes.contains("duration-100"));
        assert!(css_classes.contains("duration-150"));
        assert!(css_classes.contains("duration-200"));
        assert!(css_classes.contains("duration-300"));
        assert!(css_classes.contains("duration-500"));
        assert!(css_classes.contains("duration-700"));
        assert!(css_classes.contains("duration-1000"));
    }

    #[test]
    fn test_transition_timing_function_utilities() {
        let classes = ClassBuilder::new()
            .transition_timing_function(TransitionTimingFunction::Linear)
            .transition_timing_function(TransitionTimingFunction::In)
            .transition_timing_function(TransitionTimingFunction::Out)
            .transition_timing_function(TransitionTimingFunction::InOut)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("ease-linear"));
        assert!(css_classes.contains("ease-in"));
        assert!(css_classes.contains("ease-out"));
        assert!(css_classes.contains("ease-in-out"));
    }

    #[test]
    fn test_transition_delay_utilities() {
        let classes = ClassBuilder::new()
            .transition_delay(TransitionDelay::Delay75)
            .transition_delay(TransitionDelay::Delay100)
            .transition_delay(TransitionDelay::Delay150)
            .transition_delay(TransitionDelay::Delay200)
            .transition_delay(TransitionDelay::Delay300)
            .transition_delay(TransitionDelay::Delay500)
            .transition_delay(TransitionDelay::Delay700)
            .transition_delay(TransitionDelay::Delay1000)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("delay-75"));
        assert!(css_classes.contains("delay-100"));
        assert!(css_classes.contains("delay-150"));
        assert!(css_classes.contains("delay-200"));
        assert!(css_classes.contains("delay-300"));
        assert!(css_classes.contains("delay-500"));
        assert!(css_classes.contains("delay-700"));
        assert!(css_classes.contains("delay-1000"));
    }

    #[test]
    fn test_transition_property_utilities() {
        let classes = ClassBuilder::new()
            .transition_property(TransitionProperty::All)
            .transition_property(TransitionProperty::None)
            .transition_property(TransitionProperty::Default)
            .transition_property(TransitionProperty::Colors)
            .transition_property(TransitionProperty::Opacity)
            .transition_property(TransitionProperty::Shadow)
            .transition_property(TransitionProperty::Transform)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("transition-all"));
        assert!(css_classes.contains("transition-none"));
        assert!(css_classes.contains("transition-default"));
        assert!(css_classes.contains("transition-colors"));
        assert!(css_classes.contains("transition-opacity"));
        assert!(css_classes.contains("transition-shadow"));
        assert!(css_classes.contains("transition-transform"));
    }

    #[test]
    fn test_complex_transitions_combination() {
        let classes = ClassBuilder::new()
            .transition_duration(TransitionDuration::Duration300)
            .transition_timing_function(TransitionTimingFunction::InOut)
            .transition_delay(TransitionDelay::Delay100)
            .transition_property(TransitionProperty::Colors)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("duration-300"));
        assert!(css_classes.contains("ease-in-out"));
        assert!(css_classes.contains("delay-100"));
        assert!(css_classes.contains("transition-colors"));
    }

    /// Test that all Week 12 transition utilities are implemented
    #[test]
    fn test_week12_transition_utilities() {
        // Test all Week 12 transition utilities
        let classes = ClassBuilder::new()
            // Transition Properties
            .transition_property(TransitionProperty::None)
            .transition_property(TransitionProperty::All)
            .transition_property(TransitionProperty::Default)
            .transition_property(TransitionProperty::Colors)
            .transition_property(TransitionProperty::Opacity)
            .transition_property(TransitionProperty::Shadow)
            .transition_property(TransitionProperty::Transform)
            // Duration
            .transition_duration(TransitionDuration::Duration75)
            .transition_duration(TransitionDuration::Duration100)
            .transition_duration(TransitionDuration::Duration150)
            .transition_duration(TransitionDuration::Duration200)
            .transition_duration(TransitionDuration::Duration300)
            .transition_duration(TransitionDuration::Duration500)
            .transition_duration(TransitionDuration::Duration700)
            .transition_duration(TransitionDuration::Duration1000)
            // Ease
            .transition_timing_function(TransitionTimingFunction::Linear)
            .transition_timing_function(TransitionTimingFunction::In)
            .transition_timing_function(TransitionTimingFunction::Out)
            .transition_timing_function(TransitionTimingFunction::InOut)
            // Delay
            .transition_delay(TransitionDelay::Delay75)
            .transition_delay(TransitionDelay::Delay100)
            .transition_delay(TransitionDelay::Delay150)
            .transition_delay(TransitionDelay::Delay200)
            .transition_delay(TransitionDelay::Delay300)
            .transition_delay(TransitionDelay::Delay500)
            .transition_delay(TransitionDelay::Delay700)
            .transition_delay(TransitionDelay::Delay1000)
            .build();

        let css_classes = classes.to_css_classes();

        // Transition Properties
        assert!(css_classes.contains("transition-none"));
        assert!(css_classes.contains("transition-all"));
        assert!(css_classes.contains("transition"));
        assert!(css_classes.contains("transition-colors"));
        assert!(css_classes.contains("transition-opacity"));
        assert!(css_classes.contains("transition-shadow"));
        assert!(css_classes.contains("transition-transform"));

        // Duration
        assert!(css_classes.contains("duration-75"));
        assert!(css_classes.contains("duration-100"));
        assert!(css_classes.contains("duration-150"));
        assert!(css_classes.contains("duration-200"));
        assert!(css_classes.contains("duration-300"));
        assert!(css_classes.contains("duration-500"));
        assert!(css_classes.contains("duration-700"));
        assert!(css_classes.contains("duration-1000"));

        // Ease
        assert!(css_classes.contains("ease-linear"));
        assert!(css_classes.contains("ease-in"));
        assert!(css_classes.contains("ease-out"));
        assert!(css_classes.contains("ease-in-out"));

        // Delay
        assert!(css_classes.contains("delay-75"));
        assert!(css_classes.contains("delay-100"));
        assert!(css_classes.contains("delay-150"));
        assert!(css_classes.contains("delay-200"));
        assert!(css_classes.contains("delay-300"));
        assert!(css_classes.contains("delay-500"));
        assert!(css_classes.contains("delay-700"));
        assert!(css_classes.contains("delay-1000"));
    }
}
