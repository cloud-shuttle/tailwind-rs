//! Keyframe Animation Utilities Module
//!
//! Handles keyframe animation structures:
//! - CustomKeyframe struct for complete animations
//! - KeyframeStep struct for individual keyframe steps
//! - TransformStep struct for transform properties in keyframes

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub timing_function: super::timing::TimingFunction,
    /// Animation delay in milliseconds
    pub delay: u32,
    /// Animation iteration count
    pub iteration_count: super::iteration::AnimationIteration,
    /// Animation direction
    pub direction: super::direction::AnimationDirection,
    /// Animation fill mode
    pub fill_mode: super::fill_mode::AnimationFillMode,
    /// Animation play state
    pub play_state: super::play_state::AnimationPlayState,
}

impl CustomKeyframe {
    /// Create a new custom keyframe animation
    pub fn new(name: String) -> Self {
        Self {
            name,
            steps: Vec::new(),
            duration: 1000,
            timing_function: super::timing::TimingFunction::Ease,
            delay: 0,
            iteration_count: super::iteration::AnimationIteration::Count(1.0),
            direction: super::direction::AnimationDirection::Normal,
            fill_mode: super::fill_mode::AnimationFillMode::None,
            play_state: super::play_state::AnimationPlayState::Running,
        }
    }

    /// Add a keyframe step
    pub fn add_step(&mut self, time: f32, step: KeyframeStep) {
        self.steps.push((time, step));
        self.steps.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    }

    /// Get the CSS keyframe rule
    pub fn to_css_keyframe(&self) -> String {
        let mut css = format!("@keyframes {} {{\n", self.name);
        for (time, step) in &self.steps {
            let percentage = (time * 100.0) as u32;
            css.push_str(&format!("  {}% {{\n", percentage));
            css.push_str(&step.to_css_properties());
            css.push_str("  }\n");
        }
        css.push_str("}\n");
        css
    }

    /// Get the CSS animation property
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

impl Default for CustomKeyframe {
    fn default() -> Self {
        Self::new("unnamed-animation".to_string())
    }
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

    /// Set opacity
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }

    /// Set color
    pub fn with_color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }

    /// Set transform
    pub fn with_transform(mut self, transform: TransformStep) -> Self {
        self.transform = Some(transform);
        self
    }

    /// Add a CSS property
    pub fn with_property(mut self, name: String, value: String) -> Self {
        self.properties.insert(name, value);
        self
    }

    /// Convert to CSS properties string
    pub fn to_css_properties(&self) -> String {
        let mut css = String::new();

        // Add transform properties
        if let Some(transform) = &self.transform {
            css.push_str(&format!("    transform: {};\n", transform.to_css_value()));
        }

        // Add opacity
        if let Some(opacity) = self.opacity {
            css.push_str(&format!("    opacity: {};\n", opacity));
        }

        // Add color
        if let Some(color) = &self.color {
            css.push_str(&format!("    color: {};\n", color));
        }

        // Add other properties
        for (name, value) in &self.properties {
            css.push_str(&format!("    {}: {};\n", name, value));
        }

        css
    }
}

impl Default for KeyframeStep {
    fn default() -> Self {
        Self::new()
    }
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
    pub fn with_translate(mut self, x: f32, y: f32, z: f32) -> Self {
        self.translate = Some((x, y, z));
        self
    }

    /// Set scale
    pub fn with_scale(mut self, x: f32, y: f32, z: f32) -> Self {
        self.scale = Some((x, y, z));
        self
    }

    /// Set rotation
    pub fn with_rotate(mut self, x: f32, y: f32, z: f32) -> Self {
        self.rotate = Some((x, y, z));
        self
    }

    /// Set skew
    pub fn with_skew(mut self, x: f32, y: f32) -> Self {
        self.skew = Some((x, y));
        self
    }

    /// Convert to CSS transform value
    pub fn to_css_value(&self) -> String {
        let mut transforms = Vec::new();

        if let Some((x, y, z)) = self.translate {
            transforms.push(format!("translate3d({}px, {}px, {}px)", x, y, z));
        }

        if let Some((x, y, z)) = self.scale {
            if x == y && y == z {
                transforms.push(format!("scale({})", x));
            } else {
                transforms.push(format!("scale3d({}, {}, {})", x, y, z));
            }
        }

        if let Some((x, y, z)) = self.rotate {
            if x != 0.0 {
                transforms.push(format!("rotateX({}deg)", x));
            }
            if y != 0.0 {
                transforms.push(format!("rotateY({}deg)", y));
            }
            if z != 0.0 {
                transforms.push(format!("rotateZ({}deg)", z));
            }
        }

        if let Some((x, y)) = self.skew {
            if x != 0.0 {
                transforms.push(format!("skewX({}deg)", x));
            }
            if y != 0.0 {
                transforms.push(format!("skewY({}deg)", y));
            }
        }

        if transforms.is_empty() {
            "none".to_string()
        } else {
            transforms.join(" ")
        }
    }
}

impl Default for TransformStep {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_keyframe_creation() {
        let keyframe = CustomKeyframe::new("test-animation".to_string());
        assert_eq!(keyframe.name, "test-animation");
        assert_eq!(keyframe.duration, 1000);
        assert_eq!(keyframe.delay, 0);
    }

    #[test]
    fn keyframe_step_creation() {
        let step = KeyframeStep::new()
            .with_opacity(0.5)
            .with_color("#ff0000".to_string())
            .with_property("width".to_string(), "100px".to_string());

        assert_eq!(step.opacity, Some(0.5));
        assert_eq!(step.color, Some("#ff0000".to_string()));
        assert_eq!(step.properties.get("width"), Some(&"100px".to_string()));
    }

    #[test]
    fn transform_step_creation() {
        let transform = TransformStep::new()
            .with_translate(10.0, 20.0, 0.0)
            .with_scale(1.5, 1.5, 1.0)
            .with_rotate(0.0, 0.0, 45.0);

        assert_eq!(transform.translate, Some((10.0, 20.0, 0.0)));
        assert_eq!(transform.scale, Some((1.5, 1.5, 1.0)));
        assert_eq!(transform.rotate, Some((0.0, 0.0, 45.0)));
    }

    #[test]
    fn transform_step_css_value() {
        let transform = TransformStep::new()
            .with_translate(10.0, 20.0, 0.0)
            .with_scale(1.5, 1.5, 1.0);

        let css = transform.to_css_value();
        assert!(css.contains("translate3d(10px, 20px, 0px)"));
        assert!(css.contains("scale3d(1.5, 1.5, 1)"));
    }

    #[test]
    fn keyframe_step_css_properties() {
        let mut step = KeyframeStep::new();
        step.opacity = Some(0.8);
        step.color = Some("#00ff00".to_string());
        step.properties.insert("width".to_string(), "200px".to_string());

        let css = step.to_css_properties();
        assert!(css.contains("opacity: 0.8;"));
        assert!(css.contains("color: #00ff00;"));
        assert!(css.contains("width: 200px;"));
    }
}
