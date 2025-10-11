#[cfg(test)]
mod parser_registry_tests {
    use tailwind_rs_core::css_generator::core::operations::ParserRegistry;

    #[test]
    fn test_animate_pulse_parsing() {
        let registry = ParserRegistry::new();

        match registry.parse_class("animate-pulse") {
            Some(properties) => {
                assert_eq!(properties.len(), 1);
                assert_eq!(properties[0].name, "animation");
                assert!(properties[0].value.contains("pulse"));
                println!("✅ 'animate-pulse' parsed successfully");
            }
            None => {
                panic!("❌ 'animate-pulse' not parsed");
            }
        }
    }

    #[test]
    fn test_core_class_parsing() {
        let registry = ParserRegistry::new();

        let test_cases = vec![
            ("p-4", true),
            ("bg-blue-500", true),
            ("text-white", true),
            ("flex", true),
            ("animate-pulse", true),
            ("animate-spin", true),
            ("invalid-class", false),
        ];

        for (class, should_parse) in test_cases {
            let result = registry.parse_class(class);
            if should_parse {
                assert!(result.is_some(), "❌ {} should be parsed", class);
                println!("✅ {}: parsed", class);
            } else {
                assert!(result.is_none(), "❌ {} should not be parsed", class);
                println!("✅ {}: correctly not parsed", class);
            }
        }
    }
}
