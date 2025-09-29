#[cfg(test)]
mod tests {
    use crate::css_generator::CssGenerator;

    #[test]
    fn test_advanced_parser_functionality() {
        let mut generator = CssGenerator::new();

        // Test individual classes
        let test_classes = vec![
            "perspective-1000",
            "rotate-x-12",
            "animate-float",
            "bg-gradient-conic",
            "from-50%",
        ];

        println!("Testing individual advanced classes:");
        for class in &test_classes {
            match generator.add_class(class) {
                Ok(_) => println!("✅ {} - PARSED", class),
                Err(e) => println!("❌ {} - ERROR: {}", class, e),
            }
        }

        let css = generator.generate_css();
        println!("\nGenerated CSS length: {} characters", css.len());

        // Check for specific features
        println!("\nChecking for advanced features:");
        println!("perspective: {}", css.contains("perspective:"));
        println!("rotateX: {}", css.contains("rotateX"));
        println!("float animation: {}", css.contains("float 3s"));
        println!("conic-gradient: {}", css.contains("conic-gradient"));
        println!("50%: {}", css.contains("50%"));

        // Assertions
        assert!(css.contains("perspective:"), "perspective-1000 should generate perspective property");
        assert!(css.contains("rotateX"), "rotate-x-12 should generate rotateX transform");
        assert!(css.contains("float 3s"), "animate-float should work");
        assert!(css.contains("conic-gradient"), "bg-gradient-conic should work");
    }
}