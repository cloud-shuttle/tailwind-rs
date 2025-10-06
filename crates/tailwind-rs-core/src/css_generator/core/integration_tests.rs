//! Integration tests for CSS generation
//!
//! These tests validate end-to-end CSS generation functionality
//! including advanced variants and CSS functions.

#[cfg(test)]
mod tests {
    use super::super::CssGenerator;
    use crate::error::Result;

    /// Test basic CSS generation
    #[test]
    fn test_basic_css_generation() -> Result<()> {
        let mut generator = CssGenerator::new();

        // Test basic utility classes
        let css = generator.process_element_classes(&["bg-blue-500", "text-white", "p-4"])?;

        assert!(css.contains(".bg-blue-500"));
        assert!(css.contains("background-color"));
        assert!(css.contains(".text-white"));
        assert!(css.contains("color"));
        assert!(css.contains(".p-4"));
        assert!(css.contains("padding"));

        Ok(())
    }

    /// Test responsive variants
    #[test]
    fn test_responsive_variants() -> Result<()> {
        let mut generator = CssGenerator::new();

        let css = generator.process_element_classes(&["sm:bg-red-500", "md:text-green-600", "lg:p-8"])?;

        // Should contain media queries
        assert!(css.contains("@media"));
        assert!(css.contains("min-width"));
        assert!(css.contains(".sm\\:bg-red-500"));
        assert!(css.contains(".md\\:text-green-600"));
        assert!(css.contains(".lg\\:p-8"));

        Ok(())
    }

    /// Test state variants (hover, focus, etc.)
    #[test]
    fn test_state_variants() -> Result<()> {
        let mut generator = CssGenerator::new();

        let css = generator.process_element_classes(&["hover:bg-blue-600", "focus:text-red-500", "active:scale-110"])?;

        assert!(css.contains(".hover\\:bg-blue-600:hover"));
        assert!(css.contains(".focus\\:text-red-500:focus"));
        assert!(css.contains(".active\\:scale-110:active"));

        Ok(())
    }

    /// Test container queries
    #[test]
    fn test_container_queries() -> Result<()> {
        let mut generator = CssGenerator::new();

        let css = generator.process_element_classes(&["@container-sm:bg-green-500", "@container-md:min-w-300:text-blue-600"])?;

        assert!(css.contains("@container"));
        assert!(css.contains(".\\@container-sm\\:bg-green-500"));
        assert!(css.contains(".\\@container-md\\:min-w-300\\:text-blue-600"));

        Ok(())
    }

    /// Test arbitrary value variants
    #[test]
    fn test_arbitrary_value_variants() -> Result<()> {
        let mut generator = CssGenerator::new();

        let css = generator.process_element_classes(&["[data-state=\"open\"]:bg-green-500", "[aria-expanded=\"true\"]:text-blue-600"])?;

        assert!(css.contains("[data-state=\"open\"]"));
        assert!(css.contains("[aria-expanded=\"true\"]"));
        assert!(css.contains(".\\[data-state\\=\\\"open\\\"\\]\\:bg-green-500"));
        assert!(css.contains(".\\[aria-expanded\\=\\\"true\\\"\\]\\:text-blue-600"));

        Ok(())
    }

    /// Test device-specific variants
    #[test]
    fn test_device_variants() -> Result<()> {
        let mut generator = CssGenerator::new();

        let css = generator.process_element_classes(&["motion-reduce:opacity-50", "contrast-more:text-lg", "pointer-coarse:p-6"])?;

        assert!(css.contains("@media (prefers-reduced-motion: reduce)"));
        assert!(css.contains("@media (prefers-contrast: more)"));
        assert!(css.contains("@media (pointer: coarse)"));
        assert!(css.contains(".motion-reduce\\:opacity-50"));
        assert!(css.contains(".contrast-more\\:text-lg"));
        assert!(css.contains(".pointer-coarse\\:p-6"));

        Ok(())
    }

    /// Test transform combinations
    #[test]
    fn test_transform_combinations() -> Result<()> {
        let mut generator = CssGenerator::new();

        let css = generator.process_element_classes(&["scale-110", "rotate-3", "translate-x-2"])?;

        // Should include transform CSS custom properties
        assert!(css.contains("--tw-scale-x"));
        assert!(css.contains("--tw-rotate"));
        assert!(css.contains("--tw-translate-x"));
        assert!(css.contains("transform: var(--tw-transform)"));
        assert!(css.contains(".scale-110"));
        assert!(css.contains(".rotate-3"));
        assert!(css.contains(".translate-x-2"));

        Ok(())
    }

    /// Test gradient handling
    #[test]
    fn test_gradient_handling() -> Result<()> {
        let mut generator = CssGenerator::new();

        let css = generator.process_element_classes(&["from-blue-500", "via-purple-600", "to-pink-500", "bg-gradient-to-r"])?;

        assert!(css.contains("background-image"));
        assert!(css.contains("linear-gradient"));
        assert!(css.contains(".from-blue-500"));
        assert!(css.contains(".via-purple-600"));
        assert!(css.contains(".to-pink-500"));
        assert!(css.contains(".bg-gradient-to-r"));

        Ok(())
    }

    /// Test CSS functions (@apply, @layer, @import)
    #[test]
    fn test_css_functions() -> Result<()> {
        let mut generator = CssGenerator::new();

        // Test @apply directive processing
        let apply_result = generator.process_apply_directive("bg-blue-500 text-white p-4")?;
        assert!(apply_result.contains("background-color"));
        assert!(apply_result.contains("color"));
        assert!(apply_result.contains("padding"));

        // Test @layer directive processing
        let layer_result = generator.process_layer_directive("components", ".btn { color: blue; }")?;
        assert!(layer_result.contains("@layer components"));
        assert!(layer_result.contains(".btn { color: blue; }"));

        // Test @import directive processing
        let import_result = generator.process_import_directive("url('https://fonts.googleapis.com/css2?family=Inter&display=swap')")?;
        assert!(import_result.contains("@import"));
        assert!(import_result.contains("fonts.googleapis.com"));

        Ok(())
    }

    /// Test complex element combinations
    #[test]
    fn test_complex_element_combinations() -> Result<()> {
        let mut generator = CssGenerator::new();

        let css = generator.process_element_classes(&[
            "bg-blue-500", "hover:bg-blue-600", "sm:text-white", "md:p-8",
            "@container-sm:bg-green-500", "motion-reduce:opacity-75",
            "scale-110", "rotate-3"
        ])?;

        // Should contain all types of selectors and rules
        assert!(css.contains(".bg-blue-500"));
        assert!(css.contains(".hover\\:bg-blue-600:hover"));
        assert!(css.contains("@media (min-width: 640px)"));
        assert!(css.contains("@container"));
        assert!(css.contains("@media (prefers-reduced-motion: reduce)"));
        assert!(css.contains("--tw-scale-x"));
        assert!(css.contains("--tw-rotate"));

        Ok(())
    }

    /// Test error handling
    #[test]
    fn test_error_handling() -> Result<()> {
        let mut generator = CssGenerator::new();

        // Test invalid class in @apply
        let result = generator.process_apply_directive("nonexistent-class");
        assert!(result.is_err());

        // Test empty import
        let result = generator.process_import_directive("");
        assert!(result.is_err());

        Ok(())
    }

    /// Test performance with large class sets
    #[test]
    fn test_large_class_set_performance() -> Result<()> {
        let mut generator = CssGenerator::new();

        let classes = (0..100)
            .map(|i| format!("bg-blue-{}", 500 + (i % 10)))
            .collect::<Vec<_>>();

        let css = generator.process_element_classes(&classes)?;

        // Should generate CSS for all classes
        assert!(css.contains("background-color"));
        assert!(css.lines().count() > 50); // Reasonable amount of CSS generated

        Ok(())
    }
}
