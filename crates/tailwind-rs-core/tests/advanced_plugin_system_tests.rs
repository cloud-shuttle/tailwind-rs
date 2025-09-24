use tailwind_rs_core::utilities::advanced_plugin_system::*;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;
use std::collections::HashMap;

#[cfg(test)]
mod advanced_plugin_system_unit_tests {
    use super::*;

    #[test]
    fn test_plugin_type_utility() {
        let plugin_type = PluginType::Utility;
        assert_eq!(plugin_type.to_string(), "utility");
        assert_eq!(plugin_type.to_class_name(), "plugin-utility");
        assert_eq!(plugin_type.to_css_value(), "utility");
    }

    #[test]
    fn test_plugin_type_component() {
        let plugin_type = PluginType::Component;
        assert_eq!(plugin_type.to_string(), "component");
        assert_eq!(plugin_type.to_class_name(), "plugin-component");
        assert_eq!(plugin_type.to_css_value(), "component");
    }

    #[test]
    fn test_plugin_type_base() {
        let plugin_type = PluginType::Base;
        assert_eq!(plugin_type.to_string(), "base");
        assert_eq!(plugin_type.to_class_name(), "plugin-base");
        assert_eq!(plugin_type.to_css_value(), "base");
    }

    #[test]
    fn test_plugin_type_variant() {
        let plugin_type = PluginType::Variant;
        assert_eq!(plugin_type.to_string(), "variant");
        assert_eq!(plugin_type.to_class_name(), "plugin-variant");
        assert_eq!(plugin_type.to_css_value(), "variant");
    }

    #[test]
    fn test_plugin_type_custom() {
        let plugin_type = PluginType::Custom("custom".to_string());
        assert_eq!(plugin_type.to_string(), "custom");
        assert_eq!(plugin_type.to_class_name(), "plugin-custom");
        assert_eq!(plugin_type.to_css_value(), "custom");
    }

    #[test]
    fn test_plugin_priority_low() {
        let priority = PluginPriority::Low;
        assert_eq!(priority.to_string(), "low");
        assert_eq!(priority.to_class_name(), "plugin-priority-low");
        assert_eq!(priority.to_css_value(), "low");
    }

    #[test]
    fn test_plugin_priority_normal() {
        let priority = PluginPriority::Normal;
        assert_eq!(priority.to_string(), "normal");
        assert_eq!(priority.to_class_name(), "plugin-priority-normal");
        assert_eq!(priority.to_css_value(), "normal");
    }

    #[test]
    fn test_plugin_priority_high() {
        let priority = PluginPriority::High;
        assert_eq!(priority.to_string(), "high");
        assert_eq!(priority.to_class_name(), "plugin-priority-high");
        assert_eq!(priority.to_css_value(), "high");
    }

    #[test]
    fn test_plugin_priority_critical() {
        let priority = PluginPriority::Critical;
        assert_eq!(priority.to_string(), "critical");
        assert_eq!(priority.to_class_name(), "plugin-priority-critical");
        assert_eq!(priority.to_css_value(), "critical");
    }

    #[test]
    fn test_plugin_priority_custom() {
        let priority = PluginPriority::Custom(42);
        assert_eq!(priority.to_string(), "42");
        assert_eq!(priority.to_class_name(), "plugin-priority-42");
        assert_eq!(priority.to_css_value(), "42");
    }

    #[test]
    fn test_plugin_config_enable() {
        let config = PluginConfig::Enable;
        assert_eq!(config.to_string(), "enable");
        assert_eq!(config.to_class_name(), "plugin-config-enable");
        assert_eq!(config.to_css_value(), "enable");
    }

    #[test]
    fn test_plugin_config_disable() {
        let config = PluginConfig::Disable;
        assert_eq!(config.to_string(), "disable");
        assert_eq!(config.to_class_name(), "plugin-config-disable");
        assert_eq!(config.to_css_value(), "disable");
    }

    #[test]
    fn test_plugin_config_configure() {
        let mut options = HashMap::new();
        options.insert("key1".to_string(), "value1".to_string());
        options.insert("key2".to_string(), "value2".to_string());
        let config = PluginConfig::Configure(options);
        let config_str = config.to_string();
        assert!(config_str.contains("configure:"));
        assert!(config_str.contains("key1:value1"));
        assert!(config_str.contains("key2:value2"));
        assert_eq!(config.to_class_name(), "plugin-config-configure");
    }

    #[test]
    fn test_plugin_config_custom() {
        let config = PluginConfig::Custom("custom".to_string());
        assert_eq!(config.to_string(), "custom");
        assert_eq!(config.to_class_name(), "plugin-config-custom");
        assert_eq!(config.to_css_value(), "custom");
    }

    #[test]
    fn test_plugin_composition_replace() {
        let composition = PluginComposition::Replace;
        assert_eq!(composition.to_string(), "replace");
        assert_eq!(composition.to_class_name(), "plugin-composition-replace");
        assert_eq!(composition.to_css_value(), "replace");
    }

    #[test]
    fn test_plugin_composition_merge() {
        let composition = PluginComposition::Merge;
        assert_eq!(composition.to_string(), "merge");
        assert_eq!(composition.to_class_name(), "plugin-composition-merge");
        assert_eq!(composition.to_css_value(), "merge");
    }

    #[test]
    fn test_plugin_composition_extend() {
        let composition = PluginComposition::Extend;
        assert_eq!(composition.to_string(), "extend");
        assert_eq!(composition.to_class_name(), "plugin-composition-extend");
        assert_eq!(composition.to_css_value(), "extend");
    }

    #[test]
    fn test_plugin_composition_prepend() {
        let composition = PluginComposition::Prepend;
        assert_eq!(composition.to_string(), "prepend");
        assert_eq!(composition.to_class_name(), "plugin-composition-prepend");
        assert_eq!(composition.to_css_value(), "prepend");
    }

    #[test]
    fn test_plugin_composition_append() {
        let composition = PluginComposition::Append;
        assert_eq!(composition.to_string(), "append");
        assert_eq!(composition.to_class_name(), "plugin-composition-append");
        assert_eq!(composition.to_css_value(), "append");
    }

    #[test]
    fn test_plugin_composition_custom() {
        let composition = PluginComposition::Custom("custom".to_string());
        assert_eq!(composition.to_string(), "custom");
        assert_eq!(composition.to_class_name(), "plugin-composition-custom");
        assert_eq!(composition.to_css_value(), "custom");
    }

    #[test]
    fn test_plugin_lifecycle_initialize() {
        let lifecycle = PluginLifecycle::Initialize;
        assert_eq!(lifecycle.to_string(), "initialize");
        assert_eq!(lifecycle.to_class_name(), "plugin-lifecycle-initialize");
        assert_eq!(lifecycle.to_css_value(), "initialize");
    }

    #[test]
    fn test_plugin_lifecycle_configure() {
        let lifecycle = PluginLifecycle::Configure;
        assert_eq!(lifecycle.to_string(), "configure");
        assert_eq!(lifecycle.to_class_name(), "plugin-lifecycle-configure");
        assert_eq!(lifecycle.to_css_value(), "configure");
    }

    #[test]
    fn test_plugin_lifecycle_execute() {
        let lifecycle = PluginLifecycle::Execute;
        assert_eq!(lifecycle.to_string(), "execute");
        assert_eq!(lifecycle.to_class_name(), "plugin-lifecycle-execute");
        assert_eq!(lifecycle.to_css_value(), "execute");
    }

    #[test]
    fn test_plugin_lifecycle_cleanup() {
        let lifecycle = PluginLifecycle::Cleanup;
        assert_eq!(lifecycle.to_string(), "cleanup");
        assert_eq!(lifecycle.to_class_name(), "plugin-lifecycle-cleanup");
        assert_eq!(lifecycle.to_css_value(), "cleanup");
    }

    #[test]
    fn test_plugin_lifecycle_custom() {
        let lifecycle = PluginLifecycle::Custom("custom".to_string());
        assert_eq!(lifecycle.to_string(), "custom");
        assert_eq!(lifecycle.to_class_name(), "plugin-lifecycle-custom");
        assert_eq!(lifecycle.to_css_value(), "custom");
    }

    #[test]
    fn test_advanced_plugin_system_serialization() {
        let plugin_type = PluginType::Utility;
        let serialized = serde_json::to_string(&plugin_type).unwrap();
        let deserialized: PluginType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(plugin_type, deserialized);
    }

    #[test]
    fn test_advanced_plugin_system_clone() {
        let plugin_type = PluginType::Utility;
        let cloned = plugin_type.clone();
        assert_eq!(plugin_type, cloned);
    }

    #[test]
    fn test_advanced_plugin_system_partial_eq() {
        let plugin_type1 = PluginType::Utility;
        let plugin_type2 = PluginType::Utility;
        let plugin_type3 = PluginType::Component;
        
        assert_eq!(plugin_type1, plugin_type2);
        assert_ne!(plugin_type1, plugin_type3);
    }

    #[test]
    fn test_advanced_plugin_system_hash() {
        let plugin_type1 = PluginType::Utility;
        let plugin_type2 = PluginType::Utility;
        let plugin_type3 = PluginType::Component;
        
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher1 = DefaultHasher::new();
        plugin_type1.hash(&mut hasher1);
        let hash1 = hasher1.finish();
        
        let mut hasher2 = DefaultHasher::new();
        plugin_type2.hash(&mut hasher2);
        let hash2 = hasher2.finish();
        
        let mut hasher3 = DefaultHasher::new();
        plugin_type3.hash(&mut hasher3);
        let hash3 = hasher3.finish();
        
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_advanced_plugin_system_debug() {
        let plugin_type = PluginType::Utility;
        let debug = format!("{:?}", plugin_type);
        assert!(debug.contains("Utility"));
    }

    #[test]
    fn test_advanced_plugin_system_all_variants() {
        let plugin_types = vec![
            PluginType::Utility,
            PluginType::Component,
            PluginType::Base,
            PluginType::Variant,
            PluginType::Custom("custom".to_string()),
        ];
        
        let class_names: Vec<String> = plugin_types.iter().map(|p| p.to_class_name()).collect();
        assert!(class_names.contains(&"plugin-utility".to_string()));
        assert!(class_names.contains(&"plugin-component".to_string()));
        assert!(class_names.contains(&"plugin-base".to_string()));
        assert!(class_names.contains(&"plugin-variant".to_string()));
        assert!(class_names.contains(&"plugin-custom".to_string()));
    }
}

#[cfg(test)]
mod advanced_plugin_system_integration_tests {
    use super::*;

    #[test]
    fn test_plugin_type_with_class_builder() {
        let builder = ClassBuilder::new()
            .plugin_type(PluginType::Utility);
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("plugin-utility"));
    }

    #[test]
    fn test_plugin_priority_with_class_builder() {
        let builder = ClassBuilder::new()
            .plugin_priority(PluginPriority::High);
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("plugin-priority-high"));
    }

    #[test]
    fn test_plugin_config_with_class_builder() {
        let builder = ClassBuilder::new()
            .plugin_config(PluginConfig::Enable);
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("plugin-config-enable"));
    }

    #[test]
    fn test_plugin_composition_with_class_builder() {
        let builder = ClassBuilder::new()
            .plugin_composition(PluginComposition::Merge);
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("plugin-composition-merge"));
    }

    #[test]
    fn test_plugin_lifecycle_with_class_builder() {
        let builder = ClassBuilder::new()
            .plugin_lifecycle(PluginLifecycle::Execute);
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("plugin-lifecycle-execute"));
    }

    #[test]
    fn test_plugin_custom_with_class_builder() {
        let mut options = HashMap::new();
        options.insert("key1".to_string(), "value1".to_string());
        let builder = ClassBuilder::new()
            .plugin_custom("custom", options);
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("plugin-custom"));
    }

    #[test]
    fn test_advanced_plugin_system_convenience_methods() {
        let builder = ClassBuilder::new()
            .plugin_utility()
            .plugin_component()
            .plugin_base()
            .plugin_variant()
            .plugin_high_priority()
            .plugin_critical_priority()
            .plugin_enabled()
            .plugin_disabled()
            .plugin_merge()
            .plugin_extend()
            .plugin_initialize()
            .plugin_execute();
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("plugin-utility"));
        assert!(class_set.classes.contains("plugin-component"));
        assert!(class_set.classes.contains("plugin-base"));
        assert!(class_set.classes.contains("plugin-variant"));
        assert!(class_set.classes.contains("plugin-priority-high"));
        assert!(class_set.classes.contains("plugin-priority-critical"));
        assert!(class_set.classes.contains("plugin-config-enable"));
        assert!(class_set.classes.contains("plugin-config-disable"));
        assert!(class_set.classes.contains("plugin-composition-merge"));
        assert!(class_set.classes.contains("plugin-composition-extend"));
        assert!(class_set.classes.contains("plugin-lifecycle-initialize"));
        assert!(class_set.classes.contains("plugin-lifecycle-execute"));
    }

    #[test]
    fn test_advanced_plugin_system_with_other_utilities() {
        let builder = ClassBuilder::new()
            .plugin_type(PluginType::Utility)
            .class("text-blue-500")
            .class("font-bold");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("plugin-utility"));
        assert!(class_set.classes.contains("text-blue-500"));
        assert!(class_set.classes.contains("font-bold"));
    }

    #[test]
    fn test_advanced_plugin_system_responsive() {
        let builder = ClassBuilder::new()
            .plugin_type(PluginType::Utility)
            .responsive(Breakpoint::Md, "plugin-component");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("plugin-utility"));
        assert!(class_set.responsive.contains_key(&Breakpoint::Md));
        assert!(class_set.responsive.get(&Breakpoint::Md).unwrap().contains("plugin-component"));
    }

    #[test]
    fn test_advanced_plugin_system_conditional() {
        let builder = ClassBuilder::new()
            .plugin_type(PluginType::Utility)
            .conditional("hover", "plugin-component");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("plugin-utility"));
        assert!(class_set.conditional.contains_key("hover"));
        assert!(class_set.conditional.get("hover").unwrap().contains("plugin-component"));
    }

    #[test]
    fn test_advanced_plugin_system_custom_variant() {
        let builder = ClassBuilder::new()
            .plugin_type(PluginType::Utility)
            .custom_variant("dark", "plugin-component");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("plugin-utility"));
        assert!(class_set.conditional.contains_key("dark"));
        assert!(class_set.conditional.get("dark").unwrap().contains("plugin-component"));
    }

    #[test]
    fn test_advanced_plugin_system_multiple_plugins() {
        let builder = ClassBuilder::new()
            .plugin_type(PluginType::Utility)
            .plugin_priority(PluginPriority::High)
            .plugin_config(PluginConfig::Enable)
            .plugin_composition(PluginComposition::Merge)
            .plugin_lifecycle(PluginLifecycle::Execute);
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("plugin-utility"));
        assert!(class_set.classes.contains("plugin-priority-high"));
        assert!(class_set.classes.contains("plugin-config-enable"));
        assert!(class_set.classes.contains("plugin-composition-merge"));
        assert!(class_set.classes.contains("plugin-lifecycle-execute"));
    }

    #[test]
    fn test_advanced_plugin_system_build_string() {
        let classes = ClassBuilder::new()
            .plugin_type(PluginType::Utility)
            .class("text-blue-500")
            .build_string();
        
        assert!(classes.contains("plugin-utility"));
        assert!(classes.contains("text-blue-500"));
    }

    #[test]
    fn test_advanced_plugin_system_css_classes() {
        let class_set = ClassBuilder::new()
            .plugin_type(PluginType::Utility)
            .class("font-bold")
            .build();
        
        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("plugin-utility"));
        assert!(css_classes.contains("font-bold"));
    }

    #[test]
    fn test_advanced_plugin_system_comprehensive_usage() {
        let class_set = ClassBuilder::new()
            .plugin_type(PluginType::Utility)
            .plugin_priority(PluginPriority::High)
            .plugin_config(PluginConfig::Enable)
            .plugin_composition(PluginComposition::Merge)
            .plugin_lifecycle(PluginLifecycle::Execute)
            .plugin_utility()
            .plugin_component()
            .plugin_base()
            .plugin_variant()
            .plugin_high_priority()
            .plugin_critical_priority()
            .plugin_enabled()
            .plugin_disabled()
            .plugin_merge()
            .plugin_extend()
            .plugin_initialize()
            .plugin_execute()
            .class("text-blue-500")
            .class("font-bold")
            .responsive(Breakpoint::Md, "plugin-component")
            .conditional("hover", "plugin-component")
            .build();
        
        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("plugin-utility"));
        assert!(css_classes.contains("plugin-priority-high"));
        assert!(css_classes.contains("plugin-config-enable"));
        assert!(css_classes.contains("plugin-composition-merge"));
        assert!(css_classes.contains("plugin-lifecycle-execute"));
        assert!(css_classes.contains("plugin-component"));
        assert!(css_classes.contains("plugin-base"));
        assert!(css_classes.contains("plugin-variant"));
        assert!(css_classes.contains("plugin-priority-critical"));
        assert!(css_classes.contains("plugin-config-disable"));
        assert!(css_classes.contains("plugin-composition-extend"));
        assert!(css_classes.contains("plugin-lifecycle-initialize"));
        assert!(css_classes.contains("text-blue-500"));
        assert!(css_classes.contains("font-bold"));
        assert!(css_classes.contains("md:plugin-component"));
        assert!(css_classes.contains("hover:plugin-component"));
    }

    #[test]
    fn test_advanced_plugin_system_all_variants() {
        let class_set = ClassBuilder::new()
            .plugin_type(PluginType::Utility)
            .plugin_type(PluginType::Component)
            .plugin_type(PluginType::Base)
            .plugin_type(PluginType::Variant)
            .plugin_type(PluginType::Custom("custom".to_string()))
            .plugin_priority(PluginPriority::Low)
            .plugin_priority(PluginPriority::Normal)
            .plugin_priority(PluginPriority::High)
            .plugin_priority(PluginPriority::Critical)
            .plugin_priority(PluginPriority::Custom(42))
            .plugin_config(PluginConfig::Enable)
            .plugin_config(PluginConfig::Disable)
            .plugin_config(PluginConfig::Custom("custom".to_string()))
            .plugin_composition(PluginComposition::Replace)
            .plugin_composition(PluginComposition::Merge)
            .plugin_composition(PluginComposition::Extend)
            .plugin_composition(PluginComposition::Prepend)
            .plugin_composition(PluginComposition::Append)
            .plugin_composition(PluginComposition::Custom("custom".to_string()))
            .plugin_lifecycle(PluginLifecycle::Initialize)
            .plugin_lifecycle(PluginLifecycle::Configure)
            .plugin_lifecycle(PluginLifecycle::Execute)
            .plugin_lifecycle(PluginLifecycle::Cleanup)
            .plugin_lifecycle(PluginLifecycle::Custom("custom".to_string()))
            .build();
        
        let css_classes = class_set.to_css_classes();
        
        // Test that all advanced plugin system utilities are present
        assert!(css_classes.contains("plugin-utility"));
        assert!(css_classes.contains("plugin-component"));
        assert!(css_classes.contains("plugin-base"));
        assert!(css_classes.contains("plugin-variant"));
        assert!(css_classes.contains("plugin-custom"));
        assert!(css_classes.contains("plugin-priority-low"));
        assert!(css_classes.contains("plugin-priority-normal"));
        assert!(css_classes.contains("plugin-priority-high"));
        assert!(css_classes.contains("plugin-priority-critical"));
        assert!(css_classes.contains("plugin-priority-42"));
        assert!(css_classes.contains("plugin-config-enable"));
        assert!(css_classes.contains("plugin-config-disable"));
        assert!(css_classes.contains("plugin-config-custom"));
        assert!(css_classes.contains("plugin-composition-replace"));
        assert!(css_classes.contains("plugin-composition-merge"));
        assert!(css_classes.contains("plugin-composition-extend"));
        assert!(css_classes.contains("plugin-composition-prepend"));
        assert!(css_classes.contains("plugin-composition-append"));
        assert!(css_classes.contains("plugin-composition-custom"));
        assert!(css_classes.contains("plugin-lifecycle-initialize"));
        assert!(css_classes.contains("plugin-lifecycle-configure"));
        assert!(css_classes.contains("plugin-lifecycle-execute"));
        assert!(css_classes.contains("plugin-lifecycle-cleanup"));
        assert!(css_classes.contains("plugin-lifecycle-custom"));
    }
}
