use tailwind_rs_core::utilities::text_shadow::*;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;

#[cfg(test)]
mod text_shadow_unit_tests {
    use super::*;

    #[test]
    fn test_text_shadow_none() {
        let shadow = TextShadow::None;
        assert_eq!(shadow.to_css_value(), "none");
        assert_eq!(shadow.to_class_name(), "text-shadow-none");
    }

    #[test]
    fn test_text_shadow_sm() {
        let shadow = TextShadow::Sm;
        let css = shadow.to_css_value();
        assert!(css.contains("0 1px 2px 0 rgb(0 0 0 / 0.05)"));
        assert_eq!(shadow.to_class_name(), "text-shadow-sm");
    }

    #[test]
    fn test_text_shadow_default() {
        let shadow = TextShadow::Default;
        let css = shadow.to_css_value();
        assert!(css.contains("0 1px 3px 0 rgb(0 0 0 / 0.1)"));
        assert_eq!(shadow.to_class_name(), "text-shadow");
    }

    #[test]
    fn test_text_shadow_lg() {
        let shadow = TextShadow::Lg;
        let css = shadow.to_css_value();
        assert!(css.contains("0 10px 15px -3px rgb(0 0 0 / 0.1)"));
        assert_eq!(shadow.to_class_name(), "text-shadow-lg");
    }

    #[test]
    fn test_text_shadow_xl() {
        let shadow = TextShadow::Xl;
        let css = shadow.to_css_value();
        assert!(css.contains("0 20px 25px -5px rgb(0 0 0 / 0.1)"));
        assert_eq!(shadow.to_class_name(), "text-shadow-xl");
    }

    #[test]
    fn test_text_shadow_display_trait() {
        let shadow = TextShadow::Sm;
        assert_eq!(format!("{}", shadow), "0 1px 2px 0 rgb(0 0 0 / 0.05)");
    }

    #[test]
    fn test_text_shadow_serialization() {
        let shadow = TextShadow::Lg;
        let serialized = serde_json::to_string(&shadow).unwrap();
        let deserialized: TextShadow = serde_json::from_str(&serialized).unwrap();
        assert_eq!(shadow, deserialized);
    }

    #[test]
    fn test_text_shadow_clone() {
        let shadow = TextShadow::Lg;
        let cloned = shadow.clone();
        assert_eq!(shadow, cloned);
    }

    #[test]
    fn test_text_shadow_partial_eq() {
        let shadow1 = TextShadow::Sm;
        let shadow2 = TextShadow::Sm;
        let shadow3 = TextShadow::Lg;
        
        assert_eq!(shadow1, shadow2);
        assert_ne!(shadow1, shadow3);
    }
}

#[cfg(test)]
mod text_shadow_integration_tests {
    use super::*;

    #[test]
    fn test_text_shadow_with_class_builder() {
        let builder = ClassBuilder::new()
            .text_shadow_sm();
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("text-shadow-sm"));
    }

    #[test]
    fn test_text_shadow_with_other_utilities() {
        let builder = ClassBuilder::new()
            .text_shadow_sm()
            .class("text-blue-500")
            .class("font-bold");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("text-shadow-sm"));
        assert!(class_set.classes.contains("text-blue-500"));
        assert!(class_set.classes.contains("font-bold"));
    }

    #[test]
    fn test_text_shadow_responsive() {
        let builder = ClassBuilder::new()
            .text_shadow_sm()
            .responsive(Breakpoint::Md, "text-shadow-lg");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("text-shadow-sm"));
        assert!(class_set.responsive.contains_key(&Breakpoint::Md));
        assert!(class_set.responsive.get(&Breakpoint::Md).unwrap().contains("text-shadow-lg"));
    }

    #[test]
    fn test_text_shadow_conditional() {
        let builder = ClassBuilder::new()
            .text_shadow_sm()
            .conditional("hover", "text-shadow-lg");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("text-shadow-sm"));
        assert!(class_set.conditional.contains_key("hover"));
        assert!(class_set.conditional.get("hover").unwrap().contains("text-shadow-lg"));
    }

    #[test]
    fn test_text_shadow_custom_variant() {
        let builder = ClassBuilder::new()
            .text_shadow_sm()
            .custom_variant("dark", "text-shadow-lg");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("text-shadow-sm"));
        assert!(class_set.conditional.contains_key("dark"));
        assert!(class_set.conditional.get("dark").unwrap().contains("text-shadow-lg"));
    }

    #[test]
    fn test_text_shadow_multiple_shadows() {
        let builder = ClassBuilder::new()
            .text_shadow_sm()
            .text_shadow_lg(); // Should add both classes
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("text-shadow-lg"));
        assert!(class_set.classes.contains("text-shadow-sm"));
    }

    #[test]
    fn test_text_shadow_build_string() {
        let classes = ClassBuilder::new()
            .text_shadow_sm()
            .class("text-blue-500")
            .build_string();
        
        assert!(classes.contains("text-shadow-sm"));
        assert!(classes.contains("text-blue-500"));
    }

    #[test]
    fn test_text_shadow_css_classes() {
        let class_set = ClassBuilder::new()
            .text_shadow_lg()
            .class("font-bold")
            .build();
        
        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("text-shadow-lg"));
        assert!(css_classes.contains("font-bold"));
    }

    #[test]
    fn test_text_shadow_custom_usage() {
        let class_set = ClassBuilder::new()
            .text_shadow_custom(TextShadow::Xl)
            .build();
        
        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("text-shadow-xl"));
    }
}
