use tailwind_rs_core::utilities::modern_css_features::*;
use tailwind_rs_core::Breakpoint;
use tailwind_rs_core::ClassBuilder;

#[cfg(test)]
mod registered_custom_properties_unit_tests {
    use super::*;

    #[test]
    fn test_custom_property_color() {
        let property = CustomProperty::Color("red".to_string());
        assert_eq!(property.to_string(), "--color: red");
        assert_eq!(property.to_class_name(), "custom-color");
        assert_eq!(property.to_css_value(), "--color: red");
    }

    #[test]
    fn test_custom_property_spacing() {
        let property = CustomProperty::Spacing("1rem".to_string());
        assert_eq!(property.to_string(), "--spacing: 1rem");
        assert_eq!(property.to_class_name(), "custom-spacing");
        assert_eq!(property.to_css_value(), "--spacing: 1rem");
    }

    #[test]
    fn test_custom_property_font_size() {
        let property = CustomProperty::FontSize("16px".to_string());
        assert_eq!(property.to_string(), "--font-size: 16px");
        assert_eq!(property.to_class_name(), "custom-font-size");
        assert_eq!(property.to_css_value(), "--font-size: 16px");
    }

    #[test]
    fn test_custom_property_font_weight() {
        let property = CustomProperty::FontWeight("bold".to_string());
        assert_eq!(property.to_string(), "--font-weight: bold");
        assert_eq!(property.to_class_name(), "custom-font-weight");
        assert_eq!(property.to_css_value(), "--font-weight: bold");
    }

    #[test]
    fn test_custom_property_line_height() {
        let property = CustomProperty::LineHeight("1.5".to_string());
        assert_eq!(property.to_string(), "--line-height: 1.5");
        assert_eq!(property.to_class_name(), "custom-line-height");
        assert_eq!(property.to_css_value(), "--line-height: 1.5");
    }

    #[test]
    fn test_custom_property_border_radius() {
        let property = CustomProperty::BorderRadius("8px".to_string());
        assert_eq!(property.to_string(), "--border-radius: 8px");
        assert_eq!(property.to_class_name(), "custom-border-radius");
        assert_eq!(property.to_css_value(), "--border-radius: 8px");
    }

    #[test]
    fn test_custom_property_box_shadow() {
        let property = CustomProperty::BoxShadow("0 4px 6px -1px rgb(0 0 0 / 0.1)".to_string());
        assert_eq!(
            property.to_string(),
            "--box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1)"
        );
        assert_eq!(property.to_class_name(), "custom-box-shadow");
        assert_eq!(
            property.to_css_value(),
            "--box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1)"
        );
    }

    #[test]
    fn test_custom_property_z_index() {
        let property = CustomProperty::ZIndex("10".to_string());
        assert_eq!(property.to_string(), "--z-index: 10");
        assert_eq!(property.to_class_name(), "custom-z-index");
        assert_eq!(property.to_css_value(), "--z-index: 10");
    }

    #[test]
    fn test_custom_property_opacity() {
        let property = CustomProperty::Opacity("0.8".to_string());
        assert_eq!(property.to_string(), "--opacity: 0.8");
        assert_eq!(property.to_class_name(), "custom-opacity");
        assert_eq!(property.to_css_value(), "--opacity: 0.8");
    }

    #[test]
    fn test_custom_property_transform() {
        let property = CustomProperty::Transform("rotate(45deg)".to_string());
        assert_eq!(property.to_string(), "--transform: rotate(45deg)");
        assert_eq!(property.to_class_name(), "custom-transform");
        assert_eq!(property.to_css_value(), "--transform: rotate(45deg)");
    }

    #[test]
    fn test_custom_property_animation() {
        let property = CustomProperty::Animation("fadeIn 0.5s ease-in-out".to_string());
        assert_eq!(property.to_string(), "--animation: fadeIn 0.5s ease-in-out");
        assert_eq!(property.to_class_name(), "custom-animation");
        assert_eq!(
            property.to_css_value(),
            "--animation: fadeIn 0.5s ease-in-out"
        );
    }

    #[test]
    fn test_custom_property_transition() {
        let property = CustomProperty::Transition("all 0.3s ease".to_string());
        assert_eq!(property.to_string(), "--transition: all 0.3s ease");
        assert_eq!(property.to_class_name(), "custom-transition");
        assert_eq!(property.to_css_value(), "--transition: all 0.3s ease");
    }

    #[test]
    fn test_custom_property_generic() {
        let property = CustomProperty::Generic("custom".to_string(), "value".to_string());
        assert_eq!(property.to_string(), "--custom: value");
        assert_eq!(property.to_class_name(), "custom-custom");
        assert_eq!(property.to_css_value(), "--custom: value");
    }

    #[test]
    fn test_custom_property_serialization() {
        let property = CustomProperty::Color("red".to_string());
        let serialized = serde_json::to_string(&property).unwrap();
        let deserialized: CustomProperty = serde_json::from_str(&serialized).unwrap();
        assert_eq!(property, deserialized);
    }

    #[test]
    fn test_custom_property_clone() {
        let property = CustomProperty::Color("red".to_string());
        let cloned = property.clone();
        assert_eq!(property, cloned);
    }

    #[test]
    fn test_custom_property_partial_eq() {
        let property1 = CustomProperty::Color("red".to_string());
        let property2 = CustomProperty::Color("red".to_string());
        let property3 = CustomProperty::Color("blue".to_string());

        assert_eq!(property1, property2);
        assert_ne!(property1, property3);
    }

    #[test]
    fn test_custom_property_hash() {
        let property1 = CustomProperty::Color("red".to_string());
        let property2 = CustomProperty::Color("red".to_string());
        let property3 = CustomProperty::Color("blue".to_string());

        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        property1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        property2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        let mut hasher3 = DefaultHasher::new();
        property3.hash(&mut hasher3);
        let hash3 = hasher3.finish();

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_custom_property_debug() {
        let property = CustomProperty::Color("red".to_string());
        let debug = format!("{:?}", property);
        assert!(debug.contains("Color"));
        assert!(debug.contains("red"));
    }

    #[test]
    fn test_custom_property_all_variants() {
        let properties = vec![
            CustomProperty::Color("red".to_string()),
            CustomProperty::Spacing("1rem".to_string()),
            CustomProperty::FontSize("16px".to_string()),
            CustomProperty::FontWeight("bold".to_string()),
            CustomProperty::LineHeight("1.5".to_string()),
            CustomProperty::BorderRadius("8px".to_string()),
            CustomProperty::BoxShadow("0 4px 6px -1px rgb(0 0 0 / 0.1)".to_string()),
            CustomProperty::ZIndex("10".to_string()),
            CustomProperty::Opacity("0.8".to_string()),
            CustomProperty::Transform("rotate(45deg)".to_string()),
            CustomProperty::Animation("fadeIn 0.5s ease-in-out".to_string()),
            CustomProperty::Transition("all 0.3s ease".to_string()),
            CustomProperty::Generic("custom".to_string(), "value".to_string()),
        ];

        let class_names: Vec<String> = properties.iter().map(|p| p.to_class_name()).collect();
        assert!(class_names.contains(&"custom-color".to_string()));
        assert!(class_names.contains(&"custom-spacing".to_string()));
        assert!(class_names.contains(&"custom-font-size".to_string()));
        assert!(class_names.contains(&"custom-font-weight".to_string()));
        assert!(class_names.contains(&"custom-line-height".to_string()));
        assert!(class_names.contains(&"custom-border-radius".to_string()));
        assert!(class_names.contains(&"custom-box-shadow".to_string()));
        assert!(class_names.contains(&"custom-z-index".to_string()));
        assert!(class_names.contains(&"custom-opacity".to_string()));
        assert!(class_names.contains(&"custom-transform".to_string()));
        assert!(class_names.contains(&"custom-animation".to_string()));
        assert!(class_names.contains(&"custom-transition".to_string()));
        assert!(class_names.contains(&"custom-custom".to_string()));
    }
}

#[cfg(test)]
mod registered_custom_properties_integration_tests {
    use super::*;

    #[test]
    fn test_custom_property_with_class_builder() {
        let builder = ClassBuilder::new().custom_property("color", "red");

        let class_set = builder.build();
        assert!(class_set.custom.contains_key("color"));
        assert_eq!(class_set.custom.get("color"), Some(&"red".to_string()));
    }

    #[test]
    fn test_custom_property_value_with_class_builder() {
        let builder =
            ClassBuilder::new().custom_property_value(CustomProperty::Color("red".to_string()));

        let class_set = builder.build();
        assert!(class_set.classes.contains("custom-color"));
    }

    #[test]
    fn test_custom_property_multiple_properties() {
        let builder = ClassBuilder::new()
            .custom_property("color", "red")
            .custom_property("spacing", "1rem")
            .custom_property("font-size", "16px");

        let class_set = builder.build();
        assert!(class_set.custom.contains_key("color"));
        assert_eq!(class_set.custom.get("color"), Some(&"red".to_string()));
        assert!(class_set.custom.contains_key("spacing"));
        assert_eq!(class_set.custom.get("spacing"), Some(&"1rem".to_string()));
        assert!(class_set.custom.contains_key("font-size"));
        assert_eq!(class_set.custom.get("font-size"), Some(&"16px".to_string()));
    }

    #[test]
    fn test_custom_property_with_other_utilities() {
        let builder = ClassBuilder::new()
            .custom_property("color", "red")
            .class("text-blue-500")
            .class("font-bold");

        let class_set = builder.build();
        assert!(class_set.custom.contains_key("color"));
        assert_eq!(class_set.custom.get("color"), Some(&"red".to_string()));
        assert!(class_set.classes.contains("text-blue-500"));
        assert!(class_set.classes.contains("font-bold"));
    }

    #[test]
    fn test_custom_property_responsive() {
        let builder = ClassBuilder::new()
            .custom_property("color", "red")
            .responsive(Breakpoint::Md, "text-blue-500");

        let class_set = builder.build();
        assert!(class_set.custom.contains_key("color"));
        assert_eq!(class_set.custom.get("color"), Some(&"red".to_string()));
        assert!(class_set.responsive.contains_key(&Breakpoint::Md));
        assert!(class_set
            .responsive
            .get(&Breakpoint::Md)
            .unwrap()
            .contains("text-blue-500"));
    }

    #[test]
    fn test_custom_property_conditional() {
        let builder = ClassBuilder::new()
            .custom_property("color", "red")
            .conditional("hover", "text-blue-500");

        let class_set = builder.build();
        assert!(class_set.custom.contains_key("color"));
        assert_eq!(class_set.custom.get("color"), Some(&"red".to_string()));
        assert!(class_set.conditional.contains_key("hover"));
        assert!(class_set
            .conditional
            .get("hover")
            .unwrap()
            .contains("text-blue-500"));
    }

    #[test]
    fn test_custom_property_custom_variant() {
        let builder = ClassBuilder::new()
            .custom_property("color", "red")
            .custom_variant("dark", "text-blue-500");

        let class_set = builder.build();
        assert!(class_set.custom.contains_key("color"));
        assert_eq!(class_set.custom.get("color"), Some(&"red".to_string()));
        assert!(class_set.conditional.contains_key("dark"));
        assert!(class_set
            .conditional
            .get("dark")
            .unwrap()
            .contains("text-blue-500"));
    }

    #[test]
    fn test_custom_property_build_string() {
        let classes = ClassBuilder::new()
            .custom_property("color", "red")
            .class("text-blue-500")
            .build_string();

        assert!(classes.contains("text-blue-500"));
    }

    #[test]
    fn test_custom_property_css_classes() {
        let class_set = ClassBuilder::new()
            .custom_property("color", "red")
            .class("font-bold")
            .build();

        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("font-bold"));
    }

    #[test]
    fn test_custom_property_comprehensive_usage() {
        let class_set = ClassBuilder::new()
            .custom_property("primary-color", "#3b82f6")
            .custom_property("secondary-color", "#64748b")
            .custom_property("font-size", "16px")
            .custom_property("spacing", "1rem")
            .build();

        let css_classes = class_set.to_css_classes();
        assert!(class_set.custom.contains_key("primary-color"));
        assert_eq!(
            class_set.custom.get("primary-color"),
            Some(&"#3b82f6".to_string())
        );
        assert!(class_set.custom.contains_key("secondary-color"));
        assert_eq!(
            class_set.custom.get("secondary-color"),
            Some(&"#64748b".to_string())
        );
        assert!(class_set.custom.contains_key("font-size"));
        assert_eq!(class_set.custom.get("font-size"), Some(&"16px".to_string()));
        assert!(class_set.custom.contains_key("spacing"));
        assert_eq!(class_set.custom.get("spacing"), Some(&"1rem".to_string()));
    }

    #[test]
    fn test_custom_property_all_variants() {
        let class_set = ClassBuilder::new()
            .custom_property("color", "red")
            .custom_property("spacing", "1rem")
            .custom_property("font-size", "16px")
            .custom_property("font-weight", "bold")
            .custom_property("line-height", "1.5")
            .custom_property("border-radius", "8px")
            .custom_property("box-shadow", "0 4px 6px -1px rgb(0 0 0 / 0.1)")
            .custom_property("z-index", "10")
            .custom_property("opacity", "0.8")
            .custom_property("transform", "rotate(45deg)")
            .custom_property("animation", "fadeIn 0.5s ease-in-out")
            .custom_property("transition", "all 0.3s ease")
            .custom_property("custom", "value")
            .build();

        // Test that all custom properties are present
        assert!(class_set.custom.contains_key("color"));
        assert_eq!(class_set.custom.get("color"), Some(&"red".to_string()));
        assert!(class_set.custom.contains_key("spacing"));
        assert_eq!(class_set.custom.get("spacing"), Some(&"1rem".to_string()));
        assert!(class_set.custom.contains_key("font-size"));
        assert_eq!(class_set.custom.get("font-size"), Some(&"16px".to_string()));
        assert!(class_set.custom.contains_key("font-weight"));
        assert_eq!(
            class_set.custom.get("font-weight"),
            Some(&"bold".to_string())
        );
        assert!(class_set.custom.contains_key("line-height"));
        assert_eq!(
            class_set.custom.get("line-height"),
            Some(&"1.5".to_string())
        );
        assert!(class_set.custom.contains_key("border-radius"));
        assert_eq!(
            class_set.custom.get("border-radius"),
            Some(&"8px".to_string())
        );
        assert!(class_set.custom.contains_key("box-shadow"));
        assert_eq!(
            class_set.custom.get("box-shadow"),
            Some(&"0 4px 6px -1px rgb(0 0 0 / 0.1)".to_string())
        );
        assert!(class_set.custom.contains_key("z-index"));
        assert_eq!(class_set.custom.get("z-index"), Some(&"10".to_string()));
        assert!(class_set.custom.contains_key("opacity"));
        assert_eq!(class_set.custom.get("opacity"), Some(&"0.8".to_string()));
        assert!(class_set.custom.contains_key("transform"));
        assert_eq!(
            class_set.custom.get("transform"),
            Some(&"rotate(45deg)".to_string())
        );
        assert!(class_set.custom.contains_key("animation"));
        assert_eq!(
            class_set.custom.get("animation"),
            Some(&"fadeIn 0.5s ease-in-out".to_string())
        );
        assert!(class_set.custom.contains_key("transition"));
        assert_eq!(
            class_set.custom.get("transition"),
            Some(&"all 0.3s ease".to_string())
        );
        assert!(class_set.custom.contains_key("custom"));
        assert_eq!(class_set.custom.get("custom"), Some(&"value".to_string()));
    }
}
