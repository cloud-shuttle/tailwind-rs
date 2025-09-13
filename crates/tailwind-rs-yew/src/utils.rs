//! # Utility Functions
//!
//! This module provides utility functions for working with Tailwind classes in Yew.

use tailwind_rs_core::{Breakpoint, State};
use yew::prelude::*;

/// Utility for merging multiple class strings
pub fn merge_classes(classes: &[&str]) -> Classes {
    let merged = classes
        .iter()
        .filter(|&&class| !class.is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join(" ");

    Classes::from(merged)
}

/// Utility for conditionally including classes
pub fn conditional_classes(
    base_classes: &str,
    condition: bool,
    conditional_classes: &str,
) -> Classes {
    let classes = if condition {
        format!("{} {}", base_classes, conditional_classes)
    } else {
        base_classes.to_string()
    };

    Classes::from(classes)
}

/// Utility for creating responsive classes
pub fn responsive_classes(base_classes: &str, responsive: &[(Breakpoint, &str)]) -> Classes {
    let mut result = base_classes.to_string();

    for (breakpoint, classes) in responsive {
        result.push(' ');
        result.push_str(&format!("{}{}", breakpoint.prefix(), classes));
    }

    Classes::from(result)
}

/// Utility for creating state-based classes
pub fn state_classes(base_classes: &str, states: &[(State, &str)]) -> Classes {
    let mut result = base_classes.to_string();

    for (state, classes) in states {
        result.push(' ');
        result.push_str(&format!("{}:{}", state.prefix(), classes));
    }

    Classes::from(result)
}

/// Utility for creating variant-based classes
pub fn variant_classes(base_classes: &str, variant: &str) -> Classes {
    let classes = format!("{} {}", base_classes, variant);
    Classes::from(classes)
}

/// Utility for creating size-based classes
pub fn size_classes(base_classes: &str, size: &str) -> Classes {
    let classes = format!("{} {}", base_classes, size);
    Classes::from(classes)
}

/// Utility for creating color-based classes
pub fn color_classes(base_classes: &str, color: &str) -> Classes {
    let classes = format!("{} {}", base_classes, color);
    Classes::from(classes)
}

/// Utility for creating spacing classes
pub fn spacing_classes(base_classes: &str, spacing: &str) -> Classes {
    let classes = format!("{} {}", base_classes, spacing);
    Classes::from(classes)
}

/// Utility for creating typography classes
pub fn typography_classes(base_classes: &str, typography: &str) -> Classes {
    let classes = format!("{} {}", base_classes, typography);
    Classes::from(classes)
}

/// Utility for creating layout classes
pub fn layout_classes(base_classes: &str, layout: &str) -> Classes {
    let classes = format!("{} {}", base_classes, layout);
    Classes::from(classes)
}

/// Utility for creating flexbox classes
pub fn flex_classes(base_classes: &str, flex: &str) -> Classes {
    let classes = format!("{} {}", base_classes, flex);
    Classes::from(classes)
}

/// Utility for creating grid classes
pub fn grid_classes(base_classes: &str, grid: &str) -> Classes {
    let classes = format!("{} {}", base_classes, grid);
    Classes::from(classes)
}

/// Utility for creating border classes
pub fn border_classes(base_classes: &str, border: &str) -> Classes {
    let classes = format!("{} {}", base_classes, border);
    Classes::from(classes)
}

/// Utility for creating shadow classes
pub fn shadow_classes(base_classes: &str, shadow: &str) -> Classes {
    let classes = format!("{} {}", base_classes, shadow);
    Classes::from(classes)
}

/// Utility for creating opacity classes
pub fn opacity_classes(base_classes: &str, opacity: &str) -> Classes {
    let classes = format!("{} {}", base_classes, opacity);
    Classes::from(classes)
}

/// Utility for creating transform classes
pub fn transform_classes(base_classes: &str, transform: &str) -> Classes {
    let classes = format!("{} {}", base_classes, transform);
    Classes::from(classes)
}

/// Utility for creating transition classes
pub fn transition_classes(base_classes: &str, transition: &str) -> Classes {
    let classes = format!("{} {}", base_classes, transition);
    Classes::from(classes)
}

/// Utility for creating animation classes
pub fn animation_classes(base_classes: &str, animation: &str) -> Classes {
    let classes = format!("{} {}", base_classes, animation);
    Classes::from(classes)
}

/// Utility for creating filter classes
pub fn filter_classes(base_classes: &str, filter: &str) -> Classes {
    let classes = format!("{} {}", base_classes, filter);
    Classes::from(classes)
}

/// Utility for creating backdrop classes
pub fn backdrop_classes(base_classes: &str, backdrop: &str) -> Classes {
    let classes = format!("{} {}", base_classes, backdrop);
    Classes::from(classes)
}

/// Utility for creating ring classes
pub fn ring_classes(base_classes: &str, ring: &str) -> Classes {
    let classes = format!("{} {}", base_classes, ring);
    Classes::from(classes)
}

/// Utility for creating divide classes
pub fn divide_classes(base_classes: &str, divide: &str) -> Classes {
    let classes = format!("{} {}", base_classes, divide);
    Classes::from(classes)
}

/// Utility for creating placeholder classes
pub fn placeholder_classes(base_classes: &str, placeholder: &str) -> Classes {
    let classes = format!("{} {}", base_classes, placeholder);
    Classes::from(classes)
}

/// Utility for creating file classes
pub fn file_classes(base_classes: &str, file: &str) -> Classes {
    let classes = format!("{} {}", base_classes, file);
    Classes::from(classes)
}

/// Utility for creating marker classes
pub fn marker_classes(base_classes: &str, marker: &str) -> Classes {
    let classes = format!("{} {}", base_classes, marker);
    Classes::from(classes)
}

/// Utility for creating accent classes
pub fn accent_classes(base_classes: &str, accent: &str) -> Classes {
    let classes = format!("{} {}", base_classes, accent);
    Classes::from(classes)
}

/// Utility for creating appearance classes
pub fn appearance_classes(base_classes: &str, appearance: &str) -> Classes {
    let classes = format!("{} {}", base_classes, appearance);
    Classes::from(classes)
}

/// Utility for creating cursor classes
pub fn cursor_classes(base_classes: &str, cursor: &str) -> Classes {
    let classes = format!("{} {}", base_classes, cursor);
    Classes::from(classes)
}

/// Utility for creating outline classes
pub fn outline_classes(base_classes: &str, outline: &str) -> Classes {
    let classes = format!("{} {}", base_classes, outline);
    Classes::from(classes)
}

/// Utility for creating resize classes
pub fn resize_classes(base_classes: &str, resize: &str) -> Classes {
    let classes = format!("{} {}", base_classes, resize);
    Classes::from(classes)
}

/// Utility for creating scroll classes
pub fn scroll_classes(base_classes: &str, scroll: &str) -> Classes {
    let classes = format!("{} {}", base_classes, scroll);
    Classes::from(classes)
}

/// Utility for creating snap classes
pub fn snap_classes(base_classes: &str, snap: &str) -> Classes {
    let classes = format!("{} {}", base_classes, snap);
    Classes::from(classes)
}

/// Utility for creating touch classes
pub fn touch_classes(base_classes: &str, touch: &str) -> Classes {
    let classes = format!("{} {}", base_classes, touch);
    Classes::from(classes)
}

/// Utility for creating user select classes
pub fn user_select_classes(base_classes: &str, user_select: &str) -> Classes {
    let classes = format!("{} {}", base_classes, user_select);
    Classes::from(classes)
}

/// Utility for creating will change classes
pub fn will_change_classes(base_classes: &str, will_change: &str) -> Classes {
    let classes = format!("{} {}", base_classes, will_change);
    Classes::from(classes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_classes() {
        let classes = vec!["px-4", "py-2", "bg-blue-600"];
        let result = merge_classes(&classes);
        let class_string = result.to_string();
        assert_eq!(class_string, "px-4 py-2 bg-blue-600");
    }

    #[test]
    fn test_merge_classes_with_empty() {
        let classes = vec!["px-4", "", "bg-blue-600"];
        let result = merge_classes(&classes);
        let class_string = result.to_string();
        assert_eq!(class_string, "px-4 bg-blue-600");
    }

    #[test]
    fn test_conditional_classes() {
        let result = conditional_classes("px-4", true, "bg-blue-600");
        let class_string = result.to_string();
        assert_eq!(class_string, "px-4 bg-blue-600");

        let result = conditional_classes("px-4", false, "bg-blue-600");
        let class_string = result.to_string();
        assert_eq!(class_string, "px-4");
    }

    #[test]
    fn test_responsive_classes() {
        let responsive = vec![(Breakpoint::Sm, "text-sm"), (Breakpoint::Md, "text-base")];
        let result = responsive_classes("px-4", &responsive);
        let class_string = result.to_string();
        assert!(class_string.contains("sm:text-sm"));
        assert!(class_string.contains("md:text-base"));
    }

    #[test]
    fn test_state_classes() {
        let states = vec![
            (State::Hover, "hover:bg-blue-700"),
            (State::Focus, "focus:ring-2"),
        ];
        let result = state_classes("px-4", &states);
        let class_string = result.to_string();
        assert!(class_string.contains("hover:bg-blue-700"));
        assert!(class_string.contains("focus:ring-2"));
    }

    #[test]
    fn test_variant_classes() {
        let result = variant_classes("px-4 py-2", "bg-blue-600");
        let class_string = result.to_string();
        assert_eq!(class_string, "px-4 py-2 bg-blue-600");
    }
}
