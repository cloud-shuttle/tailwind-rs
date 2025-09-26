use tailwind_rs_core::utilities::modern_css_features::*;
use tailwind_rs_core::Breakpoint;
use tailwind_rs_core::ClassBuilder;

#[cfg(test)]
mod cascade_layers_unit_tests {
    use super::*;

    #[test]
    fn test_cascade_layer_base() {
        let layer = CascadeLayer::Base;
        assert_eq!(layer.to_string(), "base");
        assert_eq!(layer.to_class_name(), "layer-base");
        assert_eq!(layer.to_css_value(), "base");
    }

    #[test]
    fn test_cascade_layer_components() {
        let layer = CascadeLayer::Components;
        assert_eq!(layer.to_string(), "components");
        assert_eq!(layer.to_class_name(), "layer-components");
        assert_eq!(layer.to_css_value(), "components");
    }

    #[test]
    fn test_cascade_layer_utilities() {
        let layer = CascadeLayer::Utilities;
        assert_eq!(layer.to_string(), "utilities");
        assert_eq!(layer.to_class_name(), "layer-utilities");
        assert_eq!(layer.to_css_value(), "utilities");
    }

    #[test]
    fn test_cascade_layer_custom() {
        let layer = CascadeLayer::Custom("theme".to_string());
        assert_eq!(layer.to_string(), "theme");
        assert_eq!(layer.to_class_name(), "layer-theme");
        assert_eq!(layer.to_css_value(), "theme");
    }

    #[test]
    fn test_cascade_layer_custom_long_name() {
        let layer = CascadeLayer::Custom("my-custom-layer".to_string());
        assert_eq!(layer.to_string(), "my-custom-layer");
        assert_eq!(layer.to_class_name(), "layer-my-custom-layer");
        assert_eq!(layer.to_css_value(), "my-custom-layer");
    }

    #[test]
    fn test_cascade_layer_serialization() {
        let layer = CascadeLayer::Base;
        let serialized = serde_json::to_string(&layer).unwrap();
        let deserialized: CascadeLayer = serde_json::from_str(&serialized).unwrap();
        assert_eq!(layer, deserialized);
    }

    #[test]
    fn test_cascade_layer_custom_serialization() {
        let layer = CascadeLayer::Custom("theme".to_string());
        let serialized = serde_json::to_string(&layer).unwrap();
        let deserialized: CascadeLayer = serde_json::from_str(&serialized).unwrap();
        assert_eq!(layer, deserialized);
    }

    #[test]
    fn test_cascade_layer_clone() {
        let layer = CascadeLayer::Base;
        let cloned = layer.clone();
        assert_eq!(layer, cloned);
    }

    #[test]
    fn test_cascade_layer_custom_clone() {
        let layer = CascadeLayer::Custom("theme".to_string());
        let cloned = layer.clone();
        assert_eq!(layer, cloned);
    }

    #[test]
    fn test_cascade_layer_partial_eq() {
        let layer1 = CascadeLayer::Base;
        let layer2 = CascadeLayer::Base;
        let layer3 = CascadeLayer::Components;

        assert_eq!(layer1, layer2);
        assert_ne!(layer1, layer3);
    }

    #[test]
    fn test_cascade_layer_custom_partial_eq() {
        let layer1 = CascadeLayer::Custom("theme".to_string());
        let layer2 = CascadeLayer::Custom("theme".to_string());
        let layer3 = CascadeLayer::Custom("other".to_string());

        assert_eq!(layer1, layer2);
        assert_ne!(layer1, layer3);
    }

    #[test]
    fn test_cascade_layer_hash() {
        let layer1 = CascadeLayer::Base;
        let layer2 = CascadeLayer::Base;
        let layer3 = CascadeLayer::Components;

        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        layer1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        layer2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        let mut hasher3 = DefaultHasher::new();
        layer3.hash(&mut hasher3);
        let hash3 = hasher3.finish();

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_cascade_layer_debug() {
        let layer = CascadeLayer::Base;
        let debug = format!("{:?}", layer);
        assert!(debug.contains("Base"));
    }

    #[test]
    fn test_cascade_layer_custom_debug() {
        let layer = CascadeLayer::Custom("theme".to_string());
        let debug = format!("{:?}", layer);
        assert!(debug.contains("Custom"));
        assert!(debug.contains("theme"));
    }
}

#[cfg(test)]
mod cascade_layers_integration_tests {
    use super::*;

    #[test]
    fn test_cascade_layers_with_class_builder() {
        let builder = ClassBuilder::new().layer_base();

        let class_set = builder.build();
        assert!(class_set.classes.contains("layer-base"));
    }

    #[test]
    fn test_cascade_layers_components_with_class_builder() {
        let builder = ClassBuilder::new().layer_components();

        let class_set = builder.build();
        assert!(class_set.classes.contains("layer-components"));
    }

    #[test]
    fn test_cascade_layers_utilities_with_class_builder() {
        let builder = ClassBuilder::new().layer_utilities();

        let class_set = builder.build();
        assert!(class_set.classes.contains("layer-utilities"));
    }

    #[test]
    fn test_cascade_layers_custom_with_class_builder() {
        let builder = ClassBuilder::new().layer_custom("theme");

        let class_set = builder.build();
        assert!(class_set.classes.contains("layer-theme"));
    }

    #[test]
    fn test_cascade_layers_custom_value_with_class_builder() {
        let builder =
            ClassBuilder::new().layer_custom_value(CascadeLayer::Custom("theme".to_string()));

        let class_set = builder.build();
        assert!(class_set.classes.contains("layer-theme"));
    }

    #[test]
    fn test_cascade_layers_with_other_utilities() {
        let builder = ClassBuilder::new()
            .layer_base()
            .class("text-blue-500")
            .class("font-bold");

        let class_set = builder.build();
        assert!(class_set.classes.contains("layer-base"));
        assert!(class_set.classes.contains("text-blue-500"));
        assert!(class_set.classes.contains("font-bold"));
    }

    #[test]
    fn test_cascade_layers_responsive() {
        let builder = ClassBuilder::new()
            .layer_base()
            .responsive(Breakpoint::Md, "layer-components");

        let class_set = builder.build();
        assert!(class_set.classes.contains("layer-base"));
        assert!(class_set.responsive.contains_key(&Breakpoint::Md));
        assert!(class_set
            .responsive
            .get(&Breakpoint::Md)
            .unwrap()
            .contains("layer-components"));
    }

    #[test]
    fn test_cascade_layers_conditional() {
        let builder = ClassBuilder::new()
            .layer_base()
            .conditional("hover", "layer-utilities");

        let class_set = builder.build();
        assert!(class_set.classes.contains("layer-base"));
        assert!(class_set.conditional.contains_key("hover"));
        assert!(class_set
            .conditional
            .get("hover")
            .unwrap()
            .contains("layer-utilities"));
    }

    #[test]
    fn test_cascade_layers_custom_variant() {
        let builder = ClassBuilder::new()
            .layer_base()
            .custom_variant("dark", "layer-components");

        let class_set = builder.build();
        assert!(class_set.classes.contains("layer-base"));
        assert!(class_set.conditional.contains_key("dark"));
        assert!(class_set
            .conditional
            .get("dark")
            .unwrap()
            .contains("layer-components"));
    }

    #[test]
    fn test_cascade_layers_multiple_layers() {
        let builder = ClassBuilder::new()
            .layer_base()
            .layer_components()
            .layer_utilities();

        let class_set = builder.build();
        assert!(class_set.classes.contains("layer-base"));
        assert!(class_set.classes.contains("layer-components"));
        assert!(class_set.classes.contains("layer-utilities"));
    }

    #[test]
    fn test_cascade_layers_build_string() {
        let classes = ClassBuilder::new()
            .layer_base()
            .class("text-blue-500")
            .build_string();

        assert!(classes.contains("layer-base"));
        assert!(classes.contains("text-blue-500"));
    }

    #[test]
    fn test_cascade_layers_css_classes() {
        let class_set = ClassBuilder::new().layer_base().class("font-bold").build();

        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("layer-base"));
        assert!(css_classes.contains("font-bold"));
    }

    #[test]
    fn test_cascade_layers_comprehensive_usage() {
        let class_set = ClassBuilder::new()
            .layer_base()
            .layer_components()
            .layer_utilities()
            .layer_custom("theme")
            .build();

        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("layer-base"));
        assert!(css_classes.contains("layer-components"));
        assert!(css_classes.contains("layer-utilities"));
        assert!(css_classes.contains("layer-theme"));
    }

    #[test]
    fn test_cascade_layers_all_variants() {
        let class_set = ClassBuilder::new()
            .layer_base()
            .layer_components()
            .layer_utilities()
            .layer_custom("theme")
            .layer_custom("utilities")
            .layer_custom("components")
            .layer_custom("base")
            .build();

        let css_classes = class_set.to_css_classes();

        // Test that all cascade layer utilities are present
        assert!(css_classes.contains("layer-base"));
        assert!(css_classes.contains("layer-components"));
        assert!(css_classes.contains("layer-utilities"));
        assert!(css_classes.contains("layer-theme"));
        assert!(css_classes.contains("layer-utilities"));
        assert!(css_classes.contains("layer-components"));
        assert!(css_classes.contains("layer-base"));
    }
}
