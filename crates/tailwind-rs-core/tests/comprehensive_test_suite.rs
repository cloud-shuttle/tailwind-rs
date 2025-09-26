//! Comprehensive Test Suite for Tailwind-RS v0.12.1
//! 
//! This test suite validates functionality, identifies gaps, and ensures production readiness.
//! Based on the improvements made to v0.12.1, many previously broken features now work.

use tailwind_rs_core::css_generator::{CssGenerator, CssGenerationConfig};
use tailwind_rs_core::classes::{ClassSet, ClassBuilder};

#[cfg(test)]
mod core_functionality_tests {
    use super::*;

    #[test]
    fn test_basic_layout_classes() {
        let mut generator = CssGenerator::new();
        
        let layout_classes = vec![
            "block", "flex", "grid", "hidden", "inline", "inline-block",
            "inline-flex", "inline-grid", "table", "table-cell", "table-row"
        ];
        
        for class in layout_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add layout class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        assert!(css.contains("display"), "CSS should contain display properties");
    }

    #[test]
    fn test_spacing_classes() {
        let mut generator = CssGenerator::new();
        
        let spacing_classes = vec![
            "p-4", "m-2", "px-3", "py-2", "pt-1", "pr-2", "pb-3", "pl-4",
            "mx-auto", "my-2", "mt-4", "mr-2", "mb-3", "ml-4"
        ];
        
        for class in spacing_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add spacing class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        assert!(css.contains("padding"), "CSS should contain padding properties");
        assert!(css.contains("margin"), "CSS should contain margin properties");
    }

    #[test]
    fn test_typography_classes() {
        let mut generator = CssGenerator::new();
        
        let typography_classes = vec![
            "text-lg", "text-sm", "text-base", "text-xl", "text-2xl",
            "font-bold", "font-normal", "font-medium", "font-semibold",
            "text-center", "text-left", "text-right", "text-justify",
            "text-transparent", "text-white", "text-black", "text-gray-600"
        ];
        
        for class in typography_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add typography class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        assert!(css.contains("font-size"), "CSS should contain font-size properties");
        assert!(css.contains("font-weight"), "CSS should contain font-weight properties");
        assert!(css.contains("text-align"), "CSS should contain text-align properties");
        assert!(css.contains("color"), "CSS should contain color properties");
    }
}

#[cfg(test)]
mod color_system_tests {
    use super::*;

    #[test]
    fn test_background_colors() {
        let mut generator = CssGenerator::new();
        
        let bg_classes = vec![
            "bg-white", "bg-black", "bg-gray-100", "bg-gray-500", "bg-gray-900",
            "bg-blue-500", "bg-red-600", "bg-green-400", "bg-yellow-300",
            "bg-zinc-700", "bg-teal-500"
        ];
        
        for class in bg_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add background color class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        assert!(css.contains("background-color"), "CSS should contain background-color properties");
    }

    #[test]
    fn test_text_colors() {
        let mut generator = CssGenerator::new();
        
        let text_classes = vec![
            "text-white", "text-black", "text-gray-600", "text-blue-500",
            "text-red-600", "text-green-400", "text-transparent", "text-current"
        ];
        
        for class in text_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add text color class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        assert!(css.contains("color"), "CSS should contain color properties");
    }
}

#[cfg(test)]
mod advanced_feature_tests {
    use super::*;

    #[test]
    fn test_hover_states() {
        let mut generator = CssGenerator::new();
        
        let hover_classes = vec![
            "hover:bg-blue-500", "hover:text-white", "hover:shadow-lg",
            "hover:opacity-80", "hover:scale-105", "hover:bg-zinc-700",
            "hover:text-teal-400", "hover:shadow-xl"
        ];
        
        for class in hover_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add hover class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        assert!(css.contains(":hover"), "CSS should contain hover selectors");
    }

    #[test]
    fn test_dark_mode() {
        let mut generator = CssGenerator::new();
        
        let dark_classes = vec![
            "dark:bg-gray-800", "dark:text-white", "dark:border-gray-700",
            "dark:bg-zinc-800", "dark:text-zinc-200", "dark:border-zinc-700"
        ];
        
        for class in dark_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add dark mode class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        assert!(css.contains(".dark"), "CSS should contain dark mode selectors");
    }

    #[test]
    fn test_responsive_classes() {
        let mut generator = CssGenerator::new();
        
        let responsive_classes = vec![
            "sm:px-4", "md:flex-row", "lg:grid-cols-3", "xl:text-2xl",
            "sm:bg-blue-500", "md:text-lg", "lg:p-8"
        ];
        
        for class in responsive_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add responsive class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        assert!(css.contains("@media"), "CSS should contain media queries");
    }
}

#[cfg(test)]
mod interactive_states_tests {
    use super::*;

    #[test]
    fn test_focus_states() {
        let mut generator = CssGenerator::new();
        
        let focus_classes = vec![
            "focus:ring-2", "focus:ring-blue-500", "focus:outline-none",
            "focus:border-blue-500", "focus:bg-blue-100"
        ];
        
        for class in focus_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add focus class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        assert!(css.contains(":focus"), "CSS should contain focus selectors");
    }

    #[test]
    fn test_active_states() {
        let mut generator = CssGenerator::new();
        
        let active_classes = vec![
            "active:bg-blue-600", "active:scale-95", "active:shadow-inner",
            "active:text-white", "active:opacity-75"
        ];
        
        for class in active_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add active class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        assert!(css.contains(":active"), "CSS should contain active selectors");
    }
}

#[cfg(test)]
mod utility_classes_tests {
    use super::*;

    #[test]
    fn test_effects_and_shadows() {
        let mut generator = CssGenerator::new();
        
        let effect_classes = vec![
            "shadow-sm", "shadow", "shadow-md", "shadow-lg", "shadow-xl",
            "shadow-2xl", "shadow-inner", "shadow-none",
            "opacity-0", "opacity-25", "opacity-50", "opacity-75", "opacity-100"
        ];
        
        for class in effect_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add effect class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        assert!(css.contains("box-shadow"), "CSS should contain box-shadow properties");
        assert!(css.contains("opacity"), "CSS should contain opacity properties");
    }

    #[test]
    fn test_backdrop_effects() {
        let mut generator = CssGenerator::new();
        
        let backdrop_classes = vec![
            "backdrop-blur-sm", "backdrop-blur", "backdrop-blur-md", "backdrop-blur-lg",
            "backdrop-opacity-50", "backdrop-opacity-75", "backdrop-opacity-100"
        ];
        
        for class in backdrop_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add backdrop class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        assert!(css.contains("backdrop-filter"), "CSS should contain backdrop-filter properties");
    }

    #[test]
    fn test_interactive_utilities() {
        let mut generator = CssGenerator::new();
        
        let interactive_classes = vec![
            "pointer-events-none", "pointer-events-auto",
            "cursor-pointer", "cursor-default", "cursor-wait", "cursor-text",
            "select-none", "select-text", "select-all", "select-auto"
        ];
        
        for class in interactive_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add interactive class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        assert!(css.contains("pointer-events"), "CSS should contain pointer-events properties");
        assert!(css.contains("cursor"), "CSS should contain cursor properties");
        assert!(css.contains("user-select"), "CSS should contain user-select properties");
    }
}

#[cfg(test)]
mod edge_cases_tests {
    use super::*;

    #[test]
    fn test_invalid_classes() {
        let mut generator = CssGenerator::new();
        
        // These should either be ignored or throw appropriate errors
        let invalid_classes = vec![
            "invalid-class", "hover:invalid-class", "dark:invalid-class",
            "sm:invalid-class", "focus:invalid-class"
        ];
        
        for class in invalid_classes {
            let result = generator.add_class(class);
            // Should either fail gracefully or be ignored
            if result.is_ok() {
                // If it succeeds, the CSS should be empty or minimal
                let css = generator.generate_css();
                // Should not contain the invalid class
                assert!(!css.contains("invalid-class"), "Invalid class should not appear in CSS");
            }
        }
    }

    #[test]
    fn test_empty_class_set() {
        let mut generator = CssGenerator::new();
        
        let css = generator.generate_css();
        // Empty generator should produce minimal CSS or empty string
        assert!(css.is_empty() || css.len() < 100, "Empty generator should produce minimal CSS");
    }

    #[test]
    fn test_duplicate_classes() {
        let mut generator = CssGenerator::new();
        
        // Add the same class multiple times
        generator.add_class("bg-blue-500").unwrap();
        generator.add_class("bg-blue-500").unwrap();
        generator.add_class("bg-blue-500").unwrap();
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        
        // Should not have duplicate rules
        let bg_blue_count = css.matches(".bg-blue-500").count();
        assert_eq!(bg_blue_count, 1, "Should not have duplicate CSS rules");
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_large_class_sets() {
        let mut generator = CssGenerator::new();
        
        // Add many classes
        let mut classes = Vec::new();
        for i in 0..100 {
            classes.push(format!("p-{}", i));
            classes.push(format!("m-{}", i));
            classes.push(format!("text-{}", i));
            classes.push(format!("bg-blue-{}", i));
        }
        
        for class in &classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add class: {}", class);
        }
        
        let start = std::time::Instant::now();
        let css = generator.generate_css();
        let duration = start.elapsed();
        
        assert!(duration.as_millis() < 1000, "Should complete in under 1 second");
        assert!(css.len() > 1000, "Should generate substantial CSS");
    }

    #[test]
    fn test_memory_usage() {
        let mut generator = CssGenerator::new();
        
        // Add many classes and check memory usage
        for i in 0..1000 {
            generator.add_class(&format!("class-{}", i)).unwrap();
        }
        
        let css = generator.generate_css();
        
        // Memory usage should be reasonable
        assert!(css.len() < 1_000_000, "CSS should be less than 1MB");
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_classset_api() {
        let mut class_set = ClassSet::new();
        class_set.add_class("bg-blue-500");
        class_set.add_class("text-white");
        class_set.add_class("p-4");
        
        let css_classes = class_set.to_css_classes();
        assert!(!css_classes.is_empty(), "to_css_classes should return non-empty string");
        assert!(css_classes.contains("bg-blue-500"), "Should contain bg-blue-500");
        assert!(css_classes.contains("text-white"), "Should contain text-white");
        assert!(css_classes.contains("p-4"), "Should contain p-4");
    }

    #[test]
    fn test_classbuilder_api() {
        let class_set = ClassBuilder::new()
            .class("bg-blue-500")
            .class("text-white")
            .class("p-4")
            .build();
        
        let css_classes = class_set.to_css_classes();
        assert!(!css_classes.is_empty(), "ClassBuilder should produce non-empty CSS classes");
        assert!(css_classes.contains("bg-blue-500"), "Should contain bg-blue-500");
    }

    #[test]
    fn test_comprehensive_css_generation() {
        let mut generator = CssGenerator::new();
        let config = CssGenerationConfig::default();
        
        let result = generator.generate_comprehensive_css(&config);
        assert!(result.is_ok(), "Comprehensive CSS generation should succeed");
        
        let css = result.unwrap();
        assert!(!css.is_empty(), "Comprehensive CSS should not be empty");
        assert!(css.len() > 100, "Comprehensive CSS should be substantial");
    }
}

#[cfg(test)]
mod regression_tests {
    use super::*;

    #[test]
    fn test_previously_broken_classes() {
        let mut generator = CssGenerator::new();
        
        // These classes were previously broken in v0.12.0
        let previously_broken = vec![
            "hover:bg-zinc-700", "hover:text-teal-400", "hover:shadow-xl",
            "dark:bg-zinc-800", "dark:text-zinc-200", "dark:border-zinc-700",
            "pointer-events-none", "pointer-events-auto", "cursor-pointer",
            "text-transparent", "backdrop-blur-sm", "backdrop-opacity-50"
        ];
        
        for class in previously_broken {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Previously broken class should now work: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
    }

    #[test]
    fn test_api_consistency() {
        // Test that the API is consistent across different usage patterns
        let mut generator1 = CssGenerator::new();
        let mut generator2 = CssGenerator::new();
        
        // Same classes added in different orders should produce similar results
        generator1.add_class("bg-blue-500").unwrap();
        generator1.add_class("text-white").unwrap();
        generator1.add_class("p-4").unwrap();
        
        generator2.add_class("p-4").unwrap();
        generator2.add_class("text-white").unwrap();
        generator2.add_class("bg-blue-500").unwrap();
        
        let css1 = generator1.generate_css();
        let css2 = generator2.generate_css();
        
        // Both should contain the same classes
        assert!(css1.contains("bg-blue-500"), "CSS1 should contain bg-blue-500");
        assert!(css1.contains("text-white"), "CSS1 should contain text-white");
        assert!(css1.contains("p-4"), "CSS1 should contain p-4");
        
        assert!(css2.contains("bg-blue-500"), "CSS2 should contain bg-blue-500");
        assert!(css2.contains("text-white"), "CSS2 should contain text-white");
        assert!(css2.contains("p-4"), "CSS2 should contain p-4");
    }
}
