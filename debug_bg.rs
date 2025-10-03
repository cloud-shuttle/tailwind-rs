extern crate tailwind_rs_core;

use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    let generator = CssGenerator::new();
    
    let test_class = "bg-white/10";
    println!("Testing class: {}", test_class);
    
    match generator.class_to_css_rule(test_class) {
        Ok(rule) => {
            println!("✅ SUCCESS - Generated {} properties:", rule.properties.len());
            for prop in &rule.properties {
                println!("  {}: {}", prop.name, prop.value);
            }
        }
        Err(e) => {
            println!("❌ FAILED - Error: {}", e);
        }
    }
}
