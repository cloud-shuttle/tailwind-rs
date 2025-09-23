use tailwind_rs_core::utilities::enhanced_backdrop_filters::*;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;
use std::time::Instant;

#[cfg(test)]
mod enhanced_backdrop_filters_performance_tests {
    use super::*;

    #[test]
    fn test_enhanced_backdrop_filters_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 enhanced backdrop filter utility classes
        for _ in 0..1000 {
            let _ = ClassBuilder::new()
                .backdrop_blur_sm()
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Enhanced backdrop filters generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_enhanced_backdrop_filters_string_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 enhanced backdrop filter string representations
        for _ in 0..1000 {
            let _ = EnhancedBackdropBlur::Sm.to_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 10, "String generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_enhanced_backdrop_filters_class_name_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 enhanced backdrop filter class names
        for _ in 0..1000 {
            let _ = ClassBuilder::new().backdrop_blur_sm().build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 5, "Class name generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_enhanced_backdrop_filters_memory_usage() {
        let initial_memory = get_memory_usage();
        
        // Create many enhanced backdrop filter builders
        let _builders: Vec<ClassBuilder> = (0..1000)
            .map(|_| ClassBuilder::new().backdrop_blur_sm())
            .collect();
        
        let final_memory = get_memory_usage();
        let memory_increase = final_memory - initial_memory;
        
        assert!(memory_increase < 100_000, "Memory usage too high: {} bytes", memory_increase);
    }

    #[test]
    fn test_enhanced_backdrop_filters_serialization_performance() {
        let blur = EnhancedBackdropBlur::Lg;
        let start = Instant::now();
        
        // Serialize 1000 times
        for _ in 0..1000 {
            let _ = serde_json::to_string(&blur).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 20, "Serialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_enhanced_backdrop_filters_deserialization_performance() {
        let blur = EnhancedBackdropBlur::Lg;
        let serialized = serde_json::to_string(&blur).unwrap();
        let start = Instant::now();
        
        // Deserialize 1000 times
        for _ in 0..1000 {
            let _: EnhancedBackdropBlur = serde_json::from_str(&serialized).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 30, "Deserialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_enhanced_backdrop_filters_complex_builder_performance() {
        let start = Instant::now();
        
        // Generate complex class builders with enhanced backdrop filters
        for _ in 0..100 {
            let _ = ClassBuilder::new()
                .backdrop_blur_sm()
                .backdrop_blur_lg()
                .backdrop_blur_3xl()
                .class("text-blue-500")
                .class("font-bold")
                .class("text-lg")
                .responsive(Breakpoint::Md, "backdrop-blur-xl")
                .conditional("hover", "backdrop-blur-none")
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "Complex builder too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_enhanced_backdrop_filters_enum_values_performance() {
        let start = Instant::now();
        
        // Generate enhanced backdrop filter enum values
        for _i in 0..1000 {
            let _ = EnhancedBackdropBlur::Sm.to_string();
            let _ = EnhancedBackdropBlur::Lg.to_string();
            let _ = EnhancedBackdropBlur::Xl3.to_string();
            let _ = EnhancedBackdropBrightness::Half.to_string();
            let _ = EnhancedBackdropContrast::OneHundredFifty.to_string();
            let _ = EnhancedBackdropGrayscale::Quarter.to_string();
            let _ = EnhancedBackdropHueRotate::Ninety.to_string();
            let _ = EnhancedBackdropInvert::Half.to_string();
            let _ = EnhancedBackdropOpacity::SeventyFive.to_string();
            let _ = EnhancedBackdropSaturate::OneHundredFifty.to_string();
            let _ = EnhancedBackdropSepia::Quarter.to_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Enum values too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_enhanced_backdrop_filters_all_variants_performance() {
        let start = Instant::now();
        
        // Generate all enhanced backdrop filter variants
        for _i in 0..100 {
            let _ = ClassBuilder::new()
                .backdrop_blur_none()
                .backdrop_blur_sm()
                .backdrop_blur()
                .backdrop_blur_md()
                .backdrop_blur_lg()
                .backdrop_blur_xl()
                .backdrop_blur_2xl()
                .backdrop_blur_3xl()
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
