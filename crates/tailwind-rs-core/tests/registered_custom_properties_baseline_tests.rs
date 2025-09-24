use tailwind_rs_core::utilities::modern_css_features::*;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;

#[cfg(test)]
mod registered_custom_properties_baseline_tests {
    use super::*;

    #[test]
    fn test_custom_properties_css_output_baseline() {
        let builder = ClassBuilder::new()
            .custom_property("color", "red")
            .custom_property("spacing", "1rem");
        
        let class_set = builder.build();
        
        // Baseline: Should contain both custom properties
        assert!(class_set.custom.contains_key("color"));
        assert_eq!(class_set.custom.get("color"), Some(&"red".to_string()));
        assert!(class_set.custom.contains_key("spacing"));
        assert_eq!(class_set.custom.get("spacing"), Some(&"1rem".to_string()));
    }

    #[test]
    fn test_custom_properties_class_generation_baseline() {
        let builder = ClassBuilder::new()
            .custom_property("font-size", "16px")
            .custom_property("font-weight", "bold");
        
        let class_set = builder.build();
        
        // Baseline: Should contain both custom properties
        assert!(class_set.custom.contains_key("font-size"));
        assert_eq!(class_set.custom.get("font-size"), Some(&"16px".to_string()));
        assert!(class_set.custom.contains_key("font-weight"));
        assert_eq!(class_set.custom.get("font-weight"), Some(&"bold".to_string()));
    }

    #[test]
    fn test_custom_property_color_baseline() {
        let property = CustomProperty::Color("red".to_string());
        let string_value = property.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "--color: red");
    }

    #[test]
    fn test_custom_property_spacing_baseline() {
        let property = CustomProperty::Spacing("1rem".to_string());
        let string_value = property.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "--spacing: 1rem");
    }

    #[test]
    fn test_custom_property_font_size_baseline() {
        let property = CustomProperty::FontSize("16px".to_string());
        let string_value = property.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "--font-size: 16px");
    }

    #[test]
    fn test_custom_property_font_weight_baseline() {
        let property = CustomProperty::FontWeight("bold".to_string());
        let string_value = property.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "--font-weight: bold");
    }

    #[test]
    fn test_custom_property_line_height_baseline() {
        let property = CustomProperty::LineHeight("1.5".to_string());
        let string_value = property.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "--line-height: 1.5");
    }

    #[test]
    fn test_custom_property_border_radius_baseline() {
        let property = CustomProperty::BorderRadius("8px".to_string());
        let string_value = property.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "--border-radius: 8px");
    }

    #[test]
    fn test_custom_property_box_shadow_baseline() {
        let property = CustomProperty::BoxShadow("0 4px 6px -1px rgb(0 0 0 / 0.1)".to_string());
        let string_value = property.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "--box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1)");
    }

    #[test]
    fn test_custom_property_z_index_baseline() {
        let property = CustomProperty::ZIndex("10".to_string());
        let string_value = property.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "--z-index: 10");
    }

    #[test]
    fn test_custom_property_opacity_baseline() {
        let property = CustomProperty::Opacity("0.8".to_string());
        let string_value = property.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "--opacity: 0.8");
    }

    #[test]
    fn test_custom_property_transform_baseline() {
        let property = CustomProperty::Transform("rotate(45deg)".to_string());
        let string_value = property.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "--transform: rotate(45deg)");
    }

    #[test]
    fn test_custom_property_animation_baseline() {
        let property = CustomProperty::Animation("fadeIn 0.5s ease-in-out".to_string());
        let string_value = property.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "--animation: fadeIn 0.5s ease-in-out");
    }

    #[test]
    fn test_custom_property_transition_baseline() {
        let property = CustomProperty::Transition("all 0.3s ease".to_string());
        let string_value = property.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "--transition: all 0.3s ease");
    }

    #[test]
    fn test_custom_property_generic_baseline() {
        let property = CustomProperty::Generic("custom".to_string(), "value".to_string());
        let string_value = property.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "--custom: value");
    }

    #[test]
    fn test_custom_property_class_name_color_baseline() {
        let property = CustomProperty::Color("red".to_string());
        let class_name = property.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "custom-color");
    }

    #[test]
    fn test_custom_property_class_name_spacing_baseline() {
        let property = CustomProperty::Spacing("1rem".to_string());
        let class_name = property.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "custom-spacing");
    }

    #[test]
    fn test_custom_property_class_name_font_size_baseline() {
        let property = CustomProperty::FontSize("16px".to_string());
        let class_name = property.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "custom-font-size");
    }

    #[test]
    fn test_custom_property_class_name_font_weight_baseline() {
        let property = CustomProperty::FontWeight("bold".to_string());
        let class_name = property.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "custom-font-weight");
    }

    #[test]
    fn test_custom_property_class_name_line_height_baseline() {
        let property = CustomProperty::LineHeight("1.5".to_string());
        let class_name = property.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "custom-line-height");
    }

    #[test]
    fn test_custom_property_class_name_border_radius_baseline() {
        let property = CustomProperty::BorderRadius("8px".to_string());
        let class_name = property.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "custom-border-radius");
    }

    #[test]
    fn test_custom_property_class_name_box_shadow_baseline() {
        let property = CustomProperty::BoxShadow("0 4px 6px -1px rgb(0 0 0 / 0.1)".to_string());
        let class_name = property.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "custom-box-shadow");
    }

    #[test]
    fn test_custom_property_class_name_z_index_baseline() {
        let property = CustomProperty::ZIndex("10".to_string());
        let class_name = property.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "custom-z-index");
    }

    #[test]
    fn test_custom_property_class_name_opacity_baseline() {
        let property = CustomProperty::Opacity("0.8".to_string());
        let class_name = property.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "custom-opacity");
    }

    #[test]
    fn test_custom_property_class_name_transform_baseline() {
        let property = CustomProperty::Transform("rotate(45deg)".to_string());
        let class_name = property.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "custom-transform");
    }

    #[test]
    fn test_custom_property_class_name_animation_baseline() {
        let property = CustomProperty::Animation("fadeIn 0.5s ease-in-out".to_string());
        let class_name = property.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "custom-animation");
    }

    #[test]
    fn test_custom_property_class_name_transition_baseline() {
        let property = CustomProperty::Transition("all 0.3s ease".to_string());
        let class_name = property.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "custom-transition");
    }

    #[test]
    fn test_custom_property_class_name_generic_baseline() {
        let property = CustomProperty::Generic("custom".to_string(), "value".to_string());
        let class_name = property.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "custom-custom");
    }

    #[test]
    fn test_custom_property_css_value_color_baseline() {
        let property = CustomProperty::Color("red".to_string());
        let css_value = property.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "--color: red");
    }

    #[test]
    fn test_custom_property_css_value_spacing_baseline() {
        let property = CustomProperty::Spacing("1rem".to_string());
        let css_value = property.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "--spacing: 1rem");
    }

    #[test]
    fn test_custom_property_css_value_font_size_baseline() {
        let property = CustomProperty::FontSize("16px".to_string());
        let css_value = property.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "--font-size: 16px");
    }

    #[test]
    fn test_custom_property_css_value_font_weight_baseline() {
        let property = CustomProperty::FontWeight("bold".to_string());
        let css_value = property.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "--font-weight: bold");
    }

    #[test]
    fn test_custom_property_css_value_line_height_baseline() {
        let property = CustomProperty::LineHeight("1.5".to_string());
        let css_value = property.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "--line-height: 1.5");
    }

    #[test]
    fn test_custom_property_css_value_border_radius_baseline() {
        let property = CustomProperty::BorderRadius("8px".to_string());
        let css_value = property.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "--border-radius: 8px");
    }

    #[test]
    fn test_custom_property_css_value_box_shadow_baseline() {
        let property = CustomProperty::BoxShadow("0 4px 6px -1px rgb(0 0 0 / 0.1)".to_string());
        let css_value = property.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "--box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1)");
    }

    #[test]
    fn test_custom_property_css_value_z_index_baseline() {
        let property = CustomProperty::ZIndex("10".to_string());
        let css_value = property.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "--z-index: 10");
    }

    #[test]
    fn test_custom_property_css_value_opacity_baseline() {
        let property = CustomProperty::Opacity("0.8".to_string());
        let css_value = property.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "--opacity: 0.8");
    }

    #[test]
    fn test_custom_property_css_value_transform_baseline() {
        let property = CustomProperty::Transform("rotate(45deg)".to_string());
        let css_value = property.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "--transform: rotate(45deg)");
    }

    #[test]
    fn test_custom_property_css_value_animation_baseline() {
        let property = CustomProperty::Animation("fadeIn 0.5s ease-in-out".to_string());
        let css_value = property.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "--animation: fadeIn 0.5s ease-in-out");
    }

    #[test]
    fn test_custom_property_css_value_transition_baseline() {
        let property = CustomProperty::Transition("all 0.3s ease".to_string());
        let css_value = property.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "--transition: all 0.3s ease");
    }

    #[test]
    fn test_custom_property_css_value_generic_baseline() {
        let property = CustomProperty::Generic("custom".to_string(), "value".to_string());
        let css_value = property.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "--custom: value");
    }

    #[test]
    fn test_custom_properties_serialization_baseline() {
        let property = CustomProperty::Color("red".to_string());
        let serialized = serde_json::to_string(&property).unwrap();
        
        // Baseline: Should serialize to JSON
        assert!(serialized.contains("Color"));
        assert!(serialized.contains("red"));
        
        // Should deserialize back to the same value
        let deserialized: CustomProperty = serde_json::from_str(&serialized).unwrap();
        assert_eq!(property, deserialized);
    }

    #[test]
    fn test_custom_properties_equality_baseline() {
        let property1 = CustomProperty::Color("red".to_string());
        let property2 = CustomProperty::Color("red".to_string());
        let property3 = CustomProperty::Color("blue".to_string());
        
        // Baseline: Same variants should be equal
        assert_eq!(property1, property2);
        assert_ne!(property1, property3);
    }

    #[test]
    fn test_custom_properties_clone_baseline() {
        let property = CustomProperty::Color("red".to_string());
        let cloned = property.clone();
        
        // Baseline: Cloned property should be equal to original
        assert_eq!(property, cloned);
    }

    #[test]
    fn test_custom_properties_complex_builder_baseline() {
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
            .class("text-blue-500")
            .class("font-bold")
            .responsive(Breakpoint::Md, "text-lg")
            .conditional("hover", "text-xl")
            .build();
        
        let classes = class_set.to_css_classes();
        
        // Baseline: Should contain expected classes
        assert!(class_set.custom.contains_key("color"));
        assert_eq!(class_set.custom.get("color"), Some(&"red".to_string()));
        assert!(class_set.custom.contains_key("spacing"));
        assert_eq!(class_set.custom.get("spacing"), Some(&"1rem".to_string()));
        assert!(class_set.custom.contains_key("font-size"));
        assert_eq!(class_set.custom.get("font-size"), Some(&"16px".to_string()));
        assert!(class_set.custom.contains_key("font-weight"));
        assert_eq!(class_set.custom.get("font-weight"), Some(&"bold".to_string()));
        assert!(class_set.custom.contains_key("line-height"));
        assert_eq!(class_set.custom.get("line-height"), Some(&"1.5".to_string()));
        assert!(class_set.custom.contains_key("border-radius"));
        assert_eq!(class_set.custom.get("border-radius"), Some(&"8px".to_string()));
        assert!(class_set.custom.contains_key("box-shadow"));
        assert_eq!(class_set.custom.get("box-shadow"), Some(&"0 4px 6px -1px rgb(0 0 0 / 0.1)".to_string()));
        assert!(class_set.custom.contains_key("z-index"));
        assert_eq!(class_set.custom.get("z-index"), Some(&"10".to_string()));
        assert!(class_set.custom.contains_key("opacity"));
        assert_eq!(class_set.custom.get("opacity"), Some(&"0.8".to_string()));
        assert!(class_set.custom.contains_key("transform"));
        assert_eq!(class_set.custom.get("transform"), Some(&"rotate(45deg)".to_string()));
        assert!(class_set.custom.contains_key("animation"));
        assert_eq!(class_set.custom.get("animation"), Some(&"fadeIn 0.5s ease-in-out".to_string()));
        assert!(class_set.custom.contains_key("transition"));
        assert_eq!(class_set.custom.get("transition"), Some(&"all 0.3s ease".to_string()));
        assert!(class_set.custom.contains_key("custom"));
        assert_eq!(class_set.custom.get("custom"), Some(&"value".to_string()));
        assert!(classes.contains("text-blue-500"));
        assert!(classes.contains("font-bold"));
        assert!(classes.contains("md:text-lg"));
        assert!(classes.contains("hover:text-xl"));
    }
}
