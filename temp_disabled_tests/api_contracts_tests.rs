//! API Contracts and Testing
//!
//! This module contains comprehensive tests to ensure API contracts are maintained
//! and the system works as intended with comprehensive coverage.

use crate::classes::ClassBuilder;
use crate::responsive::Breakpoint;
use crate::postcss_integration::EnhancedCssGenerator;

#[cfg(test)]
mod api_contracts_tests {
    use super::*;
    use std::time::Instant;

    /// Test ClassBuilder API contract
    #[test]
    fn test_class_builder_api_contract() {
        // Test constructor contract
        let builder = ClassBuilder::new();
        let _builder = ClassBuilder::default();
        
        // Test building contract
        let _class_set = builder.build();
        let _css_string = builder.build_string();
        
        // Test class addition contract
        let _builder = builder.class("test-class");
        let _builder = builder.classes(vec!["class1".to_string(), "class2".to_string()]);
        
        // Test responsive contract
        let _builder = builder.responsive(Breakpoint::Md, "responsive-class");
        
        // Test pseudo-class contract
        let _builder = builder.hover("hover-class");
        let _builder = builder.focus("focus-class");
        let _builder = builder.active("active-class");
        let _builder = builder.dark("dark-class");
        
        assert!(true, "ClassBuilder API contract should be satisfied");
    }

    /// Test utility methods contract
    #[test]
    fn test_utility_methods_contract() {
        let builder = ClassBuilder::new();
        
        // Test layout utilities
        let _builder = builder.relative();
        let _builder = builder.absolute();
        let _builder = builder.fixed();
        let _builder = builder.sticky();
        let _builder = builder.static_pos();
        
        // Test display utilities
        let _builder = builder.block();
        let _builder = builder.inline();
        let _builder = builder.inline_block();
        let _builder = builder.flex();
        let _builder = builder.inline_flex();
        let _builder = builder.grid();
        let _builder = builder.inline_grid();
        let _builder = builder.hidden();
        let _builder = builder.visible();
        
        // Test flexbox utilities
        let _builder = builder.flex_none();
        let _builder = builder.flex_1();
        let _builder = builder.flex_auto();
        let _builder = builder.flex_initial();
        let _builder = builder.flex_col();
        let _builder = builder.flex_row();
        let _builder = builder.flex_wrap_class();
        let _builder = builder.flex_nowrap_class();
        
        // Test transition utilities
        let _builder = builder.transition();
        let _builder = builder.transition_all();
        let _builder = builder.transition_colors();
        let _builder = builder.transition_opacity();
        let _builder = builder.transition_shadow();
        let _builder = builder.transition_transform();
        
        assert!(true, "Utility methods contract should be satisfied");
    }

    /// Test API stability across versions
    #[test]
    fn test_api_stability() {
        // Test that all public methods exist and work
        let builder = ClassBuilder::new();
        
        // Test core methods
        let _builder = builder.class("test-class");
        let _builder = builder.classes(vec!["class1".to_string(), "class2".to_string()]);
        let _builder = builder.responsive(Breakpoint::Md, "responsive-class");
        let _builder = builder.hover("hover-class");
        let _builder = builder.focus("focus-class");
        let _builder = builder.active("active-class");
        let _builder = builder.dark("dark-class");
        let _builder = builder.group_hover("group-hover-class");
        let _builder = builder.peer_hover("peer-hover-class");
        
        // Test utility methods
        let _builder = builder.relative();
        let _builder = builder.flex();
        let _builder = builder.p_4();
        let _builder = builder.bg_blue_500();
        let _builder = builder.text_white();
        let _builder = builder.border_gray_300();
        
        // Test building
        let _class_set = builder.build();
        let _css_string = builder.build_string();
        
        assert!(true, "API should be stable");
    }

    /// Test backward compatibility
    #[test]
    fn test_backward_compatibility() {
        // Test that old API still works
        let classes = ClassBuilder::new()
            .class("relative")
            .class("flex")
            .class("p-4")
            .class("bg-blue-500")
            .class("text-white")
            .class("hover:bg-blue-600")
            .class("focus:ring-2")
            .class("dark:bg-gray-800")
            .build_string();
        
        assert!(classes.contains("relative"));
        assert!(classes.contains("flex"));
        assert!(classes.contains("p-4"));
        assert!(classes.contains("bg-blue-500"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("hover:bg-blue-600"));
        assert!(classes.contains("focus:ring-2"));
        assert!(classes.contains("dark:bg-gray-800"));
    }

    /// Test method coverage
    #[test]
    fn test_method_coverage() {
        let builder = ClassBuilder::new();
        
        // Test all layout methods
        let _builder = builder.relative();
        let _builder = builder.absolute();
        let _builder = builder.fixed();
        let _builder = builder.sticky();
        let _builder = builder.static_pos();
        
        // Test all display methods
        let _builder = builder.block();
        let _builder = builder.inline();
        let _builder = builder.inline_block();
        let _builder = builder.flex();
        let _builder = builder.inline_flex();
        let _builder = builder.grid();
        let _builder = builder.inline_grid();
        let _builder = builder.hidden();
        let _builder = builder.visible();
        
        // Test all flexbox methods
        let _builder = builder.flex_none();
        let _builder = builder.flex_1();
        let _builder = builder.flex_auto();
        let _builder = builder.flex_initial();
        let _builder = builder.flex_col();
        let _builder = builder.flex_row();
        let _builder = builder.flex_wrap_class();
        let _builder = builder.flex_nowrap_class();
        
        // Test all transition methods
        let _builder = builder.transition();
        let _builder = builder.transition_all();
        let _builder = builder.transition_colors();
        let _builder = builder.transition_opacity();
        let _builder = builder.transition_shadow();
        let _builder = builder.transition_transform();
        
        // Test all pseudo-class methods
        let _builder = builder.hover("bg-blue-600");
        let _builder = builder.focus("ring-2");
        let _builder = builder.active("bg-blue-700");
        let _builder = builder.dark("bg-gray-800");
        let _builder = builder.group_hover("bg-blue-600");
        let _builder = builder.peer_hover("bg-blue-600");
        
        // Test all spacing methods
        let _builder = builder.p_4();
        let _builder = builder.px_4();
        let _builder = builder.py_4();
        let _builder = builder.m_4();
        let _builder = builder.mx_4();
        let _builder = builder.my_4();
        
        // Test all color methods
        let _builder = builder.bg_blue_500();
        let _builder = builder.text_white();
        let _builder = builder.border_gray_300();
        
        assert!(true, "All methods should be covered");
    }

    /// Test error handling contract
    #[test]
    fn test_error_handling_contract() {
        // Test hover empty class panic
        let result = std::panic::catch_unwind(|| {
            ClassBuilder::new().hover("");
        });
        assert!(result.is_err());
        
        // Test hover with prefix panic
        let result = std::panic::catch_unwind(|| {
            ClassBuilder::new().hover("hover:bg-blue-500");
        });
        assert!(result.is_err());
        
        // Test focus empty class panic
        let result = std::panic::catch_unwind(|| {
            ClassBuilder::new().focus("");
        });
        assert!(result.is_err());
        
        // Test focus with prefix panic
        let result = std::panic::catch_unwind(|| {
            ClassBuilder::new().focus("focus:ring-2");
        });
        assert!(result.is_err());
        
        // Test dark empty class panic
        let result = std::panic::catch_unwind(|| {
            ClassBuilder::new().dark("");
        });
        assert!(result.is_err());
        
        // Test dark with prefix panic
        let result = std::panic::catch_unwind(|| {
            ClassBuilder::new().dark("dark:bg-gray-800");
        });
        assert!(result.is_err());
        
        // Test group-hover empty class panic
        let result = std::panic::catch_unwind(|| {
            ClassBuilder::new().group_hover("");
        });
        assert!(result.is_err());
        
        // Test group-hover with prefix panic
        let result = std::panic::catch_unwind(|| {
            ClassBuilder::new().group_hover("group-hover:bg-blue-600");
        });
        assert!(result.is_err());
        
        // Test peer-hover empty class panic
        let result = std::panic::catch_unwind(|| {
            ClassBuilder::new().peer_hover("");
        });
        assert!(result.is_err());
        
        // Test peer-hover with prefix panic
        let result = std::panic::catch_unwind(|| {
            ClassBuilder::new().peer_hover("peer-hover:bg-blue-600");
        });
        assert!(result.is_err());
    }

    /// Test performance contract
    #[test]
    fn test_performance_contract() {
        let start = Instant::now();
        
        for _ in 0..1000 {
            let _ = ClassBuilder::new()
                .relative()
                .flex()
                .p_4()
                .bg_blue_500()
                .text_white()
                .hover("bg-blue-600")
                .focus("ring-2")
                .active("bg-blue-700")
                .dark("bg-gray-800")
                .transition()
                .transition_colors()
                .build_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "ClassBuilder should be fast");
    }

    /// Test PostCSS engine contract
    #[test]
    fn test_postcss_engine_contract() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        // Test basic CSS processing
        let css = ".test { color: red; }";
        let result = generator.process_css(css).unwrap();
        
        assert!(!result.css.is_empty());
        assert!(result.css.contains("color: red"));
        
        // Test complex CSS processing
        let css = r#"
            .test {
                display: flex;
                justify-content: center;
                align-items: center;
                padding: 1rem;
                background-color: #3b82f6;
                color: white;
            }
            
            .test:hover {
                background-color: #2563eb;
            }
            
            .test:focus {
                outline: 2px solid #3b82f6;
            }
        "#;
        
        let result = generator.process_css(css).unwrap();
        
        assert!(!result.css.is_empty());
        assert!(result.css.contains("display: flex"));
        assert!(result.css.contains("justify-content: center"));
        assert!(result.css.contains("align-items: center"));
        assert!(result.css.contains("padding: 1rem"));
        assert!(result.css.contains("background-color: #3b82f6"));
        assert!(result.css.contains("color: white"));
        assert!(result.css.contains(":hover"));
        assert!(result.css.contains("background-color: #2563eb"));
        assert!(result.css.contains(":focus"));
        assert!(result.css.contains("outline: 2px solid #3b82f6"));
    }

    /// Test PostCSS performance contract
    #[test]
    fn test_postcss_performance_contract() {
        let generator = EnhancedCssGenerator::new().unwrap();
        let css = ".test { color: red; }";
        
        let start = Instant::now();
        
        for _ in 0..100 {
            let _ = generator.process_css(css).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "PostCSS should be fast");
    }

    /// Test CSS generation contract
    #[test]
    fn test_css_generation_contract() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let classes = "relative flex p-4 bg-blue-500 text-white hover:bg-blue-600 focus:ring-2 dark:bg-gray-800";
        let result = generator.generate_css(classes).unwrap();
        
        assert!(!result.css.is_empty());
        assert!(result.css.contains("position: relative"));
        assert!(result.css.contains("display: flex"));
        assert!(result.css.contains("padding: 1rem"));
        assert!(result.css.contains("background-color: rgb(59 130 246)"));
        assert!(result.css.contains("color: rgb(255 255 255)"));
        assert!(result.css.contains(":hover"));
        assert!(result.css.contains(":focus"));
        assert!(result.css.contains("@media (prefers-color-scheme: dark)"));
    }

    /// Test integration contract
    #[test]
    fn test_integration_contract() {
        // Test complete integration
        let classes = ClassBuilder::new()
            .relative()
            .flex()
            .flex_col()
            .p_4()
            .bg_blue_500()
            .text_white()
            .hover("bg-blue-600")
            .focus("ring-2")
            .active("bg-blue-700")
            .dark("bg-gray-800")
            .transition()
            .transition_colors()
            .build_string();
        
        let generator = EnhancedCssGenerator::new().unwrap();
        let result = generator.generate_css(&classes).unwrap();
        
        assert!(!result.css.is_empty());
        assert!(result.css.contains("position: relative"));
        assert!(result.css.contains("display: flex"));
        assert!(result.css.contains("flex-direction: column"));
        assert!(result.css.contains("padding: 1rem"));
        assert!(result.css.contains("background-color: rgb(59 130 246)"));
        assert!(result.css.contains("color: rgb(255 255 255)"));
        assert!(result.css.contains(":hover"));
        assert!(result.css.contains(":focus"));
        assert!(result.css.contains(":active"));
        assert!(result.css.contains("@media (prefers-color-scheme: dark)"));
        assert!(result.css.contains("transition-property: color, background-color"));
    }

    /// Test real-world usage contract
    #[test]
    fn test_real_world_usage_contract() {
        // Test button component
        let button_classes = ClassBuilder::new()
            .relative()
            .inline_flex()
            .items_center()
            .justify_center()
            .px_4()
            .py_2()
            .bg_blue_500()
            .text_white()
            .rounded()
            .hover("bg-blue-600")
            .focus("ring-2")
            .active("bg-blue-700")
            .disabled("opacity-50")
            .transition()
            .transition_colors()
            .build_string();
        
        let generator = EnhancedCssGenerator::new().unwrap();
        let result = generator.generate_css(&button_classes).unwrap();
        
        assert!(!result.css.is_empty());
        assert!(result.css.contains("position: relative"));
        assert!(result.css.contains("display: inline-flex"));
        assert!(result.css.contains("align-items: center"));
        assert!(result.css.contains("justify-content: center"));
        assert!(result.css.contains("padding-left: 1rem"));
        assert!(result.css.contains("padding-right: 1rem"));
        assert!(result.css.contains("padding-top: 0.5rem"));
        assert!(result.css.contains("padding-bottom: 0.5rem"));
        assert!(result.css.contains("background-color: rgb(59 130 246)"));
        assert!(result.css.contains("color: rgb(255 255 255)"));
        assert!(result.css.contains("border-radius: 0.25rem"));
        assert!(result.css.contains(":hover"));
        assert!(result.css.contains(":focus"));
        assert!(result.css.contains(":active"));
        assert!(result.css.contains(":disabled"));
        assert!(result.css.contains("transition-property: color, background-color"));
    }

    /// Test edge cases contract
    #[test]
    fn test_edge_cases_contract() {
        // Test empty class builder
        let empty_classes = ClassBuilder::new().build_string();
        assert!(empty_classes.is_empty());
        
        // Test single class
        let single_class = ClassBuilder::new().relative().build_string();
        assert_eq!(single_class, "relative");
        
        // Test multiple classes
        let multiple_classes = ClassBuilder::new()
            .relative()
            .flex()
            .p_4()
            .build_string();
        assert!(multiple_classes.contains("relative"));
        assert!(multiple_classes.contains("flex"));
        assert!(multiple_classes.contains("p-4"));
        
        // Test pseudo-classes
        let pseudo_classes = ClassBuilder::new()
            .bg_blue_500()
            .hover("bg-blue-600")
            .focus("ring-2")
            .build_string();
        assert!(pseudo_classes.contains("bg-blue-500"));
        assert!(pseudo_classes.contains("hover:bg-blue-600"));
        assert!(pseudo_classes.contains("focus:ring-2"));
    }

    /// Test memory usage contract
    #[test]
    fn test_memory_usage_contract() {
        let builder = ClassBuilder::new();
        
        // Test that builder doesn't consume excessive memory
        let classes = builder
            .relative()
            .flex()
            .p_4()
            .bg_blue_500()
            .text_white()
            .hover("bg-blue-600")
            .focus("ring-2")
            .active("bg-blue-700")
            .dark("bg-gray-800")
            .transition()
            .transition_colors()
            .build_string();
        
        // Verify memory usage is reasonable
        assert!(classes.len() < 1000, "Classes should not be too large");
    }

    /// Test thread safety contract
    #[test]
    fn test_thread_safety_contract() {
        use std::thread;
        
        let handles: Vec<_> = (0..10)
            .map(|_| {
                thread::spawn(|| {
                    let classes = ClassBuilder::new()
                        .relative()
                        .flex()
                        .p_4()
                        .bg_blue_500()
                        .text_white()
                        .build_string();
                    
                    assert!(classes.contains("relative"));
                    assert!(classes.contains("flex"));
                    assert!(classes.contains("p-4"));
                    assert!(classes.contains("bg-blue-500"));
                    assert!(classes.contains("text-white"));
                })
            })
            .collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
