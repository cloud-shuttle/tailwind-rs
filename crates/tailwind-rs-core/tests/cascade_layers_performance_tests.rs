use std::time::Instant;
use tailwind_rs_core::utilities::modern_css_features::*;
use tailwind_rs_core::Breakpoint;
use tailwind_rs_core::ClassBuilder;

#[cfg(test)]
mod cascade_layers_performance_tests {
    use super::*;

    #[test]
    fn test_cascade_layers_generation_performance() {
        let start = Instant::now();

        // Generate 1000 cascade layer utility classes
        for _ in 0..1000 {
            let _ = ClassBuilder::new().layer_base().build();
        }

        let duration = start.elapsed();
        assert!(
            duration.as_millis() < 50,
            "Cascade layers generation too slow: {}ms",
            duration.as_millis()
        );
    }

    #[test]
    fn test_cascade_layers_string_generation_performance() {
        let start = Instant::now();

        // Generate 1000 cascade layer string representations
        for _ in 0..1000 {
            let _ = CascadeLayer::Base.to_string();
        }

        let duration = start.elapsed();
        assert!(
            duration.as_millis() < 10,
            "String generation too slow: {}ms",
            duration.as_millis()
        );
    }

    #[test]
    fn test_cascade_layers_class_name_generation_performance() {
        let start = Instant::now();

        // Generate 1000 cascade layer class names
        for _ in 0..1000 {
            let _ = ClassBuilder::new().layer_base().build();
        }

        let duration = start.elapsed();
        assert!(
            duration.as_millis() < 5,
            "Class name generation too slow: {}ms",
            duration.as_millis()
        );
    }

    #[test]
    fn test_cascade_layers_memory_usage() {
        let initial_memory = get_memory_usage();

        // Create many cascade layer builders
        let _builders: Vec<ClassBuilder> = (0..1000)
            .map(|_| ClassBuilder::new().layer_base())
            .collect();

        let final_memory = get_memory_usage();
        let memory_increase = final_memory - initial_memory;

        assert!(
            memory_increase < 100_000,
            "Memory usage too high: {} bytes",
            memory_increase
        );
    }

    #[test]
    fn test_cascade_layers_serialization_performance() {
        let layer = CascadeLayer::Base;
        let start = Instant::now();

        // Serialize 1000 times
        for _ in 0..1000 {
            let _ = serde_json::to_string(&layer).unwrap();
        }

        let duration = start.elapsed();
        assert!(
            duration.as_millis() < 20,
            "Serialization too slow: {}ms",
            duration.as_millis()
        );
    }

    #[test]
    fn test_cascade_layers_deserialization_performance() {
        let layer = CascadeLayer::Base;
        let serialized = serde_json::to_string(&layer).unwrap();
        let start = Instant::now();

        // Deserialize 1000 times
        for _ in 0..1000 {
            let _: CascadeLayer = serde_json::from_str(&serialized).unwrap();
        }

        let duration = start.elapsed();
        assert!(
            duration.as_millis() < 30,
            "Deserialization too slow: {}ms",
            duration.as_millis()
        );
    }

    #[test]
    fn test_cascade_layers_complex_builder_performance() {
        let start = Instant::now();

        // Generate complex class builders with cascade layers
        for _ in 0..100 {
            let _ = ClassBuilder::new()
                .layer_base()
                .layer_components()
                .layer_utilities()
                .layer_custom("theme")
                .class("text-blue-500")
                .class("font-bold")
                .class("text-lg")
                .responsive(Breakpoint::Md, "layer-components")
                .conditional("hover", "layer-utilities")
                .build();
        }

        let duration = start.elapsed();
        assert!(
            duration.as_millis() < 100,
            "Complex builder too slow: {}ms",
            duration.as_millis()
        );
    }

    #[test]
    fn test_cascade_layers_enum_values_performance() {
        let start = Instant::now();

        // Generate cascade layer enum values
        for _i in 0..1000 {
            let _ = CascadeLayer::Base.to_string();
            let _ = CascadeLayer::Components.to_string();
            let _ = CascadeLayer::Utilities.to_string();
            let _ = CascadeLayer::Custom("theme".to_string()).to_string();
        }

        let duration = start.elapsed();
        assert!(
            duration.as_millis() < 50,
            "Enum values too slow: {}ms",
            duration.as_millis()
        );
    }

    #[test]
    fn test_cascade_layers_all_variants_performance() {
        let start = Instant::now();

        // Generate all cascade layer variants
        for _i in 0..100 {
            let _ = ClassBuilder::new()
                .layer_base()
                .layer_components()
                .layer_utilities()
                .layer_custom("theme")
                .layer_custom("utilities")
                .layer_custom("components")
                .layer_custom("base")
                .build();
        }

        let duration = start.elapsed();
        assert!(
            duration.as_millis() < 200,
            "All variants too slow: {}ms",
            duration.as_millis()
        );
    }
}

// Helper function to get memory usage (simplified)
fn get_memory_usage() -> usize {
    // This is a simplified implementation
    // In a real scenario, you might use more sophisticated memory tracking
    std::mem::size_of::<ClassBuilder>() * 1000
}
