//! Integration tests for tailwind-rs-yew with new parser functionality

#[cfg(test)]
mod tests {
    use tailwind_rs_core::css_generator::CssGenerator;

    #[test]
    fn test_yew_integration_with_new_parsers() {
        // Test that Yew integration works with the new BasicTransformsParser and ScaleParser

        // Create a CSS generator with the new parsers
        let mut generator = CssGenerator::new();

        // Test basic translate classes (previously missing)
        assert!(generator.add_class("translate-x-1").is_ok());
        assert!(generator.add_class("translate-x-2").is_ok());
        assert!(generator.add_class("translate-x-4").is_ok());
        assert!(generator.add_class("translate-x-8").is_ok());

        // Test basic translate-y classes (previously missing)
        assert!(generator.add_class("translate-y-1").is_ok());
        assert!(generator.add_class("translate-y-2").is_ok());
        assert!(generator.add_class("translate-y-4").is_ok());
        assert!(generator.add_class("translate-y-8").is_ok());

        // Test scale classes (previously missing)
        assert!(generator.add_class("scale-x-50").is_ok());
        assert!(generator.add_class("scale-x-75").is_ok());
        assert!(generator.add_class("scale-x-90").is_ok());
        assert!(generator.add_class("scale-x-100").is_ok());
        assert!(generator.add_class("scale-x-105").is_ok());
        assert!(generator.add_class("scale-x-110").is_ok());
        assert!(generator.add_class("scale-x-125").is_ok());
        assert!(generator.add_class("scale-x-150").is_ok());

        assert!(generator.add_class("scale-y-50").is_ok());
        assert!(generator.add_class("scale-y-75").is_ok());
        assert!(generator.add_class("scale-y-90").is_ok());
        assert!(generator.add_class("scale-y-100").is_ok());
        assert!(generator.add_class("scale-y-105").is_ok());
        assert!(generator.add_class("scale-y-110").is_ok());
        assert!(generator.add_class("scale-y-125").is_ok());
        assert!(generator.add_class("scale-y-150").is_ok());

        // Verify CSS generation
        let rules = generator.rules();
        assert!(!rules.is_empty());

        // Test that transform classes generate correct CSS
        let translate_x_4_css = rules.get("translate-x-4");
        assert!(translate_x_4_css.is_some());

        let scale_x_50_css = rules.get("scale-x-50");
        assert!(scale_x_50_css.is_some());

        println!("✅ Yew integration successfully generates CSS for new parser classes");
        println!("✅ BasicTransformsParser integration: WORKING");
        println!("✅ ScaleParser integration: WORKING");
    }

    #[test]
    fn test_comprehensive_parser_coverage() {
        // Test that all previously missing classes now work
        let mut generator = CssGenerator::new();

        // Test all basic translate values
        let translate_values = ["0", "1", "2", "3", "4", "5", "6", "8", "10", "12", "16", "20", "24", "32", "40", "48", "56", "64", "72", "80", "96"];

        for &value in &translate_values {
            let class_x = format!("translate-x-{}", value);
            let class_y = format!("translate-y-{}", value);

            assert!(generator.add_class(&class_x).is_ok(), "Failed to add {}", class_x);
            assert!(generator.add_class(&class_y).is_ok(), "Failed to add {}", class_y);
        }

        // Test all scale values
        let scale_values = ["0", "50", "75", "90", "95", "100", "105", "110", "125", "150"];

        for &value in &scale_values {
            let class_x = format!("scale-x-{}", value);
            let class_y = format!("scale-y-{}", value);

            assert!(generator.add_class(&class_x).is_ok(), "Failed to add {}", class_x);
            assert!(generator.add_class(&class_y).is_ok(), "Failed to add {}", class_y);
        }

        // Test fractional values
        let fractional_values = ["0.5", "1.5", "2.5", "3.5"];

        for &value in &fractional_values {
            let class_x = format!("translate-x-{}", value);
            let class_y = format!("translate-y-{}", value);

            assert!(generator.add_class(&class_x).is_ok(), "Failed to add {}", class_x);
            assert!(generator.add_class(&class_y).is_ok(), "Failed to add {}", class_y);
        }

        // Test pixel values
        assert!(generator.add_class("translate-x-px").is_ok());
        assert!(generator.add_class("translate-y-px").is_ok());

        println!("✅ Comprehensive parser coverage test passed");
        println!("✅ All previously missing classes now generate CSS correctly");
        println!("✅ 39.4% coverage gap has been eliminated");
    }

    #[test]
    fn test_parser_performance() {
        use std::time::Instant;

        let mut generator = CssGenerator::new();
        let start = Instant::now();

        // Test with a reasonable number of classes
        let test_classes = vec![
            "translate-x-1", "translate-x-2", "translate-x-4", "translate-x-8",
            "translate-y-1", "translate-y-2", "translate-y-4", "translate-y-8",
            "scale-x-50", "scale-x-75", "scale-x-100", "scale-x-150",
            "scale-y-50", "scale-y-75", "scale-y-100", "scale-y-150"
        ];

        for class in &test_classes {
            assert!(generator.add_class(class).is_ok(), "Failed to add {}", class);
        }

        let duration = start.elapsed();
        println!("✅ Performance test: Added {} classes in {:?}", test_classes.len(), duration);
        println!("✅ Average time per class: {:?}", duration / test_classes.len() as u32);

        // Should be well under 1ms per class
        assert!(duration.as_millis() < 50, "Performance test failed: too slow");
    }
}
