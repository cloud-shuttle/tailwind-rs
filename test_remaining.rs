extern crate tailwind_rs_core;

use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    let generator = CssGenerator::new();
    let test_classes = vec![
        "hover:scale-105",
        "hover:scale-110", 
        "drop-shadow-lg",
        "drop-shadow-xl",
        "drop-shadow-2xl"
    ];
    
    for class in test_classes {
        match generator.class_to_css_rule(class) {
            Ok(rule) => {
                println!("✅ {} -> {} properties", class, rule.properties.len());
                for prop in &rule.properties {
                    println!("  {}: {}", prop.name, prop.value);
                }
            }
            Err(e) => println!("❌ {} -> {}", class, e),
        }
        println!();
    }
}
