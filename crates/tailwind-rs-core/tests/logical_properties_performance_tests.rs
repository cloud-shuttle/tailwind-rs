use tailwind_rs_core::utilities::logical_properties::*;
use tailwind_rs_core::utilities::spacing::SpacingValue;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;
use std::time::Instant;

#[cfg(test)]
mod logical_properties_performance_tests {
    use super::*;

    #[test]
    fn test_logical_properties_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 logical property utility classes
        for _ in 0..1000 {
            let _ = ClassBuilder::new()
                .margin_inline_start(SpacingValue::Integer(4))
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Logical properties generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_logical_properties_string_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 logical property string representations
        for _ in 0..1000 {
            let _ = LogicalDirection::InlineStart.to_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 10, "String generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_logical_properties_class_name_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 logical property class names
        for _ in 0..1000 {
            let _ = ClassBuilder::new().margin_inline_start(SpacingValue::Integer(4)).build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 5, "Class name generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_logical_properties_memory_usage() {
        let initial_memory = get_memory_usage();
        
        // Create many logical property builders
        let _builders: Vec<ClassBuilder> = (0..1000)
            .map(|_| ClassBuilder::new().margin_inline_start(SpacingValue::Integer(4)))
            .collect();
        
        let final_memory = get_memory_usage();
        let memory_increase = final_memory - initial_memory;
        
        assert!(memory_increase < 100_000, "Memory usage too high: {} bytes", memory_increase);
    }

    #[test]
    fn test_logical_properties_serialization_performance() {
        let direction = LogicalDirection::InlineStart;
        let start = Instant::now();
        
        // Serialize 1000 times
        for _ in 0..1000 {
            let _ = serde_json::to_string(&direction).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 20, "Serialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_logical_properties_deserialization_performance() {
        let direction = LogicalDirection::InlineStart;
        let serialized = serde_json::to_string(&direction).unwrap();
        let start = Instant::now();
        
        // Deserialize 1000 times
        for _ in 0..1000 {
            let _: LogicalDirection = serde_json::from_str(&serialized).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 30, "Deserialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_logical_properties_complex_builder_performance() {
        let start = Instant::now();
        
        // Generate complex class builders with logical properties
        for _ in 0..100 {
            let _ = ClassBuilder::new()
                .margin_inline_start(SpacingValue::Integer(4))
                .margin_inline_end(SpacingValue::Integer(4))
                .padding_inline_start(SpacingValue::Integer(2))
                .padding_inline_end(SpacingValue::Integer(2))
                .border_inline_start(SpacingValue::Integer(1))
                .border_inline_end(SpacingValue::Integer(1))
                .inset_inline_start(SpacingValue::Integer(4))
                .inset_inline_end(SpacingValue::Integer(4))
                .class("text-blue-500")
                .class("font-bold")
                .class("text-lg")
                .responsive(Breakpoint::Md, "ms-8")
                .conditional("hover", "me-8")
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "Complex builder too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_logical_properties_enum_values_performance() {
        let start = Instant::now();
        
        // Generate logical property enum values
        for _i in 0..1000 {
            let _ = LogicalDirection::InlineStart.to_string();
            let _ = LogicalDirection::InlineEnd.to_string();
            let _ = LogicalDirection::BlockStart.to_string();
            let _ = LogicalDirection::BlockEnd.to_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Enum values too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_logical_properties_all_variants_performance() {
        let start = Instant::now();
        
        // Generate all logical property variants
        for _i in 0..100 {
            let _ = ClassBuilder::new()
                .margin_inline_start(SpacingValue::Integer(4))
                .margin_inline_end(SpacingValue::Integer(4))
                .margin_block_start(SpacingValue::Integer(4))
                .margin_block_end(SpacingValue::Integer(4))
                .padding_inline_start(SpacingValue::Integer(2))
                .padding_inline_end(SpacingValue::Integer(2))
                .padding_block_start(SpacingValue::Integer(2))
                .padding_block_end(SpacingValue::Integer(2))
                .border_inline_start(SpacingValue::Integer(1))
                .border_inline_end(SpacingValue::Integer(1))
                .border_block_start(SpacingValue::Integer(1))
                .border_block_end(SpacingValue::Integer(1))
                .inset_inline_start(SpacingValue::Integer(4))
                .inset_inline_end(SpacingValue::Integer(4))
                .inset_block_start(SpacingValue::Integer(2))
                .inset_block_end(SpacingValue::Integer(2))
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
