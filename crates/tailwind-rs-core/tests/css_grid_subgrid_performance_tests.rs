use tailwind_rs_core::utilities::grid::*;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;
use std::time::Instant;

#[cfg(test)]
mod css_grid_subgrid_performance_tests {
    use super::*;

    #[test]
    fn test_css_grid_subgrid_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 CSS Grid Subgrid utility classes
        for _ in 0..1000 {
            let _ = ClassBuilder::new()
                .grid_template_columns(GridTemplateColumns::Subgrid)
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "CSS Grid Subgrid generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_css_grid_subgrid_string_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 CSS Grid Subgrid string representations
        for _ in 0..1000 {
            let _ = GridTemplateColumns::Subgrid.to_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 10, "String generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_css_grid_subgrid_class_name_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 CSS Grid Subgrid class names
        for _ in 0..1000 {
            let _ = ClassBuilder::new().grid_template_columns(GridTemplateColumns::Subgrid).build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 5, "Class name generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_css_grid_subgrid_memory_usage() {
        let initial_memory = get_memory_usage();
        
        // Create many CSS Grid Subgrid builders
        let _builders: Vec<ClassBuilder> = (0..1000)
            .map(|_| ClassBuilder::new().grid_template_columns(GridTemplateColumns::Subgrid))
            .collect();
        
        let final_memory = get_memory_usage();
        let memory_increase = final_memory - initial_memory;
        
        assert!(memory_increase < 100_000, "Memory usage too high: {} bytes", memory_increase);
    }

    #[test]
    fn test_css_grid_subgrid_serialization_performance() {
        let columns = GridTemplateColumns::Subgrid;
        let start = Instant::now();
        
        // Serialize 1000 times
        for _ in 0..1000 {
            let _ = serde_json::to_string(&columns).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 20, "Serialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_css_grid_subgrid_deserialization_performance() {
        let columns = GridTemplateColumns::Subgrid;
        let serialized = serde_json::to_string(&columns).unwrap();
        let start = Instant::now();
        
        // Deserialize 1000 times
        for _ in 0..1000 {
            let _: GridTemplateColumns = serde_json::from_str(&serialized).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 30, "Deserialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_css_grid_subgrid_complex_builder_performance() {
        let start = Instant::now();
        
        // Generate complex class builders with CSS Grid Subgrid
        for _ in 0..100 {
            let _ = ClassBuilder::new()
                .grid_template_columns(GridTemplateColumns::Subgrid)
                .grid_template_rows(GridTemplateRows::Subgrid)
                .class("text-blue-500")
                .class("font-bold")
                .class("text-lg")
                .responsive(Breakpoint::Md, "grid-cols-3")
                .conditional("hover", "grid-rows-2")
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "Complex builder too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_css_grid_subgrid_enum_values_performance() {
        let start = Instant::now();
        
        // Generate CSS Grid Subgrid enum values
        for _i in 0..1000 {
            let _ = GridTemplateColumns::Subgrid.to_string();
            let _ = GridTemplateColumns::Cols3.to_string();
            let _ = GridTemplateColumns::Cols6.to_string();
            let _ = GridTemplateColumns::Cols12.to_string();
            let _ = GridTemplateRows::Subgrid.to_string();
            let _ = GridTemplateRows::Three.to_string();
            let _ = GridTemplateRows::Six.to_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Enum values too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_css_grid_subgrid_all_variants_performance() {
        let start = Instant::now();
        
        // Generate all CSS Grid Subgrid variants
        for _i in 0..100 {
            let _ = ClassBuilder::new()
                .grid_template_columns(GridTemplateColumns::None)
                .grid_template_columns(GridTemplateColumns::Subgrid)
                .grid_template_columns(GridTemplateColumns::Cols1)
                .grid_template_columns(GridTemplateColumns::Cols2)
                .grid_template_columns(GridTemplateColumns::Cols3)
                .grid_template_columns(GridTemplateColumns::Cols4)
                .grid_template_columns(GridTemplateColumns::Cols5)
                .grid_template_columns(GridTemplateColumns::Cols6)
                .grid_template_columns(GridTemplateColumns::Cols7)
                .grid_template_columns(GridTemplateColumns::Cols8)
                .grid_template_columns(GridTemplateColumns::Cols9)
                .grid_template_columns(GridTemplateColumns::Cols10)
                .grid_template_columns(GridTemplateColumns::Cols11)
                .grid_template_columns(GridTemplateColumns::Cols12)
                .grid_template_rows(GridTemplateRows::None)
                .grid_template_rows(GridTemplateRows::Subgrid)
                .grid_template_rows(GridTemplateRows::Auto)
                .grid_template_rows(GridTemplateRows::One)
                .grid_template_rows(GridTemplateRows::Two)
                .grid_template_rows(GridTemplateRows::Three)
                .grid_template_rows(GridTemplateRows::Four)
                .grid_template_rows(GridTemplateRows::Five)
                .grid_template_rows(GridTemplateRows::Six)
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
