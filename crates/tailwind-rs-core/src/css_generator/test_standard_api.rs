//! Test for the standard API to verify "One Class = One CSS Rule" architecture

#[cfg(test)]
mod tests {
    use super::CssGenerator;

    #[test]
    fn test_standard_api_one_class_one_rule() {
        // Create a new generator using the standard API
        let mut generator = CssGenerator::new();

        // Add several different classes that should generate separate CSS rules
        let classes = vec![
            "bg-gradient-to-r", // Gradient direction
            "from-blue-400",    // Gradient stop
            "to-purple-600",    // Gradient stop
            "via-pink-500",     // Gradient stop
            "text-center",      // Regular utility
            "hover:bg-red-500", // Hover variant
            "md:p-6",           // Responsive variant
            "animate-pulse",    // Animation
        ];

        // Add all classes
        for class in &classes {
            generator.add_class(class).unwrap();
        }

        // Generate CSS
        let css = generator.generate_css();

        // Verify that each class generates its own rule (not accumulated properties)
        assert!(css.contains(".bg-gradient-to-r"), "Missing bg-gradient-to-r rule");
        assert!(css.contains(".from-blue-400"), "Missing from-blue-400 rule");
        assert!(css.contains(".to-purple-600"), "Missing to-purple-600 rule");
        assert!(css.contains(".via-pink-500"), "Missing via-pink-500 rule");
        assert!(css.contains(".text-center"), "Missing text-center rule");
        assert!(css.contains(".hover\\:bg-red-500"), "Missing hover:bg-red-500 rule");
        assert!(css.contains(".md\\:p-6"), "Missing md:p-6 rule");
        assert!(css.contains(".animate-pulse"), "Missing animate-pulse rule");

        // Critical test: Verify properties are NOT mixed between classes
        // Check that gradient properties don't appear on text-center class
        let text_center_rule = css.lines()
            .skip_while(|line| !line.contains(".text-center"))
            .take_while(|line| !line.is_empty() && !line.contains("}"))
            .collect::<Vec<_>>()
            .join("\n");

        assert!(!text_center_rule.contains("--tw-gradient"),
                "FAILURE: Gradient properties found on text-center class (accumulation bug present)");

        // Check that animation properties don't appear on gradient classes
        let gradient_rule = css.lines()
            .skip_while(|line| !line.contains(".bg-gradient-to-r"))
            .take_while(|line| !line.is_empty() && !line.contains("}"))
            .collect::<Vec<_>>()
            .join("\n");

        assert!(!gradient_rule.contains("animation"),
                "FAILURE: Animation properties found on gradient class (accumulation bug present)");

        // Verify gradient variables are properly generated
        assert!(css.contains("--tw-gradient-from"), "Missing gradient-from variable");
        assert!(css.contains("--tw-gradient-via"), "Missing gradient-via variable");
        assert!(css.contains("--tw-gradient-to"), "Missing gradient-to variable");

        // Verify variant selectors are correct
        assert!(css.contains(".hover\\:bg-red-500:hover"), "Incorrect hover selector");
        assert!(css.contains("@media (min-width: 768px)"), "Missing responsive media query");
        assert!(css.contains(".md\\:p-6"), "Missing responsive rule");

        // Verify we have the expected number of rules
        assert_eq!(generator.rule_count(), classes.len(),
                  "Expected {} rules, got {}", classes.len(), generator.rule_count());
    }

    #[test]
    fn test_gradient_classes_work_together() {
        let mut generator = CssGenerator::new();

        // Add gradient classes in sequence
        generator.add_class("bg-gradient-to-r").unwrap();
        generator.add_class("from-blue-400").unwrap();
        generator.add_class("to-purple-600").unwrap();

        let css = generator.generate_css();

        // Verify all three rules exist
        assert!(css.contains(".bg-gradient-to-r"));
        assert!(css.contains(".from-blue-400"));
        assert!(css.contains(".to-purple-600"));

        // Verify gradient variables are set correctly
        assert!(css.contains("--tw-gradient-from"));
        assert!(css.contains("--tw-gradient-to"));
        assert!(css.contains("background-image: linear-gradient(to right"));

        // Verify the gradient rule uses the variables
        assert!(css.contains("var(--tw-gradient-from)"));
        assert!(css.contains("var(--tw-gradient-to)"));
    }

    #[test]
    fn test_css_selector_escaping() {
        let mut generator = CssGenerator::new();

        // Test that classes with slashes are properly escaped in selectors
        generator.add_class("border-gray-700/30").unwrap();
        generator.add_class("dark:border-gray-700/30").unwrap();
        generator.add_class("hover:border-gray-700/30").unwrap();

        let css = generator.generate_css();

        // Should have properly escaped selectors
        assert!(css.contains(".border-gray-700\\/30 {"));
        assert!(css.contains(".dark\\:border-gray-700\\/30 {"));
        assert!(css.contains(".hover\\:border-gray-700\\/30:hover {"));

        // Should NOT have unescaped slashes in selectors (regression test)
        assert!(!css.contains(".border-gray-700/30 {"));
        assert!(!css.contains(".dark:border-gray-700/30 {"));
        assert!(!css.contains(".hover:border-gray-700/30:hover {"));
    }
}
