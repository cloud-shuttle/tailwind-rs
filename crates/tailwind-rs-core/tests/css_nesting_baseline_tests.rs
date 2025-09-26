use tailwind_rs_core::utilities::css_nesting::*;
use tailwind_rs_core::Breakpoint;
use tailwind_rs_core::ClassBuilder;

#[cfg(test)]
mod css_nesting_baseline_tests {
    use super::*;

    #[test]
    fn test_css_nesting_css_output_baseline() {
        let builder = ClassBuilder::new()
            .nesting_selector(NestingSelector::DirectChild)
            .nesting_pseudo_class(NestingPseudoClass::Hover);

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain both CSS nesting classes
        assert!(classes.contains("nest-child"));
        assert!(classes.contains("nest-hover"));
    }

    #[test]
    fn test_css_nesting_class_generation_baseline() {
        let builder = ClassBuilder::new()
            .nesting_media_query(NestingMediaQuery::Small)
            .nesting_media_query(NestingMediaQuery::Medium);

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain both CSS nesting classes
        assert!(classes.contains("nest-sm"));
        assert!(classes.contains("nest-md"));
    }

    #[test]
    fn test_nesting_selector_direct_child_baseline() {
        let selector = NestingSelector::DirectChild;
        let string_value = selector.to_string();

        // Baseline string output
        assert_eq!(string_value, ">");
    }

    #[test]
    fn test_nesting_selector_descendant_baseline() {
        let selector = NestingSelector::Descendant;
        let string_value = selector.to_string();

        // Baseline string output
        assert_eq!(string_value, " ");
    }

    #[test]
    fn test_nesting_selector_adjacent_sibling_baseline() {
        let selector = NestingSelector::AdjacentSibling;
        let string_value = selector.to_string();

        // Baseline string output
        assert_eq!(string_value, "+");
    }

    #[test]
    fn test_nesting_selector_general_sibling_baseline() {
        let selector = NestingSelector::GeneralSibling;
        let string_value = selector.to_string();

        // Baseline string output
        assert_eq!(string_value, "~");
    }

    #[test]
    fn test_nesting_selector_custom_baseline() {
        let selector = NestingSelector::Custom("div".to_string());
        let string_value = selector.to_string();

        // Baseline string output
        assert_eq!(string_value, "div");
    }

    #[test]
    fn test_nesting_selector_class_name_direct_child_baseline() {
        let selector = NestingSelector::DirectChild;
        let class_name = selector.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-child");
    }

    #[test]
    fn test_nesting_selector_class_name_descendant_baseline() {
        let selector = NestingSelector::Descendant;
        let class_name = selector.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-descendant");
    }

    #[test]
    fn test_nesting_selector_class_name_adjacent_sibling_baseline() {
        let selector = NestingSelector::AdjacentSibling;
        let class_name = selector.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-adjacent");
    }

    #[test]
    fn test_nesting_selector_class_name_general_sibling_baseline() {
        let selector = NestingSelector::GeneralSibling;
        let class_name = selector.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-sibling");
    }

    #[test]
    fn test_nesting_selector_class_name_custom_baseline() {
        let selector = NestingSelector::Custom("div".to_string());
        let class_name = selector.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-div");
    }

    #[test]
    fn test_nesting_selector_css_value_direct_child_baseline() {
        let selector = NestingSelector::DirectChild;
        let css_value = selector.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, ">");
    }

    #[test]
    fn test_nesting_selector_css_value_descendant_baseline() {
        let selector = NestingSelector::Descendant;
        let css_value = selector.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, " ");
    }

    #[test]
    fn test_nesting_selector_css_value_adjacent_sibling_baseline() {
        let selector = NestingSelector::AdjacentSibling;
        let css_value = selector.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, "+");
    }

    #[test]
    fn test_nesting_selector_css_value_general_sibling_baseline() {
        let selector = NestingSelector::GeneralSibling;
        let css_value = selector.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, "~");
    }

    #[test]
    fn test_nesting_selector_css_value_custom_baseline() {
        let selector = NestingSelector::Custom("div".to_string());
        let css_value = selector.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, "div");
    }

    #[test]
    fn test_nesting_pseudo_class_hover_baseline() {
        let pseudo_class = NestingPseudoClass::Hover;
        let string_value = pseudo_class.to_string();

        // Baseline string output
        assert_eq!(string_value, ":hover");
    }

    #[test]
    fn test_nesting_pseudo_class_focus_baseline() {
        let pseudo_class = NestingPseudoClass::Focus;
        let string_value = pseudo_class.to_string();

        // Baseline string output
        assert_eq!(string_value, ":focus");
    }

    #[test]
    fn test_nesting_pseudo_class_active_baseline() {
        let pseudo_class = NestingPseudoClass::Active;
        let string_value = pseudo_class.to_string();

        // Baseline string output
        assert_eq!(string_value, ":active");
    }

    #[test]
    fn test_nesting_pseudo_class_visited_baseline() {
        let pseudo_class = NestingPseudoClass::Visited;
        let string_value = pseudo_class.to_string();

        // Baseline string output
        assert_eq!(string_value, ":visited");
    }

    #[test]
    fn test_nesting_pseudo_class_link_baseline() {
        let pseudo_class = NestingPseudoClass::Link;
        let string_value = pseudo_class.to_string();

        // Baseline string output
        assert_eq!(string_value, ":link");
    }

    #[test]
    fn test_nesting_pseudo_class_first_child_baseline() {
        let pseudo_class = NestingPseudoClass::FirstChild;
        let string_value = pseudo_class.to_string();

        // Baseline string output
        assert_eq!(string_value, ":first-child");
    }

    #[test]
    fn test_nesting_pseudo_class_last_child_baseline() {
        let pseudo_class = NestingPseudoClass::LastChild;
        let string_value = pseudo_class.to_string();

        // Baseline string output
        assert_eq!(string_value, ":last-child");
    }

    #[test]
    fn test_nesting_pseudo_class_nth_child_baseline() {
        let pseudo_class = NestingPseudoClass::NthChild("2n".to_string());
        let string_value = pseudo_class.to_string();

        // Baseline string output
        assert_eq!(string_value, ":nth-child(2n)");
    }

    #[test]
    fn test_nesting_pseudo_class_custom_baseline() {
        let pseudo_class = NestingPseudoClass::Custom("custom".to_string());
        let string_value = pseudo_class.to_string();

        // Baseline string output
        assert_eq!(string_value, ":custom");
    }

    #[test]
    fn test_nesting_pseudo_class_class_name_hover_baseline() {
        let pseudo_class = NestingPseudoClass::Hover;
        let class_name = pseudo_class.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-hover");
    }

    #[test]
    fn test_nesting_pseudo_class_class_name_focus_baseline() {
        let pseudo_class = NestingPseudoClass::Focus;
        let class_name = pseudo_class.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-focus");
    }

    #[test]
    fn test_nesting_pseudo_class_class_name_active_baseline() {
        let pseudo_class = NestingPseudoClass::Active;
        let class_name = pseudo_class.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-active");
    }

    #[test]
    fn test_nesting_pseudo_class_class_name_visited_baseline() {
        let pseudo_class = NestingPseudoClass::Visited;
        let class_name = pseudo_class.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-visited");
    }

    #[test]
    fn test_nesting_pseudo_class_class_name_link_baseline() {
        let pseudo_class = NestingPseudoClass::Link;
        let class_name = pseudo_class.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-link");
    }

    #[test]
    fn test_nesting_pseudo_class_class_name_first_child_baseline() {
        let pseudo_class = NestingPseudoClass::FirstChild;
        let class_name = pseudo_class.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-first-child");
    }

    #[test]
    fn test_nesting_pseudo_class_class_name_last_child_baseline() {
        let pseudo_class = NestingPseudoClass::LastChild;
        let class_name = pseudo_class.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-last-child");
    }

    #[test]
    fn test_nesting_pseudo_class_class_name_nth_child_baseline() {
        let pseudo_class = NestingPseudoClass::NthChild("2n".to_string());
        let class_name = pseudo_class.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-nth-child-2n");
    }

    #[test]
    fn test_nesting_pseudo_class_class_name_custom_baseline() {
        let pseudo_class = NestingPseudoClass::Custom("custom".to_string());
        let class_name = pseudo_class.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-custom");
    }

    #[test]
    fn test_nesting_pseudo_class_css_value_hover_baseline() {
        let pseudo_class = NestingPseudoClass::Hover;
        let css_value = pseudo_class.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, ":hover");
    }

    #[test]
    fn test_nesting_pseudo_class_css_value_focus_baseline() {
        let pseudo_class = NestingPseudoClass::Focus;
        let css_value = pseudo_class.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, ":focus");
    }

    #[test]
    fn test_nesting_pseudo_class_css_value_active_baseline() {
        let pseudo_class = NestingPseudoClass::Active;
        let css_value = pseudo_class.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, ":active");
    }

    #[test]
    fn test_nesting_pseudo_class_css_value_visited_baseline() {
        let pseudo_class = NestingPseudoClass::Visited;
        let css_value = pseudo_class.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, ":visited");
    }

    #[test]
    fn test_nesting_pseudo_class_css_value_link_baseline() {
        let pseudo_class = NestingPseudoClass::Link;
        let css_value = pseudo_class.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, ":link");
    }

    #[test]
    fn test_nesting_pseudo_class_css_value_first_child_baseline() {
        let pseudo_class = NestingPseudoClass::FirstChild;
        let css_value = pseudo_class.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, ":first-child");
    }

    #[test]
    fn test_nesting_pseudo_class_css_value_last_child_baseline() {
        let pseudo_class = NestingPseudoClass::LastChild;
        let css_value = pseudo_class.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, ":last-child");
    }

    #[test]
    fn test_nesting_pseudo_class_css_value_nth_child_baseline() {
        let pseudo_class = NestingPseudoClass::NthChild("2n".to_string());
        let css_value = pseudo_class.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, ":nth-child(2n)");
    }

    #[test]
    fn test_nesting_pseudo_class_css_value_custom_baseline() {
        let pseudo_class = NestingPseudoClass::Custom("custom".to_string());
        let css_value = pseudo_class.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, ":custom");
    }

    #[test]
    fn test_nesting_media_query_small_baseline() {
        let media_query = NestingMediaQuery::Small;
        let string_value = media_query.to_string();

        // Baseline string output
        assert_eq!(string_value, "(min-width: 640px)");
    }

    #[test]
    fn test_nesting_media_query_medium_baseline() {
        let media_query = NestingMediaQuery::Medium;
        let string_value = media_query.to_string();

        // Baseline string output
        assert_eq!(string_value, "(min-width: 768px)");
    }

    #[test]
    fn test_nesting_media_query_large_baseline() {
        let media_query = NestingMediaQuery::Large;
        let string_value = media_query.to_string();

        // Baseline string output
        assert_eq!(string_value, "(min-width: 1024px)");
    }

    #[test]
    fn test_nesting_media_query_extra_large_baseline() {
        let media_query = NestingMediaQuery::ExtraLarge;
        let string_value = media_query.to_string();

        // Baseline string output
        assert_eq!(string_value, "(min-width: 1280px)");
    }

    #[test]
    fn test_nesting_media_query_dark_baseline() {
        let media_query = NestingMediaQuery::Dark;
        let string_value = media_query.to_string();

        // Baseline string output
        assert_eq!(string_value, "(prefers-color-scheme: dark)");
    }

    #[test]
    fn test_nesting_media_query_light_baseline() {
        let media_query = NestingMediaQuery::Light;
        let string_value = media_query.to_string();

        // Baseline string output
        assert_eq!(string_value, "(prefers-color-scheme: light)");
    }

    #[test]
    fn test_nesting_media_query_print_baseline() {
        let media_query = NestingMediaQuery::Print;
        let string_value = media_query.to_string();

        // Baseline string output
        assert_eq!(string_value, "print");
    }

    #[test]
    fn test_nesting_media_query_screen_baseline() {
        let media_query = NestingMediaQuery::Screen;
        let string_value = media_query.to_string();

        // Baseline string output
        assert_eq!(string_value, "screen");
    }

    #[test]
    fn test_nesting_media_query_custom_baseline() {
        let media_query = NestingMediaQuery::Custom("(max-width: 600px)".to_string());
        let string_value = media_query.to_string();

        // Baseline string output
        assert_eq!(string_value, "(max-width: 600px)");
    }

    #[test]
    fn test_nesting_media_query_class_name_small_baseline() {
        let media_query = NestingMediaQuery::Small;
        let class_name = media_query.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-sm");
    }

    #[test]
    fn test_nesting_media_query_class_name_medium_baseline() {
        let media_query = NestingMediaQuery::Medium;
        let class_name = media_query.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-md");
    }

    #[test]
    fn test_nesting_media_query_class_name_large_baseline() {
        let media_query = NestingMediaQuery::Large;
        let class_name = media_query.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-lg");
    }

    #[test]
    fn test_nesting_media_query_class_name_extra_large_baseline() {
        let media_query = NestingMediaQuery::ExtraLarge;
        let class_name = media_query.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-xl");
    }

    #[test]
    fn test_nesting_media_query_class_name_dark_baseline() {
        let media_query = NestingMediaQuery::Dark;
        let class_name = media_query.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-dark");
    }

    #[test]
    fn test_nesting_media_query_class_name_light_baseline() {
        let media_query = NestingMediaQuery::Light;
        let class_name = media_query.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-light");
    }

    #[test]
    fn test_nesting_media_query_class_name_print_baseline() {
        let media_query = NestingMediaQuery::Print;
        let class_name = media_query.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-print");
    }

    #[test]
    fn test_nesting_media_query_class_name_screen_baseline() {
        let media_query = NestingMediaQuery::Screen;
        let class_name = media_query.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-screen");
    }

    #[test]
    fn test_nesting_media_query_class_name_custom_baseline() {
        let media_query = NestingMediaQuery::Custom("(max-width: 600px)".to_string());
        let class_name = media_query.to_class_name();

        // Baseline class name output
        assert_eq!(class_name, "nest-max-width-600px");
    }

    #[test]
    fn test_nesting_media_query_css_value_small_baseline() {
        let media_query = NestingMediaQuery::Small;
        let css_value = media_query.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, "(min-width: 640px)");
    }

    #[test]
    fn test_nesting_media_query_css_value_medium_baseline() {
        let media_query = NestingMediaQuery::Medium;
        let css_value = media_query.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, "(min-width: 768px)");
    }

    #[test]
    fn test_nesting_media_query_css_value_large_baseline() {
        let media_query = NestingMediaQuery::Large;
        let css_value = media_query.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, "(min-width: 1024px)");
    }

    #[test]
    fn test_nesting_media_query_css_value_extra_large_baseline() {
        let media_query = NestingMediaQuery::ExtraLarge;
        let css_value = media_query.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, "(min-width: 1280px)");
    }

    #[test]
    fn test_nesting_media_query_css_value_dark_baseline() {
        let media_query = NestingMediaQuery::Dark;
        let css_value = media_query.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, "(prefers-color-scheme: dark)");
    }

    #[test]
    fn test_nesting_media_query_css_value_light_baseline() {
        let media_query = NestingMediaQuery::Light;
        let css_value = media_query.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, "(prefers-color-scheme: light)");
    }

    #[test]
    fn test_nesting_media_query_css_value_print_baseline() {
        let media_query = NestingMediaQuery::Print;
        let css_value = media_query.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, "print");
    }

    #[test]
    fn test_nesting_media_query_css_value_screen_baseline() {
        let media_query = NestingMediaQuery::Screen;
        let css_value = media_query.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, "screen");
    }

    #[test]
    fn test_nesting_media_query_css_value_custom_baseline() {
        let media_query = NestingMediaQuery::Custom("(max-width: 600px)".to_string());
        let css_value = media_query.to_css_value();

        // Baseline CSS value output
        assert_eq!(css_value, "(max-width: 600px)");
    }

    #[test]
    fn test_css_nesting_serialization_baseline() {
        let selector = NestingSelector::DirectChild;
        let serialized = serde_json::to_string(&selector).unwrap();

        // Baseline: Should serialize to JSON
        assert!(serialized.contains("DirectChild"));

        // Should deserialize back to the same value
        let deserialized: NestingSelector = serde_json::from_str(&serialized).unwrap();
        assert_eq!(selector, deserialized);
    }

    #[test]
    fn test_css_nesting_equality_baseline() {
        let selector1 = NestingSelector::DirectChild;
        let selector2 = NestingSelector::DirectChild;
        let selector3 = NestingSelector::Descendant;

        // Baseline: Same variants should be equal
        assert_eq!(selector1, selector2);
        assert_ne!(selector1, selector3);
    }

    #[test]
    fn test_css_nesting_clone_baseline() {
        let selector = NestingSelector::DirectChild;
        let cloned = selector.clone();

        // Baseline: Cloned selector should be equal to original
        assert_eq!(selector, cloned);
    }

    #[test]
    fn test_css_nesting_complex_builder_baseline() {
        let class_set = ClassBuilder::new()
            .nesting_selector(NestingSelector::DirectChild)
            .nesting_pseudo_class(NestingPseudoClass::Hover)
            .nesting_media_query(NestingMediaQuery::Small)
            .nested_class(NestingSelector::Descendant, "text-blue-500")
            .nested_pseudo_class(NestingPseudoClass::Focus, "text-red-500")
            .nested_media_query(NestingMediaQuery::Medium, "text-green-500")
            .nested_hover("text-yellow-500")
            .nested_focus("text-purple-500")
            .nested_active("text-pink-500")
            .nested_first_child("text-indigo-500")
            .nested_last_child("text-cyan-500")
            .nested_sm("text-gray-500")
            .nested_md("text-white")
            .nested_lg("text-black")
            .nested_dark("text-gray-100")
            .nested_light("text-gray-900")
            .class("text-blue-500")
            .class("font-bold")
            .responsive(Breakpoint::Md, "nest-descendant")
            .conditional("hover", "nest-hover")
            .build();

        let classes = class_set.to_css_classes();

        // Baseline: Should contain expected classes
        assert!(classes.contains("nest-child"));
        assert!(classes.contains("nest-hover"));
        assert!(classes.contains("nest-sm"));
        assert!(classes.contains("nest-descendant-text-blue-500"));
        assert!(classes.contains("nest-focus-text-red-500"));
        assert!(classes.contains("nest-md-text-green-500"));
        assert!(classes.contains("nest-hover-text-yellow-500"));
        assert!(classes.contains("nest-focus-text-purple-500"));
        assert!(classes.contains("nest-active-text-pink-500"));
        assert!(classes.contains("nest-first-child-text-indigo-500"));
        assert!(classes.contains("nest-last-child-text-cyan-500"));
        assert!(classes.contains("nest-sm-text-gray-500"));
        assert!(classes.contains("nest-md-text-white"));
        assert!(classes.contains("nest-lg-text-black"));
        assert!(classes.contains("nest-dark-text-gray-100"));
        assert!(classes.contains("nest-light-text-gray-900"));
        assert!(classes.contains("text-blue-500"));
        assert!(classes.contains("font-bold"));
        assert!(classes.contains("md:nest-descendant"));
        assert!(classes.contains("hover:nest-hover"));
    }
}
