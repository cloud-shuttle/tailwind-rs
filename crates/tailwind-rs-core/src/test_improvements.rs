//! Test improvements to verify the fixes work correctly

use crate::css_generator::{CssGenerator, CssGenerationConfig};
use crate::classes::{ClassSet, ClassBuilder};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hover_states_work() {
        let mut generator = CssGenerator::new();
        
        // Test hover states that were previously broken
        let hover_classes = vec![
            "hover:bg-zinc-700",
            "hover:text-teal-400", 
            "hover:shadow-xl",
            "hover:opacity-80",
        ];
        
        for class in hover_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add hover class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        
        // Verify hover states are in the CSS
        assert!(css.contains(":hover"), "CSS should contain hover selectors");
        assert!(css.contains("background-color"), "CSS should contain background-color properties");
    }

    #[test]
    fn test_dark_mode_works() {
        let mut generator = CssGenerator::new();
        
        // Test dark mode classes that were previously broken
        let dark_classes = vec![
            "dark:bg-zinc-800",
            "dark:text-zinc-200",
            "dark:border-zinc-700",
        ];
        
        for class in dark_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add dark mode class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        
        // Verify dark mode is in the CSS
        assert!(css.contains(".dark"), "CSS should contain dark mode selectors");
    }

    #[test]
    fn test_interactive_utilities_work() {
        let mut generator = CssGenerator::new();
        
        // Test interactive utilities that were previously broken
        let interactive_classes = vec![
            "pointer-events-none",
            "pointer-events-auto",
            "cursor-pointer",
            "select-none",
        ];
        
        for class in interactive_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add interactive class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        
        // Verify interactive properties are in the CSS
        assert!(css.contains("pointer-events"), "CSS should contain pointer-events");
        assert!(css.contains("cursor"), "CSS should contain cursor");
        assert!(css.contains("user-select"), "CSS should contain user-select");
    }

    #[test]
    fn test_advanced_utilities_work() {
        let mut generator = CssGenerator::new();
        
        // Test advanced utilities that were previously broken
        let advanced_classes = vec![
            "text-transparent",
            "backdrop-blur-sm",
            "backdrop-opacity-50",
            "shadow-lg",
            "opacity-75",
        ];
        
        for class in advanced_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add advanced class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        
        // Verify advanced properties are in the CSS
        assert!(css.contains("transparent"), "CSS should contain transparent");
        assert!(css.contains("backdrop-filter"), "CSS should contain backdrop-filter");
        assert!(css.contains("box-shadow"), "CSS should contain box-shadow");
        assert!(css.contains("opacity"), "CSS should contain opacity");
    }

    #[test]
    fn test_classset_api_works() {
        let mut class_set = ClassSet::new();
        class_set.add_class("bg-blue-500");
        class_set.add_class("text-white");
        class_set.add_class("p-4");
        
        // Test the to_css_classes method that was mentioned as missing
        let css_classes = class_set.to_css_classes();
        assert!(!css_classes.is_empty(), "to_css_classes should return non-empty string");
        assert!(css_classes.contains("bg-blue-500"), "Should contain bg-blue-500");
        assert!(css_classes.contains("text-white"), "Should contain text-white");
        assert!(css_classes.contains("p-4"), "Should contain p-4");
    }

    #[test]
    fn test_classbuilder_api_works() {
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
        
        // Test the comprehensive CSS generation
        let result = generator.generate_comprehensive_css(&config);
        assert!(result.is_ok(), "Comprehensive CSS generation should succeed");
        
        let css = result.unwrap();
        assert!(!css.is_empty(), "Comprehensive CSS should not be empty");
        assert!(css.len() > 100, "Comprehensive CSS should be substantial");
    }

    #[test]
    fn test_typography_parser_works() {
        let mut generator = CssGenerator::new();
        
        // Test typography classes that should now work
        let typography_classes = vec![
            "text-lg",
            "font-bold",
            "text-center",
            "text-transparent",
            "text-zinc-700",
        ];
        
        for class in typography_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add typography class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        
        // Verify typography properties are in the CSS
        assert!(css.contains("font-size"), "CSS should contain font-size");
        assert!(css.contains("font-weight"), "CSS should contain font-weight");
        assert!(css.contains("text-align"), "CSS should contain text-align");
    }

    #[test]
    fn test_layout_parser_works() {
        let mut generator = CssGenerator::new();
        
        // Test layout classes that should now work
        let layout_classes = vec![
            "block",
            "flex",
            "grid",
            "hidden",
            "relative",
            "absolute",
            "overflow-hidden",
        ];
        
        for class in layout_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add layout class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        
        // Verify layout properties are in the CSS
        assert!(css.contains("display"), "CSS should contain display");
        assert!(css.contains("position"), "CSS should contain position");
        assert!(css.contains("overflow"), "CSS should contain overflow");
    }

    #[test]
    fn test_effects_parser_works() {
        let mut generator = CssGenerator::new();
        
        // Test effects classes that should now work
        let effects_classes = vec![
            "shadow-lg",
            "opacity-50",
            "backdrop-blur-sm",
            "backdrop-opacity-75",
        ];
        
        for class in effects_classes {
            let result = generator.add_class(class);
            assert!(result.is_ok(), "Failed to add effects class: {}", class);
        }
        
        let css = generator.generate_css();
        assert!(!css.is_empty(), "Generated CSS should not be empty");
        
        // Verify effects properties are in the CSS
        assert!(css.contains("box-shadow"), "CSS should contain box-shadow");
        assert!(css.contains("opacity"), "CSS should contain opacity");
        assert!(css.contains("backdrop-filter"), "CSS should contain backdrop-filter");
    }
}
