extern crate tailwind_rs_core;

use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    let generator = CssGenerator::new();
    let test_classes = vec![
        "bg-gradient-to-r",
        "bg-gradient-to-br"
    ];
    
    for class in test_classes {
        match generator.class_to_css_rule(class) {
            Ok(rule) => {
                println!("✅ {} -> {} properties", class, rule.properties.len());
                for prop in &rule.properties {
                    println!("  {}: {}", prop.name, prop.value);
                }
                // Check for CSS variable issues
                for prop in &rule.properties {
                    if prop.value.contains("var(--") {
                        println!("  ⚠️  CSS variable found: {}", prop.value);
                    }
                }
            }
            Err(e) => println!("❌ {} -> {}", class, e),
        }
        println!();
    }
}
