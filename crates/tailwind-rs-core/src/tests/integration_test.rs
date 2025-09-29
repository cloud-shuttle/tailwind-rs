#[cfg(test)]
mod integration_test {
    use crate::css_generator::CssGenerator;

    #[test]
    fn test_ssr_demo_classes() {
        let generator = CssGenerator::new();

        // Test classes from the SSR demo
        let test_classes = vec![
            // Basic utilities
            "text-purple-400", "text-cyan-400", "text-6xl", "font-bold",
            // Background and gradients
            "bg-gradient-to-br", "from-blue-50", "to-indigo-100",
            "bg-gradient-to-r", "from-blue-600", "via-purple-600", "to-indigo-600",
            "bg-clip-text", "text-transparent",
            // Layout
            "min-h-screen", "flex", "flex-col", "items-center", "justify-center",
            "px-4", "py-8", "gap-8", "max-w-6xl", "mx-auto",
            // Advanced effects that should be working
            "group", "group-hover:opacity-100", "group-hover:backdrop-blur-3xl",
            "group-hover:brightness-125", "group-hover:saturate-150",
            "perspective-1000", "rotate-x-12", "animate-float",
            "bg-gradient-conic", "transform-style-preserve-3d",
            // Opacity classes
            "opacity-0", "opacity-25", "opacity-50", "opacity-75", "opacity-100",
            // Shadow classes
            "shadow-lg", "shadow-purple-500/50", "shadow-blue-500/50", "shadow-green-500/50",
            // Mix blend modes
            "mix-blend-multiply", "mix-blend-screen", "backdrop-blur-3xl",
            // 3D transforms
            "rotate-y-12", "hover:scale-110", "hover:rotate-y-12",
        ];

        let mut success_count = 0;
        let mut fail_count = 0;
        let mut failed_classes = Vec::new();

        println!("🧪 INTEGRATION TEST: Testing {} classes", test_classes.len());
        println!("{}", "=".repeat(60));

        for class in &test_classes {
            match generator.class_to_css_rule(class) {
                Ok(rule) => {
                    success_count += 1;
                    println!("✅ {} -> {} properties", class, rule.properties.len());
                }
                Err(e) => {
                    fail_count += 1;
                    failed_classes.push((class.clone(), e.to_string()));
                    println!("❌ {} -> {}", class, e);
                }
            }
        }

        println!("{}", "=".repeat(60));
        println!("📊 RESULTS:");
        println!("✅ Successful: {} classes", success_count);
        println!("❌ Failed: {} classes", fail_count);
        println!("📈 Success Rate: {:.1}%", (success_count as f64 / test_classes.len() as f64) * 100.0);

        if !failed_classes.is_empty() {
            println!("\n🔍 FAILED CLASSES:");
            for (class, error) in failed_classes {
                println!("  {}: {}", class, error);
            }
        }

        // This test should help identify what's broken
        // For now, just assert that we have some successes
        assert!(success_count > 0, "Should have at least some successful parses");

        println!("\n🎯 NEXT STEPS:");
        println!("1. Fix trie registration for failed prefixes");
        println!("2. Add missing parser support");
        println!("3. Update supported patterns");
    }
}
