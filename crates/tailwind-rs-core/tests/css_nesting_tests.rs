use tailwind_rs_core::utilities::css_nesting::*;
use tailwind_rs_core::Breakpoint;
use tailwind_rs_core::ClassBuilder;

#[cfg(test)]
mod css_nesting_unit_tests {
    use super::*;

    #[test]
    fn test_nesting_selector_direct_child() {
        let selector = NestingSelector::DirectChild;
        assert_eq!(selector.to_string(), ">");
        assert_eq!(selector.to_class_name(), "nest-child");
        assert_eq!(selector.to_css_value(), ">");
    }

    #[test]
    fn test_nesting_selector_descendant() {
        let selector = NestingSelector::Descendant;
        assert_eq!(selector.to_string(), " ");
        assert_eq!(selector.to_class_name(), "nest-descendant");
        assert_eq!(selector.to_css_value(), " ");
    }

    #[test]
    fn test_nesting_selector_adjacent_sibling() {
        let selector = NestingSelector::AdjacentSibling;
        assert_eq!(selector.to_string(), "+");
        assert_eq!(selector.to_class_name(), "nest-adjacent");
        assert_eq!(selector.to_css_value(), "+");
    }

    #[test]
    fn test_nesting_selector_general_sibling() {
        let selector = NestingSelector::GeneralSibling;
        assert_eq!(selector.to_string(), "~");
        assert_eq!(selector.to_class_name(), "nest-sibling");
        assert_eq!(selector.to_css_value(), "~");
    }

    #[test]
    fn test_nesting_selector_custom() {
        let selector = NestingSelector::Custom("div".to_string());
        assert_eq!(selector.to_string(), "div");
        assert_eq!(selector.to_class_name(), "nest-div");
        assert_eq!(selector.to_css_value(), "div");
    }

    #[test]
    fn test_nesting_pseudo_class_hover() {
        let pseudo_class = NestingPseudoClass::Hover;
        assert_eq!(pseudo_class.to_string(), ":hover");
        assert_eq!(pseudo_class.to_class_name(), "nest-hover");
        assert_eq!(pseudo_class.to_css_value(), ":hover");
    }

    #[test]
    fn test_nesting_pseudo_class_focus() {
        let pseudo_class = NestingPseudoClass::Focus;
        assert_eq!(pseudo_class.to_string(), ":focus");
        assert_eq!(pseudo_class.to_class_name(), "nest-focus");
        assert_eq!(pseudo_class.to_css_value(), ":focus");
    }

    #[test]
    fn test_nesting_pseudo_class_active() {
        let pseudo_class = NestingPseudoClass::Active;
        assert_eq!(pseudo_class.to_string(), ":active");
        assert_eq!(pseudo_class.to_class_name(), "nest-active");
        assert_eq!(pseudo_class.to_css_value(), ":active");
    }

    #[test]
    fn test_nesting_pseudo_class_visited() {
        let pseudo_class = NestingPseudoClass::Visited;
        assert_eq!(pseudo_class.to_string(), ":visited");
        assert_eq!(pseudo_class.to_class_name(), "nest-visited");
        assert_eq!(pseudo_class.to_css_value(), ":visited");
    }

    #[test]
    fn test_nesting_pseudo_class_link() {
        let pseudo_class = NestingPseudoClass::Link;
        assert_eq!(pseudo_class.to_string(), ":link");
        assert_eq!(pseudo_class.to_class_name(), "nest-link");
        assert_eq!(pseudo_class.to_css_value(), ":link");
    }

    #[test]
    fn test_nesting_pseudo_class_first_child() {
        let pseudo_class = NestingPseudoClass::FirstChild;
        assert_eq!(pseudo_class.to_string(), ":first-child");
        assert_eq!(pseudo_class.to_class_name(), "nest-first-child");
        assert_eq!(pseudo_class.to_css_value(), ":first-child");
    }

    #[test]
    fn test_nesting_pseudo_class_last_child() {
        let pseudo_class = NestingPseudoClass::LastChild;
        assert_eq!(pseudo_class.to_string(), ":last-child");
        assert_eq!(pseudo_class.to_class_name(), "nest-last-child");
        assert_eq!(pseudo_class.to_css_value(), ":last-child");
    }

    #[test]
    fn test_nesting_pseudo_class_nth_child() {
        let pseudo_class = NestingPseudoClass::NthChild("2n".to_string());
        assert_eq!(pseudo_class.to_string(), ":nth-child(2n)");
        assert_eq!(pseudo_class.to_class_name(), "nest-nth-child-2n");
        assert_eq!(pseudo_class.to_css_value(), ":nth-child(2n)");
    }

    #[test]
    fn test_nesting_pseudo_class_custom() {
        let pseudo_class = NestingPseudoClass::Custom("custom".to_string());
        assert_eq!(pseudo_class.to_string(), ":custom");
        assert_eq!(pseudo_class.to_class_name(), "nest-custom");
        assert_eq!(pseudo_class.to_css_value(), ":custom");
    }

    #[test]
    fn test_nesting_media_query_small() {
        let media_query = NestingMediaQuery::Small;
        assert_eq!(media_query.to_string(), "(min-width: 640px)");
        assert_eq!(media_query.to_class_name(), "nest-sm");
        assert_eq!(media_query.to_css_value(), "(min-width: 640px)");
    }

    #[test]
    fn test_nesting_media_query_medium() {
        let media_query = NestingMediaQuery::Medium;
        assert_eq!(media_query.to_string(), "(min-width: 768px)");
        assert_eq!(media_query.to_class_name(), "nest-md");
        assert_eq!(media_query.to_css_value(), "(min-width: 768px)");
    }

    #[test]
    fn test_nesting_media_query_large() {
        let media_query = NestingMediaQuery::Large;
        assert_eq!(media_query.to_string(), "(min-width: 1024px)");
        assert_eq!(media_query.to_class_name(), "nest-lg");
        assert_eq!(media_query.to_css_value(), "(min-width: 1024px)");
    }

    #[test]
    fn test_nesting_media_query_extra_large() {
        let media_query = NestingMediaQuery::ExtraLarge;
        assert_eq!(media_query.to_string(), "(min-width: 1280px)");
        assert_eq!(media_query.to_class_name(), "nest-xl");
        assert_eq!(media_query.to_css_value(), "(min-width: 1280px)");
    }

    #[test]
    fn test_nesting_media_query_dark() {
        let media_query = NestingMediaQuery::Dark;
        assert_eq!(media_query.to_string(), "(prefers-color-scheme: dark)");
        assert_eq!(media_query.to_class_name(), "nest-dark");
        assert_eq!(media_query.to_css_value(), "(prefers-color-scheme: dark)");
    }

    #[test]
    fn test_nesting_media_query_light() {
        let media_query = NestingMediaQuery::Light;
        assert_eq!(media_query.to_string(), "(prefers-color-scheme: light)");
        assert_eq!(media_query.to_class_name(), "nest-light");
        assert_eq!(media_query.to_css_value(), "(prefers-color-scheme: light)");
    }

    #[test]
    fn test_nesting_media_query_print() {
        let media_query = NestingMediaQuery::Print;
        assert_eq!(media_query.to_string(), "print");
        assert_eq!(media_query.to_class_name(), "nest-print");
        assert_eq!(media_query.to_css_value(), "print");
    }

    #[test]
    fn test_nesting_media_query_screen() {
        let media_query = NestingMediaQuery::Screen;
        assert_eq!(media_query.to_string(), "screen");
        assert_eq!(media_query.to_class_name(), "nest-screen");
        assert_eq!(media_query.to_css_value(), "screen");
    }

    #[test]
    fn test_nesting_media_query_custom() {
        let media_query = NestingMediaQuery::Custom("(max-width: 600px)".to_string());
        assert_eq!(media_query.to_string(), "(max-width: 600px)");
        assert_eq!(media_query.to_class_name(), "nest-max-width-600px");
        assert_eq!(media_query.to_css_value(), "(max-width: 600px)");
    }

    #[test]
    fn test_css_nesting_serialization() {
        let selector = NestingSelector::DirectChild;
        let serialized = serde_json::to_string(&selector).unwrap();
        let deserialized: NestingSelector = serde_json::from_str(&serialized).unwrap();
        assert_eq!(selector, deserialized);
    }

    #[test]
    fn test_css_nesting_pseudo_class_serialization() {
        let pseudo_class = NestingPseudoClass::Hover;
        let serialized = serde_json::to_string(&pseudo_class).unwrap();
        let deserialized: NestingPseudoClass = serde_json::from_str(&serialized).unwrap();
        assert_eq!(pseudo_class, deserialized);
    }

    #[test]
    fn test_css_nesting_media_query_serialization() {
        let media_query = NestingMediaQuery::Small;
        let serialized = serde_json::to_string(&media_query).unwrap();
        let deserialized: NestingMediaQuery = serde_json::from_str(&serialized).unwrap();
        assert_eq!(media_query, deserialized);
    }

    #[test]
    fn test_css_nesting_clone() {
        let selector = NestingSelector::DirectChild;
        let cloned = selector.clone();
        assert_eq!(selector, cloned);
    }

    #[test]
    fn test_css_nesting_pseudo_class_clone() {
        let pseudo_class = NestingPseudoClass::Hover;
        let cloned = pseudo_class.clone();
        assert_eq!(pseudo_class, cloned);
    }

    #[test]
    fn test_css_nesting_media_query_clone() {
        let media_query = NestingMediaQuery::Small;
        let cloned = media_query.clone();
        assert_eq!(media_query, cloned);
    }

    #[test]
    fn test_css_nesting_partial_eq() {
        let selector1 = NestingSelector::DirectChild;
        let selector2 = NestingSelector::DirectChild;
        let selector3 = NestingSelector::Descendant;

        assert_eq!(selector1, selector2);
        assert_ne!(selector1, selector3);
    }

    #[test]
    fn test_css_nesting_pseudo_class_partial_eq() {
        let pseudo_class1 = NestingPseudoClass::Hover;
        let pseudo_class2 = NestingPseudoClass::Hover;
        let pseudo_class3 = NestingPseudoClass::Focus;

        assert_eq!(pseudo_class1, pseudo_class2);
        assert_ne!(pseudo_class1, pseudo_class3);
    }

    #[test]
    fn test_css_nesting_media_query_partial_eq() {
        let media_query1 = NestingMediaQuery::Small;
        let media_query2 = NestingMediaQuery::Small;
        let media_query3 = NestingMediaQuery::Medium;

        assert_eq!(media_query1, media_query2);
        assert_ne!(media_query1, media_query3);
    }

    #[test]
    fn test_css_nesting_hash() {
        let selector1 = NestingSelector::DirectChild;
        let selector2 = NestingSelector::DirectChild;
        let selector3 = NestingSelector::Descendant;

        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        selector1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        selector2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        let mut hasher3 = DefaultHasher::new();
        selector3.hash(&mut hasher3);
        let hash3 = hasher3.finish();

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_css_nesting_debug() {
        let selector = NestingSelector::DirectChild;
        let debug = format!("{:?}", selector);
        assert!(debug.contains("DirectChild"));
    }

    #[test]
    fn test_css_nesting_all_variants() {
        let selectors = vec![
            NestingSelector::DirectChild,
            NestingSelector::Descendant,
            NestingSelector::AdjacentSibling,
            NestingSelector::GeneralSibling,
            NestingSelector::Custom("div".to_string()),
        ];

        let class_names: Vec<String> = selectors.iter().map(|s| s.to_class_name()).collect();
        assert!(class_names.contains(&"nest-child".to_string()));
        assert!(class_names.contains(&"nest-descendant".to_string()));
        assert!(class_names.contains(&"nest-adjacent".to_string()));
        assert!(class_names.contains(&"nest-sibling".to_string()));
        assert!(class_names.contains(&"nest-div".to_string()));
    }
}

#[cfg(test)]
mod css_nesting_integration_tests {
    use super::*;

    #[test]
    fn test_css_nesting_selector_with_class_builder() {
        let builder = ClassBuilder::new().nesting_selector(NestingSelector::DirectChild);

        let class_set = builder.build();
        assert!(class_set.classes.contains("nest-child"));
    }

    #[test]
    fn test_css_nesting_pseudo_class_with_class_builder() {
        let builder = ClassBuilder::new().nesting_pseudo_class(NestingPseudoClass::Hover);

        let class_set = builder.build();
        assert!(class_set.classes.contains("nest-hover"));
    }

    #[test]
    fn test_css_nesting_media_query_with_class_builder() {
        let builder = ClassBuilder::new().nesting_media_query(NestingMediaQuery::Small);

        let class_set = builder.build();
        assert!(class_set.classes.contains("nest-sm"));
    }

    #[test]
    fn test_css_nesting_nested_class_with_selector() {
        let builder =
            ClassBuilder::new().nested_class(NestingSelector::Descendant, "text-blue-500");

        let class_set = builder.build();
        assert!(class_set.classes.contains("nest-descendant-text-blue-500"));
    }

    #[test]
    fn test_css_nesting_nested_class_with_pseudo_class() {
        let builder =
            ClassBuilder::new().nested_pseudo_class(NestingPseudoClass::Hover, "text-red-500");

        let class_set = builder.build();
        assert!(class_set.classes.contains("nest-hover-text-red-500"));
    }

    #[test]
    fn test_css_nesting_nested_class_with_media_query() {
        let builder =
            ClassBuilder::new().nested_media_query(NestingMediaQuery::Medium, "text-green-500");

        let class_set = builder.build();
        assert!(class_set.classes.contains("nest-md-text-green-500"));
    }

    #[test]
    fn test_css_nesting_convenience_methods() {
        let builder = ClassBuilder::new()
            .nested_hover("text-blue-500")
            .nested_focus("text-red-500")
            .nested_active("text-green-500")
            .nested_first_child("text-yellow-500")
            .nested_last_child("text-purple-500")
            .nested_sm("text-pink-500")
            .nested_md("text-indigo-500")
            .nested_lg("text-cyan-500")
            .nested_dark("text-gray-500")
            .nested_light("text-white");

        let class_set = builder.build();
        assert!(class_set.classes.contains("nest-hover-text-blue-500"));
        assert!(class_set.classes.contains("nest-focus-text-red-500"));
        assert!(class_set.classes.contains("nest-active-text-green-500"));
        assert!(class_set
            .classes
            .contains("nest-first-child-text-yellow-500"));
        assert!(class_set
            .classes
            .contains("nest-last-child-text-purple-500"));
        assert!(class_set.classes.contains("nest-sm-text-pink-500"));
        assert!(class_set.classes.contains("nest-md-text-indigo-500"));
        assert!(class_set.classes.contains("nest-lg-text-cyan-500"));
        assert!(class_set.classes.contains("nest-dark-text-gray-500"));
        assert!(class_set.classes.contains("nest-light-text-white"));
    }

    #[test]
    fn test_css_nesting_with_other_utilities() {
        let builder = ClassBuilder::new()
            .nesting_selector(NestingSelector::DirectChild)
            .class("text-blue-500")
            .class("font-bold");

        let class_set = builder.build();
        assert!(class_set.classes.contains("nest-child"));
        assert!(class_set.classes.contains("text-blue-500"));
        assert!(class_set.classes.contains("font-bold"));
    }

    #[test]
    fn test_css_nesting_responsive() {
        let builder = ClassBuilder::new()
            .nesting_selector(NestingSelector::DirectChild)
            .responsive(Breakpoint::Md, "nest-descendant");

        let class_set = builder.build();
        assert!(class_set.classes.contains("nest-child"));
        assert!(class_set.responsive.contains_key(&Breakpoint::Md));
        assert!(class_set
            .responsive
            .get(&Breakpoint::Md)
            .unwrap()
            .contains("nest-descendant"));
    }

    #[test]
    fn test_css_nesting_conditional() {
        let builder = ClassBuilder::new()
            .nesting_selector(NestingSelector::DirectChild)
            .conditional("hover", "nest-hover");

        let class_set = builder.build();
        assert!(class_set.classes.contains("nest-child"));
        assert!(class_set.conditional.contains_key("hover"));
        assert!(class_set
            .conditional
            .get("hover")
            .unwrap()
            .contains("nest-hover"));
    }

    #[test]
    fn test_css_nesting_custom_variant() {
        let builder = ClassBuilder::new()
            .nesting_selector(NestingSelector::DirectChild)
            .custom_variant("dark", "nest-dark");

        let class_set = builder.build();
        assert!(class_set.classes.contains("nest-child"));
        assert!(class_set.conditional.contains_key("dark"));
        assert!(class_set
            .conditional
            .get("dark")
            .unwrap()
            .contains("nest-dark"));
    }

    #[test]
    fn test_css_nesting_multiple_nesting() {
        let builder = ClassBuilder::new()
            .nesting_selector(NestingSelector::DirectChild)
            .nesting_pseudo_class(NestingPseudoClass::Hover)
            .nesting_media_query(NestingMediaQuery::Small);

        let class_set = builder.build();
        assert!(class_set.classes.contains("nest-child"));
        assert!(class_set.classes.contains("nest-hover"));
        assert!(class_set.classes.contains("nest-sm"));
    }

    #[test]
    fn test_css_nesting_build_string() {
        let classes = ClassBuilder::new()
            .nesting_selector(NestingSelector::DirectChild)
            .class("text-blue-500")
            .build_string();

        assert!(classes.contains("nest-child"));
        assert!(classes.contains("text-blue-500"));
    }

    #[test]
    fn test_css_nesting_css_classes() {
        let class_set = ClassBuilder::new()
            .nesting_selector(NestingSelector::DirectChild)
            .class("font-bold")
            .build();

        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("nest-child"));
        assert!(css_classes.contains("font-bold"));
    }

    #[test]
    fn test_css_nesting_comprehensive_usage() {
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
            .build();

        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("nest-child"));
        assert!(css_classes.contains("nest-hover"));
        assert!(css_classes.contains("nest-sm"));
        assert!(css_classes.contains("nest-descendant-text-blue-500"));
        assert!(css_classes.contains("nest-focus-text-red-500"));
        assert!(css_classes.contains("nest-md-text-green-500"));
        assert!(css_classes.contains("nest-hover-text-yellow-500"));
        assert!(css_classes.contains("nest-focus-text-purple-500"));
        assert!(css_classes.contains("nest-active-text-pink-500"));
        assert!(css_classes.contains("nest-first-child-text-indigo-500"));
        assert!(css_classes.contains("nest-last-child-text-cyan-500"));
        assert!(css_classes.contains("nest-sm-text-gray-500"));
        assert!(css_classes.contains("nest-md-text-white"));
        assert!(css_classes.contains("nest-lg-text-black"));
        assert!(css_classes.contains("nest-dark-text-gray-100"));
        assert!(css_classes.contains("nest-light-text-gray-900"));
    }

    #[test]
    fn test_css_nesting_all_variants() {
        let class_set = ClassBuilder::new()
            .nesting_selector(NestingSelector::DirectChild)
            .nesting_selector(NestingSelector::Descendant)
            .nesting_selector(NestingSelector::AdjacentSibling)
            .nesting_selector(NestingSelector::GeneralSibling)
            .nesting_selector(NestingSelector::Custom("div".to_string()))
            .nesting_pseudo_class(NestingPseudoClass::Hover)
            .nesting_pseudo_class(NestingPseudoClass::Focus)
            .nesting_pseudo_class(NestingPseudoClass::Active)
            .nesting_pseudo_class(NestingPseudoClass::Visited)
            .nesting_pseudo_class(NestingPseudoClass::Link)
            .nesting_pseudo_class(NestingPseudoClass::FirstChild)
            .nesting_pseudo_class(NestingPseudoClass::LastChild)
            .nesting_pseudo_class(NestingPseudoClass::NthChild("2n".to_string()))
            .nesting_pseudo_class(NestingPseudoClass::Custom("custom".to_string()))
            .nesting_media_query(NestingMediaQuery::Small)
            .nesting_media_query(NestingMediaQuery::Medium)
            .nesting_media_query(NestingMediaQuery::Large)
            .nesting_media_query(NestingMediaQuery::ExtraLarge)
            .nesting_media_query(NestingMediaQuery::Dark)
            .nesting_media_query(NestingMediaQuery::Light)
            .nesting_media_query(NestingMediaQuery::Print)
            .nesting_media_query(NestingMediaQuery::Screen)
            .nesting_media_query(NestingMediaQuery::Custom("(max-width: 600px)".to_string()))
            .build();

        let css_classes = class_set.to_css_classes();

        // Test that all CSS nesting utilities are present
        assert!(css_classes.contains("nest-child"));
        assert!(css_classes.contains("nest-descendant"));
        assert!(css_classes.contains("nest-adjacent"));
        assert!(css_classes.contains("nest-sibling"));
        assert!(css_classes.contains("nest-div"));
        assert!(css_classes.contains("nest-hover"));
        assert!(css_classes.contains("nest-focus"));
        assert!(css_classes.contains("nest-active"));
        assert!(css_classes.contains("nest-visited"));
        assert!(css_classes.contains("nest-link"));
        assert!(css_classes.contains("nest-first-child"));
        assert!(css_classes.contains("nest-last-child"));
        assert!(css_classes.contains("nest-nth-child-2n"));
        assert!(css_classes.contains("nest-custom"));
        assert!(css_classes.contains("nest-sm"));
        assert!(css_classes.contains("nest-md"));
        assert!(css_classes.contains("nest-lg"));
        assert!(css_classes.contains("nest-xl"));
        assert!(css_classes.contains("nest-dark"));
        assert!(css_classes.contains("nest-light"));
        assert!(css_classes.contains("nest-print"));
        assert!(css_classes.contains("nest-screen"));
        assert!(css_classes.contains("nest-max-width-600px"));
    }
}
