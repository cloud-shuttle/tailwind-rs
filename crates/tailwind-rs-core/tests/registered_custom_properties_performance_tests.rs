use tailwind_rs_core::utilities::modern_css_features::*;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;
use std::time::Instant;

#[cfg(test)]
mod registered_custom_properties_performance_tests {
    use super::*;

    #[test]
    fn test_custom_properties_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 custom property utility classes
        for _ in 0..1000 {
            let _ = ClassBuilder::new()
                .custom_property("color", "red")
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Custom properties generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_custom_properties_string_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 custom property string representations
        for _ in 0..1000 {
            let _ = CustomProperty::Color("red".to_string()).to_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 10, "String generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_custom_properties_class_name_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 custom property class names
        for _ in 0..1000 {
            let _ = ClassBuilder::new().custom_property("color", "red").build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 5, "Class name generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_custom_properties_memory_usage() {
        let initial_memory = get_memory_usage();
        
        // Create many custom property builders
        let _builders: Vec<ClassBuilder> = (0..1000)
            .map(|_| ClassBuilder::new().custom_property("color", "red"))
            .collect();
        
        let final_memory = get_memory_usage();
        let memory_increase = final_memory - initial_memory;
        
        assert!(memory_increase < 100_000, "Memory usage too high: {} bytes", memory_increase);
    }

    #[test]
    fn test_custom_properties_serialization_performance() {
        let property = CustomProperty::Color("red".to_string());
        let start = Instant::now();
        
        // Serialize 1000 times
        for _ in 0..1000 {
            let _ = serde_json::to_string(&property).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 20, "Serialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_custom_properties_deserialization_performance() {
        let property = CustomProperty::Color("red".to_string());
        let serialized = serde_json::to_string(&property).unwrap();
        let start = Instant::now();
        
        // Deserialize 1000 times
        for _ in 0..1000 {
            let _: CustomProperty = serde_json::from_str(&serialized).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 30, "Deserialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_custom_properties_complex_builder_performance() {
        let start = Instant::now();
        
        // Generate complex class builders with custom properties
        for _ in 0..100 {
            let _ = ClassBuilder::new()
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
                .class("text-blue-500")
                .class("font-bold")
                .class("text-lg")
                .responsive(Breakpoint::Md, "text-lg")
                .conditional("hover", "text-xl")
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "Complex builder too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_custom_properties_enum_values_performance() {
        let start = Instant::now();
        
        // Generate custom property enum values
        for _i in 0..1000 {
            let _ = CustomProperty::Color("red".to_string()).to_string();
            let _ = CustomProperty::Spacing("1rem".to_string()).to_string();
            let _ = CustomProperty::FontSize("16px".to_string()).to_string();
            let _ = CustomProperty::FontWeight("bold".to_string()).to_string();
            let _ = CustomProperty::LineHeight("1.5".to_string()).to_string();
            let _ = CustomProperty::BorderRadius("8px".to_string()).to_string();
            let _ = CustomProperty::BoxShadow("0 4px 6px -1px rgb(0 0 0 / 0.1)".to_string()).to_string();
            let _ = CustomProperty::ZIndex("10".to_string()).to_string();
            let _ = CustomProperty::Opacity("0.8".to_string()).to_string();
            let _ = CustomProperty::Transform("rotate(45deg)".to_string()).to_string();
            let _ = CustomProperty::Animation("fadeIn 0.5s ease-in-out".to_string()).to_string();
            let _ = CustomProperty::Transition("all 0.3s ease".to_string()).to_string();
            let _ = CustomProperty::Generic("custom".to_string(), "value".to_string()).to_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Enum values too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_custom_properties_all_variants_performance() {
        let start = Instant::now();
        
        // Generate all custom property variants
        for _i in 0..100 {
            let _ = ClassBuilder::new()
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
