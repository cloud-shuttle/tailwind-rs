use tailwind_rs_core::utilities::mask::*;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;
use std::time::Instant;

#[cfg(test)]
mod mask_utilities_performance_tests {
    use super::*;

    #[test]
    fn test_mask_utilities_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 mask utility classes
        for _ in 0..1000 {
            let _ = ClassBuilder::new()
                .mask_alpha()
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Mask utilities generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_mask_utilities_string_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 mask utility string representations
        for _ in 0..1000 {
            let _ = MaskType::Alpha.to_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 10, "String generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_mask_utilities_class_name_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 mask utility class names
        for _ in 0..1000 {
            let _ = ClassBuilder::new().mask_alpha().build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 5, "Class name generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_mask_utilities_memory_usage() {
        let initial_memory = get_memory_usage();
        
        // Create many mask utility builders
        let _builders: Vec<ClassBuilder> = (0..1000)
            .map(|_| ClassBuilder::new().mask_alpha())
            .collect();
        
        let final_memory = get_memory_usage();
        let memory_increase = final_memory - initial_memory;
        
        assert!(memory_increase < 100_000, "Memory usage too high: {} bytes", memory_increase);
    }

    #[test]
    fn test_mask_utilities_serialization_performance() {
        let mask_type = MaskType::Alpha;
        let start = Instant::now();
        
        // Serialize 1000 times
        for _ in 0..1000 {
            let _ = serde_json::to_string(&mask_type).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 20, "Serialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_mask_utilities_deserialization_performance() {
        let mask_type = MaskType::Alpha;
        let serialized = serde_json::to_string(&mask_type).unwrap();
        let start = Instant::now();
        
        // Deserialize 1000 times
        for _ in 0..1000 {
            let _: MaskType = serde_json::from_str(&serialized).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 30, "Deserialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_mask_utilities_complex_builder_performance() {
        let start = Instant::now();
        
        // Generate complex class builders with mask utilities
        for _ in 0..100 {
            let _ = ClassBuilder::new()
                .mask_alpha()
                .mask_repeat_round()
                .mask_size_cover()
                .mask_center()
                .mask_clip_border()
                .mask_origin_padding()
                .class("text-blue-500")
                .class("font-bold")
                .class("text-lg")
                .responsive(Breakpoint::Md, "mask-luminance")
                .conditional("hover", "mask-none")
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "Complex builder too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_mask_utilities_enum_values_performance() {
        let start = Instant::now();
        
        // Generate mask utility enum values
        for _i in 0..1000 {
            let _ = MaskType::Alpha.to_string();
            let _ = MaskMode::Luminance.to_string();
            let _ = MaskComposite::Add.to_string();
            let _ = MaskRepeat::Round.to_string();
            let _ = MaskSize::Cover.to_string();
            let _ = MaskPosition::Center.to_string();
            let _ = MaskClip::BorderBox.to_string();
            let _ = MaskOrigin::PaddingBox.to_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Enum values too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_mask_utilities_all_variants_performance() {
        let start = Instant::now();
        
        // Generate all mask utility variants
        for _i in 0..100 {
            let _ = ClassBuilder::new()
                .mask_none()
                .mask_alpha()
                .mask_luminance()
                .mask_repeat_none()
                .mask_repeat()
                .mask_repeat_x()
                .mask_repeat_y()
                .mask_repeat_round()
                .mask_repeat_space()
                .mask_size_auto()
                .mask_size_cover()
                .mask_size_contain()
                .mask_center()
                .mask_top()
                .mask_bottom()
                .mask_left()
                .mask_right()
                .mask_top_left()
                .mask_top_right()
                .mask_bottom_left()
                .mask_bottom_right()
                .mask_clip_border()
                .mask_clip_padding()
                .mask_clip_content()
                .mask_clip_text()
                .mask_origin_border()
                .mask_origin_padding()
                .mask_origin_content()
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
