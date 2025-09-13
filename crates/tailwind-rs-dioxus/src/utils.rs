//! # Utility Functions
//!
//! This module provides utility functions for working with Tailwind classes in Dioxus.

use dioxus::prelude::*;
use std::collections::HashMap;
use tailwind_rs_core::{Breakpoint, ClassBuilder, State};

/// Utility for merging multiple class strings
pub fn merge_classes(classes: &[&str]) -> String {
    classes
        .iter()
        .filter(|&&class| !class.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Utility for conditionally including classes
pub fn conditional_classes(
    base_classes: &str,
    condition: bool,
    conditional_classes: &str,
) -> String {
    if condition {
        format!("{} {}", base_classes, conditional_classes)
    } else {
        base_classes.to_string()
    }
}

/// Utility for creating responsive classes
pub fn responsive_classes(base_classes: &str, responsive: &[(Breakpoint, &str)]) -> String {
    let mut result = base_classes.to_string();

    for (breakpoint, classes) in responsive {
        result.push(' ');
        result.push_str(&format!("{}:{}", breakpoint.prefix(), classes));
    }

    result
}

/// Utility for creating state-based classes
pub fn state_classes(base_classes: &str, states: &[(State, &str)]) -> String {
    let mut result = base_classes.to_string();

    for (state, classes) in states {
        result.push(' ');
        result.push_str(&format!("{}:{}", state.prefix(), classes));
    }

    result
}

/// Utility for creating variant-based classes
pub fn variant_classes(base_classes: &str, variant: &str) -> String {
    format!("{} {}", base_classes, variant)
}

/// Utility for creating size-based classes
pub fn size_classes(base_classes: &str, size: &str) -> String {
    format!("{} {}", base_classes, size)
}

/// Utility for creating color-based classes
pub fn color_classes(base_classes: &str, color: &str) -> String {
    format!("{} {}", base_classes, color)
}

/// Utility for creating spacing classes
pub fn spacing_classes(base_classes: &str, spacing: &str) -> String {
    format!("{} {}", base_classes, spacing)
}

/// Utility for creating typography classes
pub fn typography_classes(base_classes: &str, typography: &str) -> String {
    format!("{} {}", base_classes, typography)
}

/// Utility for creating layout classes
pub fn layout_classes(base_classes: &str, layout: &str) -> String {
    format!("{} {}", base_classes, layout)
}

/// Utility for creating flexbox classes
pub fn flex_classes(base_classes: &str, flex: &str) -> String {
    format!("{} {}", base_classes, flex)
}

/// Utility for creating grid classes
pub fn grid_classes(base_classes: &str, grid: &str) -> String {
    format!("{} {}", base_classes, grid)
}

/// Utility for creating border classes
pub fn border_classes(base_classes: &str, border: &str) -> String {
    format!("{} {}", base_classes, border)
}

/// Utility for creating shadow classes
pub fn shadow_classes(base_classes: &str, shadow: &str) -> String {
    format!("{} {}", base_classes, shadow)
}

/// Utility for creating opacity classes
pub fn opacity_classes(base_classes: &str, opacity: &str) -> String {
    format!("{} {}", base_classes, opacity)
}

/// Utility for creating transform classes
pub fn transform_classes(base_classes: &str, transform: &str) -> String {
    format!("{} {}", base_classes, transform)
}

/// Utility for creating transition classes
pub fn transition_classes(base_classes: &str, transition: &str) -> String {
    format!("{} {}", base_classes, transition)
}

/// Utility for creating animation classes
pub fn animation_classes(base_classes: &str, animation: &str) -> String {
    format!("{} {}", base_classes, animation)
}

/// Utility for creating filter classes
pub fn filter_classes(base_classes: &str, filter: &str) -> String {
    format!("{} {}", base_classes, filter)
}

/// Utility for creating backdrop classes
pub fn backdrop_classes(base_classes: &str, backdrop: &str) -> String {
    format!("{} {}", base_classes, backdrop)
}

/// Utility for creating ring classes
pub fn ring_classes(base_classes: &str, ring: &str) -> String {
    format!("{} {}", base_classes, ring)
}

/// Utility for creating divide classes
pub fn divide_classes(base_classes: &str, divide: &str) -> String {
    format!("{} {}", base_classes, divide)
}

/// Utility for creating placeholder classes
pub fn placeholder_classes(base_classes: &str, placeholder: &str) -> String {
    format!("{} {}", base_classes, placeholder)
}

/// Utility for creating file classes
pub fn file_classes(base_classes: &str, file: &str) -> String {
    format!("{} {}", base_classes, file)
}

/// Utility for creating marker classes
pub fn marker_classes(base_classes: &str, marker: &str) -> String {
    format!("{} {}", base_classes, marker)
}

/// Utility for creating accent classes
pub fn accent_classes(base_classes: &str, accent: &str) -> String {
    format!("{} {}", base_classes, accent)
}

/// Utility for creating appearance classes
pub fn appearance_classes(base_classes: &str, appearance: &str) -> String {
    format!("{} {}", base_classes, appearance)
}

/// Utility for creating cursor classes
pub fn cursor_classes(base_classes: &str, cursor: &str) -> String {
    format!("{} {}", base_classes, cursor)
}

/// Utility for creating outline classes
pub fn outline_classes(base_classes: &str, outline: &str) -> String {
    format!("{} {}", base_classes, outline)
}

/// Utility for creating resize classes
pub fn resize_classes(base_classes: &str, resize: &str) -> String {
    format!("{} {}", base_classes, resize)
}

/// Utility for creating scroll classes
pub fn scroll_classes(base_classes: &str, scroll: &str) -> String {
    format!("{} {}", base_classes, scroll)
}

/// Utility for creating snap classes
pub fn snap_classes(base_classes: &str, snap: &str) -> String {
    format!("{} {}", base_classes, snap)
}

/// Utility for creating touch classes
pub fn touch_classes(base_classes: &str, touch: &str) -> String {
    format!("{} {}", base_classes, touch)
}

/// Utility for creating user select classes
pub fn user_select_classes(base_classes: &str, user_select: &str) -> String {
    format!("{} {}", base_classes, user_select)
}

/// Utility for creating will change classes
pub fn will_change_classes(base_classes: &str, will_change: &str) -> String {
    format!("{} {}", base_classes, will_change)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_classes() {
        let classes = vec!["px-4", "py-2", "bg-blue-600"];
        let result = merge_classes(&classes);
        assert_eq!(result, "px-4 py-2 bg-blue-600");
    }

    #[test]
    fn test_merge_classes_with_empty() {
        let classes = vec!["px-4", "", "bg-blue-600"];
        let result = merge_classes(&classes);
        assert_eq!(result, "px-4 bg-blue-600");
    }

    #[test]
    fn test_conditional_classes() {
        let result = conditional_classes("px-4", true, "bg-blue-600");
        assert_eq!(result, "px-4 bg-blue-600");

        let result = conditional_classes("px-4", false, "bg-blue-600");
        assert_eq!(result, "px-4");
    }

    #[test]
    fn test_responsive_classes() {
        let responsive = vec![(Breakpoint::Sm, "text-sm"), (Breakpoint::Md, "text-base")];
        let result = responsive_classes("px-4", &responsive);
        assert!(result.contains("sm:text-sm"));
        assert!(result.contains("md:text-base"));
    }

    #[test]
    fn test_state_classes() {
        let states = vec![
            (State::Hover, "hover:bg-blue-700"),
            (State::Focus, "focus:ring-2"),
        ];
        let result = state_classes("px-4", &states);
        assert!(result.contains("hover:bg-blue-700"));
        assert!(result.contains("focus:ring-2"));
    }

    #[test]
    fn test_variant_classes() {
        let result = variant_classes("px-4 py-2", "bg-blue-600");
        assert_eq!(result, "px-4 py-2 bg-blue-600");
    }
}
