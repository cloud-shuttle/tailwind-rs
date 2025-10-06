//! Animation Utilities Parser

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct AnimationParser;

impl AnimationParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse animation classes
    pub fn parse_animation_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            // Basic animations
            "animate-none" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            "animate-spin" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "spin 1s linear infinite".to_string(),
                important: false,
            }]),
            "animate-ping" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite".to_string(),
                important: false,
            }]),
            "animate-pulse" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite".to_string(),
                important: false,
            }]),
            "animate-bounce" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "bounce 1s infinite".to_string(),
                important: false,
            }]),

            // In/Out animations
            "animate-in" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "in 0.15s ease-out".to_string(),
                important: false,
            }]),
            "animate-in-fade" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "in-fade 0.15s ease-out".to_string(),
                important: false,
            }]),
            "animate-in-slide" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "in-slide 0.15s ease-out".to_string(),
                important: false,
            }]),
            "animate-out" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "out 0.15s ease-in".to_string(),
                important: false,
            }]),
            "animate-out-fade" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "out-fade 0.15s ease-in".to_string(),
                important: false,
            }]),
            "animate-out-slide" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "out-slide 0.15s ease-in".to_string(),
                important: false,
            }]),

            // Fade animations
            "animate-fade-in" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "fade-in 0.5s ease-out".to_string(),
                important: false,
            }]),
            "animate-fade-out" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "fade-out 0.5s ease-in".to_string(),
                important: false,
            }]),
            "animate-fade-in-up" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "fade-in-up 0.5s ease-out".to_string(),
                important: false,
            }]),
            "animate-fade-in-down" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "fade-in-down 0.5s ease-out".to_string(),
                important: false,
            }]),
            "animate-fade-in-left" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "fade-in-left 0.5s ease-out".to_string(),
                important: false,
            }]),
            "animate-fade-in-right" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "fade-in-right 0.5s ease-out".to_string(),
                important: false,
            }]),

            // Slide animations
            "animate-slide-in-up" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "slide-in-up 0.3s ease-out".to_string(),
                important: false,
            }]),
            "animate-slide-in-down" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "slide-in-down 0.3s ease-out".to_string(),
                important: false,
            }]),
            "animate-slide-in-left" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "slide-in-left 0.3s ease-out".to_string(),
                important: false,
            }]),
            "animate-slide-in-right" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "slide-in-right 0.3s ease-out".to_string(),
                important: false,
            }]),
            "animate-slide-out-up" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "slide-out-up 0.3s ease-in".to_string(),
                important: false,
            }]),
            "animate-slide-out-down" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "slide-out-down 0.3s ease-in".to_string(),
                important: false,
            }]),
            "animate-slide-out-left" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "slide-out-left 0.3s ease-in".to_string(),
                important: false,
            }]),
            "animate-slide-out-right" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "slide-out-right 0.3s ease-in".to_string(),
                important: false,
            }]),

            // Zoom animations
            "animate-zoom-in" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "zoom-in 0.2s ease-out".to_string(),
                important: false,
            }]),
            "animate-zoom-out" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "zoom-out 0.2s ease-in".to_string(),
                important: false,
            }]),

            // Advanced animations
            "animate-float" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "float 3s ease-in-out infinite".to_string(),
                important: false,
            }]),
            "animate-twinkle" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "twinkle 2s ease-in-out infinite".to_string(),
                important: false,
            }]),
            "animate-rainbow" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "rainbow 3s linear infinite".to_string(),
                important: false,
            }]),
            "animate-shimmer" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "shimmer 2s linear infinite".to_string(),
                important: false,
            }]),
            "animate-drift" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "drift 4s ease-in-out infinite".to_string(),
                important: false,
            }]),
            "animate-glow" => Some(vec![CssProperty {
                name: "animation".to_string(),
                value: "glow 2s ease-in-out infinite alternate".to_string(),
                important: false,
            }]),

            _ => {
                // Handle animation timing controls
                if let Some(anim_control) = self.parse_animation_control(class) {
                    return Some(anim_control);
                }

                // Handle custom property animations: animate-(<custom-property>)
                if let Some(custom_prop) = class.strip_prefix("animate-(") {
                    if let Some(custom_prop) = custom_prop.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "animation".to_string(),
                            value: format!("var(--{})", custom_prop),
                            important: false,
                        }]);
                    }
                }

                // Handle arbitrary animations: animate-[<value>]
                if let Some(arbitrary_value) = class.strip_prefix("animate-[") {
                    if let Some(arbitrary_value) = arbitrary_value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "animation".to_string(),
                            value: arbitrary_value.to_string(),
                            important: false,
                        }]);
                    }
                }

                None
            },
        }
    }

    /// Parse animation control utilities (duration, delay, ease, direction, fill-mode)
    fn parse_animation_control(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Animation duration: animate-duration-<value>
        if let Some(duration) = class.strip_prefix("animate-duration-") {
            return Some(vec![CssProperty {
                name: "animation-duration".to_string(),
                value: self.parse_duration_value(duration),
                important: false,
            }]);
        }

        // Animation delay: animate-delay-<value>
        if let Some(delay) = class.strip_prefix("animate-delay-") {
            return Some(vec![CssProperty {
                name: "animation-delay".to_string(),
                value: self.parse_duration_value(delay),
                important: false,
            }]);
        }

        // Animation easing: animate-ease-<value>
        if let Some(ease) = class.strip_prefix("animate-ease-") {
            return Some(vec![CssProperty {
                name: "animation-timing-function".to_string(),
                value: self.parse_ease_value(ease),
                important: false,
            }]);
        }

        // Animation direction: animate-direction-<value>
        if let Some(direction) = class.strip_prefix("animate-direction-") {
            return Some(vec![CssProperty {
                name: "animation-direction".to_string(),
                value: self.parse_direction_value(direction),
                important: false,
            }]);
        }

        // Animation fill mode: animate-fill-<value>
        if let Some(fill) = class.strip_prefix("animate-fill-") {
            return Some(vec![CssProperty {
                name: "animation-fill-mode".to_string(),
                value: self.parse_fill_mode_value(fill),
                important: false,
            }]);
        }

        // Animation iteration count: animate-count-<value>
        if let Some(count) = class.strip_prefix("animate-count-") {
            return Some(vec![CssProperty {
                name: "animation-iteration-count".to_string(),
                value: self.parse_iteration_count_value(count),
                important: false,
            }]);
        }

        None
    }

    fn parse_duration_value(&self, value: &str) -> String {
        match value {
            "75" => "75ms".to_string(),
            "100" => "100ms".to_string(),
            "150" => "150ms".to_string(),
            "200" => "200ms".to_string(),
            "300" => "300ms".to_string(),
            "500" => "500ms".to_string(),
            "700" => "700ms".to_string(),
            "1000" => "1000ms".to_string(),
            _ => format!("{}ms", value),
        }
    }

    fn parse_ease_value(&self, value: &str) -> String {
        match value {
            "linear" => "linear".to_string(),
            "in" => "ease-in".to_string(),
            "out" => "ease-out".to_string(),
            "in-out" => "ease-in-out".to_string(),
            _ => "ease".to_string(),
        }
    }

    fn parse_direction_value(&self, value: &str) -> String {
        match value {
            "normal" => "normal".to_string(),
            "reverse" => "reverse".to_string(),
            "alternate" => "alternate".to_string(),
            "alternate-reverse" => "alternate-reverse".to_string(),
            _ => "normal".to_string(),
        }
    }

    fn parse_fill_mode_value(&self, value: &str) -> String {
        match value {
            "none" => "none".to_string(),
            "forwards" => "forwards".to_string(),
            "backwards" => "backwards".to_string(),
            "both" => "both".to_string(),
            _ => "none".to_string(),
        }
    }

    fn parse_iteration_count_value(&self, value: &str) -> String {
        match value {
            "infinite" => "infinite".to_string(),
            _ => value.to_string(),
        }
    }
}

impl UtilityParser for AnimationParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_animation_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec!["animate-"]
    }

    fn get_priority(&self) -> u32 {
        50
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Animations
    }
}

impl Default for AnimationParser {
    fn default() -> Self {
        Self::new()
    }
}
