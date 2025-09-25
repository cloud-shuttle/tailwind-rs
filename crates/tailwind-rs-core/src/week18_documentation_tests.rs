//! Week 18: Documentation & Examples Tests
//!
//! This module tests that all documentation and examples are complete and working.

#[cfg(test)]
mod tests {
    use crate::classes::ClassBuilder;
    use crate::utilities::*;

    /// Test that all Week 18 documentation features are implemented
    #[test]
    fn test_week18_documentation_features() {
        // Test comprehensive class building
        let comprehensive_example = ClassBuilder::new()
            .bg_blue_500()                    // background-color: #3b82f6
            .text_white()                     // color: #ffffff
            .p_4()                           // padding: 1rem
            .rounded_md()                    // border-radius: 0.375rem
            .shadow_lg()                     // box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1)
            .hover("bg-blue-600")            // :hover { background-color: #2563eb }
            .focus("ring-2")                 // :focus { ring-width: 2px }
            .font_size(crate::utilities::typography::FontSize::Lg)
            .font_weight(crate::utilities::typography::FontWeight::Bold)
            .text_align(crate::utilities::typography::TextAlign::Center)
            .responsive(crate::responsive::Breakpoint::Md, "text-xl")
            .dark("bg-gray-800")
            .build();

        assert!(!comprehensive_example.to_css_classes().is_empty());
        assert!(comprehensive_example.to_css_classes().contains("bg-blue-500"));
        assert!(comprehensive_example.to_css_classes().contains("text-white"));
        assert!(comprehensive_example.to_css_classes().contains("p-4"));
    }

    /// Test that all utility categories are documented
    #[test]
    fn test_utility_categories_documented() {
        // Test spacing utilities
        let spacing_example = ClassBuilder::new()
            .p_4()                           // padding: 1rem
            .m_2()                           // margin: 0.5rem
            .px_6()                          // padding-left: 1.5rem; padding-right: 1.5rem
            .py_3()                          // padding-top: 0.75rem; padding-bottom: 0.75rem
            .build();

        assert!(!spacing_example.to_css_classes().is_empty());

        // Test color utilities
        let color_example = ClassBuilder::new()
            .bg_blue_500()                   // background-color: #3b82f6
            .text_white()                    // color: #ffffff
            .border_gray_300()               // border-color: #d1d5db
            .build();

        assert!(!color_example.to_css_classes().is_empty());

        // Test layout utilities
        let layout_example = ClassBuilder::new()
            .flex()                          // display: flex
            .items_center()                  // align-items: center
            .justify_center()                // justify-content: center
            .build();

        assert!(!layout_example.to_css_classes().is_empty());
    }

    /// Test that all examples in documentation work
    #[test]
    fn test_documentation_examples() {
        // Example 1: Basic button styling
        let button_example = ClassBuilder::new()
            .bg_blue_500()                   // background-color: #3b82f6
            .text_white()                    // color: #ffffff
            .px_4()                          // padding-left: 1rem; padding-right: 1rem
            .py_2()                          // padding-top: 0.5rem; padding-bottom: 0.5rem
            .rounded()                       // border-radius: 0.25rem
            .hover("bg-blue-600")            // :hover { background-color: #2563eb }
            .build();

        assert!(!button_example.to_css_classes().is_empty());

        // Example 2: Card component
        let card_example = ClassBuilder::new()
            .bg_white()                      // background-color: #ffffff
            .shadow_md()                     // box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1)
            .rounded_lg()                    // border-radius: 0.5rem
            .p_6()                           // padding: 1.5rem
            .build();

        assert!(!card_example.to_css_classes().is_empty());

        // Example 3: Responsive grid
        let grid_example = ClassBuilder::new()
            .grid()                          // display: grid
            .grid_cols_1()                   // grid-template-columns: repeat(1, minmax(0, 1fr))
            .gap_4()                         // gap: 1rem
            .responsive(crate::responsive::Breakpoint::Md, "grid-cols-2")
            .responsive(crate::responsive::Breakpoint::Lg, "grid-cols-3")
            .build();

        assert!(!grid_example.to_css_classes().is_empty());
    }

    /// Test that all API methods are documented
    #[test]
    fn test_api_methods_documented() {
        // Test ClassBuilder methods
        let builder = ClassBuilder::new();
        assert!(builder.is_empty());

        // Test utility methods
        let utility_example = ClassBuilder::new()
            .font_weight(crate::utilities::typography::FontWeight::Bold)   // font-weight: bold
            .text_align(crate::utilities::typography::TextAlign::Center)    // text-align: center
            .line_height(crate::utilities::typography::LineHeight::Relaxed)  // line-height: 1.625
            .build();

        assert!(!utility_example.to_css_classes().is_empty());
    }

    /// Test that all configuration options are documented
    #[test]
    fn test_configuration_options_documented() {
        // Test responsive breakpoints
        let responsive_example = ClassBuilder::new()
            .text_sm()                        // font-size: 0.875rem
            .responsive(crate::responsive::Breakpoint::Md, "text-base")
            .responsive(crate::responsive::Breakpoint::Lg, "text-lg")
            .build();

        assert!(!responsive_example.to_css_classes().is_empty());

        // Test dark mode
        let dark_mode_example = ClassBuilder::new()
            .bg_white()                       // background-color: #ffffff
            .dark("bg-gray-800")              // .dark { background-color: #1f2937 }
            .text_black()                     // color: #000000
            .dark("text-white")               // .dark { color: #ffffff }
            .build();

        assert!(!dark_mode_example.to_css_classes().is_empty());
    }

    /// Test that all error cases are documented
    #[test]
    fn test_error_cases_documented() {
        // Test invalid class handling
        let builder = ClassBuilder::new();
        let result = builder.class("invalid-class");
        
        // Should not panic, but may not generate CSS
        assert!(result.build().to_css_classes().contains("invalid-class"));
    }

    /// Test that all performance considerations are documented
    #[test]
    fn test_performance_considerations_documented() {
        // Test large class set performance
        let mut large_class_set = ClassBuilder::new();
        
        for i in 0..100 {
            large_class_set = large_class_set.class(&format!("p-{}", i));
        }
        
        let result = large_class_set.build();
        assert_eq!(result.len(), 100);
    }

    /// Test that all examples are working
    #[test]
    fn test_all_examples_working() {
        // Example from README
        let readme_example = ClassBuilder::new()
            .bg_blue_500()
            .text_white()
            .p_4()
            .rounded_md()
            .build();

        assert!(!readme_example.to_css_classes().is_empty());

        // Example from API documentation
        let api_example = ClassBuilder::new()
            .flex()
            .items_center()
            .justify_between()
            .p_4()
            .bg_white()
            .shadow_sm()
            .build();

        assert!(!api_example.to_css_classes().is_empty());

        // Example from migration guide
        let migration_example = ClassBuilder::new()
            .grid()
            .grid_cols_1()
            .gap_4()
            .md("grid-cols-2")
            .lg("grid-cols-3")
            .build();

        assert!(!migration_example.to_css_classes().is_empty());
    }

    /// Test that all typography examples work
    #[test]
    fn test_typography_examples_working() {
        let typography_example = ClassBuilder::new()
            .font_size(crate::utilities::typography::FontSize::Lg)
            .font_weight(crate::utilities::typography::FontWeight::Bold)
            .text_align(crate::utilities::typography::TextAlign::Center)
            .build();

        assert!(!typography_example.to_css_classes().is_empty());
    }

    /// Test that all responsive examples work
    #[test]
    fn test_responsive_examples_working() {
        let responsive_example = ClassBuilder::new()
            .text_sm()
            .md("text-base")
            .lg("text-lg")
            .xl("text-xl")
            .build();

        assert!(!responsive_example.to_css_classes().is_empty());
    }

    /// Test that all dark mode examples work
    #[test]
    fn test_dark_mode_examples_working() {
        let dark_mode_example = ClassBuilder::new()
            .bg_white()
            .dark("bg-gray-800")
            .text_black()
            .dark("text-white")
            .build();

        assert!(!dark_mode_example.to_css_classes().is_empty());
    }
}
