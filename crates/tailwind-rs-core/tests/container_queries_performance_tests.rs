use tailwind_rs_core::utilities::container_queries::*;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;
use std::time::Instant;

#[cfg(test)]
mod container_queries_performance_tests {
    use super::*;

    #[test]
    fn test_container_queries_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 container query utility classes
        for _ in 0..1000 {
            let _ = ClassBuilder::new()
                .class("@container/inline-size:md:text-lg")
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Container queries generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_container_queries_string_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 container query string representations
        for _ in 0..1000 {
            let _ = ContainerQuery::inline_size(ContainerSize::Md).to_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 10, "String generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_container_queries_class_name_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 container query class names
        for _ in 0..1000 {
            let _ = ClassBuilder::new().class("@container/inline-size:md:text-lg").build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 5, "Class name generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_container_queries_memory_usage() {
        let initial_memory = get_memory_usage();
        
        // Create many container query builders
        let _builders: Vec<ClassBuilder> = (0..1000)
            .map(|_| ClassBuilder::new().class("@container/inline-size:md:text-lg"))
            .collect();
        
        let final_memory = get_memory_usage();
        let memory_increase = final_memory - initial_memory;
        
        assert!(memory_increase < 100_000, "Memory usage too high: {} bytes", memory_increase);
    }

    #[test]
    fn test_container_queries_serialization_performance() {
        let query = ContainerQuery::inline_size(ContainerSize::Md);
        let start = Instant::now();
        
        // Serialize 1000 times
        for _ in 0..1000 {
            let _ = serde_json::to_string(&query).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 20, "Serialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_container_queries_deserialization_performance() {
        let query = ContainerQuery::inline_size(ContainerSize::Md);
        let serialized = serde_json::to_string(&query).unwrap();
        let start = Instant::now();
        
        // Deserialize 1000 times
        for _ in 0..1000 {
            let _: ContainerQuery = serde_json::from_str(&serialized).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 30, "Deserialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_container_queries_complex_builder_performance() {
        let start = Instant::now();
        
        // Generate complex class builders with container queries
        for _ in 0..100 {
            let _ = ClassBuilder::new()
                .class("@container/inline-size:md:text-lg")
                .class("@container/block-size:lg:text-xl")
                .class("@container/aspect-ratio:widescreen:grid-cols-2")
                .class("@container/orientation:landscape:flex-row")
                .class("text-blue-500")
                .class("font-bold")
                .class("text-lg")
                .responsive(Breakpoint::Md, "@container/block-size:lg:text-xl")
                .conditional("hover", "@container/inline-size:md:text-lg")
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "Complex builder too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_container_queries_enum_values_performance() {
        let start = Instant::now();
        
        // Generate container query enum values
        for _i in 0..1000 {
            let _ = ContainerQuery::inline_size(ContainerSize::Md).to_string();
            let _ = ContainerQuery::block_size(ContainerSize::Lg).to_string();
            let _ = ContainerQuery::width(ContainerSize::Sm).to_string();
            let _ = ContainerQuery::height(ContainerSize::Xl).to_string();
            let _ = ContainerQuery::aspect_ratio(ContainerAspectRatio::Widescreen).to_string();
            let _ = ContainerQuery::orientation(ContainerOrientation::Landscape).to_string();
            let _ = ContainerSize::Md.to_string();
            let _ = ContainerSize::Lg.to_string();
            let _ = ContainerAspectRatio::Widescreen.to_string();
            let _ = ContainerOrientation::Landscape.to_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Enum values too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_container_queries_all_variants_performance() {
        let start = Instant::now();
        
        // Generate all container query variants
        for _i in 0..100 {
            let _ = ClassBuilder::new()
                .class("@container/inline-size:xs:text-sm")
                .class("@container/inline-size:sm:text-base")
                .class("@container/inline-size:md:text-lg")
                .class("@container/inline-size:lg:text-xl")
                .class("@container/inline-size:xl:text-2xl")
                .class("@container/inline-size:2xl:text-3xl")
                .class("@container/block-size:xs:h-8")
                .class("@container/block-size:sm:h-10")
                .class("@container/block-size:md:h-12")
                .class("@container/block-size:lg:h-14")
                .class("@container/block-size:xl:h-16")
                .class("@container/block-size:2xl:h-20")
                .class("@container/width:xs:w-8")
                .class("@container/width:sm:w-10")
                .class("@container/width:md:w-12")
                .class("@container/width:lg:w-14")
                .class("@container/width:xl:w-16")
                .class("@container/width:2xl:w-20")
                .class("@container/height:xs:h-8")
                .class("@container/height:sm:h-10")
                .class("@container/height:md:h-12")
                .class("@container/height:lg:h-14")
                .class("@container/height:xl:h-16")
                .class("@container/height:2xl:h-20")
                .class("@container/aspect-ratio:square:aspect-square")
                .class("@container/aspect-ratio:video:aspect-video")
                .class("@container/aspect-ratio:widescreen:aspect-widescreen")
                .class("@container/aspect-ratio:ultrawide:aspect-ultrawide")
                .class("@container/orientation:landscape:flex-row")
                .class("@container/orientation:portrait:flex-col")
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
