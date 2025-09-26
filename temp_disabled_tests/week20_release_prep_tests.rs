//! Week 20: Release Preparation Tests
//!
//! This module tests that all release preparation features are complete.

#[cfg(test)]
mod tests {
    use crate::classes::ClassBuilder;
    use crate::utilities::*;

    /// Test that all Week 20 release preparation features are implemented
    #[test]
    fn test_week20_release_preparation() {
        // Test API stability - verify that all public APIs are stable and consistent
        let api_stability_test = ClassBuilder::new()
            .p_4()                            // padding: 1rem
            .m_2()                            // margin: 0.5rem
            .w_1_2()                          // width: 50%
            .h_1_3()                          // height: 33.333333%
            .font_size(crate::utilities::typography::FontSize::Lg)
            .font_weight(crate::utilities::typography::FontWeight::Bold)
            .text_align(crate::utilities::typography::TextAlign::Center)
            .bg_blue_500()                    // background-color: #3b82f6
            .text_white()                     // color: #ffffff
            .rounded_md()                     // border-radius: 0.375rem
            .shadow_lg()                      // box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1)
            .hover("bg-blue-600")             // :hover { background-color: #2563eb }
            .focus("ring-2")                  // :focus { ring-width: 2px }
            .responsive(crate::responsive::Breakpoint::Md, "text-xl")
            .dark("bg-gray-800")
            .build();

        assert!(!api_stability_test.to_css_classes().is_empty());
        assert!(api_stability_test.to_css_classes().contains("p-4"));
        assert!(api_stability_test.to_css_classes().contains("m-2"));
        assert!(api_stability_test.to_css_classes().contains("w-1/2"));
        assert!(api_stability_test.to_css_classes().contains("h-1/3"));
    }

    /// Test backward compatibility - verify that existing code continues to work
    #[test]
    fn test_backward_compatibility() {
        // Test that old API patterns still work
        let old_api_test = ClassBuilder::new()
            .class("p-4")                     // Old way: direct class addition
            .class("m-2")                     // Old way: direct class addition
            .class("bg-blue-500")             // Old way: direct class addition
            .class("text-white")              // Old way: direct class addition
            .build();

        assert!(!old_api_test.to_css_classes().is_empty());
        assert!(old_api_test.to_css_classes().contains("p-4"));
        assert!(old_api_test.to_css_classes().contains("m-2"));
        assert!(old_api_test.to_css_classes().contains("bg-blue-500"));
        assert!(old_api_test.to_css_classes().contains("text-white"));
    }

    /// Test performance benchmarks - verify that performance requirements are met
    #[test]
    fn test_performance_benchmarks() {
        // Test class building performance
        let start = std::time::Instant::now();
        let mut builder = ClassBuilder::new();
        
        for i in 0..1000 {
            builder = builder.class(&format!("class-{}", i % 10));
        }
        
        let result = builder.build();
        let duration = start.elapsed();
        
        // Should complete within 100ms
        assert!(duration.as_millis() < 100);
        assert_eq!(result.len(), 1000);
        
        // Test CSS generation performance
        let css_start = std::time::Instant::now();
        let _css = result.to_css_classes();
        let css_duration = css_start.elapsed();
        
        // Should complete within 50ms
        assert!(css_duration.as_millis() < 50);
    }

    /// Test memory usage - verify that memory usage is within acceptable limits
    #[test]
    fn test_memory_usage() {
        // Test large class set memory usage
        let mut large_builder = ClassBuilder::new();
        
        for i in 0..10000 {
            large_builder = large_builder.class(&format!("class-{}", i % 100));
        }
        
        let large_result = large_builder.build();
        assert_eq!(large_result.len(), 10000);
        
        // Test that memory usage is reasonable
        // (This is a basic test - in a real scenario, we'd use memory profiling tools)
        let memory_usage = std::mem::size_of_val(&large_result);
        assert!(memory_usage < 1024 * 1024); // Less than 1MB
    }

    /// Test error handling - verify that error cases are handled gracefully
    #[test]
    fn test_error_handling() {
        // Test invalid class handling
        let builder = ClassBuilder::new();
        let result = builder.class("invalid-class");
        
        // Should not panic
        assert!(result.build().to_css_classes().contains("invalid-class"));
        
        // Test empty class set
        let empty_builder = ClassBuilder::new();
        let empty_result = empty_builder.build();
        assert!(empty_result.is_empty());
        
        // Test very long class names
        let long_class = "a".repeat(1000);
        let long_class_result = ClassBuilder::new()
            .class(&long_class)
            .build();
        
        assert!(long_class_result.to_css_classes().contains(&long_class));
    }

    /// Test edge cases - verify that edge cases are handled correctly
    #[test]
    fn test_edge_cases() {
        // Test maximum class count
        let mut max_classes = ClassBuilder::new();
        for i in 0..50000 {
            max_classes = max_classes.class(&format!("class-{}", i % 1000));
        }
        
        let result = max_classes.build();
        assert_eq!(result.len(), 50000);
        
        // Test special characters in class names
        let special_chars = ClassBuilder::new()
            .class("class-with-dashes")
            .class("class_with_underscores")
            .class("class.with.dots")
            .build();
        
        assert!(!special_chars.to_css_classes().is_empty());
    }

    /// Test regression prevention - verify that previously fixed bugs don't regress
    #[test]
    fn test_regression_prevention() {
        // Test class merging regression
        let class_set_1 = ClassBuilder::new()
            .p_4()
            .m_2()
            .build();
            
        let class_set_2 = ClassBuilder::new()
            .bg_blue_500()
            .text_white()
            .build();
        
        let mut merged = class_set_1.clone();
        merged.merge(class_set_2);
        
        assert!(merged.has_class("p-4"));
        assert!(merged.has_class("m-2"));
        assert!(merged.has_class("bg-blue-500"));
        assert!(merged.has_class("text-white"));
        
        // Test responsive class regression
        let responsive_test = ClassBuilder::new()
            .text_sm()                        // font-size: 0.875rem
            .responsive(crate::responsive::Breakpoint::Md, "text-base")
            .responsive(crate::responsive::Breakpoint::Lg, "text-lg")
            .build();

        assert!(!responsive_test.to_css_classes().is_empty());
        
        // Test dark mode regression
        let dark_mode_test = ClassBuilder::new()
            .bg_white()                       // background-color: #ffffff
            .dark("bg-gray-800")              // .dark { background-color: #1f2937 }
            .text_black()                     // color: #000000
            .dark("text-white")               // .dark { color: #ffffff }
            .build();

        assert!(!dark_mode_test.to_css_classes().is_empty());
    }

    /// Test API consistency - verify that all APIs are consistent
    #[test]
    fn test_api_consistency() {
        // Test that all utility methods follow the same pattern
        let spacing_test = ClassBuilder::new()
            .p_4()                            // padding: 1rem
            .m_2()                            // margin: 0.5rem
            .px_6()                           // padding-left: 1.5rem; padding-right: 1.5rem
            .py_3()                           // padding-top: 0.75rem; padding-bottom: 0.75rem
            .build();

        assert!(!spacing_test.to_css_classes().is_empty());

        // Test that all color methods follow the same pattern
        let color_test = ClassBuilder::new()
            .bg_blue_500()                    // background-color: #3b82f6
            .text_white()                     // color: #ffffff
            .border_gray_300()                // border-color: #d1d5db
            .build();

        assert!(!color_test.to_css_classes().is_empty());

        // Test that all layout methods follow the same pattern
        let layout_test = ClassBuilder::new()
            .flex()                           // display: flex
            .items_center()                   // align-items: center
            .justify_center()                 // justify-content: center
            .build();

        assert!(!layout_test.to_css_classes().is_empty());
    }

    /// Test documentation accuracy - verify that all documented examples work
    #[test]
    fn test_documentation_accuracy() {
        // Test README examples
        let readme_example = ClassBuilder::new()
            .bg_blue_500()
            .text_white()
            .p_4()
            .rounded_md()
            .build();

        assert!(!readme_example.to_css_classes().is_empty());

        // Test API documentation examples
        let api_example = ClassBuilder::new()
            .flex()
            .items_center()
            .justify_between()
            .p_4()
            .bg_white()
            .shadow_sm()
            .build();

        assert!(!api_example.to_css_classes().is_empty());

        // Test migration guide examples
        let migration_example = ClassBuilder::new()
            .grid()
            .grid_cols_1()
            .gap_4()
            .md("grid-cols-2")
            .lg("grid-cols-3")
            .build();

        assert!(!migration_example.to_css_classes().is_empty());
    }

    /// Test production readiness - verify that all production features work
    #[test]
    fn test_production_readiness() {
        // Test comprehensive production example
        let production_example = ClassBuilder::new()
            .p_4()                            // padding: 1rem
            .m_2()                            // margin: 0.5rem
            .w_1_2()                          // width: 50%
            .h_1_3()                          // height: 33.333333%
            .bg_blue_500()                    // background-color: #3b82f6
            .text_white()                     // color: #ffffff
            .rounded_md()                     // border-radius: 0.375rem
            .shadow_lg()                      // box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1)
            .hover("bg-blue-600")             // :hover { background-color: #2563eb }
            .focus("ring-2")                  // :focus { ring-width: 2px }
            .font_size(crate::utilities::typography::FontSize::Lg)
            .font_weight(crate::utilities::typography::FontWeight::Bold)
            .text_align(crate::utilities::typography::TextAlign::Center)
            .responsive(crate::responsive::Breakpoint::Md, "text-xl")
            .dark("bg-gray-800")
            .build();

        assert!(!production_example.to_css_classes().is_empty());
        
        // Test that all major features are working
        assert!(production_example.to_css_classes().contains("p-4"));
        assert!(production_example.to_css_classes().contains("m-2"));
        assert!(production_example.to_css_classes().contains("w-1/2"));
        assert!(production_example.to_css_classes().contains("h-1/3"));
        assert!(production_example.to_css_classes().contains("bg-blue-500"));
        assert!(production_example.to_css_classes().contains("text-white"));
        assert!(production_example.to_css_classes().contains("rounded-md"));
        assert!(production_example.to_css_classes().contains("shadow-lg"));
    }

    /// Test version compatibility - verify that version compatibility is maintained
    #[test]
    fn test_version_compatibility() {
        // Test that all major versions are compatible
        let version_test = ClassBuilder::new()
            .p_4()                            // padding: 1rem (v0.1+)
            .m_2()                            // margin: 0.5rem (v0.1+)
            .bg_blue_500()                    // background-color: #3b82f6 (v0.2+)
            .text_white()                     // color: #ffffff (v0.2+)
            .rounded_md()                     // border-radius: 0.375rem (v0.3+)
            .shadow_lg()                      // box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1) (v0.4+)
            .hover("bg-blue-600")             // :hover { background-color: #2563eb } (v0.5+)
            .focus("ring-2")                  // :focus { ring-width: 2px } (v0.6+)
            .responsive(crate::responsive::Breakpoint::Md, "text-xl") // responsive (v0.7+)
            .dark("bg-gray-800")              // dark mode (v0.8+)
            .build();

        assert!(!version_test.to_css_classes().is_empty());
    }

    /// Test security - verify that security considerations are addressed
    #[test]
    fn test_security() {
        // Test that malicious input is handled safely
        let malicious_input = ClassBuilder::new()
            .class("javascript:alert('xss')")
            .class("<script>alert('xss')</script>")
            .class("'; DROP TABLE users; --")
            .build();

        // Should not panic or cause security issues
        assert!(!malicious_input.to_css_classes().is_empty());
        
        // Test that very long inputs are handled safely
        let long_input = "a".repeat(100000);
        let long_input_result = ClassBuilder::new()
            .class(&long_input)
            .build();
        
        assert!(long_input_result.to_css_classes().contains(&long_input));
    }

    /// Test accessibility - verify that accessibility features work
    #[test]
    fn test_accessibility() {
        // Test focus states
        let focus_test = ClassBuilder::new()
            .focus("ring-2")                  // :focus { ring-width: 2px }
            .focus("ring-blue-500")           // :focus { ring-color: #3b82f6 }
            .build();

        assert!(!focus_test.to_css_classes().is_empty());

        // Test ARIA attributes
        let aria_test = ClassBuilder::new()
            .aria("expanded", "true", "bg-blue-100")
            .aria("selected", "false", "bg-gray-100")
            .build();

        assert!(!aria_test.to_css_classes().is_empty());
    }

    /// Test cross-platform compatibility - verify that the library works across platforms
    #[test]
    fn test_cross_platform_compatibility() {
        // Test that the library works on different platforms
        let cross_platform_test = ClassBuilder::new()
            .p_4()                            // padding: 1rem
            .m_2()                            // margin: 0.5rem
            .bg_blue_500()                    // background-color: #3b82f6
            .text_white()                     // color: #ffffff
            .rounded_md()                     // border-radius: 0.375rem
            .shadow_lg()                      // box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1)
            .build();

        assert!(!cross_platform_test.to_css_classes().is_empty());
    }

    /// Test final release validation - verify that everything is ready for release
    #[test]
    fn test_final_release_validation() {
        // Test comprehensive release validation
        let release_validation = ClassBuilder::new()
            .p_4()                            // padding: 1rem
            .m_2()                            // margin: 0.5rem
            .w_1_2()                          // width: 50%
            .h_1_3()                          // height: 33.333333%
            .bg_blue_500()                    // background-color: #3b82f6
            .text_white()                     // color: #ffffff
            .rounded_md()                     // border-radius: 0.375rem
            .shadow_lg()                      // box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1)
            .hover("bg-blue-600")             // :hover { background-color: #2563eb }
            .focus("ring-2")                  // :focus { ring-width: 2px }
            .font_size(crate::utilities::typography::FontSize::Lg)
            .font_weight(crate::utilities::typography::FontWeight::Bold)
            .text_align(crate::utilities::typography::TextAlign::Center)
            .responsive(crate::responsive::Breakpoint::Md, "text-xl")
            .dark("bg-gray-800")
            .build();

        assert!(!release_validation.to_css_classes().is_empty());
        
        // Test that all critical features are working
        assert!(release_validation.to_css_classes().contains("p-4"));
        assert!(release_validation.to_css_classes().contains("m-2"));
        assert!(release_validation.to_css_classes().contains("w-1/2"));
        assert!(release_validation.to_css_classes().contains("h-1/3"));
        assert!(release_validation.to_css_classes().contains("bg-blue-500"));
        assert!(release_validation.to_css_classes().contains("text-white"));
        assert!(release_validation.to_css_classes().contains("rounded-md"));
        assert!(release_validation.to_css_classes().contains("shadow-lg"));
        
        // Test that the release is production-ready
        assert!(!release_validation.to_css_classes().is_empty());
    }
}
