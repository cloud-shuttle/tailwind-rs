//! Animation Context Module
//!
//! Handles animation classes like animate-spin, duration-300, ease-in-out, delay-100, etc.

use crate::css_generator::types::CssProperty;

/// Context for managing animation state within an element
#[derive(Debug, Clone, Default)]
pub struct AnimationContext {
    /// Animation name value
    pub animation_name: Option<String>,
    /// Animation duration value
    pub animation_duration: Option<String>,
    /// Animation timing function value
    pub animation_timing: Option<String>,
    /// Animation delay value
    pub animation_delay: Option<String>,
    /// Animation iteration count value
    pub animation_iteration: Option<String>,
    /// Animation direction value
    pub animation_direction: Option<String>,
    /// Animation fill mode value
    pub animation_fill: Option<String>,
    /// Animation play state value
    pub animation_play_state: Option<String>,
}

impl AnimationContext {
    /// Update context from an animation class
    pub fn update_from_class(&mut self, class: &str) {
        if let Some(animation_name) = Self::parse_animation_name_class(class) {
            self.animation_name = Some(animation_name);
        } else if let Some(duration) = Self::parse_animation_duration_class(class) {
            self.animation_duration = Some(duration);
        } else if let Some(timing) = Self::parse_animation_timing_class(class) {
            self.animation_timing = Some(timing);
        } else if let Some(delay) = Self::parse_animation_delay_class(class) {
            self.animation_delay = Some(delay);
        } else if let Some(iteration) = Self::parse_animation_iteration_class(class) {
            self.animation_iteration = Some(iteration);
        } else if let Some(direction) = Self::parse_animation_direction_class(class) {
            self.animation_direction = Some(direction);
        } else if let Some(fill) = Self::parse_animation_fill_class(class) {
            self.animation_fill = Some(fill);
        } else if let Some(play_state) = Self::parse_animation_play_state_class(class) {
            self.animation_play_state = Some(play_state);
        }
    }

    /// Parse animation name classes
    fn parse_animation_name_class(class: &str) -> Option<String> {
        match class {
            "animate-none" => Some("none".to_string()),
            "animate-spin" => Some("spin".to_string()),
            "animate-ping" => Some("ping".to_string()),
            "animate-pulse" => Some("pulse".to_string()),
            "animate-bounce" => Some("bounce".to_string()),
            "animate-fade-in" => Some("fade-in".to_string()),
            "animate-fade-out" => Some("fade-out".to_string()),
            "animate-slide-in-left" => Some("slide-in-left".to_string()),
            "animate-slide-in-right" => Some("slide-in-right".to_string()),
            "animate-slide-in-top" => Some("slide-in-top".to_string()),
            "animate-slide-in-bottom" => Some("slide-in-bottom".to_string()),
            "animate-zoom-in" => Some("zoom-in".to_string()),
            "animate-zoom-out" => Some("zoom-out".to_string()),
            "animate-wobble" => Some("wobble".to_string()),
            "animate-shake" => Some("shake".to_string()),
            "animate-flip" => Some("flip".to_string()),
            "animate-heartbeat" => Some("heartbeat".to_string()),
            _ => None,
        }
    }

    /// Parse animation duration classes
    fn parse_animation_duration_class(class: &str) -> Option<String> {
        match class {
            "duration-75" => Some("75ms".to_string()),
            "duration-100" => Some("100ms".to_string()),
            "duration-150" => Some("150ms".to_string()),
            "duration-200" => Some("200ms".to_string()),
            "duration-300" => Some("300ms".to_string()),
            "duration-500" => Some("500ms".to_string()),
            "duration-700" => Some("700ms".to_string()),
            "duration-1000" => Some("1000ms".to_string()),
            _ => None,
        }
    }

    /// Parse animation timing function classes
    fn parse_animation_timing_class(class: &str) -> Option<String> {
        match class {
            "ease-linear" => Some("linear".to_string()),
            "ease-in" => Some("ease-in".to_string()),
            "ease-out" => Some("ease-out".to_string()),
            "ease-in-out" => Some("ease-in-out".to_string()),
            _ => None,
        }
    }

    /// Parse animation delay classes
    fn parse_animation_delay_class(class: &str) -> Option<String> {
        match class {
            "delay-75" => Some("75ms".to_string()),
            "delay-100" => Some("100ms".to_string()),
            "delay-150" => Some("150ms".to_string()),
            "delay-200" => Some("200ms".to_string()),
            "delay-300" => Some("300ms".to_string()),
            "delay-500" => Some("500ms".to_string()),
            "delay-700" => Some("700ms".to_string()),
            "delay-1000" => Some("1000ms".to_string()),
            _ => None,
        }
    }

    /// Parse animation iteration count classes
    fn parse_animation_iteration_class(class: &str) -> Option<String> {
        match class {
            "animate-once" => Some("1".to_string()),
            "animate-infinite" => Some("infinite".to_string()),
            _ => None,
        }
    }

    /// Parse animation direction classes
    fn parse_animation_direction_class(class: &str) -> Option<String> {
        match class {
            "animate-normal" => Some("normal".to_string()),
            "animate-reverse" => Some("reverse".to_string()),
            "animate-alternate" => Some("alternate".to_string()),
            "animate-alternate-reverse" => Some("alternate-reverse".to_string()),
            _ => None,
        }
    }

    /// Parse animation fill mode classes
    fn parse_animation_fill_class(class: &str) -> Option<String> {
        match class {
            "animate-fill-none" => Some("none".to_string()),
            "animate-fill-forwards" => Some("forwards".to_string()),
            "animate-fill-backwards" => Some("backwards".to_string()),
            "animate-fill-both" => Some("both".to_string()),
            _ => None,
        }
    }

    /// Parse animation play state classes
    fn parse_animation_play_state_class(class: &str) -> Option<String> {
        match class {
            "animate-paused" => Some("paused".to_string()),
            "animate-running" => Some("running".to_string()),
            _ => None,
        }
    }

    /// Generate CSS properties from current animation state
    pub fn to_css_properties(&self) -> Vec<CssProperty> {
        let mut properties = Vec::new();

        if let Some(ref name) = self.animation_name {
            properties.push(CssProperty {
                name: "animation-name".to_string(),
                value: name.clone(),
                important: false,
            });
        }

        if let Some(ref duration) = self.animation_duration {
            properties.push(CssProperty {
                name: "animation-duration".to_string(),
                value: duration.clone(),
                important: false,
            });
        }

        if let Some(ref timing) = self.animation_timing {
            properties.push(CssProperty {
                name: "animation-timing-function".to_string(),
                value: timing.clone(),
                important: false,
            });
        }

        if let Some(ref delay) = self.animation_delay {
            properties.push(CssProperty {
                name: "animation-delay".to_string(),
                value: delay.clone(),
                important: false,
            });
        }

        if let Some(ref iteration) = self.animation_iteration {
            properties.push(CssProperty {
                name: "animation-iteration-count".to_string(),
                value: iteration.clone(),
                important: false,
            });
        }

        if let Some(ref direction) = self.animation_direction {
            properties.push(CssProperty {
                name: "animation-direction".to_string(),
                value: direction.clone(),
                important: false,
            });
        }

        if let Some(ref fill) = self.animation_fill {
            properties.push(CssProperty {
                name: "animation-fill-mode".to_string(),
                value: fill.clone(),
                important: false,
            });
        }

        if let Some(ref play_state) = self.animation_play_state {
            properties.push(CssProperty {
                name: "animation-play-state".to_string(),
                value: play_state.clone(),
                important: false,
            });
        }

        properties
    }

    /// Check if this context has any animation properties
    pub fn has_animation(&self) -> bool {
        self.animation_name.is_some() || self.animation_duration.is_some() ||
        self.animation_timing.is_some() || self.animation_delay.is_some() ||
        self.animation_iteration.is_some() || self.animation_direction.is_some() ||
        self.animation_fill.is_some() || self.animation_play_state.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn animation_name_parsing() {
        let mut context = AnimationContext::default();

        context.update_from_class("animate-spin");
        assert_eq!(context.animation_name, Some("spin".to_string()));

        context = AnimationContext::default();
        context.update_from_class("animate-bounce");
        assert_eq!(context.animation_name, Some("bounce".to_string()));

        context = AnimationContext::default();
        context.update_from_class("animate-none");
        assert_eq!(context.animation_name, Some("none".to_string()));
    }

    #[test]
    fn animation_duration_parsing() {
        let mut context = AnimationContext::default();

        context.update_from_class("duration-300");
        assert_eq!(context.animation_duration, Some("300ms".to_string()));

        context = AnimationContext::default();
        context.update_from_class("duration-1000");
        assert_eq!(context.animation_duration, Some("1000ms".to_string()));
    }

    #[test]
    fn animation_timing_parsing() {
        let mut context = AnimationContext::default();

        context.update_from_class("ease-linear");
        assert_eq!(context.animation_timing, Some("linear".to_string()));

        context = AnimationContext::default();
        context.update_from_class("ease-in-out");
        assert_eq!(context.animation_timing, Some("ease-in-out".to_string()));
    }

    #[test]
    fn animation_delay_parsing() {
        let mut context = AnimationContext::default();

        context.update_from_class("delay-200");
        assert_eq!(context.animation_delay, Some("200ms".to_string()));

        context = AnimationContext::default();
        context.update_from_class("delay-500");
        assert_eq!(context.animation_delay, Some("500ms".to_string()));
    }

    #[test]
    fn animation_iteration_parsing() {
        let mut context = AnimationContext::default();

        context.update_from_class("animate-once");
        assert_eq!(context.animation_iteration, Some("1".to_string()));

        context = AnimationContext::default();
        context.update_from_class("animate-infinite");
        assert_eq!(context.animation_iteration, Some("infinite".to_string()));
    }

    #[test]
    fn animation_direction_parsing() {
        let mut context = AnimationContext::default();

        context.update_from_class("animate-reverse");
        assert_eq!(context.animation_direction, Some("reverse".to_string()));

        context = AnimationContext::default();
        context.update_from_class("animate-alternate");
        assert_eq!(context.animation_direction, Some("alternate".to_string()));
    }

    #[test]
    fn animation_fill_parsing() {
        let mut context = AnimationContext::default();

        context.update_from_class("animate-fill-forwards");
        assert_eq!(context.animation_fill, Some("forwards".to_string()));

        context = AnimationContext::default();
        context.update_from_class("animate-fill-both");
        assert_eq!(context.animation_fill, Some("both".to_string()));
    }

    #[test]
    fn animation_play_state_parsing() {
        let mut context = AnimationContext::default();

        context.update_from_class("animate-paused");
        assert_eq!(context.animation_play_state, Some("paused".to_string()));

        context = AnimationContext::default();
        context.update_from_class("animate-running");
        assert_eq!(context.animation_play_state, Some("running".to_string()));
    }

    #[test]
    fn animation_css_generation() {
        let mut context = AnimationContext::default();
        context.update_from_class("animate-spin");
        context.update_from_class("duration-300");
        context.update_from_class("ease-linear");
        context.update_from_class("delay-100");

        let properties = context.to_css_properties();
        assert_eq!(properties.len(), 4);

        // Check specific properties
        let has_name = properties.iter().any(|p| p.name == "animation-name" && p.value == "spin");
        let has_duration = properties.iter().any(|p| p.name == "animation-duration" && p.value == "300ms");
        let has_timing = properties.iter().any(|p| p.name == "animation-timing-function" && p.value == "linear");
        let has_delay = properties.iter().any(|p| p.name == "animation-delay" && p.value == "100ms");

        assert!(has_name, "Should have animation-name property");
        assert!(has_duration, "Should have animation-duration property");
        assert!(has_timing, "Should have animation-timing-function property");
        assert!(has_delay, "Should have animation-delay property");
    }

    #[test]
    fn empty_animation_generation() {
        let context = AnimationContext::default();
        let properties = context.to_css_properties();
        assert_eq!(properties.len(), 0);
        assert!(!context.has_animation());
    }
}
