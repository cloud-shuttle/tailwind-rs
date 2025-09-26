use tailwind_rs_core::utilities::text_shadow::*;
use tailwind_rs_core::Breakpoint;
use tailwind_rs_core::ClassBuilder;

#[cfg(test)]
mod text_shadow_baseline_tests {
    use super::*;

    #[test]
    fn test_text_shadow_css_output_baseline() {
        let builder = ClassBuilder::new().text_shadow_sm().text_shadow_lg();

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain both shadow classes
        assert!(classes.contains("text-shadow-sm"));
        assert!(classes.contains("text-shadow-lg"));
    }

    #[test]
    fn test_text_shadow_class_generation_baseline() {
        let builder = ClassBuilder::new()
            .text_shadow_lg()
            .text_shadow_custom(TextShadow::Xl);

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain both shadow classes
        assert!(classes.contains("text-shadow-lg"));
        assert!(classes.contains("text-shadow-xl"));
    }

    #[test]
    fn test_text_shadow_sm_baseline() {
        let shadow = TextShadow::Sm;
        let css = shadow.to_css_value();
        let class_name = shadow.to_class_name();

        // Baseline CSS output
        assert_eq!(css, "0 1px 2px 0 rgb(0 0 0 / 0.05)");
        assert_eq!(class_name, "text-shadow-sm");
    }

    #[test]
    fn test_text_shadow_lg_baseline() {
        let shadow = TextShadow::Lg;
        let css = shadow.to_css_value();
        let class_name = shadow.to_class_name();

        // Baseline CSS output
        assert_eq!(
            css,
            "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)"
        );
        assert_eq!(class_name, "text-shadow-lg");
    }

    #[test]
    fn test_text_shadow_xl_baseline() {
        let shadow = TextShadow::Xl;
        let css = shadow.to_css_value();
        let class_name = shadow.to_class_name();

        // Baseline CSS output
        assert_eq!(
            css,
            "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)"
        );
        assert_eq!(class_name, "text-shadow-xl");
    }

    #[test]
    fn test_text_shadow_none_baseline() {
        let shadow = TextShadow::None;
        let css = shadow.to_css_value();
        let class_name = shadow.to_class_name();

        // Baseline CSS output
        assert_eq!(css, "none");
        assert_eq!(class_name, "text-shadow-none");
    }

    #[test]
    fn test_text_shadow_serialization_baseline() {
        let shadow = TextShadow::Lg;
        let serialized = serde_json::to_string(&shadow).unwrap();

        // Baseline: Should serialize to JSON
        assert!(serialized.contains("Lg"));

        // Should deserialize back to the same value
        let deserialized: TextShadow = serde_json::from_str(&serialized).unwrap();
        assert_eq!(shadow, deserialized);
    }

    #[test]
    fn test_text_shadow_display_baseline() {
        let shadow = TextShadow::Sm;
        let display_string = format!("{}", shadow);

        // Baseline: Should display as CSS value
        assert_eq!(display_string, "0 1px 2px 0 rgb(0 0 0 / 0.05)");
    }

    #[test]
    fn test_text_shadow_equality_baseline() {
        let shadow1 = TextShadow::Sm;
        let shadow2 = TextShadow::Sm;
        let shadow3 = TextShadow::Lg;

        // Baseline: Same variants should be equal
        assert_eq!(shadow1, shadow2);
        assert_ne!(shadow1, shadow3);
    }

    #[test]
    fn test_text_shadow_clone_baseline() {
        let shadow = TextShadow::Lg;
        let cloned = shadow.clone();

        // Baseline: Cloned shadow should be equal to original
        assert_eq!(shadow, cloned);

        // Cloned shadow should have the same CSS output
        assert_eq!(shadow.to_css_value(), cloned.to_css_value());
        assert_eq!(shadow.to_class_name(), cloned.to_class_name());
    }

    #[test]
    fn test_text_shadow_complex_builder_baseline() {
        let class_set = ClassBuilder::new()
            .text_shadow_sm()
            .class("text-blue-500")
            .class("font-bold")
            .responsive(Breakpoint::Md, "text-shadow-lg")
            .conditional("hover", "text-shadow-xl")
            .build();

        let classes = class_set.to_css_classes();

        // Baseline: Should contain expected classes
        assert!(classes.contains("text-shadow-sm"));
        assert!(classes.contains("text-blue-500"));
        assert!(classes.contains("font-bold"));
        assert!(classes.contains("md:text-shadow-lg"));
        assert!(classes.contains("hover:text-shadow-xl"));
    }
}
