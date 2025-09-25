//! Enhanced ClassBuilder API Tests

use crate::classes::ClassBuilder;
use crate::responsive::Breakpoint;

#[cfg(test)]
mod enhanced_api_tests {
    use super::*;

    #[test]
    fn test_layout_utilities() {
        let classes = ClassBuilder::new().relative().build_string();
        assert!(classes.contains("relative"));
    }

    #[test]
    fn test_positioning_utilities() {
        let classes = ClassBuilder::new()
            .absolute()
            .fixed()
            .sticky()
            .static_pos()
            .build_string();
        
        assert!(classes.contains("absolute"));
        assert!(classes.contains("fixed"));
        assert!(classes.contains("sticky"));
        assert!(classes.contains("static"));
    }

    #[test]
    fn test_display_utilities() {
        let classes = ClassBuilder::new()
            .block()
            .inline()
            .inline_block()
            .flex()
            .inline_flex()
            .grid()
            .inline_grid()
            .hidden()
            .visible()
            .build_string();
        
        assert!(classes.contains("block"));
        assert!(classes.contains("inline"));
        assert!(classes.contains("inline-block"));
        assert!(classes.contains("flex"));
        assert!(classes.contains("inline-flex"));
        assert!(classes.contains("grid"));
        assert!(classes.contains("inline-grid"));
        assert!(classes.contains("hidden"));
        assert!(classes.contains("visible"));
    }

    #[test]
    fn test_flexbox_utilities() {
        let classes = ClassBuilder::new()
            .flex_none()
            .flex_1()
            .flex_auto()
            .flex_initial()
            .flex_col()
            .flex_row()
            .flex_wrap_class()
            .flex_nowrap_class()
            .build_string();
        
        assert!(classes.contains("flex-none"));
        assert!(classes.contains("flex-1"));
        assert!(classes.contains("flex-auto"));
        assert!(classes.contains("flex-initial"));
        assert!(classes.contains("flex-col"));
        assert!(classes.contains("flex-row"));
        assert!(classes.contains("flex-wrap"));
        assert!(classes.contains("flex-nowrap"));
    }

    #[test]
    fn test_transition_utilities() {
        let classes = ClassBuilder::new()
            .transition()
            .transition_all()
            .transition_colors()
            .transition_opacity()
            .transition_shadow()
            .transition_transform()
            .build_string();
        
        assert!(classes.contains("transition"));
        assert!(classes.contains("transition-all"));
        assert!(classes.contains("transition-colors"));
        assert!(classes.contains("transition-opacity"));
        assert!(classes.contains("transition-shadow"));
        assert!(classes.contains("transition-transform"));
    }

    #[test]
    fn test_pseudo_class_support() {
        let classes = ClassBuilder::new()
            .bg_blue_500()
            .hover("bg-blue-600")
            .focus("ring-2")
            .active("bg-blue-700")
            .dark("bg-gray-800")
            .group_hover("bg-blue-600")
            .peer_hover("bg-blue-600")
            .build_string();
        
        assert!(classes.contains("bg-blue-500"));
        assert!(classes.contains("hover:bg-blue-600"));
        assert!(classes.contains("focus:ring-2"));
        assert!(classes.contains("active:bg-blue-700"));
        assert!(classes.contains("dark:bg-gray-800"));
        assert!(classes.contains("group-hover:bg-blue-600"));
        assert!(classes.contains("peer-hover:bg-blue-600"));
    }

    #[test]
    fn test_spacing_utilities() {
        let classes = ClassBuilder::new()
            .p_4()
            .px_4()
            .py_4()
            .m_4()
            .mx_4()
            .my_4()
            .build_string();
        
        assert!(classes.contains("p-4"));
        assert!(classes.contains("px-4"));
        assert!(classes.contains("py-4"));
        assert!(classes.contains("m-4"));
        assert!(classes.contains("mx-4"));
        assert!(classes.contains("my-4"));
    }

    #[test]
    fn test_color_utilities() {
        let classes = ClassBuilder::new()
            .bg_blue_500()
            .text_white()
            .border_gray_300()
            .build_string();
        
        assert!(classes.contains("bg-blue-500"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("border-gray-300"));
    }

    #[test]
    fn test_complex_combinations() {
        let classes = ClassBuilder::new()
            .relative()
            .flex()
            .flex_col()
            .p_4()
            .bg_blue_500()
            .text_white()
            .hover("bg-blue-600")
            .focus("ring-2")
            .dark("bg-gray-800")
            .transition()
            .transition_colors()
            .build_string();
        
        assert!(classes.contains("relative"));
        assert!(classes.contains("flex"));
        assert!(classes.contains("flex-col"));
        assert!(classes.contains("p-4"));
        assert!(classes.contains("bg-blue-500"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("hover:bg-blue-600"));
        assert!(classes.contains("focus:ring-2"));
        assert!(classes.contains("dark:bg-gray-800"));
        assert!(classes.contains("transition"));
        assert!(classes.contains("transition-colors"));
    }

    #[test]
    #[should_panic(expected = "hover: class cannot be empty")]
    fn test_hover_empty_class_panic() {
        ClassBuilder::new().hover("");
    }

    #[test]
    #[should_panic(expected = "hover: class should not contain ':' prefix")]
    fn test_hover_with_prefix_panic() {
        ClassBuilder::new().hover("hover:bg-blue-500");
    }

    #[test]
    #[should_panic(expected = "focus: class cannot be empty")]
    fn test_focus_empty_class_panic() {
        ClassBuilder::new().focus("");
    }

    #[test]
    #[should_panic(expected = "focus: class should not contain ':' prefix")]
    fn test_focus_with_prefix_panic() {
        ClassBuilder::new().focus("focus:ring-2");
    }

    #[test]
    #[should_panic(expected = "dark: class cannot be empty")]
    fn test_dark_empty_class_panic() {
        ClassBuilder::new().dark("");
    }

    #[test]
    #[should_panic(expected = "dark: class should not contain ':' prefix")]
    fn test_dark_with_prefix_panic() {
        ClassBuilder::new().dark("dark:bg-gray-800");
    }

    #[test]
    #[should_panic(expected = "group-hover: class cannot be empty")]
    fn test_group_hover_empty_class_panic() {
        ClassBuilder::new().group_hover("");
    }

    #[test]
    #[should_panic(expected = "group-hover: class should not contain ':' prefix")]
    fn test_group_hover_with_prefix_panic() {
        ClassBuilder::new().group_hover("group-hover:bg-blue-600");
    }

    #[test]
    #[should_panic(expected = "peer-hover: class cannot be empty")]
    fn test_peer_hover_empty_class_panic() {
        ClassBuilder::new().peer_hover("");
    }

    #[test]
    #[should_panic(expected = "peer-hover: class should not contain ':' prefix")]
    fn test_peer_hover_with_prefix_panic() {
        ClassBuilder::new().peer_hover("peer-hover:bg-blue-600");
    }
}
