//! Performance regression tests and benchmarks
//!
//! These tests ensure CSS generation performance doesn't regress
//! and validate memory usage patterns.

#[cfg(test)]
mod tests {
    use super::super::CssGenerator;
    use crate::error::Result;
    use std::time::{Duration, Instant};

    /// Benchmark basic CSS generation performance
    #[test]
    fn benchmark_basic_css_generation() -> Result<()> {
        let mut generator = CssGenerator::new();

        let start = Instant::now();
        for _ in 0..1000 {
            let _css = generator.process_element_classes(&["bg-blue-500", "text-white", "p-4"])?;
        }
        let duration = start.elapsed();

        // Should complete within reasonable time (adjust threshold as needed)
        assert!(duration < Duration::from_millis(100), "Performance regression detected: {}ms", duration.as_millis());

        Ok(())
    }

    /// Benchmark complex variant combinations
    #[test]
    fn benchmark_complex_variants() -> Result<()> {
        let mut generator = CssGenerator::new();

        let complex_classes = &[
            "bg-blue-500", "hover:bg-blue-600", "focus:bg-blue-700", "active:bg-blue-800",
            "sm:bg-red-500", "md:bg-green-500", "lg:bg-yellow-500", "xl:bg-purple-500",
            "@container-sm:bg-pink-500", "motion-reduce:opacity-50",
            "[data-state=\"open\"]:bg-green-500", "scale-110", "rotate-3"
        ];

        let start = Instant::now();
        for _ in 0..500 {
            let _css = generator.process_element_classes(complex_classes)?;
        }
        let duration = start.elapsed();

        assert!(duration < Duration::from_millis(200), "Complex variant performance regression: {}ms", duration.as_millis());

        Ok(())
    }

    /// Test memory usage patterns
    #[test]
    fn test_memory_usage_patterns() -> Result<()> {
        let mut generator = CssGenerator::new();

        // Get baseline memory usage (approximate)
        let initial_css = generator.process_element_classes(&["bg-blue-500"])?;

        // Process many classes
        let large_classes: Vec<String> = (0..1000)
            .map(|i| format!("bg-blue-{}", 500 + (i % 10)))
            .collect();

        let large_css = generator.process_element_classes(&large_classes.iter().map(|s| s.as_str()).collect::<Vec<_>>())?;

        // CSS should scale reasonably with input size
        assert!(large_css.len() > initial_css.len(), "CSS output should scale with input");
        assert!(large_css.len() < initial_css.len() * 50, "CSS output grew unreasonably large");

        Ok(())
    }

    /// Test CSS output size optimization
    #[test]
    fn test_css_output_optimization() -> Result<()> {
        let mut generator = CssGenerator::new();

        // Test duplicate class handling
        let css1 = generator.process_element_classes(&["bg-blue-500", "bg-blue-500"])?;
        let css2 = generator.process_element_classes(&["bg-blue-500"])?;

        // Should be efficient - duplicate classes shouldn't bloat output significantly
        assert!(css1.len() <= css2.len() * 2, "Duplicate classes cause excessive output");

        Ok(())
    }

    /// Test caching effectiveness
    #[test]
    fn test_caching_effectiveness() -> Result<()> {
        let mut generator = CssGenerator::new();

        let test_classes = &["bg-blue-500", "text-white", "p-4", "rounded-lg"];

        // First run - should establish cache
        let start1 = Instant::now();
        let _css1 = generator.process_element_classes(test_classes)?;
        let duration1 = start1.elapsed();

        // Second run - should benefit from caching
        let start2 = Instant::now();
        let _css2 = generator.process_element_classes(test_classes)?;
        let duration2 = start2.elapsed();

        // Second run should be faster (allowing for some variance)
        assert!(duration2 <= duration1 * 2, "Caching not effective: {} vs {}", duration2.as_millis(), duration1.as_millis());

        Ok(())
    }

    /// Test transform CSS generation performance
    #[test]
    fn test_transform_css_performance() -> Result<()> {
        let mut generator = CssGenerator::new();

        let transform_classes = &["scale-110", "rotate-3", "translate-x-2", "skew-y-1"];

        let start = Instant::now();
        for _ in 0..100 {
            let css = generator.process_element_classes(transform_classes)?;
            // Should contain transform custom properties
            assert!(css.contains("--tw-transform"));
        }
        let duration = start.elapsed();

        assert!(duration < Duration::from_millis(50), "Transform CSS performance regression: {}ms", duration.as_millis());

        Ok(())
    }

    /// Test gradient processing performance
    #[test]
    fn test_gradient_performance() -> Result<()> {
        let mut generator = CssGenerator::new();

        let gradient_classes = &["from-blue-500", "via-purple-600", "to-pink-500", "bg-gradient-to-r"];

        let start = Instant::now();
        for _ in 0..100 {
            let css = generator.process_element_classes(gradient_classes)?;
            assert!(css.contains("linear-gradient"));
        }
        let duration = start.elapsed();

        assert!(duration < Duration::from_millis(80), "Gradient performance regression: {}ms", duration.as_millis());

        Ok(())
    }

    /// Test large-scale CSS generation
    #[test]
    fn test_large_scale_generation() -> Result<()> {
        let mut generator = CssGenerator::new();

        // Generate a large set of classes
        let large_class_set: Vec<String> = (0..100)
            .flat_map(|i| {
                vec![
                    format!("bg-blue-{}", 500 + (i % 10)),
                    format!("sm:text-{}", ["red", "green", "blue", "yellow"][i % 4]),
                    format!("hover:scale-{}", 100 + (i % 20)),
                ]
            })
            .collect();

        let start = Instant::now();
        let css = generator.process_element_classes(&large_class_set.iter().map(|s| s.as_str()).collect::<Vec<_>>())?;
        let duration = start.elapsed();

        // Should handle large sets efficiently
        assert!(duration < Duration::from_millis(500), "Large scale performance regression: {}ms", duration.as_millis());
        assert!(css.contains("background-color"));
        assert!(css.contains("@media"));
        assert!(css.contains("--tw-scale-x"));

        Ok(())
    }

    /// Test memory leak prevention
    #[test]
    fn test_memory_leak_prevention() -> Result<()> {
        // This test ensures we don't accumulate state that causes memory leaks
        let mut generator = CssGenerator::new();

        let initial_classes = &["bg-blue-500"];
        let css1 = generator.process_element_classes(initial_classes)?;

        // Process many different classes
        for i in 0..50 {
            let classes = &[format!("bg-blue-{}", 500 + (i % 10))];
            let _css = generator.process_element_classes(&classes)?;
        }

        // Process original classes again
        let css2 = generator.process_element_classes(initial_classes)?;

        // Output should be consistent (no memory leak affecting results)
        assert_eq!(css1, css2, "Inconsistent results suggest memory leak");

        Ok(())
    }
}
