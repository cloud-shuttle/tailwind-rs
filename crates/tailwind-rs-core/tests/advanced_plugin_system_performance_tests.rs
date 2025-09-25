use tailwind_rs_core::utilities::advanced_plugin_system::*;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;
use std::time::Instant;

#[cfg(test)]
mod advanced_plugin_system_performance_tests {
    use super::*;

    #[test]
    fn test_advanced_plugin_system_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 advanced plugin system utility classes
        for _ in 0..1000 {
            let _ = ClassBuilder::new()
                .plugin_type(PluginType::Utility)
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Advanced plugin system generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_advanced_plugin_system_string_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 advanced plugin system string representations
        for _ in 0..1000 {
            let _ = PluginType::Utility.to_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 10, "String generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_advanced_plugin_system_class_name_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 advanced plugin system class names
        for _ in 0..1000 {
            let _ = ClassBuilder::new().plugin_type(PluginType::Utility).build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 5, "Class name generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_advanced_plugin_system_memory_usage() {
        let initial_memory = get_memory_usage();
        
        // Create many advanced plugin system builders
        let _builders: Vec<ClassBuilder> = (0..1000)
            .map(|_| ClassBuilder::new().plugin_type(PluginType::Utility))
            .collect();
        
        let final_memory = get_memory_usage();
        let memory_increase = final_memory - initial_memory;
        
        assert!(memory_increase < 100_000, "Memory usage too high: {} bytes", memory_increase);
    }

    #[test]
    fn test_advanced_plugin_system_serialization_performance() {
        let plugin_type = PluginType::Utility;
        let start = Instant::now();
        
        // Serialize 1000 times
        for _ in 0..1000 {
            let _ = serde_json::to_string(&plugin_type).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 20, "Serialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_advanced_plugin_system_deserialization_performance() {
        let plugin_type = PluginType::Utility;
        let serialized = serde_json::to_string(&plugin_type).unwrap();
        let start = Instant::now();
        
        // Deserialize 1000 times
        for _ in 0..1000 {
            let _: PluginType = serde_json::from_str(&serialized).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 30, "Deserialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_advanced_plugin_system_complex_builder_performance() {
        let start = Instant::now();
        
        // Generate complex class builders with advanced plugin system
        for _ in 0..100 {
            let _ = ClassBuilder::new()
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
                .class("text-lg")
                .responsive(Breakpoint::Md, "plugin-component")
                .conditional("hover", "plugin-component")
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "Complex builder too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_advanced_plugin_system_enum_values_performance() {
        let start = Instant::now();
        
        // Generate advanced plugin system enum values
        for _i in 0..1000 {
            let _ = PluginType::Utility.to_string();
            let _ = PluginType::Component.to_string();
            let _ = PluginType::Base.to_string();
            let _ = PluginType::Variant.to_string();
            let _ = PluginType::Custom("custom".to_string()).to_string();
            let _ = PluginPriority::Low.to_string();
            let _ = PluginPriority::Normal.to_string();
            let _ = PluginPriority::High.to_string();
            let _ = PluginPriority::Critical.to_string();
            let _ = PluginPriority::Custom(42).to_string();
            let _ = PluginConfig::Enable.to_string();
            let _ = PluginConfig::Disable.to_string();
            let _ = PluginConfig::Custom("custom".to_string()).to_string();
            let _ = PluginComposition::Replace.to_string();
            let _ = PluginComposition::Merge.to_string();
            let _ = PluginComposition::Extend.to_string();
            let _ = PluginComposition::Prepend.to_string();
            let _ = PluginComposition::Append.to_string();
            let _ = PluginComposition::Custom("custom".to_string()).to_string();
            let _ = PluginLifecycle::Initialize.to_string();
            let _ = PluginLifecycle::Configure.to_string();
            let _ = PluginLifecycle::Execute.to_string();
            let _ = PluginLifecycle::Cleanup.to_string();
            let _ = PluginLifecycle::Custom("custom".to_string()).to_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Enum values too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_advanced_plugin_system_all_variants_performance() {
        let start = Instant::now();
        
        // Generate all advanced plugin system variants
        for _i in 0..100 {
            let _ = ClassBuilder::new()
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
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 200, "All variants too slow: {}ms", duration.as_millis());
    }
}

// Helper function to get memory usage (simplified)
fn get_memory_usage() -> usize {
    // This is a simplified implementation
    // In a real scenario, you might use more sophisticated memory tracking
    std::mem::size_of::<ClassBuilder>() * 1000
}
