//! Advanced animation utilities for tailwind-rs
//!
//! This module provides support for custom keyframe animations,
//! animation composition, and advanced animation features.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Custom keyframe animation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomKeyframe {
    /// Animation name
    pub name: String,
    /// Keyframe steps (0.0 to 1.0)
    pub steps: Vec<(f32, KeyframeStep)>,
    /// Animation duration in milliseconds
    pub duration: u32,
    /// Animation timing function
    pub timing_function: TimingFunction,
    /// Animation delay in milliseconds
    pub delay: u32,
    /// Animation iteration count
    pub iteration_count: AnimationIteration,
    /// Animation direction
    pub direction: AnimationDirection,
    /// Animation fill mode
    pub fill_mode: AnimationFillMode,
    /// Animation play state
    pub play_state: AnimationPlayState,
}

/// Keyframe step with properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyframeStep {
    /// CSS properties for this keyframe
    pub properties: HashMap<String, String>,
    /// Transform properties
    pub transform: Option<TransformStep>,
    /// Opacity value
    pub opacity: Option<f32>,
    /// Color value
    pub color: Option<String>,
}

/// Transform step for keyframes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformStep {
    /// Translation (x, y, z)
    pub translate: Option<(f32, f32, f32)>,
    /// Scale (x, y, z)
    pub scale: Option<(f32, f32, f32)>,
    /// Rotation (x, y, z) in degrees
    pub rotate: Option<(f32, f32, f32)>,
    /// Skew (x, y) in degrees
    pub skew: Option<(f32, f32)>,
}

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

/// Step direction for steps timing function
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StepDirection {
    /// start
    Start,
    /// end
    End,
}

/// Animation iteration count
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimationIteration {
    /// infinite
    Infinite,
    /// specific number
    Count(f32),
}

/// Animation direction
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimationDirection {
    /// normal
    Normal,
    /// reverse
    Reverse,
    /// alternate
    Alternate,
    /// alternate-reverse
    AlternateReverse,
}

/// Animation fill mode
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimationFillMode {
    /// none
    None,
    /// forwards
    Forwards,
    /// backwards
    Backwards,
    /// both
    Both,
}

/// Animation play state
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimationPlayState {
    /// running
    Running,
    /// paused
    Paused,
}

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

/// Animation reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimationReference {
    /// Built-in animation name
    BuiltIn(String),
    /// Custom keyframe animation
    Custom(CustomKeyframe),
}

/// Animation properties override
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationProperties {
    /// Duration override
    pub duration: Option<u32>,
    /// Timing function override
    pub timing_function: Option<TimingFunction>,
    /// Delay override
    pub delay: Option<u32>,
    /// Iteration count override
    pub iteration_count: Option<AnimationIteration>,
    /// Direction override
    pub direction: Option<AnimationDirection>,
}

/// Composition timing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompositionTiming {
    /// Total duration in milliseconds
    pub duration: u32,
    /// Timing function for the composition
    pub timing_function: TimingFunction,
    /// Delay for the composition
    pub delay: u32,
}

impl CustomKeyframe {
    /// Create a new custom keyframe animation
    pub fn new(name: String) -> Self {
        Self {
            name,
            steps: Vec::new(),
            duration: 1000,
            timing_function: TimingFunction::Ease,
            delay: 0,
            iteration_count: AnimationIteration::Count(1.0),
            direction: AnimationDirection::Normal,
            fill_mode: AnimationFillMode::Both,
            play_state: AnimationPlayState::Running,
        }
    }

    /// Add a keyframe step
    pub fn add_step(&mut self, offset: f32, step: KeyframeStep) {
        self.steps.push((offset, step));
        // Sort by offset to maintain order
        self.steps.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    }

    /// Convert to CSS @keyframes rule
    pub fn to_css_keyframes(&self) -> String {
        let mut css = format!("@keyframes {} {{\n", self.name);

        for (offset, step) in &self.steps {
            let percentage = (offset * 100.0) as u32;
            css.push_str(&format!("  {}% {{\n", percentage));

            // Add properties
            for (property, value) in &step.properties {
                css.push_str(&format!("    {}: {};\n", property, value));
            }

            // Add transform
            if let Some(transform) = &step.transform {
                css.push_str(&format!("    transform: {};\n", transform.to_css_value()));
            }

            // Add opacity
            if let Some(opacity) = step.opacity {
                css.push_str(&format!("    opacity: {};\n", opacity));
            }

            // Add color
            if let Some(color) = &step.color {
                css.push_str(&format!("    color: {};\n", color));
            }

            css.push_str("  }\n");
        }

        css.push_str("}\n");
        css
    }

    /// Convert to class name
    pub fn to_class_name(&self) -> String {
        format!("animate-{}", self.name)
    }

    /// Convert to CSS animation property
    pub fn to_css_animation(&self) -> String {
        format!(
            "{} {}ms {} {}ms {} {} {} {}",
            self.name,
            self.duration,
            self.timing_function.to_css_value(),
            self.delay,
            self.iteration_count.to_css_value(),
            self.direction.to_css_value(),
            self.fill_mode.to_css_value(),
            self.play_state.to_css_value()
        )
    }
}

impl Default for KeyframeStep {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyframeStep {
    /// Create a new keyframe step
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
            transform: None,
            opacity: None,
            color: None,
        }
    }

    /// Add a CSS property
    pub fn add_property(&mut self, property: String, value: String) {
        self.properties.insert(property, value);
    }

    /// Set transform
    pub fn set_transform(&mut self, transform: TransformStep) {
        self.transform = Some(transform);
    }

    /// Set opacity
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = Some(opacity);
    }

    /// Set color
    pub fn set_color(&mut self, color: String) {
        self.color = Some(color);
    }
}

impl Default for TransformStep {
    fn default() -> Self {
        Self::new()
    }
}

impl TransformStep {
    /// Create a new transform step
    pub fn new() -> Self {
        Self {
            translate: None,
            scale: None,
            rotate: None,
            skew: None,
        }
    }

    /// Set translation
    pub fn set_translate(&mut self, x: f32, y: f32, z: f32) {
        self.translate = Some((x, y, z));
    }

    /// Set scale
    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale = Some((x, y, z));
    }

    /// Set rotation
    pub fn set_rotate(&mut self, x: f32, y: f32, z: f32) {
        self.rotate = Some((x, y, z));
    }

    /// Set skew
    pub fn set_skew(&mut self, x: f32, y: f32) {
        self.skew = Some((x, y));
    }

    /// Convert to CSS transform value
    pub fn to_css_value(&self) -> String {
        let mut transforms = Vec::new();

        if let Some((x, y, z)) = self.translate {
            if z == 0.0 {
                transforms.push(format!("translate({}px, {}px)", x, y));
            } else {
                transforms.push(format!("translate3d({}px, {}px, {}px)", x, y, z));
            }
        }

        if let Some((x, y, z)) = self.scale {
            if z == 1.0 {
                transforms.push(format!("scale({}, {})", x, y));
            } else {
                transforms.push(format!("scale3d({}, {}, {})", x, y, z));
            }
        }

        if let Some((x, y, z)) = self.rotate {
            if x == 0.0 && y == 0.0 {
                transforms.push(format!("rotate({}deg)", z));
            } else {
                transforms.push(format!("rotate3d({}, {}, {}, {}deg)", x, y, z, z));
            }
        }

        if let Some((x, y)) = self.skew {
            transforms.push(format!("skew({}deg, {}deg)", x, y));
        }

        transforms.join(" ")
    }
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
}

impl AnimationIteration {
    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            AnimationIteration::Infinite => "infinite".to_string(),
            AnimationIteration::Count(count) => count.to_string(),
        }
    }
}

impl AnimationDirection {
    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            AnimationDirection::Normal => "normal".to_string(),
            AnimationDirection::Reverse => "reverse".to_string(),
            AnimationDirection::Alternate => "alternate".to_string(),
            AnimationDirection::AlternateReverse => "alternate-reverse".to_string(),
        }
    }
}

impl AnimationFillMode {
    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            AnimationFillMode::None => "none".to_string(),
            AnimationFillMode::Forwards => "forwards".to_string(),
            AnimationFillMode::Backwards => "backwards".to_string(),
            AnimationFillMode::Both => "both".to_string(),
        }
    }
}

impl AnimationPlayState {
    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            AnimationPlayState::Running => "running".to_string(),
            AnimationPlayState::Paused => "paused".to_string(),
        }
    }
}

impl AnimationComposition {
    /// Create a new animation composition
    pub fn new(name: String) -> Self {
        Self {
            name,
            animations: Vec::new(),
            timing: CompositionTiming {
                duration: 1000,
                timing_function: TimingFunction::Ease,
                delay: 0,
            },
        }
    }

    /// Add an animation to the composition
    pub fn add_animation(&mut self, animation: ComposedAnimation) {
        self.animations.push(animation);
    }

    /// Convert to CSS
    pub fn to_css(&self) -> String {
        let mut css = String::new();

        // Generate keyframes for each animation
        for composed_anim in &self.animations {
            if let AnimationReference::Custom(keyframe) = &composed_anim.animation {
                css.push_str(&keyframe.to_css_keyframes());
            }
        }

        // Generate composition class
        css.push_str(&format!(".{} {{\n", self.name));
        css.push_str(&format!(
            "  animation: {} {}ms {} {}ms;\n",
            self.name,
            self.timing.duration,
            self.timing.timing_function.to_css_value(),
            self.timing.delay
        ));
        css.push_str("}\n");

        css
    }

    /// Convert to class name
    pub fn to_class_name(&self) -> String {
        format!("animate-{}", self.name)
    }
}

impl fmt::Display for CustomKeyframe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_css_animation())
    }
}

impl fmt::Display for AnimationComposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_css())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_keyframe_creation() {
        let mut keyframe = CustomKeyframe::new("fadeIn".to_string());
        keyframe.duration = 500;
        keyframe.timing_function = TimingFunction::EaseOut;

        assert_eq!(keyframe.name, "fadeIn");
        assert_eq!(keyframe.duration, 500);
        assert_eq!(keyframe.timing_function, TimingFunction::EaseOut);
    }

    #[test]
    fn test_keyframe_step_creation() {
        let mut step = KeyframeStep::new();
        step.set_opacity(0.0);
        step.add_property("transform".to_string(), "scale(0.8)".to_string());

        assert_eq!(step.opacity, Some(0.0));
        assert_eq!(
            step.properties.get("transform"),
            Some(&"scale(0.8)".to_string())
        );
    }

    #[test]
    fn test_transform_step_creation() {
        let mut transform = TransformStep::new();
        transform.set_translate(10.0, 20.0, 0.0);
        transform.set_scale(1.2, 1.2, 1.0);
        transform.set_rotate(0.0, 0.0, 45.0);

        assert_eq!(transform.translate, Some((10.0, 20.0, 0.0)));
        assert_eq!(transform.scale, Some((1.2, 1.2, 1.0)));
        assert_eq!(transform.rotate, Some((0.0, 0.0, 45.0)));
    }

    #[test]
    fn test_transform_css_generation() {
        let mut transform = TransformStep::new();
        transform.set_translate(10.0, 20.0, 0.0);
        transform.set_scale(1.2, 1.2, 1.0);
        transform.set_rotate(0.0, 0.0, 45.0);

        let css = transform.to_css_value();
        assert!(css.contains("translate(10px, 20px)"));
        assert!(css.contains("scale(1.2, 1.2)"));
        assert!(css.contains("rotate(45deg)"));
    }

    #[test]
    fn test_timing_function_css_values() {
        assert_eq!(TimingFunction::Linear.to_css_value(), "linear");
        assert_eq!(TimingFunction::Ease.to_css_value(), "ease");
        assert_eq!(TimingFunction::EaseIn.to_css_value(), "ease-in");
        assert_eq!(TimingFunction::EaseOut.to_css_value(), "ease-out");
        assert_eq!(TimingFunction::EaseInOut.to_css_value(), "ease-in-out");

        let cubic_bezier = TimingFunction::CubicBezier(0.25, 0.1, 0.25, 1.0);
        assert_eq!(
            cubic_bezier.to_css_value(),
            "cubic-bezier(0.25, 0.1, 0.25, 1)"
        );

        let steps = TimingFunction::Steps(5);
        assert_eq!(steps.to_css_value(), "steps(5)");

        let steps_with_dir = TimingFunction::StepsWithDirection(5, StepDirection::Start);
        assert_eq!(steps_with_dir.to_css_value(), "steps(5, start)");
    }

    #[test]
    fn test_animation_iteration_css_values() {
        assert_eq!(AnimationIteration::Infinite.to_css_value(), "infinite");
        assert_eq!(AnimationIteration::Count(3.0).to_css_value(), "3");
        assert_eq!(AnimationIteration::Count(0.5).to_css_value(), "0.5");
    }

    #[test]
    fn test_animation_direction_css_values() {
        assert_eq!(AnimationDirection::Normal.to_css_value(), "normal");
        assert_eq!(AnimationDirection::Reverse.to_css_value(), "reverse");
        assert_eq!(AnimationDirection::Alternate.to_css_value(), "alternate");
        assert_eq!(
            AnimationDirection::AlternateReverse.to_css_value(),
            "alternate-reverse"
        );
    }

    #[test]
    fn test_animation_fill_mode_css_values() {
        assert_eq!(AnimationFillMode::None.to_css_value(), "none");
        assert_eq!(AnimationFillMode::Forwards.to_css_value(), "forwards");
        assert_eq!(AnimationFillMode::Backwards.to_css_value(), "backwards");
        assert_eq!(AnimationFillMode::Both.to_css_value(), "both");
    }

    #[test]
    fn test_animation_play_state_css_values() {
        assert_eq!(AnimationPlayState::Running.to_css_value(), "running");
        assert_eq!(AnimationPlayState::Paused.to_css_value(), "paused");
    }

    #[test]
    fn test_custom_keyframe_css_generation() {
        let mut keyframe = CustomKeyframe::new("fadeIn".to_string());
        keyframe.duration = 500;
        keyframe.timing_function = TimingFunction::EaseOut;

        let mut step0 = KeyframeStep::new();
        step0.set_opacity(0.0);
        keyframe.add_step(0.0, step0);

        let mut step1 = KeyframeStep::new();
        step1.set_opacity(1.0);
        keyframe.add_step(1.0, step1);

        let css = keyframe.to_css_keyframes();
        assert!(css.contains("@keyframes fadeIn"));
        assert!(css.contains("0%"));
        assert!(css.contains("100%"));
        assert!(css.contains("opacity: 0"));
        assert!(css.contains("opacity: 1"));

        let animation_css = keyframe.to_css_animation();
        assert!(animation_css.contains("fadeIn"));
        assert!(animation_css.contains("500ms"));
        assert!(animation_css.contains("ease-out"));
    }

    #[test]
    fn test_animation_composition_creation() {
        let mut composition = AnimationComposition::new("complexAnimation".to_string());
        composition.timing.duration = 2000;
        composition.timing.timing_function = TimingFunction::EaseInOut;

        assert_eq!(composition.name, "complexAnimation");
        assert_eq!(composition.timing.duration, 2000);
        assert_eq!(
            composition.timing.timing_function,
            TimingFunction::EaseInOut
        );
    }

    #[test]
    fn test_animation_composition_css_generation() {
        let mut composition = AnimationComposition::new("complexAnimation".to_string());

        let mut keyframe = CustomKeyframe::new("fadeIn".to_string());
        let mut step = KeyframeStep::new();
        step.set_opacity(1.0);
        keyframe.add_step(1.0, step);

        let composed_anim = ComposedAnimation {
            animation: AnimationReference::Custom(keyframe),
            start_offset: 0.0,
            end_offset: 1.0,
            properties: None,
        };

        composition.add_animation(composed_anim);

        let css = composition.to_css();
        assert!(css.contains("@keyframes fadeIn"));
        assert!(css.contains(".complexAnimation"));
    }

    #[test]
    fn test_custom_keyframe_class_name() {
        let keyframe = CustomKeyframe::new("fadeIn".to_string());
        assert_eq!(keyframe.to_class_name(), "animate-fadeIn");
    }

    #[test]
    fn test_animation_composition_class_name() {
        let composition = AnimationComposition::new("complexAnimation".to_string());
        assert_eq!(composition.to_class_name(), "animate-complexAnimation");
    }

    #[test]
    fn test_custom_keyframe_display() {
        let keyframe = CustomKeyframe::new("fadeIn".to_string());
        let display = format!("{}", keyframe);
        assert!(display.contains("fadeIn"));
    }

    #[test]
    fn test_animation_composition_display() {
        let composition = AnimationComposition::new("complexAnimation".to_string());
        let display = format!("{}", composition);
        assert!(display.contains("complexAnimation"));
    }

    #[test]
    fn test_custom_keyframe_serialization() {
        let keyframe = CustomKeyframe::new("fadeIn".to_string());
        let serialized = serde_json::to_string(&keyframe).unwrap();
        let deserialized: CustomKeyframe = serde_json::from_str(&serialized).unwrap();
        assert_eq!(keyframe, deserialized);
    }

    #[test]
    fn test_animation_composition_serialization() {
        let composition = AnimationComposition::new("complexAnimation".to_string());
        let serialized = serde_json::to_string(&composition).unwrap();
        let deserialized: AnimationComposition = serde_json::from_str(&serialized).unwrap();
        assert_eq!(composition, deserialized);
    }
}
