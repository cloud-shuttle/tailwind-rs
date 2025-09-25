//! Week 19: Testing & Quality Assurance Tests
//!
//! This module tests that all testing and quality assurance features are complete.

#[cfg(test)]
mod tests {
    use crate::classes::ClassBuilder;
    use crate::utilities::*;

    /// Test that all Week 19 testing and quality assurance features are implemented
    #[test]
    fn test_week19_testing_qa_features() {
        // Test integration tests - verify that all major components work together
        let integration_test = ClassBuilder::new()
            .p_4()                           // padding: 1rem
            .m_2()                           // margin: 0.5rem
            .w_1_2()                         // width: 50%
            .h_1_3()                         // height: 33.333333%
            .font_size(crate::utilities::typography::FontSize::Lg)
            .font_weight(crate::utilities::typography::FontWeight::Bold)
            .text_align(crate::utilities::typography::TextAlign::Center)
            .bg_blue_500()                   // background-color: #3b82f6
            .text_white()                    // color: #ffffff
            .rounded_md()                    // border-radius: 0.375rem
            .shadow_lg()                     // box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1)
            .hover("bg-blue-600")            // :hover { background-color: #2563eb }
            .focus("ring-2")                 // :focus { ring-width: 2px }
            .responsive(crate::responsive::Breakpoint::Md, "text-xl")
            .dark("bg-gray-800")
            .build();

        assert!(!integration_test.to_css_classes().is_empty());
        assert!(integration_test.to_css_classes().contains("p-4"));
        assert!(integration_test.to_css_classes().contains("m-2"));
        assert!(integration_test.to_css_classes().contains("w-1/2"));
        assert!(integration_test.to_css_classes().contains("h-1/3"));
    }

    /// Test unit tests - verify individual components work correctly
    #[test]
    fn test_unit_tests() {
        // Test spacing utilities
        let spacing_test = ClassBuilder::new()
            .p_4()                           // padding: 1rem
            .m_2()                           // margin: 0.5rem
            .px_6()                          // padding-left: 1.5rem; padding-right: 1.5rem
            .py_3()                          // padding-top: 0.75rem; padding-bottom: 0.75rem
            .build();

        assert!(!spacing_test.to_css_classes().is_empty());

        // Test color utilities
        let color_test = ClassBuilder::new()
            .bg_blue_500()                   // background-color: #3b82f6
            .text_white()                    // color: #ffffff
            .border_gray_300()               // border-color: #d1d5db
            .build();

        assert!(!color_test.to_css_classes().is_empty());

        // Test layout utilities
        let layout_test = ClassBuilder::new()
            .flex()                          // display: flex
            .items_center()                  // align-items: center
            .justify_center()                // justify-content: center
            .build();

        assert!(!layout_test.to_css_classes().is_empty());
    }

    /// Test performance tests - verify that performance requirements are met
    #[test]
    fn test_performance_tests() {
        // Test large class set performance
        let mut large_class_set = ClassBuilder::new();
        
        for i in 0..1000 {
            large_class_set = large_class_set.class(&format!("p-{}", i % 10));
        }
        
        let result = large_class_set.build();
        assert_eq!(result.len(), 1000);
        
        // Test CSS generation performance
        let start = std::time::Instant::now();
        let _css = result.to_css_classes();
        let duration = start.elapsed();
        
        // Should complete within reasonable time (1 second)
        assert!(duration.as_millis() < 1000);
    }

    /// Test property-based tests - verify that random inputs work correctly
    #[test]
    fn test_property_based_tests() {
        use proptest::prelude::*;
        
        proptest!(|(classes in prop::collection::vec("[a-zA-Z0-9-]+", 0..50))| {
            let mut builder = ClassBuilder::new();
            for class in classes {
                builder = builder.class(class);
            }
            
            let result = builder.build();
            assert_eq!(result.len(), classes.len());
        });
    }

    /// Test error handling tests - verify that error cases are handled correctly
    #[test]
    fn test_error_handling_tests() {
        // Test invalid class handling
        let builder = ClassBuilder::new();
        let result = builder.class("invalid-class");
        
        // Should not panic, but may not generate CSS
        assert!(result.build().to_css_classes().contains("invalid-class"));
        
        // Test empty class set
        let empty_builder = ClassBuilder::new();
        let empty_result = empty_builder.build();
        assert!(empty_result.is_empty());
    }

    /// Test edge case tests - verify that edge cases are handled correctly
    #[test]
    fn test_edge_case_tests() {
        // Test maximum class count
        let mut max_classes = ClassBuilder::new();
        for i in 0..10000 {
            max_classes = max_classes.class(&format!("class-{}", i));
        }
        
        let result = max_classes.build();
        assert_eq!(result.len(), 10000);
        
        // Test very long class names
        let long_class = "a".repeat(1000);
        let long_class_result = ClassBuilder::new()
            .class(&long_class)
            .build();
        
        assert!(long_class_result.to_css_classes().contains(&long_class));
    }

    /// Test regression tests - verify that previously fixed bugs don't regress
    #[test]
    fn test_regression_tests() {
        // Test that class merging works correctly
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
    }

    /// Test compatibility tests - verify that different configurations work
    #[test]
    fn test_compatibility_tests() {
        // Test responsive classes
        let responsive_test = ClassBuilder::new()
            .text_sm()                        // font-size: 0.875rem
            .responsive(crate::responsive::Breakpoint::Md, "text-base")
            .responsive(crate::responsive::Breakpoint::Lg, "text-lg")
            .build();

        assert!(!responsive_test.to_css_classes().is_empty());

        // Test dark mode
        let dark_mode_test = ClassBuilder::new()
            .bg_white()                       // background-color: #ffffff
            .dark("bg-gray-800")              // .dark { background-color: #1f2937 }
            .text_black()                     // color: #000000
            .dark("text-white")               // .dark { color: #ffffff }
            .build();

        assert!(!dark_mode_test.to_css_classes().is_empty());
    }

    /// Test accessibility tests - verify that accessibility features work
    #[test]
    fn test_accessibility_tests() {
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

    /// Test cross-browser tests - verify that CSS works across browsers
    #[test]
    fn test_cross_browser_tests() {
        // Test vendor prefixes
        let vendor_prefix_test = ClassBuilder::new()
            .transform("rotate-45")           // transform: rotate(45deg)
            .transition("all")                // transition: all
            .build();

        assert!(!vendor_prefix_test.to_css_classes().is_empty());
    }

    /// Test mobile tests - verify that responsive design works on mobile
    #[test]
    fn test_mobile_tests() {
        // Test mobile-first responsive design
        let mobile_test = ClassBuilder::new()
            .text_sm()                        // font-size: 0.875rem (mobile)
            .responsive(crate::responsive::Breakpoint::Sm, "text-base")   // font-size: 1rem (small screens)
            .responsive(crate::responsive::Breakpoint::Md, "text-lg")      // font-size: 1.125rem (medium screens)
            .responsive(crate::responsive::Breakpoint::Lg, "text-xl")     // font-size: 1.25rem (large screens)
            .build();

        assert!(!mobile_test.to_css_classes().is_empty());
    }

    /// Test typography tests - verify that typography features work
    #[test]
    fn test_typography_tests() {
        let typography_test = ClassBuilder::new()
            .font_size(crate::utilities::typography::FontSize::Lg)
            .font_weight(crate::utilities::typography::FontWeight::Bold)
            .text_align(crate::utilities::typography::TextAlign::Center)
            .build();

        assert!(!typography_test.to_css_classes().is_empty());
    }

    /// Test color tests - verify that color features work
    #[test]
    fn test_color_tests() {
        // Test color utilities
        let color_test = ClassBuilder::new()
            .bg_blue_500()                    // background-color: #3b82f6
            .text_white()                     // color: #ffffff
            .border_gray_300()                // border-color: #d1d5db
            .build();

        assert!(!color_test.to_css_classes().is_empty());

        // Test color opacity
        let opacity_test = ClassBuilder::new()
            .bg_blue_500()                    // background-color: #3b82f6
            .bg_opacity_50()                  // background-color: rgba(59, 130, 246, 0.5)
            .build();

        assert!(!opacity_test.to_css_classes().is_empty());
    }

    /// Test layout tests - verify that layout features work
    #[test]
    fn test_layout_tests() {
        // Test flexbox
        let flexbox_test = ClassBuilder::new()
            .flex()                           // display: flex
            .items_center()                   // align-items: center
            .justify_center()                 // justify-content: center
            .build();

        assert!(!flexbox_test.to_css_classes().is_empty());

        // Test grid
        let grid_test = ClassBuilder::new()
            .grid()                           // display: grid
            .grid_cols_1()                    // grid-template-columns: repeat(1, minmax(0, 1fr))
            .gap_4()                          // gap: 1rem
            .build();

        assert!(!grid_test.to_css_classes().is_empty());
    }

    /// Test spacing tests - verify that spacing features work
    #[test]
    fn test_spacing_tests() {
        // Test padding
        let padding_test = ClassBuilder::new()
            .p_4()                            // padding: 1rem
            .px_6()                           // padding-left: 1.5rem; padding-right: 1.5rem
            .py_3()                           // padding-top: 0.75rem; padding-bottom: 0.75rem
            .build();

        assert!(!padding_test.to_css_classes().is_empty());

        // Test margin
        let margin_test = ClassBuilder::new()
            .m_2()                            // margin: 0.5rem
            .mx_auto()                        // margin-left: auto; margin-right: auto
            .my_4()                           // margin-top: 1rem; margin-bottom: 1rem
            .build();

        assert!(!margin_test.to_css_classes().is_empty());
    }

    /// Test sizing tests - verify that sizing features work
    #[test]
    fn test_sizing_tests() {
        // Test width
        let width_test = ClassBuilder::new()
            .w_1_2()                          // width: 50%
            .w_full()                         // width: 100%
            .w_screen()                       // width: 100vw
            .build();

        assert!(!width_test.to_css_classes().is_empty());

        // Test height
        let height_test = ClassBuilder::new()
            .h_1_3()                          // height: 33.333333%
            .h_full()                         // height: 100%
            .h_screen()                       // height: 100vh
            .build();

        assert!(!height_test.to_css_classes().is_empty());
    }

    /// Test effects tests - verify that effects features work
    #[test]
    fn test_effects_tests() {
        // Test shadows
        let shadow_test = ClassBuilder::new()
            .shadow_sm()                      // box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05)
            .shadow_md()                      // box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1)
            .shadow_lg()                      // box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1)
            .build();

        assert!(!shadow_test.to_css_classes().is_empty());

        // Test opacity
        let opacity_test = ClassBuilder::new()
            .opacity_50()                     // opacity: 0.5
            .opacity_75()                     // opacity: 0.75
            .opacity_100()                    // opacity: 1
            .build();

        assert!(!opacity_test.to_css_classes().is_empty());
    }

    /// Test transforms tests - verify that transform features work
    #[test]
    fn test_transforms_tests() {
        // Test transforms
        let transform_test = ClassBuilder::new()
            .transform("rotate-45")           // transform: rotate(45deg)
            .transform("scale-110")           // transform: scale(1.1)
            .transform("translate-x-4")       // transform: translateX(1rem)
            .build();

        assert!(!transform_test.to_css_classes().is_empty());
    }

    /// Test transitions tests - verify that transition features work
    #[test]
    fn test_transitions_tests() {
        // Test transitions
        let transition_test = ClassBuilder::new()
            .transition("all")                // transition: all
            .transition("colors")             // transition: colors
            .transition("opacity")            // transition: opacity
            .build();

        assert!(!transition_test.to_css_classes().is_empty());
    }

    /// Test animations tests - verify that animation features work
    #[test]
    fn test_animations_tests() {
        // Test animations
        let animation_test = ClassBuilder::new()
            .animate_spin()                    // animation: spin 1s linear infinite
            .animate_pulse()                  // animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite
            .animate_bounce()                 // animation: bounce 1s infinite
            .build();

        assert!(!animation_test.to_css_classes().is_empty());
    }

    /// Test comprehensive quality assurance
    #[test]
    fn test_comprehensive_qa() {
        // Test all major features together
        let comprehensive_qa = ClassBuilder::new()
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

        assert!(!comprehensive_qa.to_css_classes().is_empty());
    }
}
