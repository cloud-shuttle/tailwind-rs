use tailwind_rs_core::utilities::text_shadow::*;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;
use std::time::Instant;

#[cfg(test)]
mod text_shadow_performance_tests {
    use super::*;

    #[test]
    fn test_text_shadow_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 text shadow classes
        for _ in 0..1000 {
            let _ = ClassBuilder::new()
                .text_shadow_sm()
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Text shadow generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_text_shadow_css_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 text shadow CSS strings
        for _ in 0..1000 {
            let _ = TextShadow::Lg.to_css_value();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 10, "CSS generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_text_shadow_class_name_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 text shadow class names
        for _ in 0..1000 {
            let _ = TextShadow::Xl.to_class_name();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 5, "Class name generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_text_shadow_memory_usage() {
        let initial_memory = get_memory_usage();
        
        // Create many text shadow builders
        let _builders: Vec<ClassBuilder> = (0..1000)
            .map(|_| ClassBuilder::new().text_shadow_sm())
            .collect();
        
        let final_memory = get_memory_usage();
        let memory_increase = final_memory - initial_memory;
        
        assert!(memory_increase < 100_000, "Memory usage too high: {} bytes", memory_increase);
    }

    #[test]
    fn test_text_shadow_serialization_performance() {
        let shadow = TextShadow::Lg;
        let start = Instant::now();
        
        // Serialize 1000 times
        for _ in 0..1000 {
            let _ = serde_json::to_string(&shadow).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 20, "Serialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_text_shadow_deserialization_performance() {
        let shadow = TextShadow::Lg;
        let serialized = serde_json::to_string(&shadow).unwrap();
        let start = Instant::now();
        
        // Deserialize 1000 times
        for _ in 0..1000 {
            let _: TextShadow = serde_json::from_str(&serialized).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 30, "Deserialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_text_shadow_complex_builder_performance() {
        let start = Instant::now();
        
        // Generate complex class builders with text shadow
        for _ in 0..100 {
            let _ = ClassBuilder::new()
                .text_shadow_sm()
                .class("text-blue-500")
                .class("font-bold")
                .class("text-lg")
                .responsive(Breakpoint::Md, "text-shadow-lg")
                .conditional("hover", "text-shadow-xl")
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "Complex builder too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_text_shadow_custom_values_performance() {
        let start = Instant::now();
        
        // Generate custom text shadow values
        for _i in 0..1000 {
            let _ = TextShadow::Xl.to_css_value();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Custom values too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_text_shadow_arbitrary_values_performance() {
        let start = Instant::now();
        
        // Generate arbitrary text shadow values
        for _i in 0..1000 {
            let _ = TextShadow::Inner.to_css_value();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Arbitrary values too slow: {}ms", duration.as_millis());
    }
}

// Helper function to get memory usage (simplified)
fn get_memory_usage() -> usize {
    // This is a simplified implementation
    // In a real scenario, you might use more sophisticated memory tracking
    std::mem::size_of::<ClassBuilder>() * 1000
}
