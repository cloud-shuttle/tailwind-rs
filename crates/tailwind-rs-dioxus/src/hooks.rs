//! # Dioxus Hooks
//!
//! This module provides custom hooks for working with Tailwind classes in Dioxus.

use dioxus::prelude::*;
use std::collections::HashMap;
use tailwind_rs_core::ClassBuilder;

/// Hook for creating reactive classes
pub fn use_reactive_classes<F>(class_fn: F) -> Signal<String>
where
    F: Fn() -> String + 'static,
{
    use_signal(move || class_fn())
}

/// Hook for creating conditional classes
pub fn use_conditional_classes(
    base_classes: &str,
    condition: Signal<bool>,
    conditional_classes: &str,
) -> Signal<String> {
    use_signal(move || {
        if condition() {
            format!("{} {}", base_classes, conditional_classes)
        } else {
            base_classes.to_string()
        }
    })
}

/// Hook for creating responsive classes
pub fn use_responsive_classes(
    base_classes: &str,
    responsive_classes: HashMap<crate::Breakpoint, &str>,
) -> Signal<String> {
    use_signal(move || {
        let mut result = base_classes.to_string();

        for (breakpoint, classes) in &responsive_classes {
            result.push(' ');
            result.push_str(&format!("{}:{}", breakpoint.prefix(), classes));
        }

        result
    })
}

/// Hook for creating state-based classes
pub fn use_state_classes(
    base_classes: &str,
    state_classes: HashMap<crate::State, &str>,
) -> Signal<String> {
    use_signal(move || {
        let mut result = base_classes.to_string();

        for (state, classes) in &state_classes {
            result.push(' ');
            result.push_str(&format!("{}:{}", state.prefix(), classes));
        }

        result
    })
}

/// Hook for creating variant-based classes
pub fn use_variant_classes<T>(
    base_classes: &str,
    variant: Signal<T>,
    variant_classes: impl Fn(T) -> &'static str + 'static,
) -> Signal<String>
where
    T: Clone + 'static,
{
    use_signal(move || {
        let variant_value = variant();
        let variant_class = variant_classes(variant_value);
        format!("{} {}", base_classes, variant_class)
    })
}

/// Hook for creating size-based classes
pub fn use_size_classes<T>(
    base_classes: &str,
    size: Signal<T>,
    size_classes: impl Fn(T) -> &'static str + 'static,
) -> Signal<String>
where
    T: Clone + 'static,
{
    use_signal(move || {
        let size_value = size();
        let size_class = size_classes(size_value);
        format!("{} {}", base_classes, size_class)
    })
}

/// Hook for creating color-based classes
pub fn use_color_classes(base_classes: &str, color: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let color_value = color();
        format!("{} {}", base_classes, color_value)
    })
}

/// Hook for creating spacing classes
pub fn use_spacing_classes(base_classes: &str, spacing: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let spacing_value = spacing();
        format!("{} {}", base_classes, spacing_value)
    })
}

/// Hook for creating typography classes
pub fn use_typography_classes(base_classes: &str, typography: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let typography_value = typography();
        format!("{} {}", base_classes, typography_value)
    })
}

/// Hook for creating layout classes
pub fn use_layout_classes(base_classes: &str, layout: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let layout_value = layout();
        format!("{} {}", base_classes, layout_value)
    })
}

/// Hook for creating flexbox classes
pub fn use_flex_classes(base_classes: &str, flex: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let flex_value = flex();
        format!("{} {}", base_classes, flex_value)
    })
}

/// Hook for creating grid classes
pub fn use_grid_classes(base_classes: &str, grid: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let grid_value = grid();
        format!("{} {}", base_classes, grid_value)
    })
}

/// Hook for creating border classes
pub fn use_border_classes(base_classes: &str, border: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let border_value = border();
        format!("{} {}", base_classes, border_value)
    })
}

/// Hook for creating shadow classes
pub fn use_shadow_classes(base_classes: &str, shadow: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let shadow_value = shadow();
        format!("{} {}", base_classes, shadow_value)
    })
}

/// Hook for creating opacity classes
pub fn use_opacity_classes(base_classes: &str, opacity: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let opacity_value = opacity();
        format!("{} {}", base_classes, opacity_value)
    })
}

/// Hook for creating transform classes
pub fn use_transform_classes(base_classes: &str, transform: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let transform_value = transform();
        format!("{} {}", base_classes, transform_value)
    })
}

/// Hook for creating transition classes
pub fn use_transition_classes(base_classes: &str, transition: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let transition_value = transition();
        format!("{} {}", base_classes, transition_value)
    })
}

/// Hook for creating animation classes
pub fn use_animation_classes(base_classes: &str, animation: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let animation_value = animation();
        format!("{} {}", base_classes, animation_value)
    })
}

/// Hook for creating filter classes
pub fn use_filter_classes(base_classes: &str, filter: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let filter_value = filter();
        format!("{} {}", base_classes, filter_value)
    })
}

/// Hook for creating backdrop classes
pub fn use_backdrop_classes(base_classes: &str, backdrop: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let backdrop_value = backdrop();
        format!("{} {}", base_classes, backdrop_value)
    })
}

/// Hook for creating ring classes
pub fn use_ring_classes(base_classes: &str, ring: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let ring_value = ring();
        format!("{} {}", base_classes, ring_value)
    })
}

/// Hook for creating divide classes
pub fn use_divide_classes(base_classes: &str, divide: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let divide_value = divide();
        format!("{} {}", base_classes, divide_value)
    })
}

/// Hook for creating placeholder classes
pub fn use_placeholder_classes(base_classes: &str, placeholder: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let placeholder_value = placeholder();
        format!("{} {}", base_classes, placeholder_value)
    })
}

/// Hook for creating file classes
pub fn use_file_classes(base_classes: &str, file: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let file_value = file();
        format!("{} {}", base_classes, file_value)
    })
}

/// Hook for creating marker classes
pub fn use_marker_classes(base_classes: &str, marker: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let marker_value = marker();
        format!("{} {}", base_classes, marker_value)
    })
}

/// Hook for creating accent classes
pub fn use_accent_classes(base_classes: &str, accent: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let accent_value = accent();
        format!("{} {}", base_classes, accent_value)
    })
}

/// Hook for creating appearance classes
pub fn use_appearance_classes(base_classes: &str, appearance: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let appearance_value = appearance();
        format!("{} {}", base_classes, appearance_value)
    })
}

/// Hook for creating cursor classes
pub fn use_cursor_classes(base_classes: &str, cursor: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let cursor_value = cursor();
        format!("{} {}", base_classes, cursor_value)
    })
}

/// Hook for creating outline classes
pub fn use_outline_classes(base_classes: &str, outline: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let outline_value = outline();
        format!("{} {}", base_classes, outline_value)
    })
}

/// Hook for creating resize classes
pub fn use_resize_classes(base_classes: &str, resize: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let resize_value = resize();
        format!("{} {}", base_classes, resize_value)
    })
}

/// Hook for creating scroll classes
pub fn use_scroll_classes(base_classes: &str, scroll: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let scroll_value = scroll();
        format!("{} {}", base_classes, scroll_value)
    })
}

/// Hook for creating snap classes
pub fn use_snap_classes(base_classes: &str, snap: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let snap_value = snap();
        format!("{} {}", base_classes, snap_value)
    })
}

/// Hook for creating touch classes
pub fn use_touch_classes(base_classes: &str, touch: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let touch_value = touch();
        format!("{} {}", base_classes, touch_value)
    })
}

/// Hook for creating user select classes
pub fn use_user_select_classes(base_classes: &str, user_select: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let user_select_value = user_select();
        format!("{} {}", base_classes, user_select_value)
    })
}

/// Hook for creating will change classes
pub fn use_will_change_classes(base_classes: &str, will_change: Signal<String>) -> Signal<String> {
    use_signal(move || {
        let will_change_value = will_change();
        format!("{} {}", base_classes, will_change_value)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_conditional_classes() {
        let (condition, set_condition) = use_signal(|| false);

        let classes = use_conditional_classes("px-4", condition, "bg-blue-600");

        // Initially should not have conditional classes
        assert!(!classes().contains("bg-blue-600"));

        // After setting condition to true
        set_condition.set(true);
        assert!(classes().contains("bg-blue-600"));
    }

    #[test]
    fn test_use_variant_classes() {
        #[derive(Clone, Copy, PartialEq)]
        enum TestVariant {
            Primary,
            Secondary,
        }

        let (variant, set_variant) = use_signal(|| TestVariant::Primary);

        let classes = use_variant_classes("px-4", variant, |v| match v {
            TestVariant::Primary => "bg-blue-600",
            TestVariant::Secondary => "bg-gray-600",
        });

        // Test initial state
        assert!(classes().contains("bg-blue-600"));

        // Test variant change
        set_variant.set(TestVariant::Secondary);
        assert!(classes().contains("bg-gray-600"));
        assert!(!classes().contains("bg-blue-600"));
    }

    #[test]
    fn test_use_color_classes() {
        let (color, set_color) = use_signal(|| "bg-blue-600".to_string());

        let classes = use_color_classes("px-4", color);

        // Test initial state
        assert!(classes().contains("bg-blue-600"));

        // Test color change
        set_color.set("bg-red-600".to_string());
        assert!(classes().contains("bg-red-600"));
        assert!(!classes().contains("bg-blue-600"));
    }
}
