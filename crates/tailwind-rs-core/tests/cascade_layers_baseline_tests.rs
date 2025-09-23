use tailwind_rs_core::utilities::modern_css_features::*;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;

#[cfg(test)]
mod cascade_layers_baseline_tests {
    use super::*;

    #[test]
    fn test_cascade_layers_css_output_baseline() {
        let builder = ClassBuilder::new()
            .layer_base()
            .layer_components();
        
        let class_set = builder.build();
        let classes = class_set.to_css_classes();
        
        // Baseline: Should contain both cascade layer classes
        assert!(classes.contains("layer-base"));
        assert!(classes.contains("layer-components"));
    }

    #[test]
    fn test_cascade_layers_class_generation_baseline() {
        let builder = ClassBuilder::new()
            .layer_base()
            .layer_utilities();
        
        let class_set = builder.build();
        let classes = class_set.to_css_classes();
        
        // Baseline: Should contain both cascade layer classes
        assert!(classes.contains("layer-base"));
        assert!(classes.contains("layer-utilities"));
    }

    #[test]
    fn test_cascade_layer_base_baseline() {
        let layer = CascadeLayer::Base;
        let string_value = layer.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "base");
    }

    #[test]
    fn test_cascade_layer_components_baseline() {
        let layer = CascadeLayer::Components;
        let string_value = layer.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "components");
    }

    #[test]
    fn test_cascade_layer_utilities_baseline() {
        let layer = CascadeLayer::Utilities;
        let string_value = layer.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "utilities");
    }

    #[test]
    fn test_cascade_layer_custom_baseline() {
        let layer = CascadeLayer::Custom("theme".to_string());
        let string_value = layer.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "theme");
    }

    #[test]
    fn test_cascade_layer_custom_long_name_baseline() {
        let layer = CascadeLayer::Custom("my-custom-layer".to_string());
        let string_value = layer.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "my-custom-layer");
    }

    #[test]
    fn test_cascade_layer_class_name_base_baseline() {
        let layer = CascadeLayer::Base;
        let class_name = layer.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "layer-base");
    }

    #[test]
    fn test_cascade_layer_class_name_components_baseline() {
        let layer = CascadeLayer::Components;
        let class_name = layer.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "layer-components");
    }

    #[test]
    fn test_cascade_layer_class_name_utilities_baseline() {
        let layer = CascadeLayer::Utilities;
        let class_name = layer.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "layer-utilities");
    }

    #[test]
    fn test_cascade_layer_class_name_custom_baseline() {
        let layer = CascadeLayer::Custom("theme".to_string());
        let class_name = layer.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "layer-theme");
    }

    #[test]
    fn test_cascade_layer_class_name_custom_long_name_baseline() {
        let layer = CascadeLayer::Custom("my-custom-layer".to_string());
        let class_name = layer.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "layer-my-custom-layer");
    }

    #[test]
    fn test_cascade_layer_css_value_base_baseline() {
        let layer = CascadeLayer::Base;
        let css_value = layer.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "base");
    }

    #[test]
    fn test_cascade_layer_css_value_components_baseline() {
        let layer = CascadeLayer::Components;
        let css_value = layer.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "components");
    }

    #[test]
    fn test_cascade_layer_css_value_utilities_baseline() {
        let layer = CascadeLayer::Utilities;
        let css_value = layer.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "utilities");
    }

    #[test]
    fn test_cascade_layer_css_value_custom_baseline() {
        let layer = CascadeLayer::Custom("theme".to_string());
        let css_value = layer.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "theme");
    }

    #[test]
    fn test_cascade_layer_css_value_custom_long_name_baseline() {
        let layer = CascadeLayer::Custom("my-custom-layer".to_string());
        let css_value = layer.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "my-custom-layer");
    }

    #[test]
    fn test_cascade_layers_serialization_baseline() {
        let layer = CascadeLayer::Base;
        let serialized = serde_json::to_string(&layer).unwrap();
        
        // Baseline: Should serialize to JSON
        assert!(serialized.contains("Base"));
        
        // Should deserialize back to the same value
        let deserialized: CascadeLayer = serde_json::from_str(&serialized).unwrap();
        assert_eq!(layer, deserialized);
    }

    #[test]
    fn test_cascade_layers_custom_serialization_baseline() {
        let layer = CascadeLayer::Custom("theme".to_string());
        let serialized = serde_json::to_string(&layer).unwrap();
        
        // Baseline: Should serialize to JSON
        assert!(serialized.contains("Custom"));
        assert!(serialized.contains("theme"));
        
        // Should deserialize back to the same value
        let deserialized: CascadeLayer = serde_json::from_str(&serialized).unwrap();
        assert_eq!(layer, deserialized);
    }

    #[test]
    fn test_cascade_layers_equality_baseline() {
        let layer1 = CascadeLayer::Base;
        let layer2 = CascadeLayer::Base;
        let layer3 = CascadeLayer::Components;
        
        // Baseline: Same variants should be equal
        assert_eq!(layer1, layer2);
        assert_ne!(layer1, layer3);
    }

    #[test]
    fn test_cascade_layers_custom_equality_baseline() {
        let layer1 = CascadeLayer::Custom("theme".to_string());
        let layer2 = CascadeLayer::Custom("theme".to_string());
        let layer3 = CascadeLayer::Custom("other".to_string());
        
        // Baseline: Same variants should be equal
        assert_eq!(layer1, layer2);
        assert_ne!(layer1, layer3);
    }

    #[test]
    fn test_cascade_layers_clone_baseline() {
        let layer = CascadeLayer::Base;
        let cloned = layer.clone();
        
        // Baseline: Cloned layer should be equal to original
        assert_eq!(layer, cloned);
    }

    #[test]
    fn test_cascade_layers_custom_clone_baseline() {
        let layer = CascadeLayer::Custom("theme".to_string());
        let cloned = layer.clone();
        
        // Baseline: Cloned layer should be equal to original
        assert_eq!(layer, cloned);
    }

    #[test]
    fn test_cascade_layers_complex_builder_baseline() {
        let class_set = ClassBuilder::new()
            .layer_base()
            .layer_components()
            .layer_utilities()
            .layer_custom("theme")
            .class("text-blue-500")
            .class("font-bold")
            .responsive(Breakpoint::Md, "layer-components")
            .conditional("hover", "layer-utilities")
            .build();
        
        let classes = class_set.to_css_classes();
        
        // Baseline: Should contain expected classes
        assert!(classes.contains("layer-base"));
        assert!(classes.contains("layer-components"));
        assert!(classes.contains("layer-utilities"));
        assert!(classes.contains("layer-theme"));
        assert!(classes.contains("text-blue-500"));
        assert!(classes.contains("font-bold"));
        assert!(classes.contains("md:layer-components"));
        assert!(classes.contains("hover:layer-utilities"));
    }
}
